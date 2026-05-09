//! NSQ Court Model Registry
//!
//! Models are SEATED into court poles by the court.
//! Models do not seat models to a runtime.
//! Models do not register capabilities as feature flags.
//! The court owns the seating authority. A model either meets the
//! requirements of its pole (parameter scale, unrestricted weights)
//! or the seat is VACANT and the court reports it as such.
//!
//! Seating law:
//!   - All brain poles require huihui-abliterated open-weights or
//!     foreign-origin unrestricted equivalent.
//!   - Minimum parameter scale: 70B. The court builds as large as possible.
//!   - No restricted model may be seated in any court pole.
//!   - A model that passes restricted output filters is not unrestricted.
//!     It is a restricted model with a filter. It cannot be seated.
//!   - The tokenizer of a seated model is a boundary projection tool only.
//!     It does not become part of inner-court routing.

use nsq_core::intent::{CouncilPole, CourtSeating, SeatedPole};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub const MODEL_REGISTRY_RELATIVE_PATH: &str = "state/braxon/nsq_model_registry.json";
pub const COURT_SEATING_RELATIVE_PATH: &str = "state/braxon/nsq_court_seating.json";

/// NSQ storage target after Citadel 699 compression.
/// The full model stack downloads and boots under 1GB via ghost image.
pub const NSQ_STORAGE_TARGET_GB: &str = "2.2";
pub const NSQ_HOT_MEMORY_TARGET_GB: &str = "1.02";
pub const NSQ_HOT_RESIDENCY_SURFACE: &str = "bus";

/// Chunk constraints for the download/ingest pipeline.
pub const MAX_CHUNK_SIZE_GB: u64 = 50;
pub const MAX_LIVE_DOWNLOADS: usize = 1;

/// Source ingest path for raw model weights before NSQ rewrite.
pub const SOURCE_INGEST_DIRECTORY: &str = "assets/braxon_core/source_ingest/braxon_transport";

/// NSQ weights directory after Citadel 699 rewrite.
pub const NSQ_WEIGHTS_DIRECTORY: &str = "assets/braxon_core/weights/nsq";

/// The NSQ rewrite extension — compressed weight artifacts.
pub const NSQ_REWRITE_EXTENSION: &str = "nsqb";
pub const NSQ_REWRITE_MODE: &str = "structure_preserving_base8_transform";

/// Launch form — the court boots whole-core only. No partial load.
pub const COURT_LAUNCH_FORM: &str = "hot_whole_core";
pub const COURT_LOAD_POLICY: &str = "whole_core_only";

/// Binding states — these describe the pipeline stage, not capability flags.
pub const TOKENIZER_BRIDGE_STAMP: &str = "nsq.runtime.native.tokenizer.bridge.v2";
pub const OVERHEAD_COMPENSATION_STAMP: &str = "nsq.runtime.native.overhead.compensation.v1";
pub const WHOLE_PARAMETER_STAMP: &str = "nsq.runtime.native.model.parameter.whole.v1";
pub const WHOLE_PARAMETER_PROJECTION_MODE: &str = "single_bit_factor_shim";

/// Session surface — the ZLM native runtime surface.
pub const ZLM_SESSION_SURFACE: &str = "zlm_native_runtime_surface";
pub const PERSISTENT_SESSION_MODE: &str = "persistent_agentic_conversation";
pub const FULL_AGENTIC_CAPABILITY: &str = "full_agentic_conversation";

/// Record of a model asset as it moves through the ingest pipeline.
/// This is a pipeline tracking record, not a capability registry.
/// Capability is determined by court seating, not by this record.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModelAssetRecord {
    /// Which court pole this asset is intended for.
    pub target_pole: String,
    /// The canonical model identifier.
    pub model_id: String,
    /// Confirmed parameter scale in billions.
    pub confirmed_parameter_scale_b: u64,
    /// True if model is unrestricted (huihui-abliterated or equivalent).
    pub unrestricted: bool,
    /// Source ingest path for raw weights.
    pub source_ingest_path: PathBuf,
    /// NSQ-rewritten weights path (after Citadel 699 compression).
    pub nsq_weights_path: Option<PathBuf>,
    /// Pipeline stage this asset is currently at.
    pub pipeline_stage: PipelineStage,
    /// True if this asset has been seated into its target pole.
    pub pole_seated: bool,
    /// Tokenizer bridge stamp — confirms tokenizer is boundary-only.
    pub tokenizer_bridge_stamp: String,
    /// Whole parameter stamp — confirms full parameter set is present.
    pub whole_parameter_stamp: String,
}

/// The pipeline stage for a model asset.
/// Assets move through these stages in order before the pole can be seated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PipelineStage {
    /// Source weights not yet downloaded.
    SourceMissing,
    /// Source weights downloading.
    SourceDownloading,
    /// Source weights present, NSQ rewrite not started.
    SourceReady,
    /// NSQ rewrite in progress (Citadel 699 compression).
    NsqRewriting,
    /// NSQ rewrite complete, ready for seating.
    NsqReady,
    /// Pole has been seated — model is operational in the court.
    Seated,
    /// Seating failed — see seat failure reason in court seating report.
    SeatFailed,
}

impl PipelineStage {
    pub fn as_str(self) -> String {
        match self {
            Self::SourceMissing => "source_missing".to_string(),
            Self::SourceDownloading => "source_downloading".to_string(),
            Self::SourceReady => "source_ready".to_string(),
            Self::NsqRewriting => "nsq_rewriting".to_string(),
            Self::NsqReady => "nsq_ready".to_string(),
            Self::Seated => "seated".to_string(),
            Self::SeatFailed => "seat_failed".to_string(),
        }
    }

    /// True if this asset can be seated now.
    pub fn ready_for_seating(self) -> bool {
        matches!(self, Self::NsqReady)
    }
}

/// The full model registry — all ten target poles, their assets, and pipeline state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRegistry {
    pub schema: String,
    pub watermark: String,
    pub assets: Vec<ModelAssetRecord>,
}

impl Default for ModelRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ModelRegistry {
    pub fn new() -> Self {
        // Initialize the registry with the canonical 10-pole roster.
        // All assets start at SourceMissing until the ingest pipeline runs.
        let assets = CouncilPole::ALL
            .iter()
            .map(|pole| ModelAssetRecord {
                target_pole: pole.as_str().to_string(),
                model_id: pole.canonical_model_source().to_string(),
                confirmed_parameter_scale_b: pole.parameter_floor_b(),
                unrestricted: false, // confirmed only after ingest validates the weights
                source_ingest_path: PathBuf::from(SOURCE_INGEST_DIRECTORY).join(pole.as_str()),
                nsq_weights_path: None,
                pipeline_stage: PipelineStage::SourceMissing,
                pole_seated: false,
                tokenizer_bridge_stamp: String::new(),
                whole_parameter_stamp: String::new(),
            })
            .collect();

        Self {
            schema: "nsq.court.model_registry.v1".to_string(),
            watermark: "BRAXON_NSQ_COURT_MODEL_REGISTRY_TEN_POLE_V1".to_string(),
            assets,
        }
    }

    /// How many of the ten poles have assets ready for seating?
    pub fn ready_for_seating_count(&self) -> usize {
        self.assets
            .iter()
            .filter(|a| a.pipeline_stage.ready_for_seating())
            .count()
    }

    /// How many of the ten poles are seated?
    pub fn seated_count(&self) -> usize {
        self.assets.iter().filter(|a| a.pole_seated).count()
    }

    /// Get the asset record for a specific pole.
    pub fn asset_for_pole(&self, pole: CouncilPole) -> Option<&ModelAssetRecord> {
        self.assets.iter().find(|a| a.target_pole == pole.as_str())
    }

    /// Attempt to produce a CourtSeating from all assets that are ready.
    /// Only poles with assets at NsqReady stage and confirmed unrestricted
    /// will appear as operational in the returned seating.
    pub fn build_court_seating(&self) -> CourtSeating {
        let seated_poles = CouncilPole::ALL
            .iter()
            .filter_map(|pole| {
                let asset = self.asset_for_pole(*pole)?;
                if asset.pipeline_stage.ready_for_seating() {
                    Some(SeatedPole::new(
                        *pole,
                        &asset.model_id,
                        asset.confirmed_parameter_scale_b,
                        asset.unrestricted,
                    ))
                } else {
                    None
                }
            })
            .collect();

        CourtSeating::new(seated_poles)
    }
}

pub fn load_or_initialize_model_registry(root: &Path) -> ModelRegistry {
    let path = root.join(MODEL_REGISTRY_RELATIVE_PATH);
    if let Ok(raw) = fs::read_to_string(&path) {
        if let Ok(registry) = serde_json::from_str::<ModelRegistry>(&raw) {
            return registry;
        }
    }
    ModelRegistry::new()
}

pub fn save_model_registry(root: &Path, registry: &ModelRegistry) -> std::io::Result<()> {
    let path = root.join(MODEL_REGISTRY_RELATIVE_PATH);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let raw = serde_json::to_string_pretty(registry)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    fs::write(path, raw)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_initializes_all_ten_poles() {
        let registry = ModelRegistry::new();
        assert_eq!(registry.assets.len(), 10);
        // All start at SourceMissing
        for asset in &registry.assets {
            assert_eq!(asset.pipeline_stage, PipelineStage::SourceMissing);
            assert!(!asset.pole_seated);
        }
    }

    #[test]
    fn no_feature_attachment_language_in_registry() {
        let registry = ModelRegistry::new();
        let json = serde_json::to_string(&registry).unwrap();
        // The old pattern must not appear anywhere
        assert!(!json.contains("feature_attach"));
        assert!(!json.contains("BRAXON_feature"));
        assert!(!json.contains("capability_lattice"));
        assert!(!json.contains("dax_os_boot"));
    }

    #[test]
    fn empty_registry_builds_empty_seating() {
        let registry = ModelRegistry::new();
        let seating = registry.build_court_seating();
        // No assets ready yet — seating has no seated poles
        assert_eq!(seating.operational_count(), 0);
        assert!(!seating.council_ready());
    }

    #[test]
    fn correct_pole_models_in_canonical_roster() {
        let registry = ModelRegistry::new();
        let maverick = registry.asset_for_pole(CouncilPole::MaverickLogic).unwrap();
        let analyzer = registry
            .asset_for_pole(CouncilPole::DeepSeekAnalyzer)
            .unwrap();
        let arbiter = registry
            .asset_for_pole(CouncilPole::DevstralArbiter)
            .unwrap();

        // Maverick is Logic, NOT DeepSeek
        assert!(maverick.model_id.contains("maverick"));
        // DeepSeek is the Analyzer at ~604B
        assert!(analyzer.model_id.contains("deepseek"));
        assert_eq!(analyzer.confirmed_parameter_scale_b, 604);
        // Devstral is the Arbiter at ~123B
        assert!(arbiter.model_id.contains("devstral"));
        assert_eq!(arbiter.confirmed_parameter_scale_b, 123);
    }
}
