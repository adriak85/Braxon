use crate::{
    CouncilTen, OrganPerspective, OutputClassification, TokenizerBridge, TokenizerBridgeReceipt,
    UnifiedSelfState,
};
use nsq_core::{
    Nu16, CANONICAL_LEVER_MAX_POSITION, NSQ_CANONICAL_SWITCH_SHAPE, TOTAL_STATES_PER_LEVER,
    ZERO_INCLUSIVE_BIT_UNIT_STATES,
};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

pub const BRAXON_BUS_SCHEMA: &str = "braxon.bus.measurement_request.v4";
pub const BRAXON_REPLY_SCHEMA: &str = "braxon.bus.user_presentation.v2";
pub const BRAXON_BUS_ROUTE: &str = "nsq_operator_bus";

/// A derived computational priority, not an emotional or biological measurement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThoughtPressureCandidate {
    pub pole: String,
    pub intent: String,
    pub interpretation: String,
    pub priority_score: f32,
    pub coherence_score: f32,
    pub actionability_score: f32,
    pub selected: bool,
    pub nsq_lever_position: Nu16,
    pub classification: OutputClassification,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusProcessingState {
    pub input_accepted: bool,
    pub council_wake_verified: bool,
    pub token_boundary_checked: bool,
    pub universal_state_received: bool,
    pub classification: OutputClassification,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusReplyLayer {
    pub schema: String,
    pub classification: OutputClassification,
    pub generated_from_derived_state: bool,
    pub canned_reply: bool,
    pub reply: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentEnglishLoop {
    pub input: String,
    pub selected_intent: Option<String>,
    pub english: String,
    pub completed: bool,
    pub classification: OutputClassification,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BraxonBusReport {
    pub schema: String,
    pub identity: String,
    pub authority: String,
    pub canonical_semantics: String,
    pub route: String,
    pub input: String,
    pub processing: BusProcessingState,
    pub reply_layer: BusReplyLayer,
    pub intent_english_loop: IntentEnglishLoop,
    pub collective_self_state: Option<UnifiedSelfState>,
    pub tokenizer_bridge: Option<TokenizerBridgeReceipt>,
    pub pressure_candidates: Vec<ThoughtPressureCandidate>,
    pub terminal_plan: Vec<String>,
    pub terminal_plan_classification: OutputClassification,
    pub council_ten_wake_attempted: bool,
    pub council_ten_wake_passed: bool,
    pub stamp: String,
    pub address_projection: String,
    pub switch_shape: [Nu16; 8],
    pub lever_states_zero_inclusive: Nu16,
    pub bit_unit_states_zero_inclusive: String,
    pub model_pressure_control_plane_active: bool,
    pub model_weight_execution_claimed: bool,
    pub native_runtime_completion_claimed: bool,
    pub status: String,
}

impl BraxonBusReport {
    pub fn hard_runtime_valid(&self) -> bool {
        self.processing.classification.allowed_in_hard_runtime()
            && self
                .pressure_candidates
                .iter()
                .all(|candidate| candidate.classification.allowed_in_hard_runtime())
            && self
                .collective_self_state
                .as_ref()
                .map(|state| state.validate().is_ok())
                .unwrap_or(!self.processing.input_accepted)
            && (!self.processing.input_accepted
                || self
                    .tokenizer_bridge
                    .as_ref()
                    .map(|receipt| receipt.all_required_mappings_resolved())
                    .unwrap_or(false))
            && self.reply_layer.classification == OutputClassification::UserPresentation
            && self.intent_english_loop.classification == OutputClassification::UserPresentation
            && self.terminal_plan_classification == OutputClassification::UserPresentation
            && !self.model_weight_execution_claimed
            && !self.native_runtime_completion_claimed
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BraxonBus;

impl BraxonBus {
    pub fn speak(input: impl AsRef<str>) -> BraxonBusReport {
        let input = input.as_ref().trim();
        let ten = CouncilTen::new();
        let trace = ten.wake();
        if input.is_empty() {
            return rejected_report(trace, "", "input_rejected_empty_operator_request", None);
        }
        let root = std::env::current_dir().unwrap_or_else(|_| ".".into());
        let tokenizer = match TokenizerBridge::from_root(&root, "braxon_native") {
            Ok(bridge) => bridge.encode_translate_round_trip(input),
            Err(_) => return rejected_report(trace, input, "tokenizer_bridge_unavailable", None),
        };
        if !tokenizer.all_required_mappings_resolved() {
            return rejected_report(
                trace,
                input,
                "tokenizer_bridge_unresolved_or_invalid",
                Some(tokenizer),
            );
        }
        let mut candidates = pressure_candidates(input);
        let selected_index = select_by_priority_score(&candidates);
        for (index, candidate) in candidates.iter_mut().enumerate() {
            candidate.selected = index == selected_index;
        }
        let collective_self_state = collective_state(input, &candidates).ok();
        let selected = candidates.get(selected_index);
        let input_accepted = trace.all_passed && collective_self_state.is_some();
        let selected_intent = selected.map(|candidate| candidate.intent.clone());
        let reply = selected
            .map(|candidate| {
                presentation_from_measurement(input, candidate, collective_self_state.as_ref())
            })
            .unwrap_or_else(|| {
                "operator request was rejected before derived state was available".into()
            });
        let terminal_plan = selected
            .map(|candidate| terminal_plan_for(input, candidate))
            .unwrap_or_default();
        let status = if input_accepted {
            "bus_measurement_complete"
        } else {
            "bus_measurement_fail_closed"
        }
        .to_string();
        BraxonBusReport {
            schema: BRAXON_BUS_SCHEMA.into(),
            identity: "BRAXON".into(),
            authority: "NSQ_COURT".into(),
            canonical_semantics: "base8_switch_topology".into(),
            route: BRAXON_BUS_ROUTE.into(),
            input: input.into(),
            processing: BusProcessingState {
                input_accepted,
                council_wake_verified: trace.all_passed,
                token_boundary_checked: tokenizer.native_representation_retained,
                universal_state_received: tokenizer.collective_state_contribution_ready
                    && collective_self_state.is_some(),
                classification: OutputClassification::HardState,
            },
            reply_layer: BusReplyLayer {
                schema: BRAXON_REPLY_SCHEMA.into(),
                classification: OutputClassification::UserPresentation,
                generated_from_derived_state: input_accepted,
                canned_reply: false,
                reply: reply.clone(),
            },
            intent_english_loop: IntentEnglishLoop {
                input: input.into(),
                selected_intent,
                english: reply,
                completed: input_accepted,
                classification: OutputClassification::UserPresentation,
            },
            collective_self_state,
            tokenizer_bridge: Some(tokenizer),
            pressure_candidates: candidates,
            terminal_plan,
            terminal_plan_classification: OutputClassification::UserPresentation,
            council_ten_wake_attempted: true,
            council_ten_wake_passed: trace.all_passed,
            stamp: trace.stamp,
            address_projection: trace.address_projection,
            switch_shape: NSQ_CANONICAL_SWITCH_SHAPE,
            lever_states_zero_inclusive: TOTAL_STATES_PER_LEVER,
            bit_unit_states_zero_inclusive: ZERO_INCLUSIVE_BIT_UNIT_STATES.to_string(),
            model_pressure_control_plane_active: input_accepted,
            model_weight_execution_claimed: false,
            native_runtime_completion_claimed: false,
            status,
        }
    }

    pub fn terminal_plan() -> BraxonBusReport {
        Self::speak("show measured terminal launch steps")
    }
}

fn rejected_report(
    trace: crate::CouncilTenWakeTrace,
    input: &str,
    status: &str,
    tokenizer_bridge: Option<TokenizerBridgeReceipt>,
) -> BraxonBusReport {
    BraxonBusReport {
        schema: BRAXON_BUS_SCHEMA.into(),
        identity: "BRAXON".into(),
        authority: "NSQ_COURT".into(),
        canonical_semantics: "base8_switch_topology".into(),
        route: BRAXON_BUS_ROUTE.into(),
        input: input.into(),
        processing: BusProcessingState {
            input_accepted: false,
            council_wake_verified: trace.all_passed,
            token_boundary_checked: false,
            universal_state_received: false,
            classification: OutputClassification::HardState,
        },
        reply_layer: BusReplyLayer {
            schema: BRAXON_REPLY_SCHEMA.into(),
            classification: OutputClassification::UserPresentation,
            generated_from_derived_state: false,
            canned_reply: false,
            reply: "operator request rejected: input is empty".into(),
        },
        intent_english_loop: IntentEnglishLoop {
            input: String::new(),
            selected_intent: None,
            english: "operator request rejected: input is empty".into(),
            completed: false,
            classification: OutputClassification::UserPresentation,
        },
        collective_self_state: None,
        tokenizer_bridge,
        pressure_candidates: Vec::new(),
        terminal_plan: Vec::new(),
        terminal_plan_classification: OutputClassification::UserPresentation,
        council_ten_wake_attempted: true,
        council_ten_wake_passed: trace.all_passed,
        stamp: trace.stamp,
        address_projection: trace.address_projection,
        switch_shape: NSQ_CANONICAL_SWITCH_SHAPE,
        lever_states_zero_inclusive: TOTAL_STATES_PER_LEVER,
        bit_unit_states_zero_inclusive: ZERO_INCLUSIVE_BIT_UNIT_STATES.to_string(),
        model_pressure_control_plane_active: false,
        model_weight_execution_claimed: false,
        native_runtime_completion_claimed: false,
        status: status.into(),
    }
}

fn pressure_candidates(input: &str) -> Vec<ThoughtPressureCandidate> {
    let lower = input.to_ascii_lowercase();
    vec![
        candidate(
            "prefrontal_terminal",
            "verify_terminal_launch_path",
            "terminal launch-path operational priority",
            signed_score(
                &lower,
                0.62,
                &["terminal", "tasklist", "finish", "launch", "plan"],
                &["defer", "avoid"],
            ),
            0.94,
            0.98,
        ),
        candidate(
            "limbic_speech",
            "present_derived_bus_state",
            "user-presentation boundary priority",
            signed_score(
                &lower,
                0.58,
                &["speech", "speak", "english", "voice", "reply"],
                &["silence", "suppress", "reject speech"],
            ),
            0.92,
            0.88,
        ),
        candidate(
            "insular_bus",
            "verify_operator_bus_measurement",
            "operator-bus measurement priority",
            signed_score(
                &lower,
                0.60,
                &["bus", "pressure", "activation", "wake", "stamp", "model"],
                &["disable bus", "reject bus"],
            ),
            0.90,
            0.93,
        ),
        candidate(
            "anterior_action",
            "prepare_operator_next_steps",
            "operator next-step priority",
            signed_score(
                &lower,
                0.64,
                &["action", "prepare", "support", "next", "operate"],
                &["stop", "cancel", "reject"],
            ),
            0.87,
            0.96,
        ),
    ]
}

fn candidate(
    pole: &str,
    intent: &str,
    interpretation: &str,
    priority_score: f32,
    coherence_score: f32,
    actionability_score: f32,
) -> ThoughtPressureCandidate {
    ThoughtPressureCandidate {
        pole: pole.into(),
        intent: intent.into(),
        interpretation: interpretation.into(),
        priority_score,
        coherence_score,
        actionability_score,
        selected: false,
        nsq_lever_position: score_to_lever(priority_score),
        classification: OutputClassification::DerivedState,
    }
}

fn signed_score(input: &str, base: f32, positive: &[&str], negative: &[&str]) -> f32 {
    let positive_hits = positive
        .iter()
        .filter(|needle| input.contains(**needle))
        .count() as f32;
    let negative_hits = negative
        .iter()
        .filter(|needle| input.contains(**needle))
        .count() as f32;
    (base + positive_hits * 0.07 - negative_hits * 0.80).clamp(-1.0, 1.0)
}

fn select_by_priority_score(candidates: &[ThoughtPressureCandidate]) -> usize {
    candidates
        .iter()
        .enumerate()
        .max_by(|(_, left), (_, right)| {
            left.priority_score
                .partial_cmp(&right.priority_score)
                .unwrap_or(Ordering::Equal)
                .then_with(|| {
                    left.coherence_score
                        .partial_cmp(&right.coherence_score)
                        .unwrap_or(Ordering::Equal)
                })
                .then_with(|| {
                    left.actionability_score
                        .partial_cmp(&right.actionability_score)
                        .unwrap_or(Ordering::Equal)
                })
        })
        .map(|(index, _)| index)
        .unwrap_or(0)
}

fn score_to_lever(score: f32) -> Nu16 {
    let normalized = ((score.clamp(-1.0, 1.0) + 1.0) / 2.0) * CANONICAL_LEVER_MAX_POSITION as f32;
    normalized.round() as Nu16
}

fn collective_state(
    input: &str,
    candidates: &[ThoughtPressureCandidate],
) -> Result<UnifiedSelfState, String> {
    let perspectives = candidates
        .iter()
        .map(|candidate| OrganPerspective {
            organ_id: candidate.pole.clone(),
            identity: format!("braxon.organ_band.{}", candidate.pole),
            address: format!("council/{}/state", candidate.pole),
            local_input: input.into(),
            local_state: if candidate.priority_score >= 0.0 {
                "priority_nonnegative".into()
            } else {
                "priority_negative".into()
            },
            local_interpretation: candidate.interpretation.clone(),
            local_output: candidate.intent.clone(),
            translation_interface: "nsq.universal.translation.v1".into(),
            pressure: (candidate.priority_score * 1000.0).round() as i64,
            feedback_path: format!("council/{}/feedback", candidate.pole),
            classification: OutputClassification::DerivedState,
        })
        .collect();
    UnifiedSelfState::integrate(perspectives)
}

fn presentation_from_measurement(
    input: &str,
    selected: &ThoughtPressureCandidate,
    state: Option<&UnifiedSelfState>,
) -> String {
    let disagreement = state
        .map(|state| state.disagreement_present)
        .unwrap_or(false);
    format!(
        "Derived semantic route for `{input}` selected `{}` with interpretation `{}`. Individual disagreement retained: {disagreement}. Native action must be completed by the intelligent operation route.",
        selected.intent, selected.interpretation
    )
}

fn terminal_plan_for(_input: &str, selected: &ThoughtPressureCandidate) -> Vec<String> {
    vec![
        "Braxon wake".into(),
        "Braxon closure verify".into(),
        "Braxon apps verify".into(),
        "Braxon runtime registry".into(),
        format!("selected_derived_intent={}", selected.intent),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bus_returns_classified_presentation_from_derived_state() {
        let report = BraxonBus::speak("verify terminal launch path through the operator bus");
        assert!(report.processing.input_accepted);
        assert!(report.council_ten_wake_passed);
        assert!(report.reply_layer.generated_from_derived_state);
        assert_eq!(
            report.reply_layer.classification,
            OutputClassification::UserPresentation
        );
        assert!(report.hard_runtime_valid());
        assert!(report.reply_layer.reply.contains("Derived semantic route"));
        assert!(!report.model_weight_execution_claimed);
        assert!(!report.native_runtime_completion_claimed);
    }

    #[test]
    fn bus_preserves_conflicting_band_state_without_forced_consensus() {
        let report = BraxonBus::speak("verify terminal launch but reject speech");
        let state = report.collective_self_state.unwrap();
        assert!(state.disagreement_present);
        assert!(state.conflict_preserved);
        assert!(!state.forced_consensus);
        assert!(state
            .perspectives
            .iter()
            .any(|perspective| perspective.pressure > 0));
        assert!(state
            .perspectives
            .iter()
            .any(|perspective| perspective.pressure < 0));
        state.validate().unwrap();
    }

    #[test]
    fn unknown_token_fails_closed_with_an_unresolved_token_receipt() {
        let report = BraxonBus::speak("truth🙂");
        assert!(!report.processing.input_accepted);
        assert_eq!(report.status, "tokenizer_bridge_unresolved_or_invalid");
        assert_eq!(
            report.tokenizer_bridge.as_ref().unwrap().unresolved_tokens,
            vec!["🙂"]
        );
        assert!(report.hard_runtime_valid());
    }

    #[test]
    fn empty_input_fails_closed_without_defaulting_to_a_story_prompt() {
        let report = BraxonBus::speak("");
        assert!(!report.processing.input_accepted);
        assert!(report.collective_self_state.is_none());
        assert_eq!(report.status, "input_rejected_empty_operator_request");
        assert!(report.hard_runtime_valid());
    }
}
