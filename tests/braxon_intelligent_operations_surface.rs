use std::process::Command;

fn braxon_bin() -> String {
    if let Ok(path) = std::env::var("CARGO_BIN_EXE_Braxon") {
        return path;
    }
    let test_binary = std::env::current_exe().expect("test executable path unavailable");
    test_binary
        .parent()
        .and_then(|deps| deps.parent())
        .map(|debug| debug.join("Braxon"))
        .expect("Cargo test executable has no target directory")
        .to_string_lossy()
        .into_owned()
}

fn run(args: &[&str]) -> (bool, String, String) {
    let output = Command::new(braxon_bin())
        .args(args)
        .output()
        .expect("could not run Braxon command");
    (
        output.status.success(),
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

#[test]
fn operator_bus_reaches_reflexor_selected_intelligent_action_and_releases_native_state() {
    let (ok, stdout, stderr) = run(&[
        "bus", "verify", "terminal", "launch", "path", "through", "operator", "bus",
    ]);
    assert!(ok, "stdout:\n{stdout}\nstderr:\n{stderr}");
    assert!(stdout.contains("I interpreted your request as"), "{stdout}");
    assert!(stdout.contains("executed the NSQ action"), "{stdout}");
    assert!(stdout.contains("\"reflex_capability\": \"feature:operator.intelligence\""));
    assert!(stdout.contains("\"lease_released\": true"));
    assert!(stdout.contains("\"native_fired_count\": 1"));
    assert!(!stdout.contains("request_recorded_without_runtime_claim"));
}

#[test]
fn parameter_citadel_and_proven_native_benchmarks_are_public_reflexor_operations() {
    let (parameter_ok, parameter_stdout, parameter_stderr) = run(&[
        "runtime",
        "parameter-citadel",
        "--signal",
        "8",
        "--context",
        "5",
    ]);
    assert!(
        parameter_ok,
        "stdout:\n{parameter_stdout}\nstderr:\n{parameter_stderr}"
    );
    assert!(parameter_stdout.contains("\"capability\": \"feature:parameter.citadel\""));
    assert!(parameter_stdout.contains("\"generation\": 1"));
    assert!(parameter_stdout.contains("\"persistent_state_reconstructible\": true"));

    let (equivalence_ok, equivalence_stdout, equivalence_stderr) =
        run(&["runtime", "native-equivalence"]);
    assert!(
        equivalence_ok,
        "stdout:\n{equivalence_stdout}\nstderr:\n{equivalence_stderr}"
    );
    assert!(equivalence_stdout.contains("\"capability\": \"feature:benchmark.native_equivalence\""));
    assert!(equivalence_stdout.contains("\"inference_replay_equivalent\": true"));
    assert!(equivalence_stdout.contains("\"training_path_equivalent\": true"));

    let (recovery_ok, recovery_stdout, recovery_stderr) = run(&["runtime", "native-recovery"]);
    assert!(
        recovery_ok,
        "stdout:\n{recovery_stdout}\nstderr:\n{recovery_stderr}"
    );
    assert!(recovery_stdout.contains("\"capability\": \"feature:benchmark.native_recovery\""));
    assert!(recovery_stdout.contains("\"replay_equivalent\": true"));
}

#[test]
fn tensor_inference_executes_a_configured_council_ten_seed_window_without_a_whole_model_claim() {
    let (ok, stdout, stderr) = run(&["runtime", "infer", "deepseek-v3-671b", "is truth"]);
    assert!(ok, "stdout:\n{stdout}\nstderr:\n{stderr}");
    assert!(stdout.contains("\"capability\": \"feature:model.tensor_inference\""));
    assert!(stdout.contains("\"model\": \"deepseek-v3-671b\""));
    assert!(stdout.contains("bounded canonical Citadel seed operation"));
    assert!(stdout.contains("\"whole_model_execution\": false"));
    assert!(stdout.contains("\"resident_runtime_constructed\": false"));
    assert!(!stdout.contains("safetensors"));
    assert!(!stdout.contains("request_recorded_without_runtime_claim"));
    assert!(!stdout.contains("hot_live_claim"));
}
