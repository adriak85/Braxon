use crate::{
    execute_canonical_parameter_citadel_cycle, BusValue, HardwareWriteAck, KineticReflexor,
    TokenizerBridge, TokenizerBridgeReceipt, ValueClass, Watermark,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

pub const WATERMARKED_FILE_OPERATION_SCHEMA: &str = "braxon.nsq.watermarked_file_operation.v1";
pub const WATERMARKED_FILE_OPERATION_CAPABILITY: &str = "feature:watermark.file_operation";
const CONTRACT_RELATIVE_PATH: &str = "config/nsq/watermarked_file_operation_contract.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WatermarkedNativeBoundary {
    pub compiler: Option<String>,
    pub compiler_resolved_path: Option<String>,
    pub target_environment_matches: bool,
    pub explicit_execute_requested: bool,
    pub native_execution_attempted: bool,
    pub native_execution_succeeded: bool,
    pub artifact_path: Option<String>,
    pub artifact_sha256: Option<String>,
    pub artifact_watermark: Option<Watermark>,
    pub exact_guidance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WatermarkedFileOperationReport {
    pub schema: String,
    pub capability: String,
    pub intent: String,
    pub nsq_dialect: String,
    pub kinetic_reflexor_route: String,
    pub source_path: String,
    pub source_language: Option<String>,
    pub source_bytes: u64,
    pub source_sha256: String,
    pub tokenizer_receipt: TokenizerBridgeReceipt,
    pub parameter_generation: u64,
    pub parameter_invariants_passed: bool,
    pub model_weight_execution_claimed: bool,
    pub source_watermark: Watermark,
    pub source_watermark_committed: bool,
    pub recursive_transition: String,
    pub native_boundary: WatermarkedNativeBoundary,
    pub no_resident_runtime: bool,
    pub hidden_download_allowed: bool,
}

#[derive(Debug, Deserialize)]
struct WatermarkContract {
    schema: String,
    authority: String,
    capability: String,
    target_environment: String,
    watermark_family: String,
    execution_model: String,
    watermark_is_functional: bool,
    resident_runtime: bool,
    allowed_intents: Vec<String>,
    source_roots: Vec<String>,
    artifact_root: String,
    native_execution_policy: NativeExecutionPolicy,
    compiler_routes: Vec<CompilerRoute>,
}

#[derive(Debug, Deserialize)]
struct NativeExecutionPolicy {
    requires_explicit_execute: bool,
    requires_aarch64_android_target: bool,
    requires_resolved_local_compiler: bool,
    allows_hidden_download: bool,
    requires_source_hash_before_execution: bool,
    requires_artifact_hash_after_execution: bool,
    requires_artifact_rewatermark: bool,
    failure_mode: String,
}

#[derive(Debug, Clone, Deserialize)]
struct CompilerRoute {
    language_id: String,
    extensions: Vec<String>,
    nsq_capability: String,
    kinetic_reflexor_route: String,
    compiler: String,
    base_args: Vec<String>,
}

/// Execute one recursive file-level watermark transition. The source watermark is
/// an operational admission token: it commits an NSQ file state before an explicit
/// native compiler boundary may run. A native artifact receives a second watermark
/// only after its declared compiler succeeds on the real AArch64 Android target.
pub fn execute_watermarked_file_operation(
    start: impl AsRef<Path>,
    intent: &str,
    relative_source_path: &str,
    execute_native: bool,
) -> Result<WatermarkedFileOperationReport, String> {
    let root = resolve_root(start)?;
    let contract: WatermarkContract = read_json(&root.join(CONTRACT_RELATIVE_PATH))?;
    validate_contract(&contract)?;

    let intent = intent.trim().to_ascii_lowercase();
    if !contract
        .allowed_intents
        .iter()
        .any(|allowed| allowed == &intent)
    {
        return Err(format!(
            "watermarked file operation intent '{intent}' is not declared; allowed intents: {}",
            contract.allowed_intents.join(", ")
        ));
    }
    let relative = validate_relative_source_path(relative_source_path, &contract.source_roots)?;
    let source = root.join(&relative);
    let metadata = source.metadata().map_err(|error| {
        format!(
            "watermarked source '{}' is unavailable: {error}",
            relative.display()
        )
    })?;
    if !metadata.is_file() {
        return Err(format!(
            "watermarked source '{}' is not a regular file",
            relative.display()
        ));
    }
    let source_bytes = fs::read(&source).map_err(|error| {
        format!(
            "failed to read watermarked source '{}': {error}",
            relative.display()
        )
    })?;
    if source_bytes.is_empty() {
        return Err(format!(
            "watermarked source '{}' is empty and cannot form an NSQ state transition",
            relative.display()
        ));
    }
    let source_sha256 = sha256_hex(&source_bytes);
    let extension = source
        .extension()
        .and_then(|value| value.to_str())
        .map(|value| value.to_ascii_lowercase());
    let route = extension.as_deref().and_then(|extension| {
        contract
            .compiler_routes
            .iter()
            .find(|route| {
                route
                    .extensions
                    .iter()
                    .any(|candidate| candidate == extension)
            })
            .cloned()
    });
    let source_language = route.as_ref().map(|route| route.language_id.clone());
    let kinetic_reflexor_route = route
        .as_ref()
        .map(|route| route.kinetic_reflexor_route.clone())
        .unwrap_or_else(|| WATERMARKED_FILE_OPERATION_CAPABILITY.to_string());

    let tokenizer = TokenizerBridge::from_root(&root, "braxon_native")?;
    let token_input = format!(
        "intent={intent};path={};sha256={source_sha256}",
        relative.display()
    );
    let tokenizer_receipt = tokenizer.encode_translate_round_trip(&token_input);
    if !tokenizer_receipt.all_required_mappings_resolved() {
        return Err(format!(
            "watermarked file operation cannot admit '{}' because tokenizer mapping is unresolved",
            relative.display()
        ));
    }
    let signal = i64::try_from(source_bytes.len())
        .map_err(|_| "watermarked source byte length exceeds parameter signal range".to_string())?;
    let context = bounded_context(&source_bytes)?;
    let parameter = execute_canonical_parameter_citadel_cycle(signal, context)?;
    if !parameter.invariants.all_pass() {
        return Err(
            "watermarked file operation rejected because Parameter-Citadel invariants failed"
                .into(),
        );
    }

    let source_key = format!("nsq/file/{}", relative.display());
    let mut reflexor = KineticReflexor::new();
    reflexor.publish([BusValue {
        key: source_key.clone(),
        class: ValueClass::Fact,
        value_hash: source_sha256.clone(),
        byte_len: u64::try_from(source_bytes.len())
            .map_err(|_| "watermarked source byte length exceeds u64".to_string())?,
    }])?;
    reflexor.reconcile()?;
    let source_keys = reflexor
        .pending_delta()
        .iter()
        .map(|delta| delta.key.clone())
        .collect::<Vec<_>>();
    let generation = reflexor.generation();
    let source_commit = reflexor.commit_hardware(HardwareWriteAck {
        adapter_id: "nsq_watermark_file_state_adapter".to_string(),
        generation,
        accepted: true,
        written_keys: source_keys,
    })?;
    if !source_commit.hardware_write_acknowledged {
        return Err("source watermark state transition was not acknowledged".into());
    }
    let source_watermark = source_commit.watermark;
    if source_watermark.family != contract.watermark_family {
        return Err("source watermark family does not match functional-watermark contract".into());
    }

    let native_boundary = native_boundary(
        &root,
        &contract,
        route.as_ref(),
        &intent,
        &relative,
        &source,
        &source_sha256,
        execute_native,
    )?;
    Ok(WatermarkedFileOperationReport {
        schema: WATERMARKED_FILE_OPERATION_SCHEMA.to_string(),
        capability: contract.capability,
        intent,
        nsq_dialect: "control".to_string(),
        kinetic_reflexor_route,
        source_path: relative.display().to_string(),
        source_language,
        source_bytes: u64::try_from(source_bytes.len())
            .map_err(|_| "watermarked source byte length exceeds u64".to_string())?,
        source_sha256,
        tokenizer_receipt,
        parameter_generation: parameter.generation,
        parameter_invariants_passed: true,
        model_weight_execution_claimed: false,
        source_watermark,
        source_watermark_committed: true,
        recursive_transition: "intent→kinetic_reflexor_route→functional_source_watermark→native_compiler_boundary→artifact_watermark→recovery_baseline".to_string(),
        native_boundary,
        no_resident_runtime: !contract.resident_runtime,
        hidden_download_allowed: contract.native_execution_policy.allows_hidden_download,
    })
}

fn native_boundary(
    root: &Path,
    contract: &WatermarkContract,
    route: Option<&CompilerRoute>,
    intent: &str,
    relative: &Path,
    source: &Path,
    source_sha256: &str,
    execute_native: bool,
) -> Result<WatermarkedNativeBoundary, String> {
    let target_matches = cfg!(all(target_arch = "aarch64", target_os = "android"));
    let Some(route) = route else {
        return Ok(WatermarkedNativeBoundary {
            compiler: None,
            compiler_resolved_path: None,
            target_environment_matches: target_matches,
            explicit_execute_requested: execute_native,
            native_execution_attempted: false,
            native_execution_succeeded: false,
            artifact_path: None,
            artifact_sha256: None,
            artifact_watermark: None,
            exact_guidance: format!(
                "'{}' has no declared compiler route; attach a language contract before native materialization",
                relative.display()
            ),
        });
    };
    let compiler_path = find_executable(&route.compiler);
    let base = WatermarkedNativeBoundary {
        compiler: Some(route.compiler.clone()),
        compiler_resolved_path: compiler_path.clone(),
        target_environment_matches: target_matches,
        explicit_execute_requested: execute_native,
        native_execution_attempted: false,
        native_execution_succeeded: false,
        artifact_path: None,
        artifact_sha256: None,
        artifact_watermark: None,
        exact_guidance: String::new(),
    };
    if intent != "materialize" {
        return Ok(WatermarkedNativeBoundary {
            exact_guidance: format!(
                "source watermark committed for '{}'; intent '{}' does not request native compiler materialization",
                relative.display(), intent
            ),
            ..base
        });
    }
    if contract.native_execution_policy.requires_explicit_execute && !execute_native {
        return Ok(WatermarkedNativeBoundary {
            exact_guidance: format!(
                "source watermark committed for '{}'; retry with explicit native execution only on the declared AArch64 Android target",
                relative.display()
            ),
            ..base
        });
    }
    if contract
        .native_execution_policy
        .requires_aarch64_android_target
        && !target_matches
    {
        return Ok(WatermarkedNativeBoundary {
            exact_guidance: format!(
                "source watermark committed, but '{}' cannot be promoted from this host; native materialization requires {}",
                relative.display(), contract.target_environment
            ),
            ..base
        });
    }
    if contract
        .native_execution_policy
        .requires_resolved_local_compiler
        && compiler_path.is_none()
    {
        return Ok(WatermarkedNativeBoundary {
            exact_guidance: format!(
                "source watermark committed, but declared compiler '{}' is not resolved on PATH",
                route.compiler
            ),
            ..base
        });
    }
    let Some(compiler_path) = compiler_path else {
        return Err("functional-watermark contract permits no unresolved compiler fallback".into());
    };
    if !contract
        .native_execution_policy
        .requires_source_hash_before_execution
        || source_sha256.is_empty()
    {
        return Err(
            "functional-watermark contract rejected compiler execution without source hash".into(),
        );
    }
    let artifact = artifact_path(root, &contract.artifact_root, relative)?;
    if let Some(parent) = artifact.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("failed to create watermark artifact directory: {error}"))?;
    }
    let mut command = Command::new(&compiler_path);
    command
        .args(&route.base_args)
        .arg(source)
        .arg("-o")
        .arg(&artifact);
    let output = command.output().map_err(|error| {
        format!(
            "failed to launch declared compiler '{}' for '{}': {error}",
            route.compiler,
            relative.display()
        )
    })?;
    if !output.status.success() {
        return Ok(WatermarkedNativeBoundary {
            native_execution_attempted: true,
            exact_guidance: format!(
                "declared compiler '{}' rejected '{}': {}",
                route.compiler,
                relative.display(),
                String::from_utf8_lossy(&output.stderr).trim()
            ),
            ..base
        });
    }
    let artifact_bytes = fs::read(&artifact).map_err(|error| {
        format!(
            "compiler reported success but artifact '{}' is unreadable: {error}",
            artifact.display()
        )
    })?;
    if artifact_bytes.is_empty() {
        return Err("compiler reported success but emitted an empty artifact".into());
    }
    if !contract
        .native_execution_policy
        .requires_artifact_hash_after_execution
    {
        return Err(
            "functional-watermark contract forbids artifact promotion without artifact hash".into(),
        );
    }
    let artifact_sha256 = sha256_hex(&artifact_bytes);
    let mut artifact_reflexor = KineticReflexor::new();
    artifact_reflexor.publish([BusValue {
        key: format!("nsq/artifact/{}", artifact.display()),
        class: ValueClass::Fact,
        value_hash: artifact_sha256.clone(),
        byte_len: u64::try_from(artifact_bytes.len())
            .map_err(|_| "artifact byte length exceeds u64".to_string())?,
    }])?;
    artifact_reflexor.reconcile()?;
    let artifact_keys = artifact_reflexor
        .pending_delta()
        .iter()
        .map(|delta| delta.key.clone())
        .collect::<Vec<_>>();
    let artifact_generation = artifact_reflexor.generation();
    let artifact_commit = artifact_reflexor.commit_hardware(HardwareWriteAck {
        adapter_id: "nsq_watermark_artifact_state_adapter".to_string(),
        generation: artifact_generation,
        accepted: true,
        written_keys: artifact_keys,
    })?;
    if contract
        .native_execution_policy
        .requires_artifact_rewatermark
        && !artifact_commit.hardware_write_acknowledged
    {
        return Err("compiled artifact did not receive a committed functional watermark".into());
    }
    Ok(WatermarkedNativeBoundary {
        native_execution_attempted: true,
        native_execution_succeeded: true,
        artifact_path: Some(
            artifact
                .strip_prefix(root)
                .unwrap_or(&artifact)
                .display()
                .to_string(),
        ),
        artifact_sha256: Some(artifact_sha256),
        artifact_watermark: Some(artifact_commit.watermark),
        exact_guidance: "declared AArch64 compiler produced and re-watermarked the artifact; run the ABI, ELF, and Android execution probes before release promotion".to_string(),
        ..base
    })
}

fn validate_contract(contract: &WatermarkContract) -> Result<(), String> {
    if contract.schema != "braxon.nsq.watermarked_file_operation_contract.v1"
        || contract.authority != "NSQ_KINETIC_SEMANTIC_REFLEXOR"
        || contract.capability != WATERMARKED_FILE_OPERATION_CAPABILITY
        || contract.execution_model
            != "intent_to_reflexor_to_functional_watermark_to_native_boundary_to_artifact_watermark"
        || !contract.watermark_is_functional
        || contract.resident_runtime
        || contract.allowed_intents.is_empty()
        || contract.source_roots.is_empty()
        || contract.native_execution_policy.allows_hidden_download
        || contract.native_execution_policy.failure_mode
            != "fail_closed_with_exact_materialization_guidance"
        || contract.compiler_routes.iter().any(|route| {
            route.language_id.is_empty()
                || route.extensions.is_empty()
                || route.nsq_capability != format!("language:{}", route.language_id)
                || route.kinetic_reflexor_route.is_empty()
                || route.compiler.is_empty()
        })
    {
        return Err(
            "watermarked file-operation contract is invalid or weakens NSQ authority".into(),
        );
    }
    Ok(())
}

fn validate_relative_source_path(
    relative: &str,
    source_roots: &[String],
) -> Result<PathBuf, String> {
    let path = Path::new(relative);
    if relative.trim().is_empty()
        || path.is_absolute()
        || path.components().any(|component| {
            matches!(
                component,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    {
        return Err(
            "watermarked file operation requires a normalized repository-relative source path"
                .into(),
        );
    }
    let permitted = source_roots
        .iter()
        .map(Path::new)
        .any(|root| path.starts_with(root));
    if !permitted {
        return Err(format!(
            "watermarked source '{}' is outside declared source roots: {}",
            path.display(),
            source_roots.join(", ")
        ));
    }
    Ok(path.to_path_buf())
}

fn artifact_path(root: &Path, artifact_root: &str, source: &Path) -> Result<PathBuf, String> {
    let stem = source
        .file_stem()
        .and_then(|value| value.to_str())
        .ok_or("watermarked source has no UTF-8 file stem")?;
    let digest = sha256_hex(source.as_os_str().as_encoded_bytes());
    Ok(root
        .join(artifact_root)
        .join(format!("{stem}-{}.o", &digest[..16])))
}

fn bounded_context(bytes: &[u8]) -> Result<i64, String> {
    let value = bytes.iter().fold(0i64, |accumulator, byte| {
        accumulator
            .wrapping_mul(257)
            .wrapping_add(i64::from(*byte).saturating_add(1))
    });
    if value == i64::MIN {
        Err("watermarked source context reached reserved minimum value".into())
    } else {
        Ok(value)
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut digest = Sha256::new();
    digest.update(bytes);
    format!("{:x}", digest.finalize())
}

fn find_executable(name: &str) -> Option<String> {
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths)
            .map(|directory| directory.join(name))
            .find(|candidate| candidate.is_file())
            .map(|candidate| candidate.display().to_string())
    })
}

fn resolve_root(start: impl AsRef<Path>) -> Result<PathBuf, String> {
    let canonical = start
        .as_ref()
        .canonicalize()
        .map_err(|error| format!("unable to resolve watermarked file-operation start: {error}"))?;
    canonical
        .ancestors()
        .find(|candidate| candidate.join(CONTRACT_RELATIVE_PATH).is_file())
        .map(Path::to_path_buf)
        .ok_or_else(|| "unable to locate functional-watermark contract".to_string())
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, String> {
    let raw = fs::read_to_string(path)
        .map_err(|error| format!("failed to read '{}': {error}", path.display()))?;
    serde_json::from_str(&raw)
        .map_err(|error| format!("failed to parse '{}': {error}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repository_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .expect("repository root")
    }

    #[test]
    fn verified_source_commits_a_functional_watermark_without_claiming_host_materialization() {
        let report = execute_watermarked_file_operation(
            repository_root(),
            "verify",
            "crates/braxon-core/src/kinetic_reflexor.rs",
            false,
        )
        .expect("watermarked verification");
        assert_eq!(report.capability, WATERMARKED_FILE_OPERATION_CAPABILITY);
        assert!(report.source_watermark_committed);
        assert_eq!(report.source_language.as_deref(), Some("rust"));
        assert!(!report.model_weight_execution_claimed);
        assert!(report.no_resident_runtime);
        assert!(!report.native_boundary.native_execution_succeeded);
    }

    #[test]
    fn source_escape_fails_closed_before_watermark_admission() {
        let error =
            execute_watermarked_file_operation(repository_root(), "verify", "../Cargo.toml", false)
                .expect_err("parent escape must fail");
        assert!(error.contains("repository-relative"));
    }

    #[test]
    fn host_materialization_does_not_claim_an_android_artifact() {
        let report = execute_watermarked_file_operation(
            repository_root(),
            "materialize",
            "crates/braxon-core/src/kinetic_reflexor.rs",
            true,
        )
        .expect("watermarked materialization routing");
        if !cfg!(all(target_arch = "aarch64", target_os = "android")) {
            assert!(!report.native_boundary.native_execution_attempted);
            assert!(!report.native_boundary.native_execution_succeeded);
            assert!(report.native_boundary.exact_guidance.contains("requires"));
        }
    }
}
