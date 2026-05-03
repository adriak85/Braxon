use std::{
    fs,
    path::{Path, PathBuf},
};

fn workspace_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .and_then(Path::parent)
        .expect("workspace root from crates/nsq-runtime")
        .to_path_buf()
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()))
}

#[test]
fn live_scripts_do_not_point_back_to_legacy_transport_root() {
    let legacy_transport = format!("{}_{}", "qwen", "transport").replace('_', "");
    let root = workspace_root();
    let files = [
        "scripts/recode_braxon_source_to_nsqb.sh",
        "scripts/audit_braxon_qwen_ingress.sh",
        "scripts/install_braxon_weights.sh",
        "scripts/seed_braxon_nsq_envelope.sh",
        "scripts/braxon_weight_ingest_daemon.sh",
        "scripts/braxon_truth_surface.sh",
        "state/braxon/braxon_nsq_pipeline.status",
        "state/braxon/braxon_whole_core_finalize.status",
    ];

    for rel in files {
        let path = root.join(rel);
        if path.exists() {
            let text = read(&path);
            assert!(
                !text.contains(&legacy_transport),
                "live file still contains legacy transport root: {}",
                path.display()
            );
        }
    }
}

#[test]
fn live_proof_and_bench_scripts_do_not_advertise_lowering_mode() {
    let root = workspace_root();
    let files = [
        "scripts/run_family_proof.sh",
        "scripts/run_nsq_hard_bench.sh",
        "scripts/run_nsq_hardened_suite.sh",
    ];

    let banned = [
        "hook-family-proof",
        "family_lowering_enabled",
        ".lowered.txt",
    ];

    for rel in files {
        let path = root.join(rel);
        if path.exists() {
            let text = read(&path);
            for bad in banned {
                assert!(
                    !text.contains(bad),
                    "live file still contains banned lowering marker {:?}: {}",
                    bad,
                    path.display()
                );
            }
        }
    }
}
