use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn console_generates_non_canned_state_reply() {
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

    assert!(stdout.contains("braxon.bus.speak_request.v3"), "{stdout}");
    assert!(
        stdout.contains("braxon.bus.synthesized_reply.v1"),
        "{stdout}"
    );
    assert!(
        stdout.contains("\"reply_generated_from_state\": true"),
        "{stdout}"
    );
    assert!(stdout.contains("\"canned_reply\": false"), "{stdout}");
    assert!(stdout.contains("continuity"), "{stdout}");
    assert!(stdout.contains("It has been a long build"), "{stdout}");
}
