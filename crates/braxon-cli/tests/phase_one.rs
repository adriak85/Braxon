use std::process::Command;

fn braxon_cli_exe() -> std::path::PathBuf {
    for key in [
        "CARGO_BIN_EXE_Braxon-cli",
        "CARGO_BIN_EXE_BRAXON-cli",
        "CARGO_BIN_EXE_braxon-cli",
    ] {
        if let Ok(path) = std::env::var(key) {
            return std::path::PathBuf::from(path);
        }
    }

    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .expect("Braxon-cli test must live under workspace/crates/braxon-cli");

    for candidate in [
        "target/release/Braxon-cli",
        "target/debug/Braxon-cli",
        "target/release/braxon-cli",
        "target/debug/braxon-cli",
    ] {
        let path = workspace_root.join(candidate);
        if path.exists() {
            return path;
        }
    }

    panic!(
        "could not resolve Braxon-cli binary; checked Cargo runtime env vars and workspace target paths"
    );
}

#[test]
fn status_surface_returns_braxon_identity() {
    let out = Command::new(braxon_cli_exe())
        .arg("status")
        .output()
        .unwrap();

    assert!(out.status.success());

    let text = String::from_utf8_lossy(&out.stdout);
    let line = text.trim();

    assert_eq!(
        line.lines().count(),
        1,
        "expected exactly one identity line, got: {line:?}"
    );
    assert!(
        line.starts_with("Braxon "),
        "expected canonical Braxon identity prefix, got: {line:?}"
    );

    let parts: Vec<&str> = line.split_whitespace().collect();
    assert!(
        parts.len() >= 2,
        "expected '<name> <version>' shape, got: {line:?}"
    );
    assert!(
        parts[1].chars().any(|c| c.is_ascii_digit()),
        "expected a version-like second token, got: {line:?}"
    );
}
