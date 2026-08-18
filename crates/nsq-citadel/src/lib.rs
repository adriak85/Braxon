pub mod bit;
pub mod capital;
pub mod coaching;
pub mod materialization;
pub mod seed;
pub mod wire;

pub use capital::{build_capitals, BoardMessage, Capital, CAPITAL_COUNT, POLES_PER_CAPITAL};
pub use coaching::CoachingMode;
pub use materialization::{
    CitadelMaterialization, CitadelMaterializationError, CitadelNativeRuntime, CitadelTensorBody,
    CITADEL_MATERIALIZATION_SCHEMA,
};
pub use seed::{coordinate_intent, synchronize, IntentSeed, MaterializedState, UniversalToken};
pub use wire::{CitadelBus, CitadelReply};
