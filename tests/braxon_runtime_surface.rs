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
fn root_runtime_python3_records_ingress_without_runtime_claim() {
    let (ok, stdout, stderr) =
        run_runtime_command(&["runtime", "python3", "score(task='alpha', retries=3)"]);
    assert!(
        ok,
        "runtime python3 command returned nonzero\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(stdout.contains("\"surface\": \"python3_ingress_boundary\""));
    assert!(stdout.contains("\"authority\": \"NSQ_COURT\""));
    assert!(stdout.contains("\"native_runtime_constructed\": false"));
    assert!(stdout.contains("\"court_roles_duplicated_into_runtime\": false"));
    assert!(stdout.contains("\"executed_as_second_runtime\": false"));
    assert!(stdout.contains("\"status\": \"ingress_recorded_without_runtime_claim\""));
    assert!(stdout.contains("\"canonical_semantics\": \"base8_switch_topology\""));
    assert!(!stdout.contains("\"court_route\""));
}

#[test]
fn root_handover_reports_blocked_release_without_disconnect() {
    let (ok, stdout, stderr) = run_runtime_command(&["handover", "os-power-release"]);
    assert!(
        ok,
        "handover command returned nonzero\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(stdout.contains("\"full_release_complete\": false"));
    assert!(stdout.contains(
        "\"response_to_os\": \"continue_without_power_disconnect_until_full_release_validation\""
    ));
    assert!(stdout.contains("\"power_disconnect_requested\": false"));
    assert!(stdout.contains("\"all_in_check_validated\": true"));
    assert!(stdout.contains("\"ten_surface_bus_validated\": false"));
    assert!(stdout.contains("\"voice_present\": true"));
    assert!(stdout.contains("\"video_present\": true"));
    assert!(stdout.contains("\"watermark_trigger_set_completely_validated\": false"));
    assert!(stdout.contains("\"semantic_address_gate_completely_validated\": true"));
    assert!(stdout.contains("\"seven_suit_cycles_validated\": true"));
    assert!(stdout.contains("\"release_requirements_not_yet_satisfied\""));
    assert!(stdout.contains("\"watermark_trigger_set_not_yet_satisfied\""));

    let lower = stdout.to_ascii_lowercase();
    assert!(!lower.contains("mock"));
    assert!(!lower.contains("power_disconnect_requested\": true"));
}
