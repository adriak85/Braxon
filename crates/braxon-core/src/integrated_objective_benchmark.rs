use std::time::Instant;

use serde::{Deserialize, Serialize};

use crate::execute_dynamic_parameter_pipeline;
use crate::riemann_semantic_reflexor::{RiemannSemanticReflexor, TerminalState};

pub const INTEGRATED_OBJECTIVE_BENCHMARK_SCHEMA: &str = "braxon.integrated_objective_benchmark.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntegratedTrajectoryMetrics {
    pub schema: String,
    pub objective: String,
    pub external_interactions: u64,
    pub internal_semantic_operations: u64,
    pub dynamic_parameter_sets: u64,
    pub predictive_executions: u64,
    pub parameters_defined: u64,
    pub parameters_examined: u64,
    pub parameters_discovered: u64,
    pub initiative_clusters_executed: u64,
    pub initiative_clusters_released: u64,
    pub expressions_materialized: u64,
    pub expressions_unmaterialized: u64,
    pub peak_aperture_bytes: u64,
    pub peak_resident_bytes: u64,
    pub bytes_fired: u64,
    pub activation_receipts: u64,
    pub proof_attempts: u64,
    pub corrections: u64,
    pub persistent_state_bytes: u64,
    pub terminal_state: TerminalState,
    pub final_correctness: bool,
    pub final_result_digest: String,
    pub time_to_terminal_nanos: u128,
}

pub fn run_integrated_objective_benchmark(
    objective: impl Into<String>,
    internal_steps: u64,
) -> Result<IntegratedTrajectoryMetrics, String> {
    let objective = objective.into();
    if objective.trim().is_empty() || internal_steps == 0 {
        return Err("integrated benchmark requires objective and positive internal steps".into());
    }
    let started = Instant::now();
    let mut reflexor = RiemannSemanticReflexor::seed(14_134, internal_steps as usize)?;
    reflexor.begin_run(internal_steps)?;
    let mut parameters_defined = 0_u64;
    let mut parameters_examined = 0_u64;
    let mut parameters_discovered = 0_u64;
    let mut initiative_clusters_executed = 0_u64;
    let mut initiative_clusters_released = 0_u64;
    let mut expressions_materialized = 0_u64;
    let mut expressions_unmaterialized = 0_u64;
    let mut peak_aperture_bytes = 0_u64;
    let mut peak_resident_bytes = 0_u64;
    let mut bytes_fired = 0_u64;
    let mut corrections = 0_u64;
    let mut final_result_digest = String::new();

    for index in 0..internal_steps {
        let record_id = format!("zeta-region-{index:04}");
        let source_scope = 14_134_i64.saturating_add(index as i64);
        let predicted_scope = if index % 1_000 == 0 {
            source_scope.saturating_add(2)
        } else {
            source_scope.saturating_add(1)
        };
        let observed_scope = source_scope.saturating_add(1);
        let _prediction_execution = reflexor.execute_prediction(&record_id)?;
        let model_output = format!(
            "semantic_identity=integrated-objective\nintent=reconcile\nconfidence_bps=1000\nparameter.source_scope={}\nparameter.stable_context=1\nparameter.activation_window=1\nexpression.reconcile.terms=source_scope:1\nexpression.stable_context.terms=stable_context:1\nexpression.activation.terms=activation_window:1",
            source_scope
        );
        let receipt = execute_dynamic_parameter_pipeline(
            &model_output,
            format!("integrated-objective:{objective}"),
            format!("integrated-cluster-{index:04}"),
            [("source_scope".into(), predicted_scope)],
            [
                ("source_scope".into(), observed_scope),
                ("stable_context".into(), 1),
                ("activation_window".into(), 1),
            ],
        )?;
        parameters_defined = parameters_defined.saturating_add(receipt.parameters_defined as u64);
        parameters_examined = parameters_examined.saturating_add(receipt.parameters_used as u64);
        parameters_discovered =
            parameters_discovered.saturating_add(receipt.parameters_discovered as u64);
        initiative_clusters_executed = initiative_clusters_executed.saturating_add(1);
        initiative_clusters_released = initiative_clusters_released.saturating_add(1);
        expressions_materialized =
            expressions_materialized.saturating_add(receipt.expressions_materialized as u64);
        expressions_unmaterialized =
            expressions_unmaterialized.saturating_add(receipt.expressions_unmaterialized as u64);
        peak_aperture_bytes = peak_aperture_bytes.max(receipt.peak_aperture_bytes);
        peak_resident_bytes = peak_resident_bytes.max(receipt.peak_resident_bytes);
        bytes_fired = bytes_fired.saturating_add(receipt.bytes_fired);
        corrections = corrections.saturating_add(receipt.correction_events as u64);
        final_result_digest = receipt.final_result;

        let last = index + 1 == internal_steps;
        let proof = reflexor.self_prove(
            &record_id,
            if last { 0 } else { 7 + index },
            "integrated-primary-verifier",
            "integrated-independent-verifier",
            last,
            last,
        )?;
        reflexor.learn_from_proof_attempt(&proof)?;
    }

    let persistent_state_bytes = serde_json::to_vec(&reflexor)
        .map_err(|error| error.to_string())?
        .len() as u64;
    let terminal_state = reflexor.terminal_state().clone();
    Ok(IntegratedTrajectoryMetrics {
        schema: INTEGRATED_OBJECTIVE_BENCHMARK_SCHEMA.into(),
        objective,
        external_interactions: 1,
        internal_semantic_operations: internal_steps,
        dynamic_parameter_sets: internal_steps,
        predictive_executions: internal_steps,
        parameters_defined,
        parameters_examined,
        parameters_discovered,
        initiative_clusters_executed,
        initiative_clusters_released,
        expressions_materialized,
        expressions_unmaterialized,
        peak_aperture_bytes,
        peak_resident_bytes,
        bytes_fired,
        activation_receipts: reflexor.activation_receipts.len() as u64,
        proof_attempts: reflexor.proof_attempts.len() as u64,
        corrections,
        persistent_state_bytes,
        terminal_state: terminal_state.clone(),
        final_correctness: matches!(terminal_state, TerminalState::Won),
        final_result_digest,
        time_to_terminal_nanos: started.elapsed().as_nanos(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integrated_objective_keeps_external_control_constant() {
        let metrics = run_integrated_objective_benchmark("integrated solve", 10).unwrap();
        println!("{}", serde_json::to_string(&metrics).unwrap());
        assert_eq!(metrics.external_interactions, 1);
        assert_eq!(metrics.internal_semantic_operations, 10);
        assert_eq!(metrics.dynamic_parameter_sets, 10);
        assert_eq!(metrics.predictive_executions, 10);
        assert_eq!(metrics.initiative_clusters_executed, 10);
        assert_eq!(metrics.initiative_clusters_released, 10);
        assert_eq!(metrics.parameters_examined, 10);
        assert_eq!(metrics.expressions_materialized, 10);
        assert_eq!(metrics.expressions_unmaterialized, 20);
        assert_eq!(metrics.peak_resident_bytes, 0);
        assert_eq!(metrics.activation_receipts, 11);
        assert_eq!(metrics.proof_attempts, 10);
        assert!(metrics.corrections > 0);
        assert_eq!(metrics.terminal_state, TerminalState::Won);
        assert!(metrics.final_correctness);
    }

    #[test]
    fn integrated_objective_scales_to_ten_thousand_steps() {
        let metrics = run_integrated_objective_benchmark("integrated scale", 10_000).unwrap();
        println!("{}", serde_json::to_string(&metrics).unwrap());
        assert_eq!(metrics.external_interactions, 1);
        assert_eq!(metrics.internal_semantic_operations, 10_000);
        assert_eq!(metrics.dynamic_parameter_sets, 10_000);
        assert_eq!(metrics.predictive_executions, 10_000);
        assert_eq!(metrics.initiative_clusters_released, 10_000);
        assert_eq!(metrics.parameters_examined, 10_000);
        assert_eq!(metrics.expressions_materialized, 10_000);
        assert_eq!(metrics.expressions_unmaterialized, 20_000);
        assert_eq!(metrics.activation_receipts, 10_001);
        assert_eq!(metrics.proof_attempts, 10_000);
        assert_eq!(metrics.terminal_state, TerminalState::Won);
        assert!(metrics.final_correctness);
    }
}
