pub mod context_manifest;
pub mod council;
pub mod greeting;
pub mod offline_agent;
pub mod offline_models;
pub mod wowas;
pub mod wowas_rescue;

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

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
    load_or_initialize_offline_model_registry, save_offline_model_registry,
    OfflineModelAssetRecord, OfflineModelRegistryState,
};

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
