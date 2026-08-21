use std::process::Command;

fn braxon_bin() -> String {
    if let Ok(path) = std::env::var("CARGO_BIN_EXE_Braxon") {
        return path;
    }
    if let Ok(path) = std::env::var("CARGO_BIN_EXE_BRAXON") {
        return path;
    }
    let test_binary = std::env::current_exe().expect("test executable path unavailable");
    let fallback = test_binary
        .parent()
        .and_then(|deps| deps.parent())
        .map(|debug| debug.join("Braxon"))
        .expect("Cargo test executable has no target directory");
    assert!(
        fallback.is_file(),
        "Braxon binary missing at {}",
        fallback.display()
    );
    fallback.to_string_lossy().into_owned()
}

fn run_runtime_command(args: &[&str]) -> (bool, String, String) {
    let output = Command::new(braxon_bin())
        .args(args)
        .output()
        .expect("could not run Braxon runtime command");
    (
        output.status.success(),
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

#[test]
fn root_runtime_python3_routes_language_input_to_an_nsq_intelligent_action() {
    let (ok, stdout, stderr) =
        run_runtime_command(&["runtime", "python3", "score(task='alpha', retries=3)"]);
    assert!(
        ok,
        "runtime python3 command returned nonzero\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(stdout.contains("\"action\": \"python3_ingress_to_nsq_intelligent_operation\""));
    assert!(stdout.contains("\"language_capability\": \"language:python3\""));
    assert!(stdout.contains("\"execution_capability\": \"feature:operator.intelligence\""));
    assert!(stdout.contains("\"lease_released\": true"));
    assert!(stdout.contains("\"executed_as_second_runtime\": false"));
    assert!(stdout.contains("executed the NSQ action"));
    assert!(!stdout.contains("ingress_recorded_without_runtime_claim"));
    assert!(!stdout.contains("python3_ingress_boundary"));
}

#[test]
fn root_handover_materializes_and_reports_completed_release_without_disconnect() {
    let (ok, stdout, stderr) = run_runtime_command(&["handover", "os-power-release"]);
    assert!(
        ok,
        "handover command returned nonzero\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(stdout.contains("\"full_release_complete\": true"));
    assert!(stdout.contains("\"response_to_os\": \"release_without_power_disconnect\""));
    assert!(stdout.contains("\"power_disconnect_requested\": false"));
    assert!(stdout.contains("\"all_in_check_validated\": true"));
    assert!(stdout.contains("\"ten_surface_bus_validated\": true"));
    assert!(stdout.contains("\"voice_present\": true"));
    assert!(stdout.contains("\"video_present\": true"));
    assert!(stdout.contains("\"watermark_trigger_set_completely_validated\": true"));
    assert!(stdout.contains("\"semantic_address_gate_completely_validated\": true"));
    assert!(stdout.contains("\"seven_suit_cycles_validated\": true"));
    assert!(stdout.contains("\"release_requirements_not_yet_satisfied\": []"));
    assert!(stdout.contains("\"watermark_trigger_set_not_yet_satisfied\": []"));

    let lower = stdout.to_ascii_lowercase();
    assert!(!lower.contains("mock"));
    assert!(!lower.contains("power_disconnect_requested\": true"));
}

#[test]
fn root_language_python3_executes_the_bounded_declared_parse_without_native_promotion_claim() {
    let (ok, stdout, stderr) = run_runtime_command(&[
        "language",
        "python3",
        "print(\"bounded NSQ language parse\")",
    ]);
    assert!(
        ok,
        "language python3 command returned nonzero\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(stdout.contains("\"intercept_route\""));
    assert!(stdout.contains("\"language_route\""));
    assert!(stdout.contains("\"id\": \"language:python3\""));
    assert!(stdout.contains("\"semantic_parse_ready\": true"));
    assert!(stdout.contains("\"no_resident_runtime\": true"));
    assert!(
        stdout.contains("\"full_closure_verification_front_door\": \"Braxon closure language\"")
    );
    assert!(stdout.contains("\"target_execution_environment_matches\": false"));
    assert!(stdout.contains("\"native_materialization_ready\": false"));
}

#[test]
fn root_role_assistant_resolves_court_authority_and_executes_a_released_nsq_transaction() {
    let (ok, stdout, stderr) = run_runtime_command(&[
        "role",
        "execute",
        "assistant",
        "verify court-bound assistant role execution",
    ]);
    assert!(
        ok,
        "role assistant command returned nonzero\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(stdout.contains("\"capability\": \"feature:role.operation\""));
    assert!(stdout.contains("\"mode\": \"assistant\""));
    assert!(stdout.contains("\"id\": \"oracle\""));
    assert!(stdout.contains("\"completed\": true"));
    assert!(stdout.contains("\"lease_released\": true"));
    assert!(stdout.contains("\"native_fired_count\": 1"));
    assert!(stdout.contains("\"resident_runtime\": false"));
}
