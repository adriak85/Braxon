use serde::{Deserialize, Serialize};
use std::fmt;

pub mod intent;
pub mod preserve;
pub mod seating;

/// NSQ is the machine.
/// Binary is replaced by a 2x1126 substrate.
/// One NSQ bit-unit = 4 charge-anchored levers.
/// Total states per bit-unit = (2 * 1126)^4 = 2252^4 = 25,720,243,363,856 states.
pub const LEVER_STATES_PER_CHARGE: u16 = 1126;
pub const TOTAL_STATES_PER_LEVER: u16 = 2252; // 2 charges * 1126 positions
pub const CANONICAL_BIT_UNIT_LEVERS: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Charge {
    Positive, // Action/write intent (+)
    Negative, // Query/read intent (-)
}

impl Charge {
    pub fn multiplier(&self) -> i16 {
        match self {
            Charge::Positive => 1,
            Charge::Negative => -1,
        }
    }
    pub fn symbol(&self) -> char {
        match self {
            Charge::Positive => '+',
            Charge::Negative => '-',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NSQLever {
    pub charge: Charge,
    pub position: u16, // 1..=1126
}

impl NSQLever {
    pub fn new(charge: Charge, position: u16) -> Result<Self, String> {
        if position < 1 || position > LEVER_STATES_PER_CHARGE {
            return Err(format!("Lever position must be 1-{}, got {}", LEVER_STATES_PER_CHARGE, position));
        }
        Ok(Self { charge, position })
    }
    pub fn machine_value(&self) -> i16 {
        self.charge.multiplier() * (self.position as i16)
    }
    pub fn to_nsq(&self) -> String {
        format!("{}{:04}", self.charge.symbol(), self.position)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Dialect {
    Numeric = 1,
    Alphabetic = 2,
    Intent = 3,
    Symbolic = 4,
    Stamp = 5,
    Control = 6,
    Graphics = 7,
    Audio = 8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NSQSlot {
    pub dialect: Dialect,
    pub body: Vec<NSQLever>,
}

impl NSQSlot {
    pub fn to_nsq(&self) -> String {
        let mut s = format!("{:04}", self.dialect as u16);
        for lever in &self.body {
            s.push_str(&lever.to_nsq());
        }
        s
    }
}

impl fmt::Display for NSQSlot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_nsq())
    }
}
