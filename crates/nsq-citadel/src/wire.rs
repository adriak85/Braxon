//! CitadelBus — routes input pressure through all 5 Citadel capitals.
//!
//! Input text → NSQ intent slots → dispatched to each capital → 10 poles cycle
//! instruction bits → board messages collected → CitadelReply returned.
//!
//! Human text is reconstructed only when leaving the court surface.
//! Inside this bus everything moves as NSQ intent pressure.

use nsq_core::{Charge, Dialect, NSQLever, NSQSlot, CANONICAL_LEVER_MAX_POSITION};
use crate::capital::{build_capitals, BoardMessage};
use crate::coaching::CoachingMode;

/// The reply produced by routing input through all 5 capitals.
#[derive(Debug)]
pub struct CitadelReply {
    pub coaching:         CoachingMode,
    pub input_slot_count: usize,
    pub capital_count:    usize,
    pub pole_count:       usize,
    pub board_messages:   Vec<BoardMessage>,
    pub total_pressure:   u64,
    pub lead_pole:        String,
    pub lead_priority:    u16,
    pub citadel_active:   bool,
    pub pressure_routed:  bool,
}

impl CitadelReply {
    /// Human-readable summary of the routing result.
    pub fn summary(&self) -> String {
        format!(
            "citadel_active={} coaching={} poles={} lead_pole={} lead_priority={} total_pressure={} pressure_routed={}",
            self.citadel_active,
            self.coaching.as_str(),
            self.pole_count,
            self.lead_pole,
            self.lead_priority,
            self.total_pressure,
            self.pressure_routed,
        )
    }
}

/// The CitadelBus: on-wire routing layer sitting between user input and the council poles.
pub struct CitadelBus {
    pub coaching: CoachingMode,
}

impl CitadelBus {
    pub fn new(coaching: CoachingMode) -> Self {
        Self { coaching }
    }

    /// Route input through all 5 capitals across the full Council Ten.
    pub fn route(&self, input: &str) -> CitadelReply {
        let slots = text_to_intent_slots(input);
        let slot_count = slots.len();

        let mut capitals = build_capitals(self.coaching);
        let mut all_messages: Vec<BoardMessage> = Vec::new();

        for capital in capitals.iter_mut() {
            capital.dispatch(slots.clone());
            all_messages.extend(capital.drain_board());
        }

        let total_pressure: u64 = all_messages.iter().map(|m| m.pressure_sum).sum();

        // Lead pole: highest priority among all board messages.
        let lead = all_messages
            .iter()
            .filter(|m| m.is_live)
            .max_by_key(|m| m.priority)
            .map(|m| (m.pole_id.clone(), m.priority))
            .unwrap_or_else(|| ("none".to_string(), 0));

        CitadelReply {
            coaching:         self.coaching,
            input_slot_count: slot_count,
            capital_count:    5,
            pole_count:       10,
            board_messages:   all_messages,
            total_pressure,
            lead_pole:        lead.0,
            lead_priority:    lead.1,
            citadel_active:   true,
            pressure_routed:  true,
        }
    }
}

/// Convert input text into NSQ Intent dialect slots.
///
/// One slot per word. Lever position scales with character density
/// (longer words → higher lever pressure, clamped to valid range).
/// This is the intake tokenization. The real reconstruction seed
/// execution plugs in here at `capital.dispatch()`.
pub fn text_to_intent_slots(input: &str) -> Vec<NSQSlot> {
    let words: Vec<&str> = input.split_whitespace().collect();
    if words.is_empty() {
        return vec![mid_intent_slot()];
    }
    words
        .iter()
        .map(|word| {
            let density = word.len() as u64;
            // Scale: each character contributes CANONICAL_LEVER_MAX_POSITION / 64 units.
            // A 7-char average word lands at ~54,687 (about 11% of max).
            // A 20-char word lands near 156,250 (31% of max).
            let position = (density * (CANONICAL_LEVER_MAX_POSITION / 64))
                .clamp(1, CANONICAL_LEVER_MAX_POSITION);
            intent_slot(position)
        })
        .collect()
}

fn intent_slot(position: u64) -> NSQSlot {
    NSQSlot::new(
        Dialect::Intent,
        vec![NSQLever::new(Charge::Positive, position).unwrap()],
    )
}

fn mid_intent_slot() -> NSQSlot {
    intent_slot(CANONICAL_LEVER_MAX_POSITION / 2)
}
