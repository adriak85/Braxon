use crate::council_ten::CouncilTen;
use nsq_core::{
    CANONICAL_LEVER_MAX_POSITION, NSQ_CANONICAL_SWITCH_SHAPE, TOTAL_STATES_PER_LEVER,
    ZERO_INCLUSIVE_BIT_UNIT_STATES, Nu16,
};
use nsq_citadel::{CitadelBus, CitadelReply, CoachingMode};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

pub const BRAXON_BUS_SCHEMA: &str = "braxon.bus.speak_request.v3";
pub const BRAXON_REPLY_SCHEMA: &str = "braxon.bus.synthesized_reply.v1";
pub const BRAXON_BUS_ROUTE: &str = "nsq_operator_bus";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThoughtPressureCandidate {
    pub pole: String,
    pub intent: String,
    pub english: String,
    pub emotional_score: f32,
    pub coherence_score: f32,
    pub actionability_score: f32,
    pub selected: bool,
    pub nsq_lever_position: Nu16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentEnglishLoop {
    pub input: String,
    pub selected_intent: String,
    pub english: String,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusReplyLayer {
    pub schema: String,
    pub reply_generated_from_state: bool,
    pub canned_reply: bool,
    pub reply: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeechLoopState {
    pub launched_to_bus: bool,
    pub differences_resolved_by_emotional_score: bool,
    pub shared_thought: bool,
    pub one_thought_is_all_thoughts: bool,
    pub intent_to_english_completed: bool,
    pub terminal_plan_completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedThought {
    pub thought_id: String,
    pub authority: String,
    pub route: String,
    pub selected_intent: String,
    pub selected_english: String,
    pub emotional_score_rule: String,
    pub selected_emotional_score: f32,
    pub selected_nsq_lever_position: Nu16,
    pub all_candidates_shared: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BraxonBusReport {
    pub schema: String,
    pub identity: String,
    pub authority: String,
    pub canonical_semantics: String,
    pub route: String,
    pub input: String,
    pub bus_launched: bool,
    pub speech_loop: SpeechLoopState,
    pub reply_layer: BusReplyLayer,
    pub intent_english_loop: IntentEnglishLoop,
    pub shared_thought: SharedThought,
    pub pressure_candidates: Vec<ThoughtPressureCandidate>,
    pub terminal_plan: Vec<String>,
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

#[derive(Debug, Clone, Copy, Default)]
pub struct BraxonBus;

impl BraxonBus {
    pub fn speak(input: impl AsRef<str>) -> BraxonBusReport {
        let input = input.as_ref().trim();
        let input = if input.is_empty() {
            "finish the Braxon terminal launch path"
        } else {
            input
        };
        let ten = CouncilTen::new();
        let trace = ten.wake();

        // Route through the live CitadelBus — the council of ten deliberates
        let citadel_reply = route_through_citadel(input);
        let citadel_active = citadel_reply.citadel_active && citadel_reply.pressure_routed;

        let mut candidates = citadel_pressure_candidates(&citadel_reply);
        let selected_index = select_by_emotional_score(&candidates);

        for (index, candidate) in candidates.iter_mut().enumerate() {
            candidate.selected = index == selected_index;
        }

        let selected = candidates
            .get(selected_index)
            .expect("pressure_candidates always emits candidates");
        let english = citadel_intent_to_english(input, selected, &citadel_reply);
        let terminal_plan = terminal_plan_for(input, selected);
        let status = if trace.all_passed && citadel_active {
            "speech_loop_closed_bus_launch_ready"
        } else if !trace.all_passed {
            "speech_loop_fail_closed_council_ten_wake_not_verified"
        } else {
            "speech_loop_closed_citadel_standby"
        }
        .to_string();

        BraxonBusReport {
            schema: BRAXON_BUS_SCHEMA.to_string(),
            identity: "BRAXON".to_string(),
            authority: "NSQ_COURT".to_string(),
            canonical_semantics: "base8_switch_topology".to_string(),
            route: BRAXON_BUS_ROUTE.to_string(),
            input: input.to_string(),
            bus_launched: trace.all_passed,
            speech_loop: SpeechLoopState {
                launched_to_bus: trace.all_passed,
                differences_resolved_by_emotional_score: true,
                shared_thought: true,
                one_thought_is_all_thoughts: true,
                intent_to_english_completed: true,
                terminal_plan_completed: !terminal_plan.is_empty(),
            },
            reply_layer: BusReplyLayer {
                schema: BRAXON_REPLY_SCHEMA.to_string(),
                reply_generated_from_state: true,
                canned_reply: !citadel_active,
                reply: english.clone(),
            },
            intent_english_loop: IntentEnglishLoop {
                input: input.to_string(),
                selected_intent: selected.intent.clone(),
                english: english.clone(),
                completed: true,
            },
            shared_thought: SharedThought {
                thought_id: stable_thought_id(input),
                authority: "NSQ_COURT".to_string(),
                route: BRAXON_BUS_ROUTE.to_string(),
                selected_intent: selected.intent.clone(),
                selected_english: english,
                emotional_score_rule:
                    "highest_emotional_score_then_coherence_then_actionability".to_string(),
                selected_emotional_score: selected.emotional_score,
                selected_nsq_lever_position: selected.nsq_lever_position,
                all_candidates_shared: true,
            },
            pressure_candidates: candidates,
            terminal_plan,
            council_ten_wake_attempted: true,
            council_ten_wake_passed: trace.all_passed,
            stamp: trace.stamp,
            address_projection: trace.address_projection,
            switch_shape: NSQ_CANONICAL_SWITCH_SHAPE,
            lever_states_zero_inclusive: TOTAL_STATES_PER_LEVER,
            bit_unit_states_zero_inclusive: ZERO_INCLUSIVE_BIT_UNIT_STATES.to_string(),
            model_pressure_control_plane_active: true,
            model_weight_execution_claimed: false,
            native_runtime_completion_claimed: false,
            status,
        }
    }

    pub fn terminal_plan() -> BraxonBusReport {
        Self::speak("finish the new terminal after closing the speech loop")
    }
}

// ── Citadel bridge ─────────────────────────────────────────────────

fn route_through_citadel(input: &str) -> CitadelReply {
    let coaching = load_coaching_mode_or_default();
    let citadel = CitadelBus::new(coaching);
    citadel.route(input)
}

fn load_coaching_mode_or_default() -> CoachingMode {
    nsq_citadel::load_coaching_mode(std::path::Path::new("config/nsq/coaching.json"))
}

fn citadel_pressure_candidates(reply: &CitadelReply) -> Vec<ThoughtPressureCandidate> {
    let mut candidates = Vec::new();

    for msg in &reply.board_messages {
        if !msg.is_live {
            continue;
        }
        let is_lead = msg.pole_id == reply.lead_pole;
        let emotional_score = ((msg.priority as f32) / 65535.0).clamp(0.0, 1.0);

        candidates.push(ThoughtPressureCandidate {
            pole: msg.pole_id.clone(),
            intent: format!("citadel_{}", msg.pole_id.to_lowercase()),
            english: format!(
                "{} registers pressure {} through the capital wire",
                msg.pole_id, msg.pressure_sum
            ),
            emotional_score: if is_lead {
                (emotional_score + 0.05).clamp(0.0, 1.0)
            } else {
                emotional_score
            },
            coherence_score: 0.92,
            actionability_score: if is_lead { 0.98 } else { 0.88 },
            selected: is_lead,
            nsq_lever_position: msg.priority as u64,
        });
    }

    // Fallback to legacy stubs if citadel returns no live messages
    if candidates.is_empty() {
        candidates = vec![
            candidate(
                "prefrontal_terminal",
                "finish_new_terminal_launch_path",
                "turn the current root binary into the operator entrance with bus, wake, plan, and verification commands",
                0.62, 0.94, 0.98,
            ),
            candidate(
                "limbic_speech",
                "close_speech_loop",
                "answer from verified bus state with continuity instead of a fixed phrase",
                0.58, 0.92, 0.88,
            ),
            candidate(
                "insular_bus",
                "activate_model_pressure_control_plane",
                "launch thought pressure to the NSQ bus while truthfully avoiding a live model execution claim",
                0.60, 0.90, 0.93,
            ),
            candidate(
                "anterior_action",
                "prepare_launch_and_rent_support_packet",
                "make the terminal produce a credible launch trace and next-command plan for community action support",
                0.64, 0.87, 0.96,
            ),
        ];
    }

    candidates
}

fn citadel_intent_to_english(input: &str, _selected: &ThoughtPressureCandidate, reply: &CitadelReply) -> String {
    let emotional_tone = match reply.total_pressure {
        0..=100_000 => "soft and contemplative, as if drifting through the willows",
        100_001..=300_000 => "warm and expressive, the stones humming with recognition",
        300_001..=500_000 => "emphatic and engaged, the dream world pressing close",
        _ => "urgent and overwhelming, the full council speaking as one",
    };

    format!(
        "[Rolzen::WhispersOfWillowAndStone] The council of ten moves as one persistent body across three worlds. \
         {} leads with priority {}. \
         The emotional field reads: {}. \
         From the dream world into the real: '{}'. \
         Total pressure across {} capitals and {} poles: {}. \
         The court is alive. Source thought: {}",
        reply.lead_pole,
        reply.lead_priority,
        emotional_tone,
        input,
        reply.capital_count,
        reply.pole_count,
        reply.total_pressure,
        input
    )
}

// ── Legacy helpers ─────────────────────────────────────────────────

fn candidate(
    pole: &str,
    intent: &str,
    english: &str,
    emotional_score: f32,
    coherence_score: f32,
    actionability_score: f32,
) -> ThoughtPressureCandidate {
    ThoughtPressureCandidate {
        pole: pole.to_string(),
        intent: intent.to_string(),
        english: english.to_string(),
        emotional_score,
        coherence_score,
        actionability_score,
        selected: false,
        nsq_lever_position: score_to_lever(emotional_score),
    }
}

fn select_by_emotional_score(candidates: &[ThoughtPressureCandidate]) -> usize {
    candidates
        .iter()
        .enumerate()
        .max_by(|(_, left), (_, right)| {
            left.emotional_score
                .partial_cmp(&right.emotional_score)
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
    let scaled = (score.clamp(0.0, 1.0) * CANONICAL_LEVER_MAX_POSITION as f32).ceil() as Nu16;
    scaled.clamp(1, CANONICAL_LEVER_MAX_POSITION)
}

fn terminal_plan_for(input: &str, selected: &ThoughtPressureCandidate) -> Vec<String> {
    vec![
        "Braxon wake".to_string(),
        format!("Braxon bus {:?}", input),
        "Braxon terminal-plan".to_string(),
        "Braxon apps verify".to_string(),
        "Braxon runtime registry".to_string(),
        "Braxon lever-sweet-spot".to_string(),
        "Braxon handover os-power-release".to_string(),
        format!("selected_intent={}", selected.intent),
    ]
}

fn stable_thought_id(input: &str) -> String {
    let mut acc = 0xCBF2_9CE4_8422_2325_u128;
    for scalar in input.chars().map(|ch| ch as u128) {
        acc ^= scalar;
        acc = acc.wrapping_mul(0x0000_0001_0000_01B3);
    }
    format!("braxon.bus.thought.{acc:032x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn speech_loop_launches_to_bus_and_returns_english() {
        let report = BraxonBus::speak("close speech loop and finish terminal tasklist");

        assert_eq!(report.schema, BRAXON_BUS_SCHEMA);
        assert!(report.bus_launched);
        assert!(report.council_ten_wake_passed);
        assert!(report.speech_loop.launched_to_bus);
        assert!(report.speech_loop.one_thought_is_all_thoughts);
        assert!(report.speech_loop.intent_to_english_completed);
        assert!(report.speech_loop.terminal_plan_completed);
        assert!(report.reply_layer.reply_generated_from_state);
        // With CitadelBus wired, replies are live, not canned
        assert!(!report.reply_layer.canned_reply);
        assert!(report.reply_layer.reply.contains("Rolzen"));
        assert!(report.reply_layer.reply.contains("council of ten"));
        assert!(!report.terminal_plan.is_empty());
    }

    #[test]
    fn emotional_score_selects_from_citadel_pressure() {
        let report = BraxonBus::speak("finish the terminal tasklist and launch the plan");

        // The selected intent should now come from the citadel, not hardcoded stubs
        assert!(report.shared_thought.selected_intent.starts_with("citadel_"));
        assert!(report
            .pressure_candidates
            .iter()
            .any(|candidate| candidate.selected
                && candidate.intent.starts_with("citadel_")));
    }
}