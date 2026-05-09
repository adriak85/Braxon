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
