use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use nsq_runtime::OfflineModelLane;

fn env_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

fn temp_root(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "nsq_runtime_offline_inference_semantic_report_{}_{}_{}",
        name,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&root).unwrap();
    root
}

fn tokenizer_dir(root: &Path) -> PathBuf {
    root.join("assets/braxon_core/tokenizer")
}

fn write_tokenizer(root: &Path) {
    fs::create_dir_all(tokenizer_dir(root)).unwrap();
    let path = tokenizer_dir(root).join("braxon_unified_tokenizer.json");
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
    fs::write(path, raw).unwrap();
}

fn with_BRAXON_root<T>(root: &Path, f: impl FnOnce() -> T) -> T {
    let _guard = env_lock().lock().unwrap();
    let old = std::env::var("BRAXON_ROOT").ok();
    std::env::set_var("BRAXON_ROOT", root);
    let out = f();
    if let Some(value) = old {
        std::env::set_var("BRAXON_ROOT", value);
    } else {
        std::env::remove_var("BRAXON_ROOT");
    }
    out
}

#[test]
fn offline_inference_report_sets_runtime_semantic_truth_when_tokenizer_present() {
    let root = temp_root("present");
    write_tokenizer(&root);

    let report = with_BRAXON_root(&root, || {
        OfflineModelLane::default()
            .execute_request("Braxon", "repair phase triage with proof obligation scoring")
            .expect("offline inference report should build")
    });

    assert_eq!(report.runtime_semantic_feed_entries, 4);
    assert_eq!(report.runtime_compass_seed_tokens, 2);
    assert_eq!(report.runtime_semantic_patch_anchor_count, 4);
    assert!(report.runtime_semantic_tests_present);
    assert!(report.runtime_semantic_consumers_ready);
}

#[test]
fn offline_inference_report_falls_back_cleanly_when_tokenizer_missing() {
    let root = temp_root("missing");
    fs::create_dir_all(tokenizer_dir(&root)).unwrap();

    let report = with_BRAXON_root(&root, || {
        OfflineModelLane::default()
            .execute_request("Braxon", "repair phase triage with proof obligation scoring")
            .expect("offline inference report should still build without tokenizer")
    });

    assert_eq!(report.runtime_semantic_feed_entries, 0);
    assert_eq!(report.runtime_compass_seed_tokens, 0);
    assert_eq!(report.runtime_semantic_patch_anchor_count, 4);
    assert!(report.runtime_semantic_tests_present);
    assert!(!report.runtime_semantic_consumers_ready);
}

#[test]
fn offline_inference_report_keeps_semantic_truth_fields_stable_for_supported_models() {
    let root = temp_root("stable");
    write_tokenizer(&root);

    let report_a = with_BRAXON_root(&root, || {
        OfflineModelLane::default()
            .execute_request("Braxon", "native authority preservation")
            .expect("Braxon should be supported")
    });

    let report_b = with_BRAXON_root(&root, || {
        OfflineModelLane::default()
            .execute_request("Braxon", "runtime lane selection")
            .expect("Braxon should be supported")
    });

    assert_eq!(
        report_a.runtime_semantic_patch_anchor_count,
        report_b.runtime_semantic_patch_anchor_count
    );
    assert_eq!(
        report_a.runtime_semantic_tests_present,
        report_b.runtime_semantic_tests_present
    );
    assert_eq!(report_a.runtime_semantic_feed_entries, 4);
    assert_eq!(report_b.runtime_semantic_feed_entries, 4);
}

#[test]
fn offline_inference_rejects_legacy_public_model_ids() {
    let root = temp_root("legacy_ids");
    write_tokenizer(&root);

    let err_a = with_BRAXON_root(&root, || {
        OfflineModelLane::default().execute_request(
            "BRAXON_core",
            "repair phase triage with proof obligation scoring",
        )
    })
    .expect_err("legacy BRAXON_core runtime id should be rejected");
    assert!(err_a.contains("unsupported offline model asset"));

    let err_b = with_BRAXON_root(&root, || {
        OfflineModelLane::default().execute_request(
            "qwen32b_code",
            "repair phase triage with proof obligation scoring",
        )
    })
    .expect_err("legacy qwen32b_code runtime id should be rejected");
    assert!(err_b.contains("unsupported offline model asset"));
}
