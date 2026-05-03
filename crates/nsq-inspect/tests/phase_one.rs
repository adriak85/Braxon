use nsq_inspect::inspect_file;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn temp_dir(name: &str) -> PathBuf {
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let dir = std::env::temp_dir().join(format!("BRAXON_phase_one_{}_{}_{}", name, std::process::id(), stamp));
    fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn inspect_file_detects_native_marker() {
    let dir = temp_dir("nsq_inspect");
    let file = dir.join("artifact.nsqb");
    fs::write(&file, b"NSQPACK01\nabc").unwrap();

    let report = inspect_file(file.to_str().unwrap()).unwrap();
    assert!(report.marker_ok);
    assert_eq!(report.native_marker, "NSQPACK01");
    assert_eq!(report.artifact_carrier_units, 13);
    assert_eq!(report.payload_carrier_units, 3);
}

#[test]
fn binary_reports_json() {
    let dir = temp_dir("nsq_inspect_bin");
    let file = dir.join("artifact.nsqb");
    fs::write(&file, b"NSQPACK01\nabc").unwrap();

    let out = Command::new(env!("CARGO_BIN_EXE_nsq-inspect"))
        .arg(&file)
        .output()
        .unwrap();

    assert!(out.status.success());
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(text.contains("\"marker_ok\": true"));
    assert!(text.contains("\"native_marker\": \"NSQPACK01\""));
}
