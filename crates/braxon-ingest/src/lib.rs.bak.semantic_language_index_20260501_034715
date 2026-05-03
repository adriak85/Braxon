use BRAXON_core::nu128_install_oversight_status;
use nsq_runtime::audit_BRAXON_runtime_materials;
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct BRAXONIngestStatus {
    pub target_lineage: String,
    pub canonical_semantics: String,
    pub target_source_variant_gb: u64,
    pub nsq_storage_target_gb: String,
    pub nsq_hot_memory_target_gb: String,
    pub nsq_hot_residency_surface: String,
    pub active_source_lane: String,
    pub active_source_state: String,
    pub active_source_family: String,
    pub target_lineage_bound_to_active_source: bool,
    pub visible_source_host_bytes: u64,
    pub visible_source_within_chunk_window: bool,
    pub max_chunk_size_gb: u64,
    pub max_live_downloads: usize,
    pub current_materialized_shards: usize,
    pub required_shards: usize,
    pub pointer_shards: usize,
    pub direct_source_path_ready: bool,
    pub runtime_authority_bound: bool,
    pub next_chunk_allowed: bool,
    pub target_manifest_bound: bool,
    pub target_manifest_state: String,
    pub next_action: String,
}

pub fn workspace_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .and_then(Path::parent)
        .expect("workspace root from crates/Braxon-ingest")
        .to_path_buf()
}

pub fn BRAXON_ingest_status(root: &Path) -> BRAXONIngestStatus {
    let oversight = nu128_install_oversight_status(root);
    let runtime_material_audit = audit_BRAXON_runtime_materials(root);
    let active_source_lane = if runtime_material_audit.authority.donor_source_lane.is_empty() {
        oversight.source_authority_lane.clone()
    } else {
        runtime_material_audit.authority.donor_source_lane.clone()
    };
    let active_source_family = infer_source_family(&active_source_lane).to_string();
    let visible_source_host_bytes = lane_visible_host_bytes(root, &active_source_lane);
    let max_chunk_size_host_bytes = oversight.max_chunk_size_gb.saturating_mul(1024_u64.pow(3));
    let target_manifest_bound = false;
    let target_manifest_state = "unbound_remote_manifest".to_string();
    let target_lineage_bound_to_active_source =
        target_lineage_matches_source(&oversight.model_lineage, &active_source_family);

    BRAXONIngestStatus {
        target_lineage: oversight.model_lineage.clone(),
        canonical_semantics: oversight.canonical_semantics.clone(),
        target_source_variant_gb: oversight.target_source_variant_gb,
        nsq_storage_target_gb: oversight.nsq_storage_target_gb.clone(),
        nsq_hot_memory_target_gb: oversight.nsq_hot_memory_target_gb.clone(),
        nsq_hot_residency_surface: oversight.nsq_hot_residency_surface.clone(),
        active_source_lane: active_source_lane.clone(),
        active_source_state: runtime_material_audit.authority.donor_source_state.clone(),
        active_source_family,
        target_lineage_bound_to_active_source,
        visible_source_host_bytes,
        visible_source_within_chunk_window: visible_source_host_bytes <= max_chunk_size_host_bytes,
        max_chunk_size_gb: oversight.max_chunk_size_gb,
        max_live_downloads: oversight.max_live_downloads,
        current_materialized_shards: runtime_material_audit.donor.materialized_shard_count,
        required_shards: runtime_material_audit.donor.required_shard_count,
        pointer_shards: runtime_material_audit.donor.pointer_shard_count,
        direct_source_path_ready: oversight.direct_source_path_ready,
        runtime_authority_bound: oversight.runtime_authority_bound,
        next_chunk_allowed: oversight.next_chunk_allowed,
        target_manifest_bound,
        target_manifest_state: target_manifest_state.clone(),
        next_action: next_action(
            &oversight.model_lineage,
            target_lineage_bound_to_active_source,
            oversight.target_source_variant_gb,
            &oversight.nsq_storage_target_gb,
            &oversight.nsq_hot_memory_target_gb,
            &oversight.nsq_hot_residency_surface,
            target_manifest_bound,
            &target_manifest_state,
            &runtime_material_audit.authority.donor_source_state,
            runtime_material_audit.donor.materialized_shard_count,
            runtime_material_audit.donor.required_shard_count,
            oversight.runtime_authority_bound,
        ),
    }
}

fn infer_source_family(active_source_lane: &str) -> &'static str {
    if active_source_lane.contains("qwen_transport") || active_source_lane.contains("braxon_transport")
    {
        "huihui_qwen3_5_27b_abliterated"
    } else {
        "unclassified_donor_lane"
    }
}

fn target_lineage_matches_source(target_lineage: &str, active_source_family: &str) -> bool {
    let target_family = infer_target_family(target_lineage);
    target_family != "unclassified_target_lineage" && target_family == active_source_family
}

fn infer_target_family(target_lineage: &str) -> &'static str {
    let target_lineage = target_lineage.to_ascii_lowercase();
    if target_lineage.contains("qwen") {
        "huihui_qwen3_5_27b_abliterated"
    } else if target_lineage.contains("llama") && target_lineage.contains("604b") {
        "llama_4_2_604b_fp32_abliterated"
    } else {
        "unclassified_target_lineage"
    }
}

fn next_action(
    target_lineage: &str,
    target_lineage_bound_to_active_source: bool,
    target_source_variant_gb: u64,
    nsq_storage_target_gb: &str,
    nsq_hot_memory_target_gb: &str,
    nsq_hot_residency_surface: &str,
    target_manifest_bound: bool,
    target_manifest_state: &str,
    active_source_state: &str,
    current_materialized_shards: usize,
    required_shards: usize,
    runtime_authority_bound: bool,
) -> String {
    if !target_lineage_bound_to_active_source {
        return format!(
            "download_and_convert_target_lineage_to_nsq:{target_lineage}:source_variant_{target_source_variant_gb}gb:storage_{nsq_storage_target_gb}gb:hot_{nsq_hot_memory_target_gb}gb:{nsq_hot_residency_surface}"
        );
    }
    if !target_manifest_bound {
        return format!(
            "bind_real_target_manifest_for_{target_lineage}:{target_manifest_state}"
        );
    }
    if active_source_state == "direct_source_materialization_required" {
        return "replace_pointer_catalog_with_direct_materialization".to_string();
    }
    if current_materialized_shards < required_shards {
        return format!(
            "continue_50gb_chunk_materialization:{current_materialized_shards}/{required_shards}"
        );
    }
    if !runtime_authority_bound {
        return "bind_pointer_free_runtime_authority".to_string();
    }

    "advance_nsq_recode_for_next_chunk".to_string()
}

fn lane_visible_host_bytes(root: &Path, relative_lane: &str) -> u64 {
    fn sum_dir_bytes(path: &Path) -> u64 {
        let Ok(entries) = fs::read_dir(path) else {
            return 0;
        };

        let mut total = 0u64;
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                total = total.saturating_add(sum_dir_bytes(&path));
                continue;
            }
            if let Ok(metadata) = entry.metadata() {
                total = total.saturating_add(metadata.len());
            }
        }
        total
    }

    sum_dir_bytes(&root.join(relative_lane))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repo_status_keeps_604b_target_separate_from_the_qwen_donor_lane() {
        let status = BRAXON_ingest_status(&workspace_root());
        assert_eq!(status.target_lineage, "llama_4.2_604b_fp32_abliterated_800gb");
        assert_eq!(status.target_source_variant_gb, 800);
        assert_eq!(status.nsq_storage_target_gb, "2.2");
        assert_eq!(status.nsq_hot_memory_target_gb, "1.02");
        assert_eq!(status.nsq_hot_residency_surface, "bus");
        assert_eq!(
            status.active_source_family,
            "huihui_qwen3_5_27b_abliterated"
        );
        assert!(!status.target_lineage_bound_to_active_source);
        assert_eq!(status.current_materialized_shards, 6);
        assert_eq!(status.required_shards, 11);
        assert_eq!(status.pointer_shards, 0);
        assert!(status.visible_source_within_chunk_window);
        assert!(!status.target_manifest_bound);
        assert!(status
            .next_action
            .starts_with("download_and_convert_target_lineage_to_nsq:"));
    }
}

// council_six_expected_model_count=6
// The Council of Six registry supersedes the old 604B-only target count while preserving
// the separation rule between target identity and donor/source lane.
