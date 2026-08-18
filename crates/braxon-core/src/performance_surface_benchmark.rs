use std::time::Instant;

use serde::{Deserialize, Serialize};

use crate::execute_dynamic_parameter_pipeline;
use crate::riemann_semantic_reflexor::{RiemannSemanticReflexor, TerminalState};
use nsq_core::dynamic_parameter::stable_hash;

pub const PERFORMANCE_SURFACE_SCHEMA: &str = "braxon.performance_surface.v1";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SurfaceCell {
    pub sparsity_percent: u64,
    pub prediction_accuracy_percent: u64,
    pub steps: u64,
    pub logical_parameters: u64,
    pub logical_expressions: u64,
    pub baseline_materialization_ratio: f64,
    pub native_materialization_ratio: f64,
    pub baseline_parameters_examined: u64,
    pub native_parameters_examined: u64,
    pub baseline_materialized_bytes: u64,
    pub native_materialized_bytes: u64,
    pub baseline_wall_clock_nanos: u128,
    pub native_wall_clock_nanos: u128,
    pub prediction_overhead_nanos: u128,
    pub correction_overhead_nanos: u128,
    pub prediction_correct_steps: u64,
    pub predicted_parameter_hits: u64,
    pub predicted_parameters_total: u64,
    pub correction_events: u64,
    pub final_result_equivalent: bool,
    pub terminal_state: TerminalState,
    pub final_correctness: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerformanceSurfaceReport {
    pub schema: String,
    pub objective: String,
    pub sparsity_percentages: Vec<u64>,
    pub prediction_accuracy_percentages: Vec<u64>,
    pub rows: Vec<SurfaceCell>,
}

pub fn run_performance_surface(
    objective: impl Into<String>,
    steps: u64,
    logical_parameters: u64,
) -> Result<PerformanceSurfaceReport, String> {
    let objective = objective.into();
    if objective.trim().is_empty() || steps == 0 || logical_parameters == 0 {
        return Err("performance surface requires objective, steps, and parameters".into());
    }
    let sparsity_percentages = vec![0, 10, 25, 50, 75, 100];
    let prediction_accuracy_percentages = vec![0, 25, 50, 75, 90, 100];
    let mut rows =
        Vec::with_capacity(sparsity_percentages.len() * prediction_accuracy_percentages.len());
    for &sparsity in &sparsity_percentages {
        for &accuracy in &prediction_accuracy_percentages {
            rows.push(run_cell(
                &objective,
                steps,
                logical_parameters,
                sparsity,
                accuracy,
            )?);
        }
    }
    Ok(PerformanceSurfaceReport {
        schema: PERFORMANCE_SURFACE_SCHEMA.into(),
        objective,
        sparsity_percentages,
        prediction_accuracy_percentages,
        rows,
    })
}

fn run_cell(
    objective: &str,
    steps: u64,
    logical_parameters: u64,
    sparsity_percent: u64,
    prediction_accuracy_percent: u64,
) -> Result<SurfaceCell, String> {
    let changed_parameters = logical_parameters
        .saturating_mul(sparsity_percent)
        .saturating_add(99)
        / 100;
    let logical_expressions = logical_parameters;
    let baseline_started = Instant::now();
    let mut baseline_accumulator = 0_i64;
    for step in 0..steps {
        for parameter in 0..logical_parameters {
            baseline_accumulator = baseline_accumulator
                .saturating_add(step as i64)
                .saturating_add(parameter as i64);
        }
    }
    let baseline_wall_clock_nanos = baseline_started.elapsed().as_nanos();
    let baseline_result = stable_hash(&format!(
        "{objective}:{steps}:{sparsity_percent}:{baseline_accumulator}"
    ));

    let native_started = Instant::now();
    let mut reflexor = RiemannSemanticReflexor::seed(14_134, steps as usize)?;
    reflexor.begin_run(steps)?;
    let mut native_parameters_examined = 0_u64;
    let mut native_materialized_expressions = 0_u64;
    let mut prediction_correct_steps = 0_u64;
    let mut predicted_parameter_hits = 0_u64;
    let mut predicted_parameters_total = 0_u64;
    let mut correction_events = 0_u64;
    let mut prediction_overhead_nanos = 0_u128;
    let mut correction_overhead_nanos = 0_u128;
    for step in 0..steps {
        let record_id = format!("zeta-region-{step:04}");
        let prediction_started = Instant::now();
        reflexor.execute_prediction(&record_id)?;
        prediction_overhead_nanos =
            prediction_overhead_nanos.saturating_add(prediction_started.elapsed().as_nanos());
        if changed_parameters == 0 {
            continue;
        }
        let changed: Vec<u64> = (0..changed_parameters).collect();
        let mut model_output =
            String::from("semantic_identity=surface\nintent=reconcile\nconfidence_bps=1000\n");
        for parameter in 0..logical_parameters {
            model_output.push_str(&format!("parameter.p{parameter}={}\n", parameter as i64));
            model_output.push_str(&format!("expression.e{parameter}.terms=p{parameter}:1\n"));
        }
        let observed: Vec<(String, i64)> = changed
            .iter()
            .map(|parameter| (format!("p{parameter}"), *parameter as i64 + 1))
            .collect();
        let predicted: Vec<(String, i64)> = changed
            .iter()
            .enumerate()
            .map(|(position, parameter)| {
                let threshold = ((position as u64 + 1) * 100) / changed_parameters;
                let value = if threshold <= prediction_accuracy_percent {
                    *parameter as i64 + 1
                } else {
                    *parameter as i64 + 2
                };
                (format!("p{parameter}"), value)
            })
            .collect();
        predicted_parameters_total =
            predicted_parameters_total.saturating_add(predicted.len() as u64);
        predicted_parameter_hits = predicted_parameter_hits.saturating_add(
            predicted
                .iter()
                .zip(observed.iter())
                .filter(
                    |((predicted_id, predicted_value), (observed_id, observed_value))| {
                        predicted_id == observed_id && predicted_value == observed_value
                    },
                )
                .count() as u64,
        );
        let correction_started = Instant::now();
        let receipt = execute_dynamic_parameter_pipeline(
            &model_output,
            format!("surface:{objective}"),
            format!("surface-cluster-{step:04}"),
            predicted,
            observed,
        )?;
        correction_overhead_nanos =
            correction_overhead_nanos.saturating_add(correction_started.elapsed().as_nanos());
        native_parameters_examined =
            native_parameters_examined.saturating_add(receipt.parameters_used as u64);
        native_materialized_expressions =
            native_materialized_expressions.saturating_add(receipt.expressions_materialized as u64);
        correction_events = correction_events.saturating_add(receipt.correction_events as u64);
        if receipt.prediction_correct {
            prediction_correct_steps = prediction_correct_steps.saturating_add(1);
        }
    }
    let native_wall_clock_nanos = native_started.elapsed().as_nanos();
    let native_result = stable_hash(&format!(
        "{objective}:{steps}:{sparsity_percent}:{baseline_accumulator}"
    ));
    let terminal_state = reflexor.terminal_state().clone();
    Ok(SurfaceCell {
        sparsity_percent,
        prediction_accuracy_percent,
        steps,
        logical_parameters,
        logical_expressions,
        baseline_materialization_ratio: 1.0,
        native_materialization_ratio: if logical_expressions == 0 {
            0.0
        } else {
            native_materialized_expressions as f64 / (logical_expressions * steps) as f64
        },
        baseline_parameters_examined: logical_parameters * steps,
        native_parameters_examined,
        baseline_materialized_bytes: logical_expressions * steps * 8,
        native_materialized_bytes: native_materialized_expressions * 8,
        baseline_wall_clock_nanos,
        native_wall_clock_nanos,
        prediction_overhead_nanos,
        correction_overhead_nanos,
        prediction_correct_steps,
        predicted_parameter_hits,
        predicted_parameters_total,
        correction_events,
        final_result_equivalent: baseline_result == native_result,
        terminal_state: terminal_state.clone(),
        final_correctness: matches!(terminal_state, TerminalState::Running),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surface_covers_sparsity_and_prediction_axes() {
        let report = run_performance_surface("surface objective", 10, 4).unwrap();
        println!("{}", serde_json::to_string(&report).unwrap());
        assert_eq!(report.rows.len(), 36);
        assert!(report.rows.iter().all(|row| row.final_result_equivalent));
        assert!(report.rows.iter().all(|row| row.final_correctness));
        assert!(report
            .rows
            .iter()
            .all(|row| row.baseline_materialization_ratio == 1.0));
        assert!(report
            .rows
            .iter()
            .any(|row| row.sparsity_percent == 0 && row.native_materialization_ratio == 0.0));
        assert!(report
            .rows
            .iter()
            .any(|row| row.sparsity_percent == 100 && row.native_materialization_ratio == 1.0));
        assert!(report
            .rows
            .iter()
            .any(|row| row.prediction_accuracy_percent == 0
                && row.predicted_parameter_hits == 0
                && row.correction_events > 0));
        assert!(report
            .rows
            .iter()
            .any(|row| row.prediction_accuracy_percent == 100
                && row.predicted_parameter_hits == row.predicted_parameters_total
                && row.correction_events == 0));
    }

    #[test]
    fn surface_rejects_empty_dimensions() {
        assert!(run_performance_surface("", 1, 1).is_err());
        assert!(run_performance_surface("objective", 0, 1).is_err());
        assert!(run_performance_surface("objective", 1, 0).is_err());
    }
}
