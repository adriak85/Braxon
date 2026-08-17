//! Seed-first construction surface for Whispers of Willow and Stone.
//!
//! This is intentionally deterministic and lightweight: it proves that a
//! world can be reconstructed from a compact seed and advanced without
//! shipping a giant pre-expanded world. It is a world-construction test lane,
//! not a substitute for the native cognitive runtime.

use crate::seed_citadel::{build_seed_plan, materialize_window, UniversalTokenizerSeed};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorldSeed {
    pub seed_id: String,
    pub world_name: String,
    pub universe_name: String,
    pub epoch: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorldEntity {
    pub id: u64,
    pub name: String,
    pub archetype: String,
    pub location: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorldFrame {
    pub tick: u64,
    pub entities: Vec<WorldEntity>,
    pub narration: String,
}

#[derive(Debug, Clone)]
pub struct WhispersWorld {
    pub seed: WorldSeed,
    pub tokenizer: UniversalTokenizerSeed,
    pub frame: WorldFrame,
}

impl WhispersWorld {
    pub fn from_seed(seed: WorldSeed) -> Self {
        let tokenizer = UniversalTokenizerSeed::canonical();
        let plan = build_seed_plan(&seed.seed_id, 1 << 44, 1 << 28);
        let _citadel = materialize_window(&plan, 0, plan.active_window_bits);
        let willow = WorldEntity {
            id: stable("willow"),
            name: "Willow".into(),
            archetype: "keeper_of_the_well".into(),
            location: "The First Clearing".into(),
        };
        let stone = WorldEntity {
            id: stable("stone"),
            name: "Stone".into(),
            archetype: "memory_bearer".into(),
            location: "The First Clearing".into(),
        };
        Self {
            seed,
            tokenizer,
            frame: WorldFrame {
                tick: 0,
                entities: vec![willow, stone],
                narration: "At the edge of the first clearing, the willow remembers the wind and the stone remembers the rain.".into(),
            },
        }
    }

    pub fn play(&mut self, utterance: &str) -> WorldFrame {
        let token = self.tokenizer.synchronize(utterance);
        self.frame.tick += 1;
        let direction = match self.frame.tick % 4 {
            0 => "north toward the old river",
            1 => "east beneath the willow",
            2 => "south where the stone path descends",
            _ => "west toward the sleeping hills",
        };
        self.frame.narration = format!(
            "The world hears '{}'. Willow and Stone share the universal semantic mark {} and turn {}.",
            utterance, token.universal_id, direction
        );
        self.frame.clone()
    }
}

fn stable(value: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut h);
    h.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_reconstructs_from_compact_seed() {
        let world = WhispersWorld::from_seed(WorldSeed {
            seed_id: "whispers-seed-v1".into(),
            world_name: "Whispers of Willow and Stone".into(),
            universe_name: "The Willowstone Continuum".into(),
            epoch: 1,
        });
        assert_eq!(world.frame.entities.len(), 2);
        assert_eq!(world.frame.tick, 0);
    }

    #[test]
    fn world_advances_without_rebuilding_the_world() {
        let mut world = WhispersWorld::from_seed(WorldSeed {
            seed_id: "whispers-seed-v1".into(),
            world_name: "Whispers of Willow and Stone".into(),
            universe_name: "The Willowstone Continuum".into(),
            epoch: 1,
        });
        let frame = world.play("follow the whisper beyond the stones");
        assert_eq!(frame.tick, 1);
        assert!(frame.narration.contains("universal semantic mark"));
    }
}
