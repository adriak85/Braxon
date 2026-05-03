use nsq_prime::prime_report;
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
fn prime_report_tracks_required_surfaces() {
    let root = temp_dir("nsq_prime_ok");

    fs::create_dir_all(root.join("crates/nsq-source/src")).unwrap();
    fs::create_dir_all(root.join("crates/nsq-compile/src")).unwrap();
    fs::create_dir_all(root.join("crates/nsq-pack/src")).unwrap();
    fs::create_dir_all(root.join("crates/nsq-inspect/src")).unwrap();

    fs::write(root.join("crates/nsq-source/src/lib.rs"), "").unwrap();
    fs::write(root.join("crates/nsq-compile/src/main.rs"), "").unwrap();
    fs::write(root.join("crates/nsq-pack/src/lib.rs"), "").unwrap();
    fs::write(root.join("crates/nsq-inspect/src/lib.rs"), "").unwrap();

    let report = prime_report(root.to_str().unwrap());
    assert!(report.ok);
}

#[test]
fn binary_returns_nonzero_when_surface_missing() {
    let dir = temp_dir("nsq_prime_bin");
    let out = Command::new(env!("CARGO_BIN_EXE_nsq-prime"))
        .current_dir(&dir)
        .output()
        .unwrap();

    assert!(!out.status.success());
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(text.contains("\"ok\": false"));
}
