use std::io::Write;
use std::process::{Command, Stdio};

fn braxon_bin() -> String {
    std::env::var("CARGO_BIN_EXE_Braxon")
        .or_else(|_| std::env::var("CARGO_BIN_EXE_BRAXON"))
        .expect("Cargo did not provide a Braxon binary path")
}

fn run_transcript(lines: &[&str]) -> (bool, String, String) {
    let mut child = Command::new(braxon_bin())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn Braxon binary");

    {
        let stdin = child.stdin.as_mut().expect("stdin unavailable");
        for line in lines {
            writeln!(stdin, "{line}").expect("failed to write transcript line");
        }
    }

    let out = child.wait_with_output().expect("failed to wait for Braxon");
    (
        out.status.success(),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

#[test]
fn natural_chat_must_not_emit_internal_placeholder_tokens() {
    let (ok, stdout, stderr) = run_transcript(&["hello Braxon", "/exit"]);
    assert!(
        ok,
        "Braxon process failed\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );

    let combined = format!("{stdout}\n{stderr}");

    let banned = [
        "assistant=offline_request_bound(",
        "representation=stamp_bound_manifest",
        "prompt_chars=",
        "turn_count=",
        "memory_window_turns=",
        "conversation_digest=",
        "capability_profile=",
        "session_budget_state=",
    ];

    for bad in banned {
        assert!(
            !combined.contains(bad),
            "natural conversation leaked internal placeholder token `{bad}`\nstdout:\n{stdout}\nstderr:\n{stderr}"
        );
    }
}

#[test]
fn natural_chat_must_not_render_machine_accounting_as_the_reply() {
    let (ok, stdout, stderr) = run_transcript(&["hey", "/exit"]);
    assert!(
        ok,
        "Braxon process failed\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );

    let machine_prefixes = [
        "assistant=",
        "identity=",
        "launch_state=",
        "entrance=",
        "client_surface=",
        "client_boot_state=",
        "launch_path=",
        "model=",
        "session=",
        "session_reused=",
        "session_surface=",
        "session_mode=",
        "agentic_capability=",
        "client_features_active=",
        "workspace_green=",
        "nsq_core_ready=",
        "core_runtime_ready=",
        "source_ingest_status=",
        "nsq_envelope_status=",
        "nsq_recode_status=",
        "whole_core_runtime_status=",
        "source_blake3_status=",
        "ingest_daemon_mode=",
        "console_mode=",
        "console_commands=",
        "console_chat=",
        "turn_count=",
        "memory_window_turns=",
        "conversation_digest=",
        "session_budget_state=",
    ];

    let suspicious: Vec<&str> = stdout
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter(|line| !line.starts_with("Braxon>"))
        .filter(|line| machine_prefixes.iter().any(|p| line.starts_with(p)))
        .collect();

    assert!(
        suspicious.is_empty(),
        "natural conversation still rendered machine-accounting lines as visible output:\n{:#?}\n\nfull stdout:\n{}\n\nstderr:\n{}",
        suspicious,
        stdout,
        stderr
    );
}
