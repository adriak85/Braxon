use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use serde_json::Value;

fn workspace_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .and_then(Path::parent)
        .expect("workspace root from crates/nsq-runtime")
        .to_path_buf()
}

fn read_json(path: &Path) -> Value {
    let raw = fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|e| panic!("failed to parse json {}: {e}", path.display()))
}

fn read_status_map(path: &Path) -> BTreeMap<String, String> {
    let raw = fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));
    raw.lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                return None;
            }
            let (k, v) = trimmed.split_once('=')?;
            Some((k.trim().to_string(), v.trim().to_string()))
        })
        .collect()
}

fn s<'a>(v: &'a Value, path: &[&str]) -> &'a str {
    let mut cur = v;
    for key in path {
        cur = &cur[*key];
    }
    cur.as_str()
        .unwrap_or_else(|| panic!("path {:?} was not a string", path))
}

fn b(v: &Value, path: &[&str]) -> bool {
    let mut cur = v;
    for key in path {
        cur = &cur[*key];
    }
    cur.as_bool()
        .unwrap_or_else(|| panic!("path {:?} was not a bool", path))
}

#[test]
fn BRAXON_whole_core_native_lane_is_bound_to_BRAXON_identity() {
    let root = workspace_root();

    let manifest = read_json(&root.join("models/braxon/manifest.json"));
    assert_eq!(s(&manifest, &["id"]), "Braxon");
    assert_eq!(s(&manifest, &["label"]), "BRAXON");
    assert_eq!(s(&manifest, &["provider_family"]), "Braxon");
    assert_eq!(
        s(&manifest, &["authority_lane"]),
        "offline_model_native_runtime_lane"
    );
    assert_eq!(
        s(&manifest, &["runtime_authority"]),
        "rust_native_offline_model_lane"
    );
    assert!(b(&manifest, &["offline_only"]));
    assert!(!b(&manifest, &["cxx_runtime_authority"]));
    assert_eq!(s(&manifest, &["external_tool_host"]), "none");
    assert_eq!(
        s(&manifest, &["representation_mode"]),
        "stamp_bound_manifest"
    );
    assert_eq!(s(&manifest, &["runtime_load_policy"]), "whole_core_only");
    assert_eq!(s(&manifest, &["launch_form"]), "hot_whole_core");

    assert_eq!(
        s(&manifest, &["source_weights_directory"]),
        "assets/braxon_core/source_ingest/braxon_transport"
    );
    assert_eq!(
        s(&manifest, &["weights_directory"]),
        "assets/braxon_core/weights/nsq"
    );
    assert_eq!(
        s(&manifest, &["nsq_rewrite_artifact"]),
        "assets/braxon_core/weights/nsq/Braxon-27B_extended.nsqb"
    );
    assert_eq!(
        s(&manifest, &["nsq_envelope_artifact"]),
        "assets/braxon_core/weights/nsq/Braxon-27B_extended.nsqb.meta"
    );
    assert_eq!(
        s(&manifest, &["whole_parameter_stamp"]),
        "nsq.runtime.native.model.parameter.whole.v1"
    );

    assert_eq!(
        s(&manifest, &["delta_extension_mode"]),
        "sealed_reference_structure"
    );
    assert_eq!(
        s(&manifest, &["delta_extension_activation_mode"]),
        "semantic_score_alignment"
    );
    assert_eq!(
        s(&manifest, &["supermodel_extension_mode"]),
        "sealed_reference_structure"
    );
    assert_eq!(
        s(&manifest, &["supermodel_extension_activation_mode"]),
        "semantic_score_alignment"
    );
    assert_eq!(
        s(&manifest, &["grid_26d_mode"]),
        "sealed_reference_structure"
    );
    assert_eq!(
        s(&manifest, &["grid_26d_activation_mode"]),
        "semantic_score_alignment"
    );

    assert_eq!(
        s(&manifest, &["source_ingest_status"]),
        "direct_source_materialization_required"
    );
    assert_eq!(
        s(&manifest, &["source_authority_state"]),
        "direct_source_materialization_required"
    );
    assert_eq!(s(&manifest, &["nsq_artifact_state"]), "manifest_bundle_only");
    assert_eq!(s(&manifest, &["runtime_authority_lane"]), "none_bound");
    assert_eq!(s(&manifest, &["runtime_authority_state"]), "unbound");
    assert!(!b(&manifest, &["runtime_authority_bound"]));
    assert_eq!(s(&manifest, &["nsq_envelope_status"]), "updated");
    assert_eq!(s(&manifest, &["nsq_recode_status"]), "manifest_bundle_only");
    assert_eq!(
        s(&manifest, &["whole_core_runtime_status"]),
        "manifest_verified_not_hot_live"
    );
    assert_eq!(
        s(&manifest, &["tokenizer_binding_state"]),
        "runtime_selected_donor_rooted_unified_overlay"
    );
    assert_eq!(
        s(&manifest, &["parameter_binding_state"]),
        "direct_source_materialization_required"
    );
}

#[test]
fn BRAXON_registry_and_binding_match_whole_core_native_truth() {
    let root = workspace_root();

    let registry = read_json(&root.join("state/braxon/offline_model_registry.json"));
    assert_eq!(
        s(&registry, &["lane_surface"]),
        "offline_model_native_runtime_lane"
    );
    assert_eq!(
        s(&registry, &["runtime_authority"]),
        "rust_native_offline_model_lane"
    );

    let assets = registry["assets"].as_array().expect("assets array");
    let Braxon = assets
        .iter()
        .find(|asset| asset["id"].as_str() == Some("Braxon"))
        .expect("Braxon asset present");

    assert_eq!(Braxon["label"].as_str().unwrap(), "BRAXON");
    assert_eq!(
        Braxon["authority_lane"].as_str().unwrap(),
        "offline_model_native_runtime_lane"
    );
    assert_eq!(
        Braxon["runtime_authority"].as_str().unwrap(),
        "rust_native_offline_model_lane"
    );
    assert_eq!(
        Braxon["source_ingest_status"].as_str().unwrap(),
        "direct_source_materialization_required"
    );
    assert_eq!(
        Braxon["source_authority_state"].as_str().unwrap(),
        "direct_source_materialization_required"
    );
    assert_eq!(Braxon["nsq_artifact_state"].as_str().unwrap(), "manifest_bundle_only");
    assert_eq!(Braxon["runtime_authority_lane"].as_str().unwrap(), "none_bound");
    assert_eq!(Braxon["runtime_authority_state"].as_str().unwrap(), "unbound");
    assert!(!Braxon["runtime_authority_bound"].as_bool().unwrap());
    assert_eq!(Braxon["nsq_envelope_status"].as_str().unwrap(), "updated");
    assert_eq!(
        Braxon["nsq_rewrite_status"].as_str().unwrap(),
        "manifest_bundle_only"
    );
    assert_eq!(
        Braxon["whole_core_runtime_status"].as_str().unwrap(),
        "manifest_verified_not_hot_live"
    );
    assert_eq!(
        Braxon["delta_extension_mode"].as_str().unwrap(),
        "sealed_reference_structure"
    );
    assert_eq!(
        Braxon["delta_extension_activation_mode"].as_str().unwrap(),
        "semantic_score_alignment"
    );
    assert!(Braxon["offline_only"].as_bool().unwrap());
    assert!(!Braxon["cxx_runtime_authority"].as_bool().unwrap());
    assert_eq!(Braxon["external_tool_host"].as_str().unwrap(), "none");

    let binding = read_json(&root.join("state/braxon/braxon_binding.json"));
    assert_eq!(s(&binding, &["model_name"]), "BRAXON");
    assert_eq!(s(&binding, &["core_identity"]), "BRAXON_core_primary_model");
    assert_eq!(s(&binding, &["binding_state"]), "manifest_bound_not_hot_live");
    assert_eq!(
        s(&binding, &["tokenizer", "binding_state"]),
        "runtime_selected_donor_rooted_unified_overlay"
    );
    assert_eq!(
        s(&binding, &["parameters", "binding_state"]),
        "direct_source_materialization_required"
    );
    assert_eq!(
        s(&binding, &["runtime_packaging", "source_ingest_directory"]),
        "assets/braxon_core/source_ingest/braxon_transport"
    );
    assert_eq!(
        s(&binding, &["runtime_packaging", "nsq_rewrite_directory"]),
        "assets/braxon_core/weights/nsq"
    );
    assert_eq!(
        s(&binding, &["runtime_packaging", "runtime_load_policy"]),
        "whole_core_only"
    );
    assert_eq!(
        s(&binding, &["runtime_packaging", "launch_form"]),
        "hot_whole_core"
    );
    assert_eq!(
        s(&binding, &["runtime_packaging", "source_ingest_status"]),
        "direct_source_materialization_required"
    );
    assert_eq!(
        s(&binding, &["runtime_packaging", "source_authority_state"]),
        "direct_source_materialization_required"
    );
    assert_eq!(
        s(&binding, &["runtime_packaging", "nsq_artifact_state"]),
        "manifest_bundle_only"
    );
    assert_eq!(
        s(&binding, &["runtime_packaging", "runtime_authority_lane"]),
        "none_bound"
    );
    assert_eq!(
        s(&binding, &["runtime_packaging", "runtime_authority_state"]),
        "unbound"
    );
    assert!(!b(&binding, &["runtime_packaging", "runtime_authority_bound"]));
    assert_eq!(
        s(&binding, &["runtime_packaging", "nsq_envelope_status"]),
        "updated"
    );
    assert_eq!(
        s(&binding, &["runtime_packaging", "nsq_recode_status"]),
        "manifest_bundle_only"
    );
    assert_eq!(
        s(
            &binding,
            &["runtime_packaging", "whole_core_runtime_status"]
        ),
        "manifest_verified_not_hot_live"
    );
    assert_eq!(
        s(&binding, &["parameters", "source_config"]),
        "assets/braxon_core/source_ingest/braxon_transport/config.json"
    );
    assert_eq!(
        s(&binding, &["parameters", "generation_config"]),
        "assets/braxon_core/source_ingest/braxon_transport/generation_config.json"
    );
}

#[test]
fn BRAXON_pipeline_and_sessions_prove_agent_surface_is_live() {
    let root = workspace_root();

    let pipeline = read_status_map(&root.join("state/braxon/braxon_nsq_pipeline.status"));
    assert_eq!(
        pipeline.get("source_ingest_status").map(String::as_str),
        Some("direct_source_materialization_required")
    );
    assert_eq!(
        pipeline.get("source_authority_state").map(String::as_str),
        Some("direct_source_materialization_required")
    );
    assert_eq!(
        pipeline.get("nsq_artifact_state").map(String::as_str),
        Some("manifest_bundle_only")
    );
    assert_eq!(
        pipeline.get("runtime_authority_lane").map(String::as_str),
        Some("none_bound")
    );
    assert_eq!(
        pipeline.get("runtime_authority_state").map(String::as_str),
        Some("unbound")
    );
    assert_eq!(
        pipeline.get("nsq_envelope_status").map(String::as_str),
        Some("updated")
    );
    assert_eq!(
        pipeline.get("nsq_recode_status").map(String::as_str),
        Some("manifest_bundle_only")
    );
    assert_eq!(
        pipeline
            .get("whole_core_runtime_status")
            .map(String::as_str),
        Some("manifest_verified_not_hot_live")
    );
    assert_eq!(
        pipeline.get("verification_state").map(String::as_str),
        Some("manifest_only")
    );
    assert_eq!(
        pipeline
            .get("artifact_verification_status")
            .map(String::as_str),
        Some("manifest_bundle_verified")
    );
    assert_eq!(
        pipeline
            .get("reserved_runtime_artifact_present")
            .map(String::as_str),
        Some("yes")
    );

    let finalize = read_status_map(&root.join("state/braxon/braxon_whole_core_finalize.status"));
    assert_eq!(
        finalize.get("whole_core_ready").map(String::as_str),
        Some("false")
    );
    assert_eq!(
        finalize
            .get("whole_core_runtime_status")
            .map(String::as_str),
        Some("manifest_verified_not_hot_live")
    );
    assert_eq!(
        finalize.get("runtime_authority_lane").map(String::as_str),
        Some("none_bound")
    );
    assert_eq!(
        finalize.get("runtime_authority_state").map(String::as_str),
        Some("unbound")
    );

    let sessions = read_json(&root.join("state/braxon/runtime_sessions.json"));
    let session_list = sessions["sessions"].as_array().expect("sessions array");

    let BRAXON_sessions: Vec<&Value> = session_list
        .iter()
        .filter(|s| s["model_id"].as_str() == Some("Braxon"))
        .collect();

    assert!(
        !BRAXON_sessions.is_empty(),
        "expected at least one Braxon session"
    );

    assert!(
        BRAXON_sessions.iter().any(|s| {
            s["session_surface"].as_str() == Some("zlm_native_runtime_surface")
                && s["session_mode"].as_str() == Some("persistent_agentic_conversation")
                && s["agentic_capability"].as_str() == Some("full_agentic_conversation")
        }),
        "expected at least one live Braxon agentic session"
    );

    assert!(
        BRAXON_sessions
            .iter()
            .all(|s| { s["BRAXON_core_identity"].as_str() == Some("BRAXON_core_primary_model") }),
        "all Braxon sessions must converge to BRAXON_core_primary_model"
    );
}
