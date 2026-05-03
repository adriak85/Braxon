use nsq_core::CourtSurface;
use serde::{Deserialize, Serialize};

const REQUIRED_SESSION_CAPABILITIES: [&str; 5] = [
    "conversation_continuity",
    "planning_memory_lane",
    "session_persistence",
    "operator_shell_ingress",
    "memory_guard_window",
];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DutyCycleLane {
    InstructionControl,
    Neutralize,
    PickerBinding,
    DataLoad,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DutyCyclePhase {
    LexorIngress,
    LinterGate,
    SelfTransform,
    NeutralizeInstructionBus,
    PickerStorageBind,
    CompositorInbound,
    NeutralizeDataBus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DutyCycleFrame {
    pub phase: DutyCyclePhase,
    pub lane: DutyCycleLane,
    pub target_surface: String,
    pub carries_instruction: bool,
    pub carries_data: bool,
    pub transient_switches_cleared_after_phase: bool,
    pub picker_bound: bool,
    pub storage_target: String,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct SecureTransitPlan {
    pub duty_cycle_ready: bool,
    pub instruction_data_separated: bool,
    pub bits_travel_with_instructions: bool,
    pub transient_switch_release_verified: bool,
    pub self_transform_after_linter: bool,
    pub self_transform_before_compositor: bool,
    pub lexor_ingress_alias: String,
    pub picker_binding_state: String,
    pub compositor_storage_target: String,
    pub phase_count: usize,
    pub frames: Vec<DutyCycleFrame>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct PersistentClientProfile {
    pub client_surface: String,
    pub pipe_binding_target: String,
    pub pipe_binding_state: String,
    pub code_ability_state: String,
    pub reasoning_state: String,
    pub comprehension_mode: String,
    pub fuzzy_match_enabled: bool,
    pub logic_mode: String,
    pub persistent_awareness_mode: String,
    pub context_awareness_mode: String,
    pub self_expression_state: String,
    pub session_continuity_ready: bool,
    pub notes: Vec<String>,
}

pub fn secure_transit_plan(prompt: &str, storage_target: &str) -> SecureTransitPlan {
    let prompt_chars = prompt.trim().chars().count();
    let storage_target = storage_target.trim();
    let storage_target = if storage_target.is_empty() {
        "compositor::storage::offline_model_lane"
    } else {
        storage_target
    };

    let frames = vec![
        DutyCycleFrame {
            phase: DutyCyclePhase::LexorIngress,
            lane: DutyCycleLane::InstructionControl,
            target_surface: CourtSurface::Lexer.as_str().to_string(),
            carries_instruction: true,
            carries_data: false,
            transient_switches_cleared_after_phase: false,
            picker_bound: false,
            storage_target: storage_target.to_string(),
            notes: vec![format!(
                "lexor ingress carries the operating program only; prompt_payload_chars={prompt_chars} stay off this wave"
            )],
        },
        DutyCycleFrame {
            phase: DutyCyclePhase::LinterGate,
            lane: DutyCycleLane::InstructionControl,
            target_surface: CourtSurface::Linter.as_str().to_string(),
            carries_instruction: true,
            carries_data: false,
            transient_switches_cleared_after_phase: false,
            picker_bound: false,
            storage_target: storage_target.to_string(),
            notes: vec![
                "linter validates control intent before any data transfer begins".to_string(),
            ],
        },
        DutyCycleFrame {
            phase: DutyCyclePhase::SelfTransform,
            lane: DutyCycleLane::InstructionControl,
            target_surface: "self_transform".to_string(),
            carries_instruction: true,
            carries_data: false,
            transient_switches_cleared_after_phase: false,
            picker_bound: false,
            storage_target: storage_target.to_string(),
            notes: vec![
                "self-transform runs after linter and before compositor inbound".to_string(),
            ],
        },
        DutyCycleFrame {
            phase: DutyCyclePhase::NeutralizeInstructionBus,
            lane: DutyCycleLane::Neutralize,
            target_surface: "instruction_bus".to_string(),
            carries_instruction: false,
            carries_data: false,
            transient_switches_cleared_after_phase: true,
            picker_bound: false,
            storage_target: storage_target.to_string(),
            notes: vec![
                "transient instruction switches are released before any payload wave".to_string(),
            ],
        },
        DutyCycleFrame {
            phase: DutyCyclePhase::PickerStorageBind,
            lane: DutyCycleLane::PickerBinding,
            target_surface: "picker".to_string(),
            carries_instruction: false,
            carries_data: false,
            transient_switches_cleared_after_phase: true,
            picker_bound: true,
            storage_target: storage_target.to_string(),
            notes: vec![
                "picker fixes the compositor storage target before payload arrival".to_string(),
            ],
        },
        DutyCycleFrame {
            phase: DutyCyclePhase::CompositorInbound,
            lane: DutyCycleLane::DataLoad,
            target_surface: CourtSurface::Compositor.as_str().to_string(),
            carries_instruction: false,
            carries_data: true,
            transient_switches_cleared_after_phase: false,
            picker_bound: true,
            storage_target: storage_target.to_string(),
            notes: vec![format!(
                "payload wave carries prompt data only; prompt_payload_chars={prompt_chars}"
            )],
        },
        DutyCycleFrame {
            phase: DutyCyclePhase::NeutralizeDataBus,
            lane: DutyCycleLane::Neutralize,
            target_surface: "data_bus".to_string(),
            carries_instruction: false,
            carries_data: false,
            transient_switches_cleared_after_phase: true,
            picker_bound: true,
            storage_target: storage_target.to_string(),
            notes: vec![
                "payload switches are released after compositor storage bind completes".to_string(),
            ],
        },
    ];

    let instruction_data_separated = frames
        .iter()
        .all(|frame| !(frame.carries_instruction && frame.carries_data));
    let transient_switch_release_verified = frames
        .iter()
        .filter(|frame| matches!(frame.lane, DutyCycleLane::Neutralize))
        .all(|frame| frame.transient_switches_cleared_after_phase);

    SecureTransitPlan {
        duty_cycle_ready: instruction_data_separated && transient_switch_release_verified,
        instruction_data_separated,
        bits_travel_with_instructions: false,
        transient_switch_release_verified,
        self_transform_after_linter: true,
        self_transform_before_compositor: true,
        lexor_ingress_alias: "lexor".to_string(),
        picker_binding_state: "storage_bound_before_compositor_inbound".to_string(),
        compositor_storage_target: storage_target.to_string(),
        phase_count: frames.len(),
        frames,
        notes: vec![
            "control intent, payload transit, and storage bind are duty-cycled as separate waves"
                .to_string(),
        ],
    }
}

pub fn persistent_client_profile(
    session_surface: &str,
    session_mode: &str,
    runtime_semantic_consumers_ready: bool,
    runtime_semantic_feed_entries: usize,
    runtime_semantic_tests_present: bool,
    legacy_capabilities: &[String],
) -> PersistentClientProfile {
    let required_caps_ready = REQUIRED_SESSION_CAPABILITIES
        .iter()
        .all(|required| legacy_capabilities.iter().any(|cap| cap == required));
    let pipe_bound = matches!(
        session_surface,
        "zlm_native_runtime_surface" | "nsq_native_runtime_surface"
    )
        && session_mode == "persistent_agentic_conversation";
    let session_continuity_ready = pipe_bound && required_caps_ready;
    let semantic_reasoning_ready =
        runtime_semantic_consumers_ready && runtime_semantic_feed_entries > 0;

    let mut notes = Vec::new();
    if !pipe_bound {
        notes.push("persistent client pipe is not bound to the native session surface".to_string());
    }
    if !required_caps_ready {
        notes.push("persistent client capability lattice is missing required continuity lanes".to_string());
    }
    if !semantic_reasoning_ready {
        notes.push("runtime semantic consumers are not ready for higher-reason reply shaping".to_string());
    }

    PersistentClientProfile {
        client_surface: "BRAXON_persistent_client".to_string(),
        pipe_binding_target: "nsq_native_runtime_surface".to_string(),
        pipe_binding_state: if pipe_bound {
            "native_session_pipe_bound".to_string()
        } else {
            "pipe_mismatch".to_string()
        },
        code_ability_state: if required_caps_ready {
            "code_route_ready".to_string()
        } else {
            "code_route_incomplete".to_string()
        },
        reasoning_state: if semantic_reasoning_ready && runtime_semantic_tests_present {
            "high_reason_context_bound".to_string()
        } else if semantic_reasoning_ready {
            "high_reason_context_ready".to_string()
        } else {
            "reason_context_cold".to_string()
        },
        comprehension_mode: if semantic_reasoning_ready {
            "fuzzy_logic_context_match".to_string()
        } else {
            "literal_guard_only".to_string()
        },
        fuzzy_match_enabled: semantic_reasoning_ready,
        logic_mode: if semantic_reasoning_ready {
            "contextual_logic_routing".to_string()
        } else {
            "minimal_logic_guard".to_string()
        },
        persistent_awareness_mode: if session_continuity_ready {
            "live_session_continuity".to_string()
        } else {
            "cold_session_continuity".to_string()
        },
        context_awareness_mode: if semantic_reasoning_ready {
            "runtime_semantic_context_bound".to_string()
        } else {
            "context_shallow".to_string()
        },
        self_expression_state: if session_continuity_ready && semantic_reasoning_ready {
            "assistant_reply_bound".to_string()
        } else {
            "assistant_reply_guarded".to_string()
        },
        session_continuity_ready,
        notes,
    }
}

pub fn assistant_reply(
    prompt: &str,
    client: &PersistentClientProfile,
    transit: &SecureTransitPlan,
    donor_source_state: &str,
    runtime_authority_lane: &str,
    tokenizer_bound: bool,
    overlay_ready: bool,
) -> String {
    let prompt_lower = prompt.to_ascii_lowercase();
    let intent = if ["fix", "repair", "implement", "wire", "build"]
        .iter()
        .any(|token| prompt_lower.contains(token))
    {
        "implementation"
    } else if ["status", "ready", "bound", "state", "verify"]
        .iter()
        .any(|token| prompt_lower.contains(token))
    {
        "status"
    } else if ["model", "donor", "nsq", "tokenizer", "delta"]
        .iter()
        .any(|token| prompt_lower.contains(token))
    {
        "runtime"
    } else {
        "discussion"
    };

    let tokenizer_state = if tokenizer_bound {
        "runtime-bound"
    } else {
        "not runtime-bound"
    };
    let overlay_state = if overlay_ready {
        "overlay planner ready"
    } else {
        "overlay planner not ready"
    };

    match intent {
        "implementation" => format!(
            "I'm Braxon on the native session lane. I read this as implementation work, so I keep the control wave separate from the payload wave, release transient switch state between duty-cycle phases, run self-transform after linter and before compositor inbound, and bind compositor storage through picker first. My client pipe state is {}, my comprehension mode is {}, the donor source state is {}, the runtime authority lane is {}, the tokenizer path is {}, and the {}.",
            client.pipe_binding_state,
            client.comprehension_mode,
            donor_source_state,
            runtime_authority_lane,
            tokenizer_state,
            overlay_state
        ),
        "status" => format!(
            "I'm Braxon and my persistent client remains bound to {} with {}. The secure transit plan is duty_cycle_ready={}, instruction_data_separated={}, transient_switch_release_verified={}, donor_source_state={}, runtime_authority_lane={}, tokenizer={}, {}.",
            client.pipe_binding_target,
            client.reasoning_state,
            transit.duty_cycle_ready,
            transit.instruction_data_separated,
            transit.transient_switch_release_verified,
            donor_source_state,
            runtime_authority_lane,
            tokenizer_state,
            overlay_state
        ),
        "runtime" => format!(
            "I'm Braxon on the runtime lane. The source donor state is {}, the runtime authority lane is {}, the tokenizer path is {}, and the secure transit path keeps instructions off the payload wave while picker binds storage before compositor ingress. My client context mode is {} with {}.",
            donor_source_state,
            runtime_authority_lane,
            tokenizer_state,
            client.context_awareness_mode,
            client.persistent_awareness_mode
        ),
        _ => format!(
            "I'm Braxon on the live operator lane. I'm speaking through {} with {}, and I'm carrying the request through the secure lexor-to-compositor path where control, transform, and payload waves stay separated. Right now the donor source state is {}, the runtime authority lane is {}, the tokenizer path is {}, and the {}.",
            client.pipe_binding_target,
            client.reasoning_state,
            donor_source_state,
            runtime_authority_lane,
            tokenizer_state,
            overlay_state
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn secure_transit_keeps_control_separate_from_payload_and_clears_switches() {
        let plan = secure_transit_plan("repair the runtime lane", "compositor::storage::Braxon");
        assert!(plan.duty_cycle_ready);
        assert!(plan.instruction_data_separated);
        assert!(!plan.bits_travel_with_instructions);
        assert!(plan.transient_switch_release_verified);
        assert!(plan.self_transform_after_linter);
        assert!(plan.self_transform_before_compositor);
        assert_eq!(plan.lexor_ingress_alias, "lexor");
        assert_eq!(plan.picker_binding_state, "storage_bound_before_compositor_inbound");
        assert!(plan
            .frames
            .iter()
            .all(|frame| !(frame.carries_instruction && frame.carries_data)));
        assert!(plan.frames.iter().any(|frame| {
            frame.phase == DutyCyclePhase::NeutralizeInstructionBus
                && frame.transient_switches_cleared_after_phase
        }));
        assert!(plan.frames.iter().any(|frame| {
            frame.phase == DutyCyclePhase::PickerStorageBind && frame.picker_bound
        }));
    }

    #[test]
    fn persistent_client_profile_detects_correct_native_pipe_and_context_modes() {
        let profile = persistent_client_profile(
            "nsq_native_runtime_surface",
            "persistent_agentic_conversation",
            true,
            12,
            true,
            &[
                "conversation_continuity".to_string(),
                "planning_memory_lane".to_string(),
                "session_persistence".to_string(),
                "operator_shell_ingress".to_string(),
                "memory_guard_window".to_string(),
            ],
        );
        assert_eq!(profile.pipe_binding_state, "native_session_pipe_bound");
        assert_eq!(profile.reasoning_state, "high_reason_context_bound");
        assert_eq!(profile.comprehension_mode, "fuzzy_logic_context_match");
        assert!(profile.fuzzy_match_enabled);
        assert!(profile.session_continuity_ready);
        assert_eq!(profile.self_expression_state, "assistant_reply_bound");
    }
}
