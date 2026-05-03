use crate::{Charge, Dialect, NSQLever, NSQSlot};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Intent {
    pub target: String,
    pub levers: Vec<NSQLever>,
    pub dialect: Dialect,
}

impl Intent {
    pub fn new(target: &str, dialect: Dialect) -> Self {
        Self {
            target: target.to_string(),
            levers: vec![NSQLever {
                charge: Charge::Positive,
                position: 1,
            }],
            dialect,
        }
    }

    pub fn to_slot(&self) -> NSQSlot {
        NSQSlot::new(self.dialect.clone(), self.levers.clone())
    }
}

pub fn parse_intent(input: &str) -> Result<Intent, String> {
    Ok(Intent::new(input, Dialect::Intent))
}
