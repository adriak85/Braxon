use nsq_compose::compose_repo_surface;
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
fn compose_repo_surface_writes_expected_lines() {
    let dir = temp_dir("nsq_compose");
    let out = dir.join("nested/output.nsq");
    let lines = vec!["alpha beta".to_string(), "gamma delta".to_string()];

    compose_repo_surface(&lines, out.to_str().unwrap()).unwrap();

    let written = fs::read_to_string(&out).unwrap();
    assert_eq!(written, "alpha beta\ngamma delta\n");
}

#[test]
fn binary_composes_sample_surface() {
    let dir = temp_dir("nsq_compose_bin");
    let out_path = dir.join("sample.nsq");

    let out = Command::new(env!("CARGO_BIN_EXE_nsq-compose"))
        .arg(&out_path)
        .output()
        .unwrap();

    assert!(out.status.success());
    let text = fs::read_to_string(&out_path).unwrap();
    assert!(text.contains("repo.core"));
    assert!(text.contains("nsq.source"));
    assert!(text.contains("nsq.compile"));
    assert!(text.contains("nsq.inspect"));
}
