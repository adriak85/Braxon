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
fn root_handover_emits_os_power_release_response_without_disconnect() {
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

    let lower = stdout.to_ascii_lowercase();
    assert!(!lower.contains("fail"));
    assert!(!lower.contains("mock"));
    assert!(!lower.contains("partial"));
    assert!(!lower.contains("block"));
}
