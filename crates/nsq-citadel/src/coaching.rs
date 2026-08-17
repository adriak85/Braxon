use serde::{Deserialize, Serialize};

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
            "logical" => Self::Logical,
            "artistic" => Self::Artistic,
            _ => Self::Balanced,
        }
    }
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Balanced => "balanced",
            Self::Logical => "logical",
            Self::Artistic => "artistic",
        }
    }
    pub fn pole_priorities(&self, a: &str, b: &str) -> [u16; 2] {
        [self.weight(a), self.weight(b)]
    }
    fn weight(&self, p: &str) -> u16 {
        match self {
            Self::Balanced => 200,
            Self::Logical => match p {
                "maverick" | "analyzer" | "arbiter" => 255,
                "support" => 200,
                "qwen" | "limbic" => 150,
                _ => 128,
            },
            Self::Artistic => match p {
                "qwen" | "limbic" | "voice" | "image" => 255,
                "support" => 220,
                "maverick" | "analyzer" => 150,
                _ => 180,
            },
        }
    }
}
