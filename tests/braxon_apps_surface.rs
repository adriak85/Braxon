use std::process::Command;

fn braxon_bin() -> String {
    std::env::var("CARGO_BIN_EXE_Braxon")
        .or_else(|_| std::env::var("CARGO_BIN_EXE_BRAXON"))
        .expect("Cargo did not provide a Braxon binary path")
}

fn run_app_command(args: &[&str]) -> (bool, String, String) {
    let output = Command::new(braxon_bin())
        .args(args)
        .output()
        .expect("could not run Braxon apps command");
    (
        output.status.success(),
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

#[test]
fn apps_list_includes_expected_workspace_bins() {
    let (ok, stdout, stderr) = run_app_command(&["apps", "list"]);
    assert!(
        ok,
        "apps list command returned nonzero\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(stdout.contains("app_total="));
    assert!(stdout.contains("Braxon :: package=Braxon-universal"));
    assert!(stdout.contains("nsq-cli :: package=nsq-cli"));
    assert!(stdout.contains("Braxon-cli :: package=Braxon-cli"));
    assert!(stdout.contains("Braxon-court :: package=Braxon-court"));
    assert!(!stdout.contains("nsq-court ::"));
    assert!(!stdout.contains("native_runtime_lane"));
}

#[test]
fn apps_show_reports_root_launchable_details() {
    let (ok, stdout, stderr) = run_app_command(&["apps", "show", "nsq-cli"]);
    assert!(
        ok,
        "apps show command returned nonzero\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(stdout.contains("app=nsq-cli"));
    assert!(stdout.contains("package=nsq-cli"));
    assert!(stdout.contains("bin_name=nsq-cli"));
    assert!(stdout.contains("root_launchable=true"));
}

#[test]
fn apps_verify_reports_validated_root_launch_coverage() {
    let (ok, stdout, stderr) = run_app_command(&["apps", "verify"]);
    assert!(
        ok,
        "apps verify command returned nonzero\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(stdout.contains("app_total="));
    assert!(stdout.contains("root_launchable_total="));
    assert!(stdout.contains("root_launch_coverage_validated=true"));
}
