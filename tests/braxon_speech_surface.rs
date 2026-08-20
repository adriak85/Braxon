use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn console_generates_derived_intelligent_action_without_audit_leakage() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_Braxon"))
        .arg("console")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn Braxon console");

    {
        let stdin = child.stdin.as_mut().expect("stdin");
        stdin
            .write_all(b"Hey Braxon, been a long journey, hasnt it?\nexit\n")
            .expect("write input");
    }

    let output = child.wait_with_output().expect("console output");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("I interpreted your request as"), "{stdout}");
    assert!(stdout.contains("executed the NSQ action"), "{stdout}");
    assert!(stdout.contains("address was released"), "{stdout}");
    assert!(
        !stdout.contains("braxon.bus.measurement_request.v4"),
        "{stdout}"
    );
    assert!(
        !stdout.contains("braxon.bus.user_presentation.v2"),
        "{stdout}"
    );
    assert!(!stdout.contains("generated_from_derived_state"), "{stdout}");
    assert!(
        !stdout.contains("native_representation_retained"),
        "{stdout}"
    );
    assert!(!stdout.contains("Measured operator request"), "{stdout}");
}
