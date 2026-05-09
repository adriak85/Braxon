use crate::{Charge, Dialect, NSQLever, NSQSlot, CANONICAL_LEVER_MAX_POSITION};

pub enum BitState {
    Active,
    Destabilizing,
    Reapproaching,
    Persistent,
}

pub struct NsqBit {
    pub id: u128,
    pub state: BitState,
    pub instructions: Vec<NSQSlot>,
    pub priority: u16,
}

impl NsqBit {
    pub fn new(id: u128) -> Self {
        Self {
            id,
            state: BitState::Persistent,
            instructions: Vec::new(),
            priority: 255, // Max priority by default
        }
    }

    /// Check in for new instructions while preserving state
    pub fn check_in(&mut self, new_instructions: Vec<NSQSlot>) {
        if let BitState::Destabilizing = self.state {
            self.release_and_reapproach();
        }
        self.instructions.extend(new_instructions);
    }

    /// Release failing process and re-approach with a corrected system
    fn release_and_reapproach(&mut self) {
        self.state = BitState::Reapproaching;
        self.instructions.clear();
        // Re-approach logic: Load a "corrected" intent slot
        self.instructions.push(NSQSlot::new(
            Dialect::Intent,
            vec![NSQLever::new(Charge::Positive, CANONICAL_LEVER_MAX_POSITION).unwrap()],
        ));
    }
}
