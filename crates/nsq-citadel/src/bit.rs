//! CitadelBit — wire-resident instruction-cycling unit.
//!
//! Mirrors NsqBit semantics: a bit receives instruction slots, cycles through
//! them, and transfers results. Lives on-wire inside a Citadel capital cluster.
//! Defined here so nsq-citadel stays self-contained; when the nsq-core NsqBit
//! export path is confirmed, the two can be unified.

use nsq_core::{Charge, Dialect, NSQLever, NSQSlot, CANONICAL_LEVER_MAX_POSITION};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CitadelBitState {
    /// Normal operation — instructions cycling.
    Active,
    /// A cycle failed; bit is releasing before re-approach.
    Destabilizing,
    /// Released failed process; loading corrected intent.
    Reapproaching,
    /// Stable long-running state; instructions persist across check-ins.
    Persistent,
}

impl CitadelBitState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active        => "active",
            Self::Destabilizing => "destabilizing",
            Self::Reapproaching => "reapproaching",
            Self::Persistent    => "persistent",
        }
    }
}

/// One wire-resident instruction bit assigned to a single council pole lane.
pub struct CitadelBit {
    /// Pole lane index (1-indexed, 1..=10).
    pub lane: u128,
    pub state: CitadelBitState,
    pub instructions: Vec<NSQSlot>,
    pub priority: u16,
}

impl CitadelBit {
    pub fn new(lane: u128) -> Self {
        Self {
            lane,
            state: CitadelBitState::Persistent,
            instructions: Vec::new(),
            priority: 255,
        }
    }

    /// Receive new instruction slots. If destabilizing, release and re-approach first.
    pub fn check_in(&mut self, new_instructions: Vec<NSQSlot>) {
        if self.state == CitadelBitState::Destabilizing {
            self.release_and_reapproach();
        }
        self.instructions.extend(new_instructions);
        if self.state == CitadelBitState::Persistent {
            self.state = CitadelBitState::Active;
        }
    }

    /// Release failing state and re-approach with a corrected intent slot.
    fn release_and_reapproach(&mut self) {
        self.state = CitadelBitState::Reapproaching;
        self.instructions.clear();
        // Re-approach: load a full-positive intent lever as corrected seed.
        let corrected = NSQSlot::new(
            Dialect::Intent,
            vec![NSQLever::new(Charge::Positive, CANONICAL_LEVER_MAX_POSITION).unwrap()],
        );
        self.instructions.push(corrected);
        self.state = CitadelBitState::Active;
    }

    /// True if the bit has live instructions and is not destabilizing.
    pub fn is_live(&self) -> bool {
        !self.instructions.is_empty()
            && self.state != CitadelBitState::Destabilizing
    }

    /// Total lever pressure across all instructions (sum of positive positions).
    pub fn pressure_sum(&self) -> u64 {
        self.instructions
            .iter()
            .flat_map(|slot| slot.body.iter())
            .filter(|lever| lever.charge == Charge::Positive)
            .map(|lever| lever.position)
            .sum()
    }
}
