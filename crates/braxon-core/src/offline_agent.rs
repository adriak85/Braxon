use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub const OFFLINE_AGENT_STATE_RELATIVE_PATH: &str = "state/braxon/offline_agent_state.json";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OfflineTaskStatus {
    Pending,
    InProgress,
    Done,
    Blocked,
}

impl OfflineTaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::InProgress => "in_progress",
            Self::Done => "done",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OfflineTaskAction {
    VerifyRootLaunchPath,
    VerifyTaskQueueControl,
    VerifyPython3RuntimeLane,
    PrepareOfflineModelLane,
    RegisterBRAXONCoreAsset,
    PrepareWowasFinalProse,
}

impl OfflineTaskAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::VerifyRootLaunchPath => "verify_root_launch_path",
            Self::VerifyTaskQueueControl => "verify_task_queue_control",
            Self::VerifyPython3RuntimeLane => "verify_python3_runtime_lane",
            Self::PrepareOfflineModelLane => "prepare_offline_model_lane",
            Self::RegisterBRAXONCoreAsset => "register_BRAXON_core_asset",
            Self::PrepareWowasFinalProse => "prepare_wowas_final_prose",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OfflineTaskVerification {
    pub verifier: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OfflineTask {
    pub id: String,
    pub phase: String,
    pub title: String,
    pub detail: String,
    pub status: OfflineTaskStatus,
    pub action: OfflineTaskAction,
    pub last_verification: Option<OfflineTaskVerification>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OfflineAgentState {
    pub mission: String,
    pub charter: String,
    pub cycle_count: usize,
    pub tasks: Vec<OfflineTask>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OfflineTaskCounts {
    pub pending: usize,
    pub in_progress: usize,
    pub done: usize,
    pub blocked: usize,
}

impl OfflineAgentState {
    pub fn default_mission() -> Self {
        Self {
            mission: "BRAXON_offline_first".to_string(),
            charter: "boot offline, work the local task list, verify each step, then expand native runtime and model lanes under Braxon".to_string(),
            cycle_count: 0,
            tasks: vec![
                OfflineTask {
                    id: "BRAXON_offline_agent_boot".to_string(),
                    phase: "phase_1".to_string(),
                    title: "Braxon offline agent boot".to_string(),
                    detail: "The root launch path must boot Braxon as an actual offline agent surface.".to_string(),
                    status: OfflineTaskStatus::Pending,
                    action: OfflineTaskAction::VerifyRootLaunchPath,
                    last_verification: None,
                },
                OfflineTask {
                    id: "BRAXON_self_list_execution".to_string(),
                    phase: "phase_1".to_string(),
                    title: "Braxon self-list execution".to_string(),
                    detail: "Braxon must inspect, select, mark, execute, verify, and continue through the internal queue.".to_string(),
                    status: OfflineTaskStatus::Pending,
                    action: OfflineTaskAction::VerifyTaskQueueControl,
                    last_verification: None,
                },
                OfflineTask {
                    id: "nsq_native_runtime_lane_smoke".to_string(),
                    phase: "phase_1".to_string(),
                    title: "NSQ native runtime lane smoke".to_string(),
                    detail: "Braxon must exercise a native NSQ runtime lane through the root agent surface.".to_string(),
                    status: OfflineTaskStatus::Pending,
                    action: OfflineTaskAction::VerifyPython3RuntimeLane,
                    last_verification: None,
                },
                OfflineTask {
                    id: "nsq_native_offline_model_lane".to_string(),
                    phase: "phase_2".to_string(),
                    title: "NSQ-native offline model lane".to_string(),
                    detail: "Create an explicit offline inference/runtime lane under Braxon and NSQ without C++ authority.".to_string(),
                    status: OfflineTaskStatus::Pending,
                    action: OfflineTaskAction::PrepareOfflineModelLane,
                    last_verification: None,
                },
                OfflineTask {
                    id: "BRAXON_core_under_BRAXON".to_string(),
                    phase: "phase_2".to_string(),
                    title: "BRAXON Core under Braxon".to_string(),
                    detail: "Register BRAXON Core as an offline runtime asset under the native Braxon path.".to_string(),
                    status: OfflineTaskStatus::Pending,
                    action: OfflineTaskAction::RegisterBRAXONCoreAsset,
                    last_verification: None,
                },
                OfflineTask {
                    id: "wowas_final_prose_through_BRAXON".to_string(),
                    phase: "phase_4".to_string(),
                    title: "WoWaS final prose through Braxon".to_string(),
                    detail: "Final prose assembly happens only after Braxon can carry the work through its own queue.".to_string(),
                    status: OfflineTaskStatus::Pending,
                    action: OfflineTaskAction::PrepareWowasFinalProse,
                    last_verification: None,
                },
            ],
        }
    }

    pub fn counts(&self) -> OfflineTaskCounts {
        let mut counts = OfflineTaskCounts {
            pending: 0,
            in_progress: 0,
            done: 0,
            blocked: 0,
        };

        for task in &self.tasks {
            match task.status {
                OfflineTaskStatus::Pending => counts.pending += 1,
                OfflineTaskStatus::InProgress => counts.in_progress += 1,
                OfflineTaskStatus::Done => counts.done += 1,
                OfflineTaskStatus::Blocked => counts.blocked += 1,
            }
        }

        counts
    }

    pub fn next_actionable(&self) -> Option<&OfflineTask> {
        self.tasks
            .iter()
            .find(|task| task.status == OfflineTaskStatus::Pending)
    }

    pub fn task(&self, id: &str) -> Option<&OfflineTask> {
        self.tasks.iter().find(|task| task.id == id)
    }

    pub fn mark_status(&mut self, id: &str, status: OfflineTaskStatus) -> Result<(), String> {
        let task = self
            .tasks
            .iter_mut()
            .find(|task| task.id == id)
            .ok_or_else(|| format!("unknown offline task '{id}'"))?;
        task.status = status;
        Ok(())
    }

    pub fn record_success(
        &mut self,
        id: &str,
        verifier: impl Into<String>,
        summary: impl Into<String>,
    ) -> Result<(), String> {
        let task = self
            .tasks
            .iter_mut()
            .find(|task| task.id == id)
            .ok_or_else(|| format!("unknown offline task '{id}'"))?;
        task.status = OfflineTaskStatus::Done;
        task.last_verification = Some(OfflineTaskVerification {
            verifier: verifier.into(),
            summary: summary.into(),
        });
        self.cycle_count += 1;
        Ok(())
    }

    pub fn record_failure(
        &mut self,
        id: &str,
        verifier: impl Into<String>,
        summary: impl Into<String>,
    ) -> Result<(), String> {
        let task = self
            .tasks
            .iter_mut()
            .find(|task| task.id == id)
            .ok_or_else(|| format!("unknown offline task '{id}'"))?;
        task.status = OfflineTaskStatus::Blocked;
        task.last_verification = Some(OfflineTaskVerification {
            verifier: verifier.into(),
            summary: summary.into(),
        });
        self.cycle_count += 1;
        Ok(())
    }
}

pub fn offline_agent_state_path(root: &Path) -> PathBuf {
    root.join(OFFLINE_AGENT_STATE_RELATIVE_PATH)
}

pub fn load_or_initialize_offline_agent_state(root: &Path) -> Result<OfflineAgentState, String> {
    let path = offline_agent_state_path(root);
    if path.exists() {
        let raw = fs::read_to_string(&path).map_err(|err| {
            format!(
                "failed to read offline agent state '{}': {err}",
                path.display()
            )
        })?;
        serde_json::from_str(&raw).map_err(|err| {
            format!(
                "failed to parse offline agent state '{}': {err}",
                path.display()
            )
        })
    } else {
        let state = OfflineAgentState::default_mission();
        save_offline_agent_state(root, &state)?;
        Ok(state)
    }
}

pub fn save_offline_agent_state(root: &Path, state: &OfflineAgentState) -> Result<(), String> {
    let path = offline_agent_state_path(root);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| {
            format!(
                "failed to create offline agent state directory '{}': {err}",
                parent.display()
            )
        })?;
    }

    let body = serde_json::to_string_pretty(state)
        .map_err(|err| format!("failed to serialize offline agent state: {err}"))?;
    fs::write(&path, body).map_err(|err| {
        format!(
            "failed to write offline agent state '{}': {err}",
            path.display()
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_mission_starts_with_phase_one_boot_tasks() {
        let state = OfflineAgentState::default_mission();
        assert_eq!(state.tasks[0].id, "BRAXON_offline_agent_boot");
        assert_eq!(state.tasks[1].id, "BRAXON_self_list_execution");
        assert_eq!(state.tasks[2].id, "nsq_native_runtime_lane_smoke");
    }

    #[test]
    fn next_actionable_skips_completed_items() {
        let mut state = OfflineAgentState::default_mission();
        state
            .mark_status("BRAXON_offline_agent_boot", OfflineTaskStatus::Done)
            .unwrap();
        assert_eq!(
            state.next_actionable().map(|task| task.id.as_str()),
            Some("BRAXON_self_list_execution")
        );
    }

    #[test]
    fn record_success_preserves_verification_summary() {
        let mut state = OfflineAgentState::default_mission();
        state
            .record_success(
                "BRAXON_offline_agent_boot",
                "offline_agent",
                "root launch path verified",
            )
            .unwrap();
        let task = state.task("BRAXON_offline_agent_boot").unwrap();
        assert_eq!(task.status, OfflineTaskStatus::Done);
        assert_eq!(
            task.last_verification.as_ref().unwrap().summary,
            "root launch path verified"
        );
    }
}
