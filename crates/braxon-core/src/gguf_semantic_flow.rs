use crate::{BusValue, HardwareWriteAck, KineticReflexor, ValueClass, Watermark};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::env;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};

pub const GGUF_SEMANTIC_FLOW_SCHEMA: &str = "braxon.nsq.gguf_semantic_flow.v1";
pub const GGUF_SEMANTIC_FLOW_CAPABILITY: &str = "feature:gguf.semantic_flow";
const CONTRACT_PATH: &str = "config/nsq/gguf_semantic_flow_contract.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GgufSemanticCell {
    pub cell: String,
    pub virtual_address: String,
    pub environment_variable: String,
    pub duty_cycle_slot: u64,
    pub value_sha256: String,
    pub environment_binding_verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GgufSemanticFlowReport {
    pub schema: String,
    pub capability: String,
    pub repository_relative_container: String,
    pub container_bytes: u64,
    pub header_sha256: String,
    pub format_version: u32,
    pub tensor_count: u64,
    pub metadata_count: u64,
    pub tensor_payload_loaded: bool,
    pub external_gguf_runtime_authority: bool,
    pub parameter_cells_persistently_live: bool,
    pub container_reloaded_during_piston_refresh: bool,
    pub stable_addresses: Vec<GgufSemanticCell>,
    pub state_watermark: Watermark,
    pub state_path: String,
    pub exact_next_action: String,
}

#[derive(Debug, Deserialize)]
struct Contract {
    schema: String,
    capability: String,
    allowed_container_magic: String,
    supported_versions: Vec<u32>,
    maximum_header_bytes: usize,
    storage_root: String,
    external_gguf_runtime_authority: bool,
    tensor_payload_loaded: bool,
    parameter_cells_persistently_live: bool,
    container_reloaded_during_piston_refresh: bool,
    stable_address_bindings: Vec<Binding>,
}

#[derive(Debug, Deserialize)]
struct Binding {
    cell: String,
    virtual_address: String,
    environment_variable: String,
    duty_cycle_slot: u64,
}

/// Extract the GGUF container header into stable NSQ/KSR semantic cells. This operation
/// reads exactly the fixed header and never loads tensor payloads or treats GGUF as runtime authority.
pub fn extract_gguf_semantic_flow(
    start: impl AsRef<Path>,
    repository_relative_container: impl AsRef<Path>,
) -> Result<GgufSemanticFlowReport, String> {
    let root = resolve_root(start.as_ref())?;
    let contract: Contract = read_json(&root.join(CONTRACT_PATH))?;
    if contract.schema != "braxon.nsq.gguf_semantic_flow_contract.v1"
        || contract.capability != GGUF_SEMANTIC_FLOW_CAPABILITY
        || contract.external_gguf_runtime_authority
        || contract.tensor_payload_loaded
        || !contract.parameter_cells_persistently_live
        || contract.container_reloaded_during_piston_refresh
        || contract.maximum_header_bytes != 24
    {
        return Err("GGUF semantic-flow contract weakens its no-payload persistent-cell boundary".into());
    }
    let relative = repository_relative_container.as_ref();
    if relative.is_absolute() {
        return Err("GGUF semantic-flow requires a repository-relative container path".into());
    }
    let root_canonical = root.canonicalize().map_err(|error| error.to_string())?;
    let container = root.join(relative).canonicalize().map_err(|error| {
        format!("GGUF semantic-flow cannot resolve declared container '{}': {error}", relative.display())
    })?;
    if !container.starts_with(&root_canonical) {
        return Err("GGUF semantic-flow rejected a container path outside the repository".into());
    }
    let mut header = [0u8; 24];
    File::open(&container)
        .map_err(|error| error.to_string())?
        .read_exact(&mut header)
        .map_err(|error| format!("GGUF semantic-flow failed to read fixed header: {error}"))?;
    if &header[..4] != contract.allowed_container_magic.as_bytes() {
        return Err("GGUF semantic-flow rejected non-GGUF container magic".into());
    }
    let format_version = u32::from_le_bytes(header[4..8].try_into().map_err(|_| "invalid GGUF version bytes")?);
    if !contract.supported_versions.contains(&format_version) {
        return Err(format!("GGUF semantic-flow rejected unsupported format version {format_version}"));
    }
    let tensor_count = u64::from_le_bytes(header[8..16].try_into().map_err(|_| "invalid GGUF tensor count bytes")?);
    let metadata_count = u64::from_le_bytes(header[16..24].try_into().map_err(|_| "invalid GGUF metadata count bytes")?);
    let container_bytes = fs::metadata(&container).map_err(|error| error.to_string())?.len();
    let header_sha256 = sha256(&header);
    let relative_container = display_relative(&root, &container)?;
    let values = [
        ("container_identity", format!("{}:{header_sha256}", relative_container)),
        ("format_version", format_version.to_string()),
        ("tensor_count", tensor_count.to_string()),
        ("metadata_count", metadata_count.to_string()),
        ("source_provenance", format!("{}:{container_bytes}", relative_container)),
        ("compatibility_boundary", "header_only_no_tensor_payload_no_external_runtime".to_string()),
    ];
    let mut cells = Vec::with_capacity(contract.stable_address_bindings.len());
    for binding in &contract.stable_address_bindings {
        let value = values.iter().find(|(cell, _)| *cell == binding.cell).map(|(_, value)| value)
            .ok_or_else(|| format!("GGUF contract binding references unknown cell '{}'", binding.cell))?;
        let environment_binding_verified = match env::var(&binding.environment_variable) {
            Ok(address) if address == binding.virtual_address => true,
            Ok(_) => return Err(format!("GGUF semantic-flow rejected environment address mismatch for {}", binding.environment_variable)),
            Err(env::VarError::NotPresent) => false,
            Err(error) => return Err(format!("GGUF semantic-flow could not read {}: {error}", binding.environment_variable)),
        };
        cells.push(GgufSemanticCell {
            cell: binding.cell.clone(),
            virtual_address: binding.virtual_address.clone(),
            environment_variable: binding.environment_variable.clone(),
            duty_cycle_slot: binding.duty_cycle_slot,
            value_sha256: sha256(value.as_bytes()),
            environment_binding_verified,
        });
    }
    if cells.iter().any(|cell| !cell.environment_binding_verified) {
        return Err("GGUF semantic-flow requires every declared stable-address environment binding before it can enter the persistent live bus".into());
    }
    let mut reflexor = KineticReflexor::new();
    reflexor.publish(cells.iter().map(|cell| BusValue {
        key: cell.virtual_address.clone(),
        class: ValueClass::Parameter,
        value_hash: cell.value_sha256.clone(),
        byte_len: 32,
    }))?;
    reflexor.reconcile()?;
    let generation = reflexor.generation();
    let written_keys = reflexor.pending_delta().iter().map(|delta| delta.key.clone()).collect();
    let state_watermark = reflexor.commit_hardware(HardwareWriteAck {
        adapter_id: "gguf_stable_address_live_bus_adapter".to_string(),
        generation,
        accepted: true,
        written_keys,
    })?.watermark;
    let state_path = root.join(&contract.storage_root).join("active_stable_cells.json");
    write_json(&state_path, &cells)?;
    Ok(GgufSemanticFlowReport {
        schema: GGUF_SEMANTIC_FLOW_SCHEMA.to_string(),
        capability: GGUF_SEMANTIC_FLOW_CAPABILITY.to_string(),
        repository_relative_container: relative_container,
        container_bytes,
        header_sha256,
        format_version,
        tensor_count,
        metadata_count,
        tensor_payload_loaded: false,
        external_gguf_runtime_authority: false,
        parameter_cells_persistently_live: true,
        container_reloaded_during_piston_refresh: false,
        stable_addresses: cells,
        state_watermark,
        state_path: display_relative(&root, &state_path)?,
        exact_next_action: "KSR has persisted header-derived semantic cells at their stable declared addresses. A persistent Android process is separately required to keep firing duty-cycle windows while no command is running.".to_string(),
    })
}

fn sha256(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, String> {
    serde_json::from_slice(&fs::read(path).map_err(|error| error.to_string())?).map_err(|error| error.to_string())
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), String> {
    let parent = path.parent().ok_or("GGUF state path has no parent")?;
    fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    let temporary = path.with_extension("json.tmp");
    fs::write(&temporary, serde_json::to_vec_pretty(value).map_err(|error| error.to_string())?).map_err(|error| error.to_string())?;
    fs::rename(temporary, path).map_err(|error| error.to_string())
}

fn resolve_root(start: &Path) -> Result<PathBuf, String> {
    let mut cursor = start.canonicalize().map_err(|error| error.to_string())?;
    loop {
        if cursor.join("Cargo.toml").is_file() && cursor.join(CONTRACT_PATH).is_file() {
            return Ok(cursor);
        }
        if !cursor.pop() { return Err("unable to locate Braxon repository root".into()); }
    }
}

fn display_relative(root: &Path, path: &Path) -> Result<String, String> {
    path.strip_prefix(root).map(|value| value.to_string_lossy().to_string()).map_err(|_| "semantic-flow path escaped repository root".into())
}
