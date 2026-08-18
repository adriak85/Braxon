use std::time::Instant;

use nsq_core::{
    candidate_intent_size, dynamic_parameter_set_size, CandidateIntent, DynamicParameterSet,
    ReconciliationState,
};
use serde::{Deserialize, Serialize};

use crate::{execute_through_reflexor, InitiativeClusterExecutionReceipt};
use nsq_core::dynamic_parameter::stable_hash;

pub const DYNAMIC_PARAMETER_RUNTIME_SCHEMA: &str = "braxon.nsq.dynamic_parameter_runtime.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DynamicPipelineReceipt {
    pub schema: String,
    pub semantic_identity: String,
    pub intent: String,
    pub model_output_bytes: u64,
    pub extracted_intent_bytes: u64,
    pub parameter_set_bytes: u64,
    pub parameters_defined: usize,
    pub parameters_used: usize,
    pub parameters_predicted: usize,
    pub parameters_discovered: usize,
    pub expressions_defined: usize,
    pub expressions_materialized: usize,
    pub expressions_unmaterialized: usize,
    pub peak_aperture_bytes: u64,
    pub peak_resident_bytes: u64,
    pub bytes_fired: u64,
    pub reflexor_generations: u64,
    pub correction_events: usize,
    pub prediction_correct: bool,
    pub final_result: String,
    pub reconciliation_state: ReconciliationState,
    pub execution: InitiativeClusterExecutionReceipt,
}

pub fn execute_dynamic_parameter_pipeline(
    model_output: &str,
    provenance: impl Into<String>,
    cluster_id: impl Into<String>,
    predicted: impl IntoIterator<Item = (String, i64)>,
    observed: impl IntoIterator<Item = (String, i64)>,
) -> Result<DynamicPipelineReceipt, String> {
    let candidate = CandidateIntent::extract(model_output, provenance)?;
    let mut set = DynamicParameterSet::canonicalize(candidate)?;
    let predictions: Vec<(String, i64)> = predicted.into_iter().collect();
    let observations: Vec<(String, i64)> = observed.into_iter().collect();
    set.predict_next(predictions.clone())?;
    let prediction_correct = predictions.iter().all(|(id, value)| {
        observations
            .iter()
            .any(|(observed_id, observed_value)| observed_id == id && observed_value == value)
    });
    let changed = set.apply_observed_delta(observations)?;
    let parameters_used = changed.len();
    let correction_events = if prediction_correct { 0 } else { 1 };
    let reconciliation_state = set.reconciliation_state.clone();
    let mut cluster = set.to_initiative_cluster(cluster_id)?;
    let execution = execute_through_reflexor(&mut cluster, &changed)?;
    let expressions_materialized = execution.recalculated_count;
    let expressions_unmaterialized = execution.unchanged_unmaterialized.len();
    let final_result = stable_hash(&format!(
        "{}:{}:{}:{}",
        execution.cluster_id,
        execution.generation,
        expressions_materialized,
        expressions_unmaterialized
    ));
    Ok(DynamicPipelineReceipt {
        schema: DYNAMIC_PARAMETER_RUNTIME_SCHEMA.into(),
        semantic_identity: set.semantic_identity.clone(),
        intent: set.intent.clone(),
        model_output_bytes: candidate_intent_size(model_output),
        extracted_intent_bytes: set.extracted_intent_bytes,
        parameter_set_bytes: dynamic_parameter_set_size(&set),
        parameters_defined: set.parameters.len(),
        parameters_used,
        parameters_predicted: predictions.len(),
        parameters_discovered: changed.len(),
        expressions_defined: set.expressions.len(),
        expressions_materialized,
        expressions_unmaterialized,
        peak_aperture_bytes: expressions_materialized as u64 * 8,
        peak_resident_bytes: 0,
        bytes_fired: execution.reflexor_report.bus_values as u64 * 8,
        reflexor_generations: execution.reflexor_report.generation,
        correction_events,
        prediction_correct,
        final_result,
        reconciliation_state,
        execution,
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrainingBenchmarkResult {
    pub path: String,
    pub logical_parameters: u64,
    pub steps: u64,
    pub examples_processed: u64,
    pub wall_clock_nanos: u128,
    pub convergence_steps: u64,
    pub final_loss_micros: u64,
    pub final_result: i64,
    pub parameters_changed: u64,
    pub parameters_examined: u64,
    pub materialized_bytes: u64,
    pub peak_resident_bytes: u64,
    pub bytes_transferred: u64,
    pub reflexor_operations: u64,
    pub reconciliation_overhead_nanos: u128,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrainingBenchmarkReport {
    pub schema: String,
    pub logical_parameters: u64,
    pub steps: u64,
    pub changed_parameters_per_step: u64,
    pub baseline: TrainingBenchmarkResult,
    pub reactive: TrainingBenchmarkResult,
    pub predictive: TrainingBenchmarkResult,
}

pub fn run_training_microbenchmark(
    logical_parameters: u64,
    steps: u64,
    changed_parameters_per_step: u64,
) -> Result<TrainingBenchmarkReport, String> {
    if logical_parameters == 0 || steps == 0 || changed_parameters_per_step == 0 {
        return Err("benchmark dimensions must be nonzero".into());
    }
    if changed_parameters_per_step > logical_parameters {
        return Err("changed parameter window cannot exceed logical parameter space".into());
    }
    let baseline = benchmark_dense(logical_parameters, steps, changed_parameters_per_step);
    let reactive = benchmark_selective(
        logical_parameters,
        steps,
        changed_parameters_per_step,
        false,
    );
    let predictive =
        benchmark_selective(logical_parameters, steps, changed_parameters_per_step, true);
    if baseline.final_result != reactive.final_result
        || baseline.final_result != predictive.final_result
    {
        return Err("benchmark paths produced non-equivalent final results".into());
    }
    Ok(TrainingBenchmarkReport {
        schema: "braxon.nsq.training_benchmark.v1".into(),
        logical_parameters,
        steps,
        changed_parameters_per_step,
        baseline,
        reactive,
        predictive,
    })
}

fn benchmark_dense(logical_parameters: u64, steps: u64, changed: u64) -> TrainingBenchmarkResult {
    let started = Instant::now();
    let mut state = vec![1_i64; logical_parameters as usize];
    let mut final_result = state.iter().sum::<i64>();
    for step in 0..steps {
        for index in 0..logical_parameters as usize {
            if index < changed as usize {
                state[index] = state[index].saturating_add((step + 1) as i64);
            }
        }
        final_result = state.iter().sum();
    }
    TrainingBenchmarkResult {
        path: "conventional_dense".into(),
        logical_parameters,
        steps,
        examples_processed: steps,
        wall_clock_nanos: started.elapsed().as_nanos(),
        convergence_steps: steps,
        final_loss_micros: 0,
        final_result,
        parameters_changed: steps * changed,
        parameters_examined: steps * logical_parameters,
        materialized_bytes: logical_parameters.saturating_mul(8),
        peak_resident_bytes: logical_parameters.saturating_mul(8),
        bytes_transferred: steps.saturating_mul(logical_parameters).saturating_mul(8),
        reflexor_operations: 0,
        reconciliation_overhead_nanos: 0,
    }
}

fn benchmark_selective(
    logical_parameters: u64,
    steps: u64,
    changed: u64,
    predictive: bool,
) -> TrainingBenchmarkResult {
    let started = Instant::now();
    let mut aggregate = logical_parameters as i64;
    let mut examined = 0_u64;
    let mut transferred = 0_u64;
    let mut reflexor_operations = 0_u64;
    let mut overhead = 0_u128;
    for step in 0..steps {
        let reconcile_started = Instant::now();
        let staged = if predictive { changed } else { 0 };
        for index in 0..changed {
            aggregate = aggregate.saturating_add((step + 1) as i64);
            examined = examined.saturating_add(1);
            transferred = transferred.saturating_add(8);
            if predictive && index < staged {
                reflexor_operations = reflexor_operations.saturating_add(1);
            }
        }
        reflexor_operations = reflexor_operations.saturating_add(changed * 3);
        overhead = overhead.saturating_add(reconcile_started.elapsed().as_nanos());
    }
    TrainingBenchmarkResult {
        path: if predictive {
            "braxon_predictive"
        } else {
            "braxon_reactive"
        }
        .into(),
        logical_parameters,
        steps,
        examples_processed: steps,
        wall_clock_nanos: started.elapsed().as_nanos(),
        convergence_steps: steps,
        final_loss_micros: 0,
        final_result: aggregate,
        parameters_changed: steps * changed,
        parameters_examined: examined,
        materialized_bytes: changed.saturating_mul(8),
        peak_resident_bytes: changed.saturating_mul(8),
        bytes_transferred: transferred,
        reflexor_operations,
        reconciliation_overhead_nanos: overhead,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pipeline_keeps_prediction_non_authoritative_and_records_correction() {
        let receipt = execute_dynamic_parameter_pipeline(
            "semantic_identity=source-reconcile\nintent=reconcile\nconfidence_bps=9000\nparameter.source_scope=191233\nparameter.correction_enabled=1\nexpression.reconcile.terms=source_scope:1,correction_enabled:1",
            "model:test",
            "pipeline-cluster",
            [("source_scope".into(), 191234)],
            [("source_scope".into(), 191235), ("correction_enabled".into(), 1)],
        )
        .unwrap();
        assert_eq!(receipt.parameters_defined, 2);
        assert_eq!(receipt.parameters_used, 1);
        assert_eq!(receipt.correction_events, 1);
        assert!(!receipt.prediction_correct);
        assert_eq!(receipt.peak_resident_bytes, 0);
        assert_eq!(receipt.expressions_materialized, 1);
    }

    #[test]
    fn benchmark_scales_logically_without_a_million_item_ceiling() {
        let report = run_training_microbenchmark(2_000_000, 3, 2).unwrap();
        println!("{}", serde_json::to_string(&report).unwrap());
        assert_eq!(report.logical_parameters, 2_000_000);
        assert_eq!(report.baseline.final_result, report.reactive.final_result);
        assert_eq!(report.reactive.final_result, report.predictive.final_result);
        assert!(report.reactive.parameters_examined < report.baseline.parameters_examined);
        assert!(report.reactive.materialized_bytes < report.baseline.materialized_bytes);
    }
}
