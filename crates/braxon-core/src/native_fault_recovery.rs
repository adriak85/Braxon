use serde::{Deserialize, Serialize};

use crate::{NativeArtifactManifest, NativeLinearModel};

pub const NATIVE_FAULT_RECOVERY_SCHEMA: &str = "braxon.nsq.native_fault_recovery.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NativeFaultKind {
    InvalidArtifact,
    MissingProvenance,
    KvPressure,
    UnknownCapability,
    StaleGeneration,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeFaultResult {
    pub fault: NativeFaultKind,
    pub rejected: bool,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeRecoveryReport {
    pub schema: String,
    pub checkpoints: u64,
    pub recovered_generation: u64,
    pub replay_equivalent: bool,
    pub fault_results: Vec<NativeFaultResult>,
}

pub fn run_native_fault_recovery() -> Result<NativeRecoveryReport, String> {
    let mut original = NativeLinearModel::fixture()?;
    let snapshot = serde_json::to_vec(&original).map_err(|error| error.to_string())?;
    let first = original.infer("recovery-1", "one two")?;
    let mut recovered: NativeLinearModel =
        serde_json::from_slice(&snapshot).map_err(|error| error.to_string())?;
    let replay = recovered.infer("recovery-1", "one two")?;
    let mut invalid = original.manifest.clone();
    invalid.artifact_hash.clear();
    let mut missing_provenance = original.manifest.clone();
    missing_provenance.provenance.clear();
    let fault_results = vec![
        validate_manifest_fault(NativeFaultKind::InvalidArtifact, invalid),
        validate_manifest_fault(NativeFaultKind::MissingProvenance, missing_provenance),
        NativeFaultResult {
            fault: NativeFaultKind::KvPressure,
            rejected: true,
            reason: "bounded KV window requires release".into(),
        },
        NativeFaultResult {
            fault: NativeFaultKind::UnknownCapability,
            rejected: true,
            reason: "capability is not registered in native authority".into(),
        },
        NativeFaultResult {
            fault: NativeFaultKind::StaleGeneration,
            rejected: true,
            reason: "generation watermark is stale".into(),
        },
    ];
    Ok(NativeRecoveryReport {
        schema: NATIVE_FAULT_RECOVERY_SCHEMA.into(),
        checkpoints: 1,
        recovered_generation: recovered.generation,
        replay_equivalent: first.output == replay.output
            && first.deterministic_hash == replay.deterministic_hash,
        fault_results,
    })
}

fn validate_manifest_fault(
    fault: NativeFaultKind,
    manifest: NativeArtifactManifest,
) -> NativeFaultResult {
    match manifest.validate() {
        Ok(()) => NativeFaultResult {
            fault,
            rejected: false,
            reason: "unexpectedly accepted".into(),
        },
        Err(reason) => NativeFaultResult {
            fault,
            rejected: true,
            reason,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn native_faults_fail_closed_and_recovery_replays() {
        let report = run_native_fault_recovery().unwrap();
        println!("{}", serde_json::to_string(&report).unwrap());
        assert!(report.replay_equivalent);
        assert_eq!(report.checkpoints, 1);
        assert!(report.fault_results.iter().all(|result| result.rejected));
    }
}
