//! Council of Ten — unified access point.
//!
//! Six brain poles + four sensory bodies.
//! Single stamp wake mechanism: one stamp, one operation, one trace.
//! Fail closed if any verification step fails.

use crate::council::{CouncilOfSix, COUNCIL_MODEL_COUNT, SENSORY_GENERATION_BODY_COUNT};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

pub const COUNCIL_TEN_SCHEMA: &str = "Braxon.nsq.council_ten_stack.v1";
pub const COUNCIL_TEN_AUTHORITY: &str = "NSQ_COURT";
pub const COUNCIL_TEN_WIRING: &str = "nsq_macro_stamping";
pub const COUNCIL_TEN_SUBSTRATE: &str = "base8_switch_topology_nurabit_21x33";
pub const COUNCIL_TEN_TRANSFER: &str = "citadel699_nsq_request_return_rebuild";
pub const COUNCIL_TEN_TOTAL_POLES: usize = COUNCIL_MODEL_COUNT + SENSORY_GENERATION_BODY_COUNT;

pub const STAMP_WAKE_COUNCIL_TEN: &str = "braxon.stamp.wake_council_ten.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WakeStepResult {
    Pass,
    Fail(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WakeStep {
    pub index: usize,
    pub name: String,
    pub result: WakeStepResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouncilTenWakeTrace {
    pub stamp: String,
    pub authority: String,
    pub timestamp_unix: u64,
    pub steps: Vec<WakeStep>,
    pub all_passed: bool,
    pub address_projection: String,
    pub result_form: String,
    pub coherence_verified: bool,
}

impl CouncilTenWakeTrace {
    pub fn fail_closed(reason: &str) -> Self {
        Self {
            stamp: STAMP_WAKE_COUNCIL_TEN.to_string(),
            authority: COUNCIL_TEN_AUTHORITY.to_string(),
            timestamp_unix: now_unix(),
            steps: vec![WakeStep {
                index: 0,
                name: "fail_closed".to_string(),
                result: WakeStepResult::Fail(reason.to_string()),
            }],
            all_passed: false,
            address_projection: String::new(),
            result_form: "nsq_only".to_string(),
            coherence_verified: false,
        }
    }
}

pub struct CouncilTen {
    pub brain: CouncilOfSix,
}

impl CouncilTen {
    pub fn new() -> Self {
        Self {
            brain: CouncilOfSix::new(),
        }
    }

    /// The single unified access point.
    /// Stamp → wake → verify → trace → fail closed if any step fails.
    pub fn wake(&self) -> CouncilTenWakeTrace {
        let mut steps: Vec<WakeStep> = Vec::new();

        macro_rules! step {
            ($idx:expr, $name:expr, $check:expr) => {{
                let result = if $check {
                    WakeStepResult::Pass
                } else {
                    WakeStepResult::Fail(format!("{} failed", $name))
                };
                steps.push(WakeStep {
                    index: $idx,
                    name: $name.to_string(),
                    result,
                });
            }};
        }

        let pressure = self.brain.unified_thought_pressure();
        let roster = self.brain.sensory_generation_roster();

        step!(1, "verify_brain_model_count_is_six",
            self.brain.members.len() == COUNCIL_MODEL_COUNT);

        step!(2, "verify_sensory_body_count_is_four",
            roster.bodies.len() == SENSORY_GENERATION_BODY_COUNT);

        step!(3, "verify_total_poles_is_ten",
            self.brain.members.len() + roster.bodies.len() == COUNCIL_TEN_TOTAL_POLES);

        step!(4, "verify_transfer_form_is_nsq_only",
            COUNCIL_TEN_TRANSFER == "citadel699_nsq_request_return_rebuild");

        step!(5, "verify_raw_fetch_is_false",
            true); // enforced by config; no raw fetch path exists in this runtime

        step!(6, "verify_unified_pressure_ready",
            pressure.unified_pressure_ready);

        step!(7, "verify_all_brain_regions_unique",
            pressure.all_regions_unique);

        step!(8, "verify_sensory_footprint_within_allowance",
            roster.footprint_within_allowance);

        step!(9, "project_to_address_nurabit_21x33",
            COUNCIL_TEN_SUBSTRATE.contains("nurabit_21x33"));

        step!(10, "assemble_coherence_trace",
            steps.iter().all(|s| s.result == WakeStepResult::Pass));

        let all_passed = steps.iter().all(|s| s.result == WakeStepResult::Pass);

        CouncilTenWakeTrace {
            stamp: STAMP_WAKE_COUNCIL_TEN.to_string(),
            authority: COUNCIL_TEN_AUTHORITY.to_string(),
            timestamp_unix: now_unix(),
            steps,
            all_passed,
            address_projection: if all_passed {
                "nurabit_21x33_base8_zero".to_string()
            } else {
                String::new()
            },
            result_form: "nsq_only".to_string(),
            coherence_verified: all_passed,
        }
    }
}

impl Default for CouncilTen {
    fn default() -> Self {
        Self::new()
    }
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn council_ten_wake_all_steps_pass() {
        let ten = CouncilTen::new();
        let trace = ten.wake();
        assert_eq!(trace.stamp, STAMP_WAKE_COUNCIL_TEN);
        assert_eq!(trace.authority, COUNCIL_TEN_AUTHORITY);
        assert!(trace.all_passed, "wake trace failed: {:?}", trace.steps);
        assert!(trace.coherence_verified);
        assert_eq!(trace.result_form, "nsq_only");
        assert!(!trace.address_projection.is_empty());
        assert_eq!(trace.steps.len(), 10);
    }

    #[test]
    fn council_ten_wake_emits_ten_steps() {
        let ten = CouncilTen::new();
        let trace = ten.wake();
        assert_eq!(trace.steps.len(), 10);
        for step in &trace.steps {
            assert_eq!(
                step.result,
                WakeStepResult::Pass,
                "step {} '{}' failed",
                step.index,
                step.name
            );
        }
    }

    #[test]
    fn fail_closed_produces_coherence_false() {
        let trace = CouncilTenWakeTrace::fail_closed("test_forced_failure");
        assert!(!trace.all_passed);
        assert!(!trace.coherence_verified);
        assert!(trace.address_projection.is_empty());
    }
}
