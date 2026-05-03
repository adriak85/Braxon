use std::fs;
use std::path::{Path, PathBuf};

use nsq_runtime::{
    load_runtime_semantic_context_from_root, load_runtime_semantic_evidence_from_root,
    semantic_algorithm_lever_hint, semantic_bias_for_text, semantic_runtime_lane_hint,
    RuntimeSemanticContext,
};

fn temp_root(name: &str) -> PathBuf {
    let base = std::env::temp_dir().join(format!(
        "nsq_runtime_semantic_patch_{}_{}_{}",
        name,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(base.join("assets/braxon_core/tokenizer")).unwrap();
    base
}

fn write_tokenizer(root: &Path) {
    let tok = root.join("assets/braxon_core/tokenizer/braxon_unified_tokenizer.json");
    let raw = r#"{
  "build_metadata": {},
  "compass_seed": {
    "seed_path": "semantic_tokenizer_candidate_terms.json",
    "schema_path": "semantic_tokenizer_candidate_terms.schema.json",
    "tokens": [
      { "token": "runtime lane selection" },
      { "token": "proof obligation scoring" }
    ]
  },
  "semantic_feed": {
    "generated_at_epoch_sec": 0,
    "path": "braxon_unified_tokenizer.json",
    "entry_count": 4,
    "source_kind_counts": {
      "semantic_tokenizer_candidate_terms": 4
    },
    "active_state_counts": {
      "active": 4
    },
    "entries": [
      {
        "term": "semantic bug triage",
        "group": "code_resolution",
        "source_kind": "semantic_tokenizer_candidate_terms",
        "active_state": "active"
      },
      {
        "term": "native authority preservation",
        "group": "code_resolution",
        "source_kind": "semantic_tokenizer_candidate_terms",
        "active_state": "active"
      },
      {
        "term": "26d delta grid",
        "group": "26d_core",
        "source_kind": "semantic_tokenizer_candidate_terms",
        "active_state": "active"
      },
      {
        "term": "psychological axioms",
        "group": "emotion_psychology",
        "source_kind": "semantic_tokenizer_candidate_terms",
        "active_state": "active"
      }
    ]
  },
  "size": 3,
  "vocab": {
    "a": 0,
    "b": 1,
    "c": 2
  }
}"#;
    fs::write(tok, raw).unwrap();
}

#[test]
fn semantic_context_loader_reads_entries_and_tokens() {
    let root = temp_root("loader");
    write_tokenizer(&root);

    let ctx = load_runtime_semantic_context_from_root(&root);

    assert!(ctx.entry_terms.iter().any(|s| s == "semantic bug triage"));
    assert!(ctx.entry_terms.iter().any(|s| s == "26d delta grid"));
    assert!(ctx
        .compass_tokens
        .iter()
        .any(|s| s == "runtime lane selection"));
    assert_eq!(
        ctx.source_kind_counts
            .get("semantic_tokenizer_candidate_terms")
            .copied()
            .unwrap_or(0),
        8
    );
    assert!(ctx.group_terms.contains_key("26d_core"));
}

#[test]
fn semantic_evidence_reads_real_patch_sources_and_test_surface() {
    let root = temp_root("evidence");
    write_tokenizer(&root);

    let evidence = load_runtime_semantic_evidence_from_root(&root);

    assert!(evidence.consumers_ready);
    assert_eq!(evidence.feed_entries, 4);
    assert_eq!(evidence.compass_seed_tokens, 2);
    assert_eq!(evidence.patch_anchor_count, 4);
    assert!(evidence.tests_present);
}

#[test]
fn semantic_bias_changes_for_repair_and_proof_language() {
    let ctx = RuntimeSemanticContext {
        entry_terms: vec![
            "semantic bug triage".to_string(),
            "proof obligation scoring".to_string(),
            "native authority preservation".to_string(),
        ],
        compass_tokens: vec!["runtime lane selection".to_string()],
        ..RuntimeSemanticContext::default()
    };

    let bias = semantic_bias_for_text(
        &ctx,
        "run semantic bug triage and proof obligation scoring through the native runtime lane",
    );

    assert!(bias.repair_score > 0);
    assert!(bias.proof_score > 0);
    assert!(bias.authority_score > 0 || bias.route_score > 0);
}

#[test]
fn semantic_algorithm_hint_changes_for_repair_text() {
    let root = temp_root("lever");
    write_tokenizer(&root);
    std::env::set_var("BRAXON_ROOT", &root);

    let hint = semantic_algorithm_lever_hint(
        "repair phase triage with proof obligation scoring and semantic bug triage",
    );

    assert!(hint.is_some());
}

#[test]
fn runtime_lane_hint_changes_for_native_runtime_text() {
    let root = temp_root("lane");
    write_tokenizer(&root);
    std::env::set_var("BRAXON_ROOT", &root);

    let hint = semantic_runtime_lane_hint(
        "native authority preservation for runtime lane selection under semantic code routing",
    );

    assert_eq!(hint, Some("offline_model_native_runtime_lane"));
}
