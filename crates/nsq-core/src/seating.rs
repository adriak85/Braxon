use crate::{Charge, NSQLever, CANONICAL_LEVER_MAX_POSITION};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SeatingState {
    Unseated,
    Seating,
    Seated,
    Verified,
}

pub struct SeatingVerification {
    pub state: SeatingState,
    pub levers: Vec<NSQLever>,
}

impl SeatingVerification {
    pub fn new() -> Self {
        Self {
            state: SeatingState::Unseated,
            levers: vec![],
        }
    }

    pub fn seat(&mut self) -> Result<(), String> {
        self.state = SeatingState::Seated;
        self.levers
            .push(NSQLever::new(Charge::Positive, CANONICAL_LEVER_MAX_POSITION)?);
        Ok(())
    }

    pub fn verify(&mut self) -> Result<(), String> {
        if self.state == SeatingState::Seated {
            self.state = SeatingState::Verified;
            Ok(())
        } else {
            Err("Cannot verify unseated".to_string())
        }
    }
}

pub fn seat_all() -> Result<SeatingVerification, String> {
    let mut sv = SeatingVerification::new();
    sv.seat()?;
    sv.verify()?;
    Ok(sv)
}
