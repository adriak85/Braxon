use nsq_core::{Charge, Dialect, NSQLever, NSQSlot};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WoWaSCharacter {
    pub id: String,
    pub name: String,
    pub bit_addr: u128,
    pub ddc: u16,
    pub lcc: u32,
    pub semantic_score: u16,
}

pub struct WoWaSWorldState {
    pub characters: HashMap<String, WoWaSCharacter>,
    pub target_scene_count: u64,
    pub current_book: String,
}

impl WoWaSWorldState {
    pub fn new() -> Self {
        Self {
            characters: HashMap::new(),
            target_scene_count: 55_000_000,
            current_book: "Whispers of Willow and Stone".to_string(),
        }
    }

    pub fn resolve_interaction(&self, char_id: &str, _intent: &str) -> NSQSlot {
        let charge = self
            .characters
            .get(char_id)
            .map(|character| {
                if character.semantic_score > 500 {
                    Charge::Positive
                } else {
                    Charge::Negative
                }
            })
            .unwrap_or(Charge::Positive);

        NSQSlot::new(
            Dialect::Intent,
            vec![
                NSQLever::new(charge, 1001).unwrap(),
                NSQLever::new(Charge::Positive, 1126).unwrap(),
            ],
        )
    }
}

impl Default for WoWaSWorldState {
    fn default() -> Self {
        Self::new()
    }
}
