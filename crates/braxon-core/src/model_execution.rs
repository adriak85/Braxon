//! Bounded model-band operation through the canonical Council Ten Citadel seed.
//!
//! This module is intentionally not a safetensors loader. A model request is
//! admitted only for a configured Council Ten band, then represented as a fresh
//! deterministic IntentSeed, materialized across the ten-body Citadel, fired on
//! the NSQ bus, and released before the operation returns. The result is an
//! executed bounded seed-window operation, not a claim of whole-model learned
//! weight execution or a resident runtime.

use crate::donor_model_readiness::{
    assess_donor_model_readiness, DONOR_CITADEL_SEED_SCHEME, DONOR_CITADEL_STACK_RELATIVE_PATH,
};
use crate::offline_models::ModelExecutionState;
use crate::tokenizer_bridge::{TokenizerBridge, TokenizerBridgeReceipt};
use nsq_citadel::{coordinate_intent, CitadelNativeRuntime, CoachingMode, IntentSeed};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

pub const TENSOR_INFERENCE_OPERATION_SCHEMA: &str = "braxon.nsq.tensor_inference_operation.v2";
pub const TENSOR_INFERENCE_CAPABILITY: &str = "feature:model.tensor_inference";
pub const DONOR_CITADEL_SEED_CONTRACT_RELATIVE_PATH: &str = DONOR_CITADEL_STACK_RELATIVE_PATH;
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

/// Executes a bounded selected-band operation through the Council Ten Citadel
/// seed path. The configured topology is revalidated before each request and
/// every materialized bus address is released before this function returns.
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
            "canonical Citadel seed inference is blocked by unresolved tokenizer input: {}; extend assets/braxon_core/tokenizer/braxon_unified_tokenizer.json before retry",
            tokenizer.unresolved_tokens.join(",")
        ));
    }
    let readiness = assess_donor_model_readiness(&root)?;
    let band = readiness
        .bands
        .iter()
        .find(|band| band.model_id == model)
        .ok_or_else(|| {
            format!(
                "model '{model}' is not a configured Council Ten band; select one of: {}",
                readiness
                    .bands
                    .iter()
                    .map(|band| band.model_id.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })?;
    if !readiness.complete_ten_body_window_proven
        || !band.seed_window_materialized
        || !band.piston_lease_released
    {
        return Err(format!(
            "canonical Council Ten Citadel seed readiness is incomplete for '{model}'; the ten-body bounded materialization and release proof must pass before inference"
        ));
    }

    let seed_intent = format!("{} {}", model, prompt);
    let seed_hash = sha256_hex(seed_intent.as_bytes());
    let seed = IntentSeed {
        identity: format!("{DONOR_CITADEL_SEED_SCHEME}-runtime-{}", &seed_hash[..16]),
        intent: seed_intent.clone(),
        coordinates: coordinate_intent(&seed_intent),
        sections: vec![model.to_string(), "runtime_inference".to_string()],
    };
    let mut runtime = CitadelNativeRuntime::new(CoachingMode::Balanced);
    let materialization = runtime.materialize_seed(&seed, 1).map_err(|error| {
        format!("canonical Citadel seed inference materialization failed: {error}")
    })?;
    let body = materialization
        .bodies
        .iter()
        .find(|body| body.pole_id == band.materialized_pole)
        .ok_or_else(|| {
            format!(
                "canonical Citadel materialization did not produce selected pole '{}' for model '{model}'",
                band.materialized_pole
            )
        })?
        .clone();
    let parameter_response = *materialization
        .parameter_outputs
        .get(&band.materialized_pole)
        .ok_or_else(|| {
            format!(
                "canonical Citadel materialization did not produce a parameter response for selected pole '{}'",
                band.materialized_pole
            )
        })?;
    let active_parameter_bytes = u64::try_from(body.shape.iter().product::<u64>() as usize)
        .map_err(|_| "Citadel tensor element count exceeds u64")?
        .checked_mul(4)
        .ok_or("Citadel tensor byte count overflow")?;
    if active_parameter_bytes == 0 || active_parameter_bytes > MAX_ON_DEMAND_TENSOR_BYTES {
        return Err(format!(
            "canonical Citadel seed selected '{}' with {} active bytes; the on-demand window permits 1..={} bytes",
            body.tensor_name, active_parameter_bytes, MAX_ON_DEMAND_TENSOR_BYTES
        ));
    }
    let release = runtime
        .release_materialization(&materialization)
        .map_err(|error| format!("canonical Citadel seed inference release failed: {error}"))?;
    if release.released != materialization.bodies.len()
        || release.executed != materialization.bodies.len()
    {
        return Err(format!(
            "canonical Citadel seed inference did not release its bounded window: released={}, bodies={}",
            release.released,
            materialization.bodies.len()
        ));
    }

    let execution = ModelExecutionState {
        configured: true,
        available: true,
        loaded: true,
        initialized: true,
        executing: true,
    };
    execution.validate()?;
    let answer = format!(
        "I executed a bounded canonical Citadel seed operation for `{model}` on pole `{}`. The tokenizer boundary resolved {} input units; the selected NSQ body was set and fired with {} active seed-window bytes, produced parameter response {parameter_response:.6}, and the complete ten-body bus window was released. This is an executed seed-window operation, not a claim that learned model weights, a whole conversational model, or a resident runtime are active.",
        band.materialized_pole,
        tokenizer.projections.len(),
        active_parameter_bytes,
    );
    Ok(TensorInferenceOperation {
        schema: TENSOR_INFERENCE_OPERATION_SCHEMA.into(),
        capability: TENSOR_INFERENCE_CAPABILITY.into(),
        model: model.into(),
        answer,
        tokenizer,
        selected_tensor: body.tensor_name,
        source_path: body.source_seed_id,
        source_sha256: body.source_seed_hash,
        materialization_sha256: body.materialization_hash,
        active_parameter_bytes,
        bounded_window_bytes: active_parameter_bytes.min(TENSOR_WINDOW_BYTES as u64),
        parameter_response,
        execution,
        whole_model_execution: false,
        resident_runtime_constructed: false,
    })
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
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
    fn absent_tokenizer_bridge_fails_before_any_citadel_window_is_claimed() {
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
