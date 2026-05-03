use nsq_ir::{read_if_present, registry_paths, NSQ_IR_VERSION};
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
fn registry_paths_use_canonical_locations() {
    let root = PathBuf::from("/tmp/Braxon-root");
    let p = registry_paths(&root);

    assert_eq!(p.runtime_domain_registry, root.join("nsq/runtime_native/databases/runtime_domain_registry.db"));
    assert_eq!(p.graded_selector_registry, root.join("nsq/runtime_native/databases/graded_selector_registry.db"));
    assert_eq!(p.local_package_repo_registry, root.join("nsq/runtime_native/databases/local_package_repo_registry.db"));
    assert_eq!(p.package_db_multiport_registry, root.join("nsq/runtime_native/databases/package_db_multiport_registry.db"));
    assert_eq!(p.human_machine_doc_registry, root.join("nsq/runtime_native/databases/human_machine_doc_registry.db"));
    assert_eq!(p.tokenizer_bridge_registry, root.join("nsq/runtime_native/databases/tokenizer_bridge_registry.db"));
    assert_eq!(p.language_master_seed, root.join("config/nsq/language_master_seed.json"));
    assert_eq!(p.minimum_integration_registry, root.join("nsq/runtime_native/databases/minimum_integration_registry.db"));
    assert_eq!(p.translation_priority_registry, root.join("nsq/write_nsq/databases/translation_priority_registry.db"));
}

#[test]
fn read_if_present_reads_or_returns_none() {
    let dir = temp_dir("nsq_ir");
    let file = dir.join("present.txt");
    fs::write(&file, "hello world").unwrap();

    assert_eq!(read_if_present(&file).as_deref(), Some("hello world"));
    assert_eq!(read_if_present(&dir.join("missing.txt")), None);
}

#[test]
fn binary_emits_registry_lines() {
    let dir = temp_dir("nsq_ir_bin");
    let out = Command::new(env!("CARGO_BIN_EXE_nsq-ir"))
        .arg(&dir)
        .output()
        .unwrap();

    assert!(out.status.success());
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(text.contains("nsq-ir"));
    assert!(text.contains(&format!("version={}", NSQ_IR_VERSION)));
    assert!(text.contains("runtime_domain_registry="));
    assert!(text.contains("translation_priority_registry="));
}
