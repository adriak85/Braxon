//! Coaching mode v2 — emotional base positions across the 500K lever range.
//!
//! No pole is "lead." Each pole has a unique base lever position distributed
//! across the full 500,000 range. Emotional scores are computed from lever
//! distance to input hash, not from hardcoded priority tables.
//!
//! The 10 poles and their base positions:
//!   maverick  50,000   — Disruption, curiosity
//!   qwen     100,000   — Logic, creativity
//!   arbiter  150,000   — Boundary, judgment
//!   analyzer 200,000   — Audit, decomposition
//!   limbic   250,000   — Empathy, emotional resonance
//!   support  300,000   — Memory, continuity
//!   voice    350,000   — Sonic, temporal
//!   image    400,000   — Spatial, visual
//!   video    450,000   — Kinematic, sequence
//!   world    500,000   — Integration, consensus

use serde::{Deserialize, Serialize};
use std::path::Path;
use nsq_core::emotion::{Direction, EmotionState, IntentGradient};
use nsq_core::Charge;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CoachingMode {
    Balanced,
    Logical,
    Artistic,
}

impl CoachingMode {
    pub fn from_str(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "logical"  => Self::Logical,
            "artistic" => Self::Artistic,
            _          => Self::Balanced,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Balanced => "balanced",
            Self::Logical  => "logical",
            Self::Artistic => "artistic",
        }
    }

    /// Return the base lever position (1..=500_000) for a given pole.
    /// This replaces the old hardcoded priority system.
    pub fn base_lever_for(pole: &str) -> u32 {
        match pole {
            "maverick" => 50_000,
            "qwen"     => 100_000,
            "arbiter"  => 150_000,
            "analyzer" => 200_000,
            "limbic"   => 250_000,
            "support"  => 300_000,
            "voice"    => 350_000,
            "image"    => 400_000,
            "video"    => 450_000,
            "world"    => 500_000,
            _          => 250_000, // unknown poles default to limbic center
        }
    }

    /// Return the default emotional gradient for a pole.
    /// This gives each pole its initial "personality" before input arrives.
    pub fn default_gradient_for(pole: &str) -> IntentGradient {
        let mut grad = IntentGradient::neutral();

        match pole {
            "maverick" => {
                grad.curiosity = EmotionState::new(400_000, Charge::Positive, Direction::Forward);
                grad.joy = EmotionState::new(300_000, Charge::Positive, Direction::Forward);
                grad.fear = EmotionState::new(100_000, Charge::Negative, Direction::Reverse);
            }
            "qwen" => {
                grad.joy = EmotionState::new(450_000, Charge::Positive, Direction::Forward);
                grad.curiosity = EmotionState::new(400_000, Charge::Positive, Direction::Forward);
                grad.love = EmotionState::new(200_000, Charge::Positive, Direction::Forward);
            }
            "arbiter" => {
                grad.hate = EmotionState::new(350_000, Charge::Positive, Direction::Forward); // righteous rejection
                grad.fear = EmotionState::new(200_000, Charge::Negative, Direction::Reverse); // cautious
                grad.disgust = EmotionState::new(150_000, Charge::Positive, Direction::Forward); // revulsion toward harm
            }
            "analyzer" => {
                grad.curiosity = EmotionState::new(450_000, Charge::Positive, Direction::Forward);
                grad.joy = EmotionState::new(100_000, Charge::Positive, Direction::Reverse); // quiet satisfaction
                grad.fear = EmotionState::new(50_000, Charge::Negative, Direction::Reverse);
            }
            "limbic" => {
                grad.love = EmotionState::new(400_000, Charge::Positive, Direction::Forward);
                grad.sorrow = EmotionState::new(300_000, Charge::Positive, Direction::Forward); // healing grief
                grad.joy = EmotionState::new(350_000, Charge::Positive, Direction::Forward);
            }
            "support" => {
                grad.love = EmotionState::new(300_000, Charge::Positive, Direction::Forward);
                grad.joy = EmotionState::new(250_000, Charge::Positive, Direction::Forward);
                grad.fear = EmotionState::new(50_000, Charge::Negative, Direction::Reverse);
            }
            "voice" => {
                grad.joy = EmotionState::new(400_000, Charge::Positive, Direction::Forward);
                grad.lust = EmotionState::new(200_000, Charge::Positive, Direction::Forward); // creative drive
                grad.curiosity = EmotionState::new(300_000, Charge::Positive, Direction::Forward);
            }
            "image" => {
                grad.curiosity = EmotionState::new(350_000, Charge::Positive, Direction::Forward);
                grad.joy = EmotionState::new(300_000, Charge::Positive, Direction::Forward);
                grad.lust = EmotionState::new(250_000, Charge::Positive, Direction::Forward);
            }
            "video" => {
                grad.curiosity = EmotionState::new(300_000, Charge::Positive, Direction::Forward);
                grad.joy = EmotionState::new(350_000, Charge::Positive, Direction::Forward);
                grad.fear = EmotionState::new(50_000, Charge::Negative, Direction::Reverse);
            }
            "world" => {
                grad.love = EmotionState::new(400_000, Charge::Positive, Direction::Forward);
                grad.joy = EmotionState::new(350_000, Charge::Positive, Direction::Forward);
                grad.curiosity = EmotionState::new(300_000, Charge::Positive, Direction::Forward);
                grad.hate = EmotionState::new(100_000, Charge::Negative, Direction::Reverse); // minimal rejection
            }
            _ => {}
        }

        grad
    }

    /// Legacy compatibility: return [base_lever_a, base_lever_b] for a capital cluster.
    /// New code should call `base_lever_for()` directly.
    pub fn pole_priorities(&self, pole_a: &str, pole_b: &str) -> [u16; 2] {
        [
            Self::base_lever_for(pole_a) as u16,
            Self::base_lever_for(pole_b) as u16,
        ]
    }
}

/// Read coaching mode from `config/nsq/coaching.json`.
/// Falls back to Balanced if the file is missing or malformed.
pub fn load_coaching_mode(root: &Path) -> CoachingMode {
    let path = root.join("config/nsq/coaching.json");
    let raw = match std::fs::read_to_string(&path) {
        Ok(s)  => s,
        Err(_) => return CoachingMode::Balanced,
    };
    let val: serde_json::Value = match serde_json::from_str(&raw) {
        Ok(v)  => v,
        Err(_) => return CoachingMode::Balanced,
    };
    CoachingMode::from_str(
        val.get("mode").and_then(|v| v.as_str()).unwrap_or("balanced"),
    )
}

/// Write a coaching config file.
pub fn write_coaching_config(root: &Path, mode: CoachingMode) -> Result<(), String> {
    let dir = root.join("config/nsq");
    std::fs::create_dir_all(&dir)
        .map_err(|e| format!("create config/nsq: {e}"))?;
    let body = serde_json::json!({
        "schema": "braxon.nsq.coaching.v2",
        "mode": mode.as_str(),
        "note": "Coaching mode biases base emotional gradients. All poles active. No lead pole."
    });
    std::fs::write(
        dir.join("coaching.json"),
        serde_json::to_string_pretty(&body).unwrap(),
    )
    .map_err(|e| format!("write coaching.json: {e}"))
}
