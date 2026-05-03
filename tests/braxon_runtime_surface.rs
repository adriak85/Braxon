use std::process::Command;

fn braxon_bin() -> String {
    std::env::var("CARGO_BIN_EXE_Braxon")
        .or_else(|_| std::env::var("CARGO_BIN_EXE_BRAXON"))
        .expect("Cargo did not provide a Braxon binary path")
}

fn run_runtime_command(args: &[&str]) -> (bool, String, String) {
    let output = Command::new(braxon_bin())
        .args(args)
        .output()
        .expect("failed to run Braxon runtime command");
    (
        output.status.success(),
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

#[test]
fn root_runtime_python3_executes_native_slice() {
    let (ok, stdout, stderr) =
        run_runtime_command(&["runtime", "python3", "score(task='alpha', retries=3)"]);
    assert!(
        ok,
        "runtime python3 failed\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(stdout.contains("\"lane\": \"python3_native_runtime_lane\""));
    assert!(stdout.contains("\"canonical_semantics\": \"base8_switch_topology\""));
    assert!(stdout.contains("\"policer\""));
    assert!(stdout.contains("\"inspector\""));
}
