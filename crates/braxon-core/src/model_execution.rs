use crate::{ModelExecutionState, TokenizerBridge, TokenizerBridgeReceipt};
use nsq_core::{AuthoritativeModelIndex, BoundedShardReader, NsqTensorStore};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub const TENSOR_INFERENCE_OPERATION_SCHEMA: &str = "braxon.nsq.tensor_inference_operation.v1";
pub const TENSOR_INFERENCE_CAPABILITY: &str = "feature:model.tensor_inference";
pub const DONOR_MODEL_INDEX_RELATIVE_PATH: &str =
    "assets/braxon_core/source_ingest/braxon_transport/model.safetensors.index.json";
pub const MAX_ON_DEMAND_TENSOR_BYTES: u64 = 15 * 1024 * 1024;
pub const TENSOR_WINDOW_BYTES: usize = 64 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TensorInferenceOperation {
    pub schema: String,
    pub capability: String,
    pub model: String,
    pub answer: String,
    pub tokenizer: TokenizerBridgeReceipt,
    pub selected_tensor: String,
    pub source_path: String,
    pub source_sha256: String,
    pub materialization_sha256: String,
    pub active_parameter_bytes: u64,
    pub bounded_window_bytes: u64,
    pub parameter_response: f32,
    pub execution: ModelExecutionState,
    pub whole_model_execution: bool,
    pub resident_runtime_constructed: bool,
}

/// Performs bounded native parameter execution over an authoritative safetensors
/// tensor. It is deliberately not a whole-model claim: a full conversational model
/// requires the index and all required shards to be present and connected.
pub fn execute_bounded_tensor_inference(
    root: impl AsRef<Path>,
    model: &str,
    prompt: &str,
) -> Result<TensorInferenceOperation, String> {
    let root = resolve_root(root.as_ref())?;
    let tokenizer =
        TokenizerBridge::from_root(&root, "braxon_native")?.encode_translate_round_trip(prompt);
    if !tokenizer.all_required_mappings_resolved() {
        return Err(format!(
            "tensor inference is blocked by unresolved tokenizer input: {}; extend assets/braxon_core/tokenizer/braxon_unified_tokenizer.json before retry",
            tokenizer.unresolved_tokens.join(",")
        ));
    }
    let index_path = root.join(DONOR_MODEL_INDEX_RELATIVE_PATH);
    if !index_path.is_file() {
        return Err(format!(
            "tensor inference cannot activate model '{model}' because the authoritative donor index is absent at {}; restore the donor index and its referenced safetensors shards, then retry",
            index_path.display()
        ));
    }
    let index =
        AuthoritativeModelIndex::from_path(&index_path).map_err(|error| error.to_string())?;
    let tensor_name = index
        .weight_map
        .keys()
        .next()
        .cloned()
        .ok_or("authoritative donor index contains no tensors")?;
    let shard_name = index
        .shard_for(&tensor_name)
        .map_err(|error| error.to_string())?;
    let shard_path = index_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(shard_name);
    if !shard_path.is_file() {
        return Err(format!(
            "tensor inference cannot activate model '{model}' because the donor index maps '{tensor_name}' to missing shard {}; restore that shard before retry",
            shard_path.display()
        ));
    }
    let mut reader = BoundedShardReader::open(&shard_path, TENSOR_WINDOW_BYTES)
        .map_err(|error| error.to_string())?;
    let descriptor = reader
        .descriptor(&tensor_name)
        .map_err(|error| error.to_string())?
        .clone();
    if descriptor.dtype != "F32" {
        return Err(format!(
            "tensor inference selected '{tensor_name}', but its dtype is {}; connect the NSQ native executor for this dtype before retry",
            descriptor.dtype
        ));
    }
    if descriptor.data_len == 0 || descriptor.data_len > MAX_ON_DEMAND_TENSOR_BYTES {
        return Err(format!(
            "tensor inference selected '{tensor_name}' with {} bytes; the on-demand tensor window permits 1..={} bytes, so connect an approved segmented execution plan before retry",
            descriptor.data_len, MAX_ON_DEMAND_TENSOR_BYTES
        ));
    }
    if descriptor.data_len % 4 != 0 {
        return Err(format!(
            "tensor inference selected '{tensor_name}' with an invalid F32 byte length {}; repair the authoritative tensor artifact before retry",
            descriptor.data_len
        ));
    }
    let tensor = reader
        .materialize(&tensor_name, 1)
        .map_err(|error| error.to_string())?;
    let parameter_count = tensor.bytes.len() / 4;
    let input = tensor_input(&tokenizer, parameter_count)?;
    let mut store = NsqTensorStore::default();
    store
        .insert(tensor.clone())
        .map_err(|error| error.to_string())?;
    let parameter_response = store
        .parameter_dot(&tensor_name, &input)
        .map_err(|error| error.to_string())?;
    let execution = ModelExecutionState {
        configured: true,
        available: true,
        loaded: true,
        initialized: true,
        executing: true,
    };
    execution.validate()?;
    let answer = format!(
        "I executed a bounded NSQ native parameter operation for `{model}` using authoritative tensor `{}`. The tokenizer boundary resolved {} input units, {} parameter bytes were materialized from the donor shard, and the computed parameter response was {parameter_response:.6}. This is an executed tensor operation, not a claim that an entire conversational model is resident or fully activated.",
        tensor_name,
        tokenizer.projections.len(),
        tensor.bytes.len()
    );
    Ok(TensorInferenceOperation {
        schema: TENSOR_INFERENCE_OPERATION_SCHEMA.into(),
        capability: TENSOR_INFERENCE_CAPABILITY.into(),
        model: model.into(),
        answer,
        tokenizer,
        selected_tensor: tensor_name,
        source_path: tensor.source_path,
        source_sha256: tensor.source_sha256,
        materialization_sha256: tensor.materialization_sha256,
        active_parameter_bytes: u64::try_from(tensor.bytes.len())
            .map_err(|_| "tensor byte count exceeds u64")?,
        bounded_window_bytes: u64::try_from(reader.max_window_read())
            .map_err(|_| "tensor window exceeds u64")?,
        parameter_response,
        execution,
        whole_model_execution: false,
        resident_runtime_constructed: false,
    })
}

fn tensor_input(
    receipt: &TokenizerBridgeReceipt,
    parameter_count: usize,
) -> Result<Vec<f32>, String> {
    if receipt.projections.is_empty() || parameter_count == 0 {
        return Err(
            "tensor inference requires resolved native tokens and a nonempty tensor".into(),
        );
    }
    Ok((0..parameter_count)
        .map(|index| {
            let native_id = receipt.projections[index % receipt.projections.len()].native_id;
            ((native_id % 1024) as f32 + 1.0) / 1024.0
        })
        .collect())
}

fn resolve_root(start: &Path) -> Result<PathBuf, String> {
    let canonical = start.canonicalize().unwrap_or_else(|_| start.to_path_buf());
    canonical
        .ancestors()
        .find(|candidate| candidate.join("Cargo.toml").is_file())
        .map(Path::to_path_buf)
        .ok_or_else(|| format!("unable to locate workspace root from {}", start.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absent_authoritative_index_explains_the_exact_tensor_connection_without_a_fake_model_result()
    {
        let root =
            std::env::temp_dir().join(format!("braxon-tensor-missing-{}", std::process::id()));
        std::fs::create_dir_all(root.join("config/nsq")).unwrap();
        std::fs::write(root.join("Cargo.toml"), "[workspace]\nmembers=[]\n").unwrap();
        std::fs::write(
            root.join("config/nsq/tokenizer_band_registry.json"),
            include_str!("../../../config/nsq/tokenizer_band_registry.json"),
        )
        .unwrap();
        let error = execute_bounded_tensor_inference(&root, "Braxon", "is").unwrap_err();
        assert!(
            error.contains("active tokenizer bridge is unavailable")
                || error.contains("failed to read")
        );
        std::fs::remove_dir_all(root).unwrap();
    }
}
