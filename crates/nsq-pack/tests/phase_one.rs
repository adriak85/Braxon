use nsq_pack::pack_files;
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
fn pack_files_writes_marker_and_manifest() {
    let dir = temp_dir("nsq_pack");
    let in1 = dir.join("a.txt");
    let in2 = dir.join("b.txt");
    let out = dir.join("out/artifact.nsqb");

    fs::write(&in1, b"abc").unwrap();
    fs::write(&in2, b"de").unwrap();

    let inputs = vec![in1.to_string_lossy().to_string(), in2.to_string_lossy().to_string()];
    let manifest = pack_files(&inputs, out.to_str().unwrap()).unwrap();

    assert_eq!(manifest.version, 1);
    assert_eq!(manifest.artifacts.len(), 2);
    assert_eq!(manifest.native_marker, "NSQPACK01");
    assert_eq!(manifest.source_carrier_units, 5);

    let bytes = fs::read(&out).unwrap();
    assert!(bytes.starts_with(b"NSQPACK01\n"));
    assert_eq!(manifest.artifact_carrier_units as usize, bytes.len());
}

#[test]
fn binary_packs_and_reports() {
    let dir = temp_dir("nsq_pack_bin");
    let in1 = dir.join("a.txt");
    let out_path = dir.join("artifact.nsqb");
    fs::write(&in1, b"abc").unwrap();

    let out = Command::new(env!("CARGO_BIN_EXE_nsq-pack"))
        .arg(&out_path)
        .arg(&in1)
        .output()
        .unwrap();

    assert!(out.status.success());
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(text.contains("\"native_marker\": \"NSQPACK01\""));
    assert!(out_path.exists());
}
