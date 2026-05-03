use nsq_core::NsqSurfaceValue;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BitObjectState {
    Idle,
    Active,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum HandoffMode {
    Scan,
    Communicate,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BitObject {
    pub id: String,
    pub addressed_bit: NsqSurfaceValue,
    pub current_surface: String,
    pub state: BitObjectState,
    pub duty_cycles: usize,
    pub life_extensions: usize,
    pub completed_jobs: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BitJob {
    pub id: String,
    pub addressed_bit: NsqSurfaceValue,
    pub operation: String,
    pub route_id: String,
    pub handoff_mode: HandoffMode,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LaneControl {
    pub route_id: String,
    pub stages: Vec<String>,
    pub communication_process: String,
    pub allowed_handoffs: Vec<HandoffMode>,
    pub strict: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BitJobBoardReport {
    pub strict_lane_controls: bool,
    pub non_consumptive_cycles: bool,
    pub object_migration_enabled: bool,
    pub scan_or_communicate_only: bool,
    pub idle_pool_ready: bool,
    pub completed_job_count: usize,
    pub pending_job_count: usize,
    pub idle_bit_count: usize,
    pub life_extension_total: usize,
    pub journal: Vec<String>,
}

#[derive(Debug, Clone)]
struct RamJobBoard {
    idle_bits: Vec<BitObject>,
    pending_jobs: VecDeque<BitJob>,
    completed_jobs: Vec<BitJob>,
    lane_controls: Vec<LaneControl>,
    journal: Vec<String>,
}

pub fn simulate_bit_job_board(prompt: &str) -> BitJobBoardReport {
    let addressed_primary = NsqSurfaceValue::one();
    let addressed_secondary = NsqSurfaceValue::new("2").unwrap();
    let route = canonical_lane_control();

    let mut board = RamJobBoard {
        idle_bits: vec![
            BitObject {
                id: "bit_alpha".to_string(),
                addressed_bit: addressed_primary.clone(),
                current_surface: "ram_job_board".to_string(),
                state: BitObjectState::Idle,
                duty_cycles: 0,
                life_extensions: 0,
                completed_jobs: 0,
            },
            BitObject {
                id: "bit_beta".to_string(),
                addressed_bit: addressed_secondary.clone(),
                current_surface: "ram_job_board".to_string(),
                state: BitObjectState::Idle,
                duty_cycles: 0,
                life_extensions: 0,
                completed_jobs: 0,
            },
        ],
        pending_jobs: VecDeque::from([BitJob {
            id: "prompt_runtime_job".to_string(),
            addressed_bit: addressed_primary,
            operation: format!("logical_route_for_prompt_chars_{}", prompt.trim().chars().count()),
            route_id: route.route_id.clone(),
            handoff_mode: HandoffMode::Communicate,
        }]),
        completed_jobs: Vec::new(),
        lane_controls: vec![route],
        journal: Vec::new(),
    };

    while board.run_next_cycle().is_ok() {}

    BitJobBoardReport {
        strict_lane_controls: board.lane_controls.iter().all(|lane| lane.strict),
        non_consumptive_cycles: board
            .idle_bits
            .iter()
            .all(|bit| bit.state == BitObjectState::Idle),
        object_migration_enabled: board
            .journal
            .iter()
            .any(|entry| entry.contains("migrated:lexor->linter")),
        scan_or_communicate_only: board
            .lane_controls
            .iter()
            .all(|lane| lane.allowed_handoffs.iter().all(|mode| {
                matches!(mode, HandoffMode::Scan | HandoffMode::Communicate)
            })),
        idle_pool_ready: !board.idle_bits.is_empty(),
        completed_job_count: board.completed_jobs.len(),
        pending_job_count: board.pending_jobs.len(),
        idle_bit_count: board.idle_bits.len(),
        life_extension_total: board
            .idle_bits
            .iter()
            .map(|bit| bit.life_extensions)
            .sum::<usize>(),
        journal: board.journal,
    }
}

impl RamJobBoard {
    fn run_next_cycle(&mut self) -> Result<(), String> {
        let Some(job) = self.pending_jobs.pop_front() else {
            return Err("job_board_empty".to_string());
        };
        let lane = self
            .lane_controls
            .iter()
            .find(|lane| lane.route_id == job.route_id)
            .cloned()
            .ok_or_else(|| format!("unknown_route:{}", job.route_id))?;
        if !lane.allowed_handoffs.contains(&job.handoff_mode) {
            return Err(format!("handoff_not_allowed:{}", job.route_id));
        }

        let bit = self
            .idle_bits
            .iter_mut()
            .find(|bit| bit.addressed_bit == job.addressed_bit)
            .ok_or_else(|| format!("idle_bit_missing:{}", job.addressed_bit.as_text()))?;

        bit.state = BitObjectState::Active;
        self.journal.push(format!(
            "{}:picked:{}:{}",
            bit.id,
            job.id,
            job.addressed_bit.as_text()
        ));

        for window in lane.stages.windows(2) {
            let from = &window[0];
            let to = &window[1];
            bit.current_surface = to.clone();
            self.journal.push(format!(
                "{}:migrated:{}->{}:{}",
                bit.id, from, to, lane.communication_process
            ));
        }

        bit.duty_cycles += 1;
        bit.life_extensions += lane.stages.len();
        bit.completed_jobs += 1;
        bit.state = BitObjectState::Idle;
        bit.current_surface = "ram_job_board".to_string();

        self.journal.push(format!(
            "{}:returned_to_idle_pool:duty_cycles={}:life_extensions={}",
            bit.id, bit.duty_cycles, bit.life_extensions
        ));
        self.completed_jobs.push(job);
        Ok(())
    }
}

fn canonical_lane_control() -> LaneControl {
    LaneControl {
        route_id: "lexor_linter_transform_picker_compositor_route".to_string(),
        stages: vec![
            "lexor".to_string(),
            "linter".to_string(),
            "self_transform".to_string(),
            "picker".to_string(),
            "compositor".to_string(),
            "ram_job_board".to_string(),
        ],
        communication_process: "scan_then_communicate".to_string(),
        allowed_handoffs: vec![HandoffMode::Scan, HandoffMode::Communicate],
        strict: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn job_board_keeps_bit_objects_in_circulation_instead_of_consuming_them() {
        let report = simulate_bit_job_board("repair secure transit");
        assert!(report.strict_lane_controls);
        assert!(report.non_consumptive_cycles);
        assert!(report.object_migration_enabled);
        assert!(report.scan_or_communicate_only);
        assert!(report.idle_pool_ready);
        assert_eq!(report.completed_job_count, 1);
        assert_eq!(report.pending_job_count, 0);
        assert!(report.life_extension_total >= 6);
        assert!(report
            .journal
            .iter()
            .any(|entry| entry.contains("returned_to_idle_pool")));
    }
}
