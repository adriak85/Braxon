//! Five Citadel capitals — wire-resident micro-systems.
//!
//! Each capital owns a 2-pole cluster of the Council Ten.
//! No persistent memory reservation. Receives instruction stamps, dispatches
//! CitadelBit instruction cycles to both poles, collects board messages, routes back.
//!
//! Capital layout (Council Ten):
//!   C1: maverick(1) + qwen(2)       — Logic + Creativity
//!   C2: arbiter(3)  + analyzer(4)   — Judge + Auditor
//!   C3: limbic(5)   + support(6)    — Empath + Memory
//!   C4: voice(7)    + image(8)      — IndexTTS2 + FLUX
//!   C5: video(9)    + world(10)     — Wan2.1 + Hunyuan3D

use nsq_core::{NSQSlot};
use crate::bit::CitadelBit;
use crate::coaching::CoachingMode;

pub const CAPITAL_COUNT: usize = 5;
pub const POLES_PER_CAPITAL: usize = 2;

/// A message written to the capital's board after a dispatch cycle.
#[derive(Debug, Clone)]
pub struct BoardMessage {
    pub capital_id:   usize,
    pub pole_lane:    usize,
    pub pole_id:      String,
    pub slot_count:   usize,
    pub priority:     u16,
    pub state:        String,
    pub pressure_sum: u64,
    pub is_live:      bool,
}

/// One Citadel capital.
pub struct Capital {
    pub id:       usize,
    pub cluster:  [(usize, &'static str); 2], // (1-indexed lane, pole_id)
    pub board:    Vec<BoardMessage>,
    pub coaching: CoachingMode,
}

impl Capital {
    pub fn new(
        id: usize,
        cluster: [(usize, &'static str); 2],
        coaching: CoachingMode,
    ) -> Self {
        Self { id, cluster, board: Vec::new(), coaching }
    }

    /// Dispatch instruction slots to both poles in the cluster.
    /// Cycles CitadelBits, writes results to the message board, returns bits.
    pub fn dispatch(&mut self, slots: Vec<NSQSlot>) -> [CitadelBit; 2] {
        let priorities = self.coaching.pole_priorities(
            self.cluster[0].1,
            self.cluster[1].1,
        );

        let mut bit0 = CitadelBit::new(self.cluster[0].0 as u128);
        let mut bit1 = CitadelBit::new(self.cluster[1].0 as u128);
        bit0.priority = priorities[0];
        bit1.priority = priorities[1];

        // Each bit gets its own copy — they cycle independently on the wire.
        bit0.check_in(slots.clone());
        bit1.check_in(slots);

        self.board.push(BoardMessage {
            capital_id:   self.id,
            pole_lane:    self.cluster[0].0,
            pole_id:      self.cluster[0].1.to_string(),
            slot_count:   bit0.instructions.len(),
            priority:     bit0.priority,
            state:        bit0.state.as_str().to_string(),
            pressure_sum: bit0.pressure_sum(),
            is_live:      bit0.is_live(),
        });
        self.board.push(BoardMessage {
            capital_id:   self.id,
            pole_lane:    self.cluster[1].0,
            pole_id:      self.cluster[1].1.to_string(),
            slot_count:   bit1.instructions.len(),
            priority:     bit1.priority,
            state:        bit1.state.as_str().to_string(),
            pressure_sum: bit1.pressure_sum(),
            is_live:      bit1.is_live(),
        });

        [bit0, bit1]
    }

    /// Drain the message board (read soot, clear for next cycle).
    pub fn drain_board(&mut self) -> Vec<BoardMessage> {
        std::mem::take(&mut self.board)
    }

    pub fn board_depth(&self) -> usize {
        self.board.len()
    }
}

/// Build all 5 Citadel capitals from the Council Ten pole layout.
pub fn build_capitals(coaching: CoachingMode) -> [Capital; 5] {
    [
        Capital::new(1, [(1, "maverick"),  (2, "qwen")],     coaching),
        Capital::new(2, [(3, "arbiter"),   (4, "analyzer")], coaching),
        Capital::new(3, [(5, "limbic"),    (6, "support")],  coaching),
        Capital::new(4, [(7, "voice"),     (8, "image")],    coaching),
        Capital::new(5, [(9, "video"),     (10, "world")],   coaching),
    ]
}
