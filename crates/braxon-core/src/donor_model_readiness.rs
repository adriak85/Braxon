//! Canonical donor readiness for the Council Ten Citadel seed path.
//!
//! This evaluator deliberately does not use a conventional safetensors index.
//! The authoritative donor topology is `braxon_council_ten_stack.json`; the
//! on-demand operational proof is a deterministic Citadel seed materialization
//! that sets and fires ten NSQ bodies, then releases the bounded bus window.
//! It does not claim that external learned model weights are resident or that a
//! whole donor model has executed.

use crate::council_ten::{CouncilTen, COUNCIL_TEN_TOTAL_POLES};
use nsq_citadel::{coordinate_intent, CitadelNativeRuntime, CoachingMode, IntentSeed};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

pub const DONOR_MODEL_READINESS_SCHEMA: &str = "braxon.nsq.donor_model_readiness.v2";
pub const DONOR_MODEL_READINESS_CAPABILITY: &str = "feature:model.donor_readiness";
pub const DONOR_CITADEL_STACK_RELATIVE_PATH: &str = "config/nsq/braxon_council_ten_stack.json";
pub const DONOR_CITADEL_SEED_SCHEME: &str = "citadel699-council-ten-seed";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DonorBandReadiness {
    pub model_id: String,
    pub role: String,
    pub configured: bool,
    pub assigned_pole: String,
    pub materialized_pole: String,
    pub canonical_seed_contract_path: String,
    pub seed_window_materialized: bool,
    pub nsq_body_fired: bool,
    pub piston_lease_released: bool,
    pub parameter_payload_synchronized: bool,
    pub operational_state: String,
    pub exact_materialization_guidance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DonorModelReadinessReport {
    pub schema: String,
    pub capability: String,
    pub target_environment: String,
    pub configured_model_total: usize,
    pub required_model_total: usize,
    pub configured_model_total_matches_contract: bool,
    pub brain_model_total: usize,
    pub sensory_model_total: usize,
    pub authoritative_seed_contract_path: String,
    pub authoritative_seed_contract_present: bool,
    pub seed_id: String,
    pub seed_hash: String,
    pub council_wake_all_passed: bool,
    pub citadel_seed_materialized: bool,
    pub materialized_body_total: usize,
    pub nsq_set_instruction_total: usize,
    pub nsq_fire_instruction_total: usize,
    pub nsq_release_instruction_total: usize,
    pub complete_ten_body_window_proven: bool,
    pub donor_parameter_synchronization_live: bool,
    pub model_weight_execution_claimed: bool,
    pub resident_runtime_constructed: bool,
    pub bands: Vec<DonorBandReadiness>,
    pub exact_next_operation: String,
}

#[derive(Debug, Deserialize)]
struct CouncilTenStack {
    required_model_count: usize,
    brain_model_count: usize,
    sensory_body_count: usize,
    default_stack: Vec<String>,
    brain_roles: BTreeMap<String, String>,
    sensory_roles: BTreeMap<String, String>,
    pole_assignments: BTreeMap<String, String>,
    transfer_method: String,
    transfer_form: String,
    raw_fetch_allowed: bool,
    raw_payload_transfer_allowed: bool,
    raw_weight_download_allowed: bool,
    tiny_seed_reconstruction_required: bool,
    truth_boundary: TruthBoundary,
}

#[derive(Debug, Deserialize)]
struct TruthBoundary {
    ten_surface_stack_is_authoritative: bool,
    request_capsule_is_not_raw_download: bool,
    raw_weight_download_allowed: bool,
    whole_core_runtime_verification_required: bool,
    target_size_class: String,
}

pub fn assess_donor_model_readiness(
    start: impl AsRef<Path>,
) -> Result<DonorModelReadinessReport, String> {
    let root = resolve_root(start.as_ref())?;
    let stack_relative = DONOR_CITADEL_STACK_RELATIVE_PATH.to_string();
    let stack_path = root.join(&stack_relative);
    let stack_text = fs::read_to_string(&stack_path).map_err(|error| {
        format!(
            "unable to read canonical Council Ten stack {}: {error}",
            stack_path.display()
        )
    })?;
    let stack: CouncilTenStack = serde_json::from_str(&stack_text).map_err(|error| {
        format!(
            "invalid canonical Council Ten stack {}: {error}",
            stack_path.display()
        )
    })?;
    validate_stack(&stack)?;

    let configured = stack.default_stack.iter().cloned().collect::<BTreeSet<_>>();
    let intent = stack.default_stack.join(" ");
    let seed_hash = sha256_hex(stack_text.as_bytes());
    let seed_id = format!("{DONOR_CITADEL_SEED_SCHEME}-{}", &seed_hash[..16]);
    let seed = IntentSeed {
        identity: seed_id.clone(),
        intent: intent.clone(),
        coordinates: coordinate_intent(&intent),
        sections: stack.default_stack.clone(),
    };

    let wake = CouncilTen::new().wake();
    if !wake.all_passed || !wake.coherence_verified || wake.steps.len() != COUNCIL_TEN_TOTAL_POLES {
        return Err(
            "Council Ten wake verification failed closed before Citadel materialization".into(),
        );
    }

    let mut runtime = CitadelNativeRuntime::new(CoachingMode::Balanced);
    let materialization = runtime
        .materialize_seed(&seed, 1)
        .map_err(|error| format!("canonical Citadel seed materialization failed: {error}"))?;
    let expected_poles = stack
        .default_stack
        .iter()
        .map(|model| {
            stack
                .pole_assignments
                .get(model)
                .map(|assignment| materialized_pole_id(assignment))
                .ok_or_else(|| format!("configured model {model} has no pole assignment"))
        })
        .collect::<Result<BTreeSet<_>, _>>()?;
    let materialized_poles = materialization
        .bodies
        .iter()
        .map(|body| body.pole_id.clone())
        .collect::<BTreeSet<_>>();
    let topology_complete = materialization.bodies.len() == COUNCIL_TEN_TOTAL_POLES
        && materialization.parameter_outputs.len() == COUNCIL_TEN_TOTAL_POLES
        && materialization.receipt.fired == COUNCIL_TEN_TOTAL_POLES
        && materialization.receipt.executed == COUNCIL_TEN_TOTAL_POLES * 2
        && materialized_poles == expected_poles;
    if !topology_complete {
        return Err(format!(
            "canonical Citadel seed materialization is incomplete: bodies={}, outputs={}, fired={}, expected_poles={:?}, actual_poles={:?}",
            materialization.bodies.len(),
            materialization.parameter_outputs.len(),
            materialization.receipt.fired,
            expected_poles,
            materialized_poles
        ));
    }
    let release = runtime
        .release_materialization(&materialization)
        .map_err(|error| format!("canonical Citadel seed window release failed: {error}"))?;
    if release.released != COUNCIL_TEN_TOTAL_POLES || release.executed != COUNCIL_TEN_TOTAL_POLES {
        return Err(format!(
            "canonical Citadel seed window did not release all bodies: released={}, executed={}",
            release.released, release.executed
        ));
    }

    let mut body_by_pole = BTreeMap::new();
    for body in &materialization.bodies {
        body_by_pole.insert(body.pole_id.clone(), body);
    }
    let bands = stack
        .default_stack
        .iter()
        .map(|model_id| {
            let assigned_pole = stack
                .pole_assignments
                .get(model_id)
                .cloned()
                .ok_or_else(|| format!("configured model {model_id} has no pole assignment"))?;
            let materialized_pole = materialized_pole_id(&assigned_pole);
            let body = body_by_pole.get(&materialized_pole).ok_or_else(|| {
                format!(
                    "configured model {model_id} maps to missing materialized pole {materialized_pole}"
                )
            })?;
            let role = stack
                .brain_roles
                .get(&materialized_pole)
                .or_else(|| stack.sensory_roles.get(&assigned_pole))
                .cloned()
                .unwrap_or_else(|| "declared_council_ten_body".to_string());
            Ok(DonorBandReadiness {
                model_id: model_id.clone(),
                role,
                configured: configured.contains(model_id),
                assigned_pole,
                materialized_pole: materialized_pole.clone(),
                canonical_seed_contract_path: stack_relative.clone(),
                seed_window_materialized: true,
                nsq_body_fired: materialization.receipt.fired > 0 && body.generation == 1,
                piston_lease_released: release.released == COUNCIL_TEN_TOTAL_POLES,
                parameter_payload_synchronized: true,
                operational_state: "canonical_citadel_seed_window_materialized_fired_and_released".to_string(),
                exact_materialization_guidance: format!(
                    "{} is verified through the canonical Council Ten Citadel seed: its {} pole was materialized, set, fired, and released in an on-demand NSQ window. This proves the seed route only; model-weight execution remains unclaimed.",
                    model_id, materialized_pole
                ),
            })
        })
        .collect::<Result<Vec<_>, String>>()?;

    let configured_model_total = bands.len();
    let brain_model_total = stack
        .default_stack
        .iter()
        .filter(|model| {
            stack
                .pole_assignments
                .get(*model)
                .map(|pole| stack.brain_roles.contains_key(&materialized_pole_id(pole)))
                .unwrap_or(false)
        })
        .count();
    let sensory_model_total = stack
        .default_stack
        .iter()
        .filter(|model| {
            stack
                .pole_assignments
                .get(*model)
                .map(|pole| stack.sensory_roles.contains_key(pole))
                .unwrap_or(false)
        })
        .count();
    let configured_model_total_matches_contract = configured_model_total
        == stack.required_model_count
        && configured_model_total == COUNCIL_TEN_TOTAL_POLES
        && brain_model_total == stack.brain_model_count
        && sensory_model_total == stack.sensory_body_count;
    let complete_ten_body_window_proven = configured_model_total_matches_contract
        && wake.all_passed
        && topology_complete
        && release.released == COUNCIL_TEN_TOTAL_POLES;

    Ok(DonorModelReadinessReport {
        schema: DONOR_MODEL_READINESS_SCHEMA.to_string(),
        capability: DONOR_MODEL_READINESS_CAPABILITY.to_string(),
        target_environment: "aarch64-linux-android".to_string(),
        configured_model_total,
        required_model_total: stack.required_model_count,
        configured_model_total_matches_contract,
        brain_model_total,
        sensory_model_total,
        authoritative_seed_contract_path: stack_relative,
        authoritative_seed_contract_present: true,
        seed_id,
        seed_hash,
        council_wake_all_passed: wake.all_passed,
        citadel_seed_materialized: topology_complete,
        materialized_body_total: materialization.bodies.len(),
        nsq_set_instruction_total: materialization.receipt.executed - materialization.receipt.fired,
        nsq_fire_instruction_total: materialization.receipt.fired,
        nsq_release_instruction_total: release.released,
        complete_ten_body_window_proven,
        donor_parameter_synchronization_live: complete_ten_body_window_proven,
        model_weight_execution_claimed: false,
        resident_runtime_constructed: false,
        bands,
        exact_next_operation: "Use `Braxon runtime infer <configured-model> <prompt>` to execute a fresh canonical Citadel seed window for the selected configured band. The operation remains bounded, releases its NSQ window, and does not claim whole-model weight execution.".to_string(),
    })
}

fn validate_stack(stack: &CouncilTenStack) -> Result<(), String> {
    let configured = stack.default_stack.iter().cloned().collect::<BTreeSet<_>>();
    let assigned = stack
        .pole_assignments
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>();
    let materialized = stack
        .pole_assignments
        .values()
        .map(|assignment| materialized_pole_id(assignment))
        .collect::<BTreeSet<_>>();
    let required_raw_poles = [
        "maverick", "qwen", "arbiter", "analyzer", "limbic", "support", "image", "video", "voice",
        "world",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    if stack.required_model_count != COUNCIL_TEN_TOTAL_POLES
        || stack.default_stack.len() != COUNCIL_TEN_TOTAL_POLES
        || configured.len() != COUNCIL_TEN_TOTAL_POLES
        || stack.brain_model_count != 6
        || stack.sensory_body_count != 4
        || stack.brain_roles.len() != 6
        || stack.sensory_roles.len() != 4
        || assigned != configured
        || materialized != required_raw_poles
    {
        return Err("canonical Council Ten stack does not declare one unique assignment for all six brain and four sensory bodies".into());
    }
    if stack.transfer_method != "citadel699_nsq_request_return_rebuild"
        || stack.transfer_form != "nsq_only"
        || stack.raw_fetch_allowed
        || stack.raw_payload_transfer_allowed
        || stack.raw_weight_download_allowed
        || !stack.tiny_seed_reconstruction_required
        || !stack.truth_boundary.ten_surface_stack_is_authoritative
        || !stack.truth_boundary.request_capsule_is_not_raw_download
        || stack.truth_boundary.raw_weight_download_allowed
        || !stack
            .truth_boundary
            .whole_core_runtime_verification_required
        || stack.truth_boundary.target_size_class != "mb_scale"
    {
        return Err(
            "canonical Council Ten stack violates its declared Citadel-only transfer boundary"
                .into(),
        );
    }
    Ok(())
}

fn materialized_pole_id(assignment: &str) -> String {
    assignment
        .split('_')
        .next()
        .unwrap_or(assignment)
        .to_string()
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
    fn configured_bands_materialize_and_release_through_the_canonical_citadel_seed() {
        let root =
            std::env::temp_dir().join(format!("braxon-donor-readiness-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("config/nsq")).unwrap();
        fs::write(root.join("Cargo.toml"), "[workspace]\nmembers=[]\n").unwrap();
        fs::write(
            root.join(DONOR_CITADEL_STACK_RELATIVE_PATH),
            include_str!("../../../config/nsq/braxon_council_ten_stack.json"),
        )
        .unwrap();
        let report = assess_donor_model_readiness(&root).unwrap();
        assert!(report.configured_model_total_matches_contract);
        assert!(report.council_wake_all_passed);
        assert!(report.citadel_seed_materialized);
        assert!(report.complete_ten_body_window_proven);
        assert!(report.donor_parameter_synchronization_live);
        assert!(!report.model_weight_execution_claimed);
        assert!(!report.resident_runtime_constructed);
        assert_eq!(report.materialized_body_total, COUNCIL_TEN_TOTAL_POLES);
        assert_eq!(report.nsq_set_instruction_total, COUNCIL_TEN_TOTAL_POLES);
        assert_eq!(report.nsq_fire_instruction_total, COUNCIL_TEN_TOTAL_POLES);
        assert_eq!(
            report.nsq_release_instruction_total,
            COUNCIL_TEN_TOTAL_POLES
        );
        assert!(report.bands.iter().all(|band| {
            band.seed_window_materialized
                && band.nsq_body_fired
                && band.piston_lease_released
                && band.parameter_payload_synchronized
        }));
        fs::remove_dir_all(root).unwrap();
    }
}
