use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn temp_dir(name: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!(
        "BRAXON_phase_one_{}_{}_{}",
        name,
        std::process::id(),
        stamp
    ));
    fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn status_surface_is_live() {
    let out = Command::new(env!("CARGO_BIN_EXE_nsq-cli"))
        .arg("status")
        .output()
        .unwrap();

    assert!(out.status.success());
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(text.contains("NSQ status: ready"));
    assert!(text.contains("workspace: Braxon"));
}

#[test]
fn ingest_fails_closed_for_missing_path() {
    let out = Command::new(env!("CARGO_BIN_EXE_nsq-cli"))
        .args(["ingest", "/definitely/not/here"])
        .output()
        .unwrap();

    assert!(!out.status.success());
    let text = String::from_utf8_lossy(&out.stderr);
    assert!(text.contains("ingest error: path does not exist"));
}

#[test]
fn doctor_surface_reports_environment() {
    let dir = temp_dir("nsq_cli_doctor");
    let out = Command::new(env!("CARGO_BIN_EXE_nsq-cli"))
        .arg("doctor")
        .current_dir(&dir)
        .output()
        .unwrap();

    assert!(out.status.success());
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(text.contains("NSQ doctor"));
    assert!(text.contains("check:cargo_toml=false"));
}

#[test]
fn eval_surface_dispatches_to_native_intent_state() {
    let out = Command::new(env!("CARGO_BIN_EXE_nsq-cli"))
        .args(["eval", "agency relation truth"])
        .output()
        .unwrap();

    assert!(out.status.success());
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(text.contains("capability: guile.rebuild_intent"));
    assert!(text.contains("result: language-intent-rebuilt"));
    assert!(!text.contains("stub-ok"));
}

#[test]
fn select_surface_discovers_native_capability() {
    let out = Command::new(env!("CARGO_BIN_EXE_nsq-cli"))
        .args(["select", "tree"])
        .output()
        .unwrap();

    assert!(out.status.success());
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(text.contains("capability: tree_sitter.parse"));
    assert!(text.contains("native_entry: nsq-core::RawNsqEngine::parse"));
}

#[test]
fn select_surface_fails_closed_for_unknown_capability() {
    let out = Command::new(env!("CARGO_BIN_EXE_nsq-cli"))
        .args(["select", "not-a-native-capability"])
        .output()
        .unwrap();

    assert!(!out.status.success());
    let text = String::from_utf8_lossy(&out.stderr);
    assert!(text.contains("selection rejected: no native capability matches"));
}

#[test]
fn fetch_surface_reads_repository_file_metadata() {
    let workspace_cargo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../Cargo.toml");
    let out = Command::new(env!("CARGO_BIN_EXE_nsq-cli"))
        .args(["fetch", workspace_cargo.to_str().unwrap()])
        .output()
        .unwrap();

    assert!(out.status.success());
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(text.contains("NSQ fetch"));
    assert!(text.contains("kind: file"));
    assert!(text.contains("bytes: "));
    assert!(!text.contains("stub-ok"));
}

#[test]
fn parse_surface_fails_closed_for_unbalanced_input() {
    let out = Command::new(env!("CARGO_BIN_EXE_nsq-cli"))
        .args(["parse", "("])
        .output()
        .unwrap();

    assert!(!out.status.success());
    let text = String::from_utf8_lossy(&out.stderr);
    assert!(text.contains("parse error: unclosed delimiter"));
}
