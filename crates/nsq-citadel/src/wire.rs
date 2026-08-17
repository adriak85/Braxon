use crate::{
    capital::{build_capitals, BoardMessage},
    coaching::CoachingMode,
};
use nsq_core::{Charge, Dialect, NSQLever, NSQSlot, CANONICAL_LEVER_MAX_POSITION};
#[derive(Debug)]
pub struct CitadelReply {
    pub input_slot_count: usize,
    pub capital_count: usize,
    pub pole_count: usize,
    pub board_messages: Vec<BoardMessage>,
    pub total_pressure: u64,
    pub lead_pole: String,
    pub lead_priority: u16,
}
pub struct CitadelBus {
    pub coaching: CoachingMode,
}
impl CitadelBus {
    pub fn new(coaching: CoachingMode) -> Self {
        Self { coaching }
    }
    pub fn route(&self, input: &str) -> CitadelReply {
        let slots = text_to_intent_slots(input);
        let mut caps = build_capitals(self.coaching);
        let mut board = Vec::new();
        for c in caps.iter_mut() {
            c.dispatch(slots.clone());
            board.extend(c.drain_board());
        }
        let total = board.iter().map(|m| m.pressure_sum).sum();
        let lead = board
            .iter()
            .filter(|m| m.is_live)
            .max_by_key(|m| m.priority)
            .map(|m| (m.pole_id.clone(), m.priority))
            .unwrap_or(("none".into(), 0));
        CitadelReply {
            input_slot_count: slots.len(),
            capital_count: 5,
            pole_count: 10,
            board_messages: board,
            total_pressure: total,
            lead_pole: lead.0,
            lead_priority: lead.1,
        }
    }
}
pub fn text_to_intent_slots(input: &str) -> Vec<NSQSlot> {
    let words: Vec<_> = input.split_whitespace().collect();
    if words.is_empty() {
        return vec![intent_slot(CANONICAL_LEVER_MAX_POSITION / 2)];
    }
    words
        .into_iter()
        .map(|w| {
            intent_slot(
                (w.chars().count() as u64 * (CANONICAL_LEVER_MAX_POSITION / 64))
                    .clamp(1, CANONICAL_LEVER_MAX_POSITION),
            )
        })
        .collect()
}
fn intent_slot(position: u64) -> NSQSlot {
    NSQSlot::new(
        Dialect::Intent,
        vec![NSQLever::new(Charge::Positive, position).unwrap()],
    )
}
