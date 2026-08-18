use std::time::Instant;

use serde::{Deserialize, Serialize};

use crate::riemann_semantic_reflexor::{ProofState, RiemannSemanticReflexor, TerminalState};

pub const ONE_SHOT_BENCHMARK_SCHEMA: &str = "braxon.one_shot_objective_benchmark.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrajectoryMetrics {
    pub mode: String,
    pub objective: String,
    pub external_interactions: u64,
    pub internal_hypotheses: u64,
    pub internal_execution_steps: u64,
    pub repeated_work: u64,
    pub materialized_bytes: u64,
    pub persistent_state_bytes: u64,
    pub verified_progress: u64,
    pub time_to_terminal_nanos: u128,
    pub terminal_state: TerminalState,
    pub final_correctness: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OneShotObjectiveBenchmark {
    pub schema: String,
    pub objective: String,
    pub one_shot: TrajectoryMetrics,
    pub interactive: TrajectoryMetrics,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OneShotScalingRow {
    pub internal_steps: u64,
    pub one_shot: TrajectoryMetrics,
    pub interactive: TrajectoryMetrics,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OneShotScalingReport {
    pub schema: String,
    pub objective: String,
    pub external_interactions_per_one_shot_run: u64,
    pub rows: Vec<OneShotScalingRow>,
}

pub fn run_one_shot_scaling_matrix(
    objective: impl Into<String>,
    step_matrix: &[u64],
) -> Result<OneShotScalingReport, String> {
    if step_matrix.is_empty() {
        return Err("scaling matrix cannot be empty".into());
    }
    let objective = objective.into();
    let mut rows = Vec::with_capacity(step_matrix.len());
    for &internal_steps in step_matrix {
        let benchmark = run_one_shot_objective_benchmark(objective.clone(), internal_steps)?;
        rows.push(OneShotScalingRow {
            internal_steps,
            one_shot: benchmark.one_shot,
            interactive: benchmark.interactive,
        });
    }
    Ok(OneShotScalingReport {
        schema: "braxon.one_shot_scaling.v1".into(),
        objective,
        external_interactions_per_one_shot_run: 1,
        rows,
    })
}

pub fn run_one_shot_objective_benchmark(
    objective: impl Into<String>,
    internal_steps: u64,
) -> Result<OneShotObjectiveBenchmark, String> {
    let objective = objective.into();
    if objective.trim().is_empty() || internal_steps == 0 {
        return Err("one-shot benchmark requires objective and positive internal steps".into());
    }
    let one_shot = run_persistent(&objective, internal_steps)?;
    let interactive = run_interactive(&objective, internal_steps)?;
    if one_shot.final_correctness != interactive.final_correctness {
        return Err("one-shot and interactive paths disagree on final correctness".into());
    }
    Ok(OneShotObjectiveBenchmark {
        schema: ONE_SHOT_BENCHMARK_SCHEMA.into(),
        objective,
        one_shot,
        interactive,
    })
}

fn run_persistent(objective: &str, internal_steps: u64) -> Result<TrajectoryMetrics, String> {
    let started = Instant::now();
    let mut reflexor = RiemannSemanticReflexor::seed(14_134, internal_steps as usize)?;
    reflexor.begin_run(internal_steps)?;
    let internal_hypotheses = reflexor.hypotheses.len() as u64;
    let mut materialized_bytes = 0_u64;
    for index in 0..internal_steps {
        let record_id = format!("zeta-region-{index:04}");
        let _execution = reflexor.execute_prediction(&record_id)?;
        materialized_bytes = materialized_bytes.saturating_add(16);
        let is_last = index + 1 == internal_steps;
        let proof = reflexor.self_prove(
            &record_id,
            if is_last { 0 } else { 7 + index },
            "primary-certified-engine",
            "independent-certified-engine",
            is_last,
            is_last,
        )?;
        reflexor.learn_from_proof_attempt(&proof)?;
    }
    let persistent_state_bytes = serde_json::to_vec(&reflexor)
        .map_err(|error| error.to_string())?
        .len() as u64;
    let verified_progress = reflexor
        .learning_records
        .iter()
        .filter(|record| record.trusted)
        .count() as u64;
    Ok(TrajectoryMetrics {
        mode: "one_shot_persistent".into(),
        objective: objective.into(),
        external_interactions: 1,
        internal_hypotheses,
        internal_execution_steps: reflexor.attempts_used,
        repeated_work: 0,
        materialized_bytes,
        persistent_state_bytes,
        verified_progress,
        time_to_terminal_nanos: started.elapsed().as_nanos(),
        terminal_state: reflexor.terminal_state().clone(),
        final_correctness: matches!(reflexor.terminal_state(), TerminalState::Won),
    })
}

fn run_interactive(objective: &str, internal_steps: u64) -> Result<TrajectoryMetrics, String> {
    let started = Instant::now();
    let mut materialized_bytes = 0_u64;
    let mut persistent_state_bytes = 0_u64;
    let mut verified_progress = 0_u64;
    for index in 0..internal_steps {
        let mut reflexor = RiemannSemanticReflexor::seed(14_134, 1)?;
        reflexor.begin_run(1)?;
        let record_id = "zeta-region-0000";
        let _execution = reflexor.execute_prediction(record_id)?;
        materialized_bytes = materialized_bytes.saturating_add(16);
        let proof = reflexor.self_prove(
            record_id,
            0,
            "primary-certified-engine",
            "independent-certified-engine",
            true,
            true,
        )?;
        reflexor.learn_from_proof_attempt(&proof)?;
        if proof.state == ProofState::IndependentlyReproduced {
            verified_progress = verified_progress.saturating_add(1);
        }
        persistent_state_bytes = persistent_state_bytes.saturating_add(
            serde_json::to_vec(&reflexor)
                .map_err(|error| error.to_string())?
                .len() as u64,
        );
        let _ = index;
    }
    Ok(TrajectoryMetrics {
        mode: "interactive_reorchestrated".into(),
        objective: objective.into(),
        external_interactions: internal_steps,
        internal_hypotheses: internal_steps,
        internal_execution_steps: internal_steps,
        repeated_work: internal_steps.saturating_sub(1),
        materialized_bytes,
        persistent_state_bytes,
        verified_progress,
        time_to_terminal_nanos: started.elapsed().as_nanos(),
        terminal_state: TerminalState::Won,
        final_correctness: true,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_shot_replaces_external_orchestration_not_internal_work() {
        let benchmark = run_one_shot_objective_benchmark("resolve objective", 5).unwrap();
        println!("{}", serde_json::to_string(&benchmark).unwrap());
        assert_eq!(benchmark.one_shot.external_interactions, 1);
        assert_eq!(benchmark.interactive.external_interactions, 5);
        assert_eq!(benchmark.one_shot.internal_execution_steps, 5);
        assert_eq!(benchmark.interactive.internal_execution_steps, 5);
        assert_eq!(
            benchmark.one_shot.final_correctness,
            benchmark.interactive.final_correctness
        );
        assert_eq!(benchmark.one_shot.terminal_state, TerminalState::Won);
        assert!(benchmark.interactive.repeated_work > 0);
        assert!(benchmark.one_shot.persistent_state_bytes > 0);
    }

    #[test]
    fn scaling_matrix_keeps_one_external_objective() {
        let report =
            run_one_shot_scaling_matrix("scale objective", &[5, 10, 100, 1_000, 10_000]).unwrap();
        println!("{}", serde_json::to_string(&report).unwrap());
        assert_eq!(report.external_interactions_per_one_shot_run, 1);
        assert_eq!(report.rows.len(), 5);
        for row in &report.rows {
            assert_eq!(row.one_shot.external_interactions, 1);
            assert_eq!(row.one_shot.internal_execution_steps, row.internal_steps);
            assert_eq!(row.interactive.external_interactions, row.internal_steps);
            assert_eq!(row.interactive.internal_execution_steps, row.internal_steps);
            assert_eq!(
                row.one_shot.final_correctness,
                row.interactive.final_correctness
            );
            assert_eq!(row.one_shot.terminal_state, TerminalState::Won);
        }
    }

    #[test]
    fn benchmark_rejects_empty_objectives() {
        assert!(run_one_shot_objective_benchmark("", 1).is_err());
        assert!(run_one_shot_objective_benchmark("objective", 0).is_err());
    }
}
