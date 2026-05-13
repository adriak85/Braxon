//! Coaching mode — biases Citadel capital routing priority across the ten poles.
//!
//! Logical mode boosts the executive/audit/judge brain poles.
//! Artistic mode boosts the creative/empathic/sensory poles.
//! Balanced gives equal weight to all poles.
//!
//! Coaching is per-device: one phone runs logical, one runs artistic.
//! Both run the full Council Ten roster; only the routing weights differ.

use serde::{Deserialize, Serialize};
use std::path::Path;

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

    /// Priority pair [pole_a, pole_b] for a capital cluster.
    /// 255 = maximum routing weight, 128 = minimum.
    pub fn pole_priorities(&self, pole_a: &str, pole_b: &str) -> [u16; 2] {
        match self {
            Self::Balanced => [200, 200],
            Self::Logical  => [logical_weight(pole_a), logical_weight(pole_b)],
            Self::Artistic => [artistic_weight(pole_a), artistic_weight(pole_b)],
        }
    }
}

/// Logical mode: executive logic, audit, judgment poles get full weight.
fn logical_weight(pole: &str) -> u16 {
    match pole {
        "maverick" | "analyzer" | "arbiter" => 255,
        "support"                           => 200,
        "qwen" | "limbic"                   => 150,
        _                                   => 128, // sensory bodies — present but lower weight
    }
}

/// Artistic mode: creativity, empathy, voice, image poles get full weight.
fn artistic_weight(pole: &str) -> u16 {
    match pole {
        "qwen" | "limbic" | "voice" | "image" => 255,
        "support"                              => 220,
        "maverick" | "analyzer"               => 150,
        _                                      => 180, // video/world bodies still active
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
        "schema": "braxon.nsq.coaching.v1",
        "mode": mode.as_str(),
        "note": "Set mode to 'logical', 'artistic', or 'balanced'. One device per mode for dual-phone setup."
    });
    std::fs::write(
        dir.join("coaching.json"),
        serde_json::to_string_pretty(&body).unwrap(),
    )
    .map_err(|e| format!("write coaching.json: {e}"))
}
