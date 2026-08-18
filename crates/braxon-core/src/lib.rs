#![allow(non_snake_case)]
pub mod bus;
pub mod content_surface;
pub mod context_manifest;
pub mod council;
pub mod council_ten;
pub mod dynamic_parameter_runtime;
pub mod ghost_memory;
pub mod greeting;
pub mod initiative_cluster_runtime;
pub mod kinetic_reflexor;
pub mod native_bus;
pub mod native_stack;
pub mod nsq_native;
pub mod offline_agent;
pub mod offline_models;
pub mod piston_memory;
pub mod riemann_semantic_reflexor;
pub mod seed_citadel;
pub mod semantic_link;
pub mod target_field;
pub mod wowas;
pub mod wowas_rescue;
pub mod wowas_seeded;

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub use content_surface::{
    daydream_frame, DaydreamFrame, FactRecord, NarrativeRecord, DAYDREAM_SCHEMA, FACT_SCHEMA,
    NARRATIVE_SCHEMA,
};
pub use dynamic_parameter_runtime::{
    execute_dynamic_parameter_pipeline, run_training_microbenchmark, DynamicPipelineReceipt,
    TrainingBenchmarkReport, TrainingBenchmarkResult, DYNAMIC_PARAMETER_RUNTIME_SCHEMA,
};
pub use ghost_memory::{
    FireDecision, FireReport, FiringLease, GhostMemoryBus, PageState, WireKind, WirePage,
    DEFAULT_PAGE_BYTES, FIRING_WINDOW_BYTES, GHOST_MEMORY_SCHEMA, VIRTUAL_EXTENSION_BASE,
    VIRTUAL_EXTENSION_LIMIT,
};
pub use initiative_cluster_runtime::{
    execute_through_reflexor, InitiativeClusterExecutionReceipt, INITIATIVE_CLUSTER_RUNTIME_SCHEMA,
};
pub use kinetic_reflexor::{
    BusValue, HardwareWriteAck, KineticReflexor, ReflexorPhase, ReflexorReport, ValueClass,
    ValueDelta, Watermark, KINETIC_REFLEXOR_SCHEMA, WATERMARK_FAMILY,
};
pub use native_bus::{NativeNsqBus, NATIVE_BRAXON_BUS_SCHEMA};
pub use native_stack::NativeNsqStack;
pub use nsq_native::{
    AddressLease, CouncilSurface, DaydreamWorkload, IntentOutcome, NsqIntent, NsqIntentDecision,
    NsqNativeBus, PistonPhase, NSQ_NATIVE_BUS_SCHEMA, NSQ_NATIVE_INTENT_SCHEMA,
};
pub use piston_memory::{
    MemoryDecision, MemoryDecisionReport, MemoryLease, MemoryRegion, PistonMemory,
    PistonPhase as MemoryPistonPhase, RegionKind, Residency, PISTON_MEMORY_SCHEMA,
};
pub use riemann_semantic_reflexor::{
    ReflexorSearchStep, RiemannSemanticReflexor, ZeroObservation, ZeroRegionHypothesis,
    RIEMANN_REFLEXOR_SCHEMA,
};
pub use semantic_link::{
    SemanticLinkReceipt, SemanticLinkRequest, SemanticLinkResolution, SemanticLinkSurface,
    SEMANTIC_LINK_SCHEMA,
};
pub use target_field::{TargetField, TargetFieldActuation, TARGET_FIELD_PATH, TARGET_FIELD_SCHEMA};

pub use seed_citadel::{
    build_seed_plan, materialize_window, CitadelState, SeedMaterializationPlan, TokenSection,
    UniversalToken, UniversalTokenizerSeed, CITADEL_MATERIALIZATION_VERSION,
    UNIVERSAL_TOKENIZER_VERSION,
};
pub use wowas_seeded::{WhispersWorld, WorldEntity, WorldFrame, WorldSeed};

pub use bus::{
    BraxonBus, BraxonBusReport, BusReplyLayer, IntentEnglishLoop, SharedThought, SpeechLoopState,
    ThoughtPressureCandidate, BRAXON_BUS_ROUTE, BRAXON_BUS_SCHEMA, BRAXON_REPLY_SCHEMA,
};
pub use council_ten::{
    CouncilTen, CouncilTenWakeTrace, WakeStep, WakeStepResult, COUNCIL_TEN_AUTHORITY,
    STAMP_WAKE_COUNCIL_TEN,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BRAXONIdentity {
    pub name: String,
    pub version: String,
    pub canonical_semantics: String,
}

impl BRAXONIdentity {
    pub fn current() -> Self {
        Self {
            name: "Braxon".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            canonical_semantics: "base8_switch_topology".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Nu128InstallOversightStatus {
    pub model_lineage: String,
    pub canonical_semantics: String,
    pub target_source_variant_gb: u64,
    pub nsq_storage_target_gb: String,
    pub nsq_hot_memory_target_gb: String,
    pub nsq_hot_residency_surface: String,
    pub max_chunk_size_gb: u64,
    pub max_live_downloads: usize,
    pub source_authority_lane: String,
    pub direct_source_path_ready: bool,
    pub runtime_authority_bound: bool,
    pub next_chunk_allowed: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct Nu128InstallOversightConfig {
    model_lineage: Option<String>,
    canonical_semantics: Option<String>,
    target_source_variant_gb: Option<u64>,
    nsq_storage_target_gb: Option<String>,
    nsq_hot_memory_target_gb: Option<String>,
    nsq_hot_residency_surface: Option<String>,
    max_chunk_size_gb: Option<u64>,
    max_live_downloads: Option<usize>,
    source_authority_lane: Option<String>,
    runtime_authority_required: Option<bool>,
    require_ingress_recode_before_next_chunk: Option<bool>,
}

pub use context_manifest::{
    braxon_context_manifest_path, braxon_context_manifest_status, braxon_wake_linked_change_report,
    braxon_wake_linked_change_report_from_env, braxon_wake_linked_change_report_with_db,
    load_braxon_chain_root_db, load_braxon_context_manifest, BraxonContextManifest,
    BraxonContextManifestStatus, ChainRootRecord, ContextChainRootDb, LeftOutContext,
    LinkedChangeSuggestion, LinkedSurfaceTag, MissingContextPointer, RuntimeAdjustmentLane,
    SemanticPointer, WakeActionAnnouncement, WakeChainLink, WakeLinkedChangeReport,
};
pub use offline_agent::{
    load_or_initialize_offline_agent_state, save_offline_agent_state, OfflineAgentState,
    OfflineTaskAction, OfflineTaskCounts, OfflineTaskStatus,
};
pub use offline_models::{
    load_or_initialize_model_registry, save_model_registry, ModelAssetRecord, ModelRegistry,
};

pub use offline_models::load_or_initialize_model_registry as load_or_initialize_offline_model_registry;
pub use offline_models::save_model_registry as save_offline_model_registry;
pub use offline_models::ModelAssetRecord as OfflineModelAssetRecord;
pub use offline_models::ModelRegistry as OfflineModelRegistryState;

pub fn nu128_install_oversight_status(root: &Path) -> Nu128InstallOversightStatus {
    let config_path = root.join("config/nsq/nu128_install_oversight.json");
    let config = fs::read_to_string(&config_path)
        .ok()
        .and_then(|raw| serde_json::from_str::<Nu128InstallOversightConfig>(&raw).ok());

    let source_authority_lane = config
        .as_ref()
        .and_then(|cfg| cfg.source_authority_lane.clone())
        .unwrap_or_else(|| "assets/braxon_core/source_ingest/braxon_transport".to_string());

    let direct_source_path_ready = root.join(&source_authority_lane).exists();
    let runtime_authority_required = config
        .as_ref()
        .and_then(|cfg| cfg.runtime_authority_required)
        .unwrap_or(true);

    Nu128InstallOversightStatus {
        model_lineage: config
            .as_ref()
            .and_then(|cfg| cfg.model_lineage.clone())
            .unwrap_or_else(|| "llama_4.2_604b_fp32_abliterated_800gb".to_string()),
        canonical_semantics: config
            .as_ref()
            .and_then(|cfg| cfg.canonical_semantics.clone())
            .unwrap_or_else(|| "base8_switch_topology".to_string()),
        target_source_variant_gb: config
            .as_ref()
            .and_then(|cfg| cfg.target_source_variant_gb)
            .unwrap_or(800),
        nsq_storage_target_gb: config
            .as_ref()
            .and_then(|cfg| cfg.nsq_storage_target_gb.clone())
            .unwrap_or_else(|| "2.2".to_string()),
        nsq_hot_memory_target_gb: config
            .as_ref()
            .and_then(|cfg| cfg.nsq_hot_memory_target_gb.clone())
            .unwrap_or_else(|| "1.02".to_string()),
        nsq_hot_residency_surface: config
            .as_ref()
            .and_then(|cfg| cfg.nsq_hot_residency_surface.clone())
            .unwrap_or_else(|| "bus".to_string()),
        max_chunk_size_gb: config
            .as_ref()
            .and_then(|cfg| cfg.max_chunk_size_gb)
            .unwrap_or(50),
        max_live_downloads: config
            .as_ref()
            .and_then(|cfg| cfg.max_live_downloads)
            .unwrap_or(1),
        source_authority_lane,
        direct_source_path_ready,
        runtime_authority_bound: !runtime_authority_required,
        next_chunk_allowed: !config
            .as_ref()
            .and_then(|cfg| cfg.require_ingress_recode_before_next_chunk)
            .unwrap_or(true),
    }
}
