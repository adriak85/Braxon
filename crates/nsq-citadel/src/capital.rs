use crate::{bit::CitadelBit, coaching::CoachingMode};
use nsq_core::NSQSlot;

pub const CAPITAL_COUNT: usize = 5;
pub const POLES_PER_CAPITAL: usize = 2;
#[derive(Debug, Clone)]
pub struct BoardMessage {
    pub capital_id: usize,
    pub pole_lane: usize,
    pub pole_id: String,
    pub slot_count: usize,
    pub priority: u16,
    pub state: String,
    pub pressure_sum: u64,
    pub is_live: bool,
}
pub struct Capital {
    pub id: usize,
    pub cluster: [(usize, &'static str); 2],
    pub board: Vec<BoardMessage>,
    pub coaching: CoachingMode,
}
impl Capital {
    pub fn new(id: usize, cluster: [(usize, &'static str); 2], coaching: CoachingMode) -> Self {
        Self {
            id,
            cluster,
            board: Vec::new(),
            coaching,
        }
    }
    pub fn dispatch(&mut self, slots: Vec<NSQSlot>) -> [CitadelBit; 2] {
        let p = self
            .coaching
            .pole_priorities(self.cluster[0].1, self.cluster[1].1);
        let mut a = CitadelBit::new(self.cluster[0].0 as u128);
        let mut b = CitadelBit::new(self.cluster[1].0 as u128);
        a.priority = p[0];
        b.priority = p[1];
        a.check_in(slots.clone());
        b.check_in(slots);
        self.board.push(msg(self.id, self.cluster[0], &a));
        self.board.push(msg(self.id, self.cluster[1], &b));
        [a, b]
    }
    pub fn drain_board(&mut self) -> Vec<BoardMessage> {
        std::mem::take(&mut self.board)
    }
}
fn msg(id: usize, c: (usize, &'static str), b: &CitadelBit) -> BoardMessage {
    BoardMessage {
        capital_id: id,
        pole_lane: c.0,
        pole_id: c.1.to_string(),
        slot_count: b.instructions.len(),
        priority: b.priority,
        state: b.state.as_str().into(),
        pressure_sum: b.pressure_sum(),
        is_live: b.is_live(),
    }
}
pub fn build_capitals(coaching: CoachingMode) -> [Capital; 5] {
    [
        Capital::new(1, [(1, "maverick"), (2, "qwen")], coaching),
        Capital::new(2, [(3, "arbiter"), (4, "analyzer")], coaching),
        Capital::new(3, [(5, "limbic"), (6, "support")], coaching),
        Capital::new(4, [(7, "voice"), (8, "image")], coaching),
        Capital::new(5, [(9, "video"), (10, "world")], coaching),
    ]
}
