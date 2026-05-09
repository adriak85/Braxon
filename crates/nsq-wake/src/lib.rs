use nsq_grid::NsqEightDimensionalCoordinate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WakeStamp {
    pub stamp_id: String,
    pub target_position: u64,
    pub coordinate: NsqEightDimensionalCoordinate,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WakeFramework {
    pub framework_id: String,
    pub precompiled: bool,
    pub symbol_radius: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WakeDispatch {
    pub stamp: WakeStamp,
    pub framework: WakeFramework,
    pub hydrated_symbols: Vec<String>,
    pub ready: bool,
}

impl WakeDispatch {
    pub fn build(stamp: WakeStamp, framework: WakeFramework, hydrated_symbols: Vec<String>) -> Self {
        let ready = framework.precompiled && !hydrated_symbols.is_empty();
        Self {
            stamp,
            framework,
            hydrated_symbols,
            ready,
        }
    }
}
