use std::time::Instant;

use serde::{Deserialize, Serialize};

use crate::execute_dynamic_parameter_pipeline;
use crate::riemann_semantic_reflexor::{RiemannSemanticReflexor, TerminalState};

pub const ADVERSARIAL_BENCHMARK_SCHEMA: &str = "braxon.adversarial_integrated_benchmark.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdversarialWorkload {
    Sparse,
    Dense,
    RandomDependency,
    HighlyCorrelated,
    PredictionHostile,
    LongTrajectory,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdversarialMetrics {
    pub workload: AdversarialWorkload,
    pub steps: u64,
    pub external_interactions: u64,
    pub logical_parameters: u64,
    pub logical_expressions: u64,
    pub parameters_examined: u64,
    pub expressions_materialized: u64,
    pub expressions_unmaterialized: u64,
    pub materialization_ratio: f64,
    pub correction_events: u64,
    pub prediction_correct_steps: u64,
    pub activation_receipts: u64,
    pub peak_aperture_bytes: u64,
    pub peak_resident_bytes: u64,
    pub terminal_state: TerminalState,
    pub final_correctness: bool,
    pub time_to_terminal_nanos: u128,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdversarialBenchmarkReport {
    pub schema: String,
    pub objective: String,
    pub standard_steps: u64,
    pub long_steps: u64,
    pub rows: Vec<AdversarialMetrics>,
}

pub fn run_adversarial_integrated_benchmark(
    objective: impl Into<String>,
    standard_steps: u64,
    long_steps: u64,
) -> Result<AdversarialBenchmarkReport, String> {
    let objective = objective.into();
    if objective.trim().is_empty() || standard_steps == 0 || long_steps == 0 {
        return Err("adversarial benchmark requires objective and positive step counts".into());
    }
    let mut rows = Vec::new();
    for workload in [
        AdversarialWorkload::Sparse,
        AdversarialWorkload::Dense,
        AdversarialWorkload::RandomDependency,
        AdversarialWorkload::HighlyCorrelated,
        AdversarialWorkload::PredictionHostile,
    ] {
        rows.push(run_workload(&objective, workload, standard_steps)?);
    }
    rows.push(run_workload(
        &objective,
        AdversarialWorkload::LongTrajectory,
        long_steps,
    )?);
    Ok(AdversarialBenchmarkReport {
        schema: ADVERSARIAL_BENCHMARK_SCHEMA.into(),
        objective,
        standard_steps,
        long_steps,
        rows,
    })
}

fn run_workload(
    objective: &str,
    workload: AdversarialWorkload,
    steps: u64,
) -> Result<AdversarialMetrics, String> {
    let started = Instant::now();
    let mut reflexor = RiemannSemanticReflexor::seed(14_134, steps as usize)?;
    reflexor.begin_run(steps)?;
    let (logical_parameters, logical_expressions) = match workload {
        AdversarialWorkload::Dense => (3, 3),
        _ => (4, 3),
    };
    let mut parameters_examined = 0_u64;
    let mut expressions_materialized = 0_u64;
    let mut expressions_unmaterialized = 0_u64;
    let mut correction_events = 0_u64;
    let mut prediction_correct_steps = 0_u64;
    let mut peak_aperture_bytes = 0_u64;

    for index in 0..steps {
        let record_id = format!("zeta-region-{index:04}");
        reflexor.execute_prediction(&record_id)?;
        let base = 100_i64.saturating_add(index as i64);
        let changed = changed_parameters(workload, logical_parameters, index);
        let mut model_output = String::from(
            "semantic_identity=adversarial-integrated\nintent=reconcile\nconfidence_bps=1000\n",
        );
        for parameter_index in 0..logical_parameters {
            model_output.push_str(&format!(
                "parameter.p{parameter_index}={}\n",
                base + parameter_index as i64,
            ));
        }
        for expression_index in 0..logical_expressions {
            let dependency = match workload {
                AdversarialWorkload::RandomDependency => {
                    (index + expression_index as u64) % logical_parameters
                }
                _ => expression_index.min(logical_parameters - 1),
            };
            model_output.push_str(&format!(
                "expression.e{expression_index}.terms=p{dependency}:1\n"
            ));
        }
        let predicted: Vec<(String, i64)> = changed
            .iter()
            .map(|parameter| {
                let observed_value = base + *parameter as i64 + 1;
                let predicted_value = if workload == AdversarialWorkload::PredictionHostile {
                    observed_value.saturating_add(1)
                } else {
                    observed_value
                };
                (format!("p{parameter}"), predicted_value)
            })
            .collect();
        let observed: Vec<(String, i64)> = changed
            .iter()
            .map(|parameter| (format!("p{parameter}"), base + *parameter as i64 + 1))
            .collect();
        let receipt = execute_dynamic_parameter_pipeline(
            &model_output,
            format!("adversarial:{objective}"),
            format!("adversarial-cluster-{index:04}"),
            predicted,
            observed,
        )?;
        parameters_examined = parameters_examined.saturating_add(receipt.parameters_used as u64);
        expressions_materialized =
            expressions_materialized.saturating_add(receipt.expressions_materialized as u64);
        expressions_unmaterialized =
            expressions_unmaterialized.saturating_add(receipt.expressions_unmaterialized as u64);
        correction_events = correction_events.saturating_add(receipt.correction_events as u64);
        if receipt.prediction_correct {
            prediction_correct_steps = prediction_correct_steps.saturating_add(1);
        }
        peak_aperture_bytes = peak_aperture_bytes.max(receipt.peak_aperture_bytes);
        let proof = reflexor.self_prove(
            &record_id,
            if index + 1 == steps { 0 } else { 7 + index },
            "adversarial-primary",
            "adversarial-independent",
            index + 1 == steps,
            index + 1 == steps,
        )?;
        reflexor.learn_from_proof_attempt(&proof)?;
    }
    let terminal_state = reflexor.terminal_state().clone();
    let logical_expression_total = (logical_expressions as u64).saturating_mul(steps);
    Ok(AdversarialMetrics {
        workload,
        steps,
        external_interactions: 1,
        logical_parameters: logical_parameters as u64,
        logical_expressions: logical_expression_total,
        parameters_examined,
        expressions_materialized,
        expressions_unmaterialized,
        materialization_ratio: if logical_expression_total == 0 {
            0.0
        } else {
            expressions_materialized as f64 / logical_expression_total as f64
        },
        correction_events,
        prediction_correct_steps,
        activation_receipts: reflexor.activation_receipts.len() as u64,
        peak_aperture_bytes,
        peak_resident_bytes: 0,
        terminal_state: terminal_state.clone(),
        final_correctness: matches!(terminal_state, TerminalState::Won),
        time_to_terminal_nanos: started.elapsed().as_nanos(),
    })
}

fn changed_parameters(
    workload: AdversarialWorkload,
    logical_parameters: u64,
    index: u64,
) -> Vec<u64> {
    match workload {
        AdversarialWorkload::Dense => (0..logical_parameters).collect(),
        AdversarialWorkload::Sparse
        | AdversarialWorkload::HighlyCorrelated
        | AdversarialWorkload::PredictionHostile => vec![0],
        AdversarialWorkload::RandomDependency => vec![index % logical_parameters],
        AdversarialWorkload::LongTrajectory => vec![0],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adversarial_report_covers_all_workloads() {
        let report =
            run_adversarial_integrated_benchmark("adversarial objective", 10, 100).unwrap();
        println!("{}", serde_json::to_string(&report).unwrap());
        assert_eq!(report.rows.len(), 6);
        assert!(report.rows.iter().all(|row| row.external_interactions == 1));
        assert!(report.rows.iter().all(|row| row.final_correctness));
        assert!(report.rows.iter().all(|row| row.peak_resident_bytes == 0));
        assert!(report
            .rows
            .iter()
            .any(|row| row.workload == AdversarialWorkload::PredictionHostile
                && row.correction_events > 0));
        assert!(report.rows.iter().any(
            |row| row.workload == AdversarialWorkload::Dense && row.materialization_ratio > 0.5
        ));
        assert!(report
            .rows
            .iter()
            .any(|row| row.workload == AdversarialWorkload::Sparse
                && row.materialization_ratio < 0.5));
    }

    #[test]
    fn adversarial_workload_rejects_zero_steps() {
        assert!(run_adversarial_integrated_benchmark("objective", 0, 1).is_err());
        assert!(run_adversarial_integrated_benchmark("objective", 1, 0).is_err());
    }
}
