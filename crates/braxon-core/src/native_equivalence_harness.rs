use serde::{Deserialize, Serialize};

use crate::{run_native_training, NativeLinearModel, NativeTrainingPath, NativeTrainingSample};

pub const NATIVE_EQUIVALENCE_HARNESS_SCHEMA: &str = "braxon.nsq.native_equivalence_harness.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeEquivalenceReport {
    pub schema: String,
    pub inference_replay_equivalent: bool,
    pub training_path_equivalent: bool,
    pub zero_prediction_accuracy_correct: bool,
    pub inference_hash: String,
    pub dense_training_hash: String,
    pub reactive_training_hash: String,
    pub predictive_training_hash: String,
    pub predictive_corrections: u64,
}

pub fn run_native_fixture_equivalence() -> Result<NativeEquivalenceReport, String> {
    let mut first = NativeLinearModel::fixture()?;
    let mut second = NativeLinearModel::fixture()?;
    let first_receipt = first.infer("equivalence-1", "one two")?;
    let second_receipt = second.infer("equivalence-1", "one two")?;
    let samples = vec![
        NativeTrainingSample {
            input: vec![1, 2],
            target: 5,
        },
        NativeTrainingSample {
            input: vec![2, 1],
            target: 4,
        },
        NativeTrainingSample {
            input: vec![1, 1],
            target: 3,
        },
    ];
    let dense = run_native_training(NativeTrainingPath::Dense, &samples, 0)?;
    let reactive = run_native_training(NativeTrainingPath::Reactive, &samples, 0)?;
    let predictive = run_native_training(NativeTrainingPath::Predictive, &samples, 0)?;
    let inference_replay_equivalent = first_receipt.deterministic_hash
        == second_receipt.deterministic_hash
        && first_receipt.output == second_receipt.output;
    let training_path_equivalent = dense.deterministic_hash == reactive.deterministic_hash
        && reactive.deterministic_hash == predictive.deterministic_hash
        && dense.final_parameters == reactive.final_parameters
        && reactive.final_parameters == predictive.final_parameters;
    Ok(NativeEquivalenceReport {
        schema: NATIVE_EQUIVALENCE_HARNESS_SCHEMA.into(),
        inference_replay_equivalent,
        training_path_equivalent,
        zero_prediction_accuracy_correct: training_path_equivalent
            && predictive.prediction_misses > 0
            && predictive.correction_events > 0,
        inference_hash: first_receipt.deterministic_hash,
        dense_training_hash: dense.deterministic_hash,
        reactive_training_hash: reactive.deterministic_hash,
        predictive_training_hash: predictive.deterministic_hash,
        predictive_corrections: predictive.correction_events,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_fixture_equivalence_is_executable_and_fail_closed() {
        let report = run_native_fixture_equivalence().unwrap();
        println!("{}", serde_json::to_string(&report).unwrap());
        assert!(report.inference_replay_equivalent);
        assert!(report.training_path_equivalent);
        assert!(report.zero_prediction_accuracy_correct);
        assert!(report.predictive_corrections > 0);
    }
}
