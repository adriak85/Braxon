use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NsqEightDimensionalCoordinate {
    pub intent: u64,
    pub function: u64,
    pub state: u64,
    pub emotion: u64,
    pub consequence: u64,
    pub proof: u64,
    pub knowledge: u64,
    pub action: u64,
}

impl NsqEightDimensionalCoordinate {
    pub fn centered(value: u64) -> Self {
        Self {
            intent: value,
            function: value,
            state: value,
            emotion: value,
            consequence: value,
            proof: value,
            knowledge: value,
            action: value,
        }
    }

    pub fn drift_width(&self) -> u64 {
        let vals = [
            self.intent,
            self.function,
            self.state,
            self.emotion,
            self.consequence,
            self.proof,
            self.knowledge,
            self.action,
        ];
        let min = vals.iter().copied().min().unwrap_or(0);
        let max = vals.iter().copied().max().unwrap_or(0);
        max.saturating_sub(min)
    }

    pub fn coherent_under(&self, limit: u64) -> bool {
        self.drift_width() <= limit
    }
}
