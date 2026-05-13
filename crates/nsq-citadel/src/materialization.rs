//! Materialization — writes the Citadel699 proof files.
//!
//! `BraxonBus` reads `state/nsq/proofs/citadel699_current_rebuild.json` to find
//! the rebuild directory, then reads `{rebuild_dir}/council_ten.materialization.json`
//! to check that transfer_form=nsq_only, target_size_class=mb_scale, and that
//! model_count == required_model_count. Run `nsq-citadel-materialize` once after
//! initial setup (and after any model roster change) to write these files.

use serde_json::json;
use std::path::Path;

pub const REBUILD_DIR: &str = "state/nsq/citadel699/current_rebuild";

pub fn write_materialization(root: &Path) -> Result<(), String> {
    // 1. Create rebuild directory.
    let rebuild_path = root.join(REBUILD_DIR);
    std::fs::create_dir_all(&rebuild_path)
        .map_err(|e| format!("create rebuild dir: {e}"))?;

    // 2. Write council_ten.materialization.json
    //    transfer_form + target_size_class must match for nsq_only_mb_bus=true.
    let materialization = json!({
        "schema": "braxon.nsq.council_ten.materialization.v1",
        "authority": "NSQ_COURT",
        "transfer_form": "nsq_only",
        "target_size_class": "mb_scale",
        "required_model_count": 10,
        "capital_count": 5,
        "poles_per_capital": 2,
        "coaching_config": "config/nsq/coaching.json",
        "models": [
            {
                "pole": "maverick",  "lane": 1,  "capital": 1,
                "model": "deepseek-v3-671b",         "role": "maverick_logic",
                "reconstruct_local": true,           "source_identity": "offline_minimal_seed_reconstruction::maverick_logic"
            },
            {
                "pole": "qwen",      "lane": 2,  "capital": 1,
                "model": "qwen3-235b-a22b",          "role": "qwen_creativity",
                "reconstruct_local": true,           "source_identity": "offline_minimal_seed_reconstruction::qwen_creativity"
            },
            {
                "pole": "arbiter",   "lane": 3,  "capital": 2,
                "model": "qwen2.5-72b",              "role": "arbiter_judge",
                "reconstruct_local": true,           "source_identity": "offline_minimal_seed_reconstruction::arbiter_judge"
            },
            {
                "pole": "analyzer",  "lane": 4,  "capital": 2,
                "model": "deepseek-v3-671b-analyzer", "role": "analyzer_auditor",
                "reconstruct_local": true,           "source_identity": "offline_minimal_seed_reconstruction::analyzer_auditor"
            },
            {
                "pole": "limbic",    "lane": 5,  "capital": 3,
                "model": "llama3.3-70b",             "role": "limbic_empath",
                "reconstruct_local": true,           "source_identity": "offline_minimal_seed_reconstruction::limbic_empath"
            },
            {
                "pole": "support",   "lane": 6,  "capital": 3,
                "model": "gemma3-27b",               "role": "support_memory",
                "reconstruct_local": true,           "source_identity": "offline_minimal_seed_reconstruction::support_memory"
            },
            {
                "pole": "voice",     "lane": 7,  "capital": 4,
                "model": "IndexTTS2",                "role": "voice_body",
                "reconstruct_local": true,           "source_identity": "offline_minimal_seed_reconstruction::voice_body"
            },
            {
                "pole": "image",     "lane": 8,  "capital": 4,
                "model": "FLUX.1-dev",               "role": "image_cortex",
                "reconstruct_local": true,           "source_identity": "offline_minimal_seed_reconstruction::image_cortex"
            },
            {
                "pole": "video",     "lane": 9,  "capital": 5,
                "model": "Wan2.1-T2V-14B",           "role": "video_cortex",
                "reconstruct_local": true,           "source_identity": "offline_minimal_seed_reconstruction::video_cortex"
            },
            {
                "pole": "world",     "lane": 10, "capital": 5,
                "model": "Hunyuan3D-2.1",            "role": "world_body_3d",
                "reconstruct_local": true,           "source_identity": "offline_minimal_seed_reconstruction::world_body_3d"
            }
        ]
    });

    let mat_path = rebuild_path.join("council_ten.materialization.json");
    std::fs::write(
        &mat_path,
        serde_json::to_string_pretty(&materialization).unwrap(),
    )
    .map_err(|e| format!("write materialization: {e}"))?;

    // 3. Write the proof pointer.
    let proofs_dir = root.join("state/nsq/proofs");
    std::fs::create_dir_all(&proofs_dir)
        .map_err(|e| format!("create proofs dir: {e}"))?;

    let proof = json!({
        "schema": "braxon.nsq.citadel699.rebuild_proof.v1",
        "authority": "NSQ_COURT",
        "rebuild_dir": REBUILD_DIR,
        "capital_count": 5,
        "pole_count": 10,
        "transfer_form": "nsq_only",
        "target_size_class": "mb_scale",
        "citadel_wire_active": true,
        "reconstruction_mode": "offline_minimal_seed_reconstruction"
    });

    let proof_path = proofs_dir.join("citadel699_current_rebuild.json");
    std::fs::write(
        &proof_path,
        serde_json::to_string_pretty(&proof).unwrap(),
    )
    .map_err(|e| format!("write proof: {e}"))?;

    Ok(())
}
