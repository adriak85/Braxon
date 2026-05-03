use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::time::Instant;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Phase {
    phase: String,
    ms: f64,
    ok: bool,
    note: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ProfileReport {
    input: String,
    spine_path: String,
    prime_path: String,
    phases: Vec<Phase>,
}

fn usage() -> ! {
    eprintln!("usage: nsq-profile <input.nsq> <workdir>");
    std::process::exit(2);
}

fn run_capture(bin: &str, args: &[String]) -> Result<(String, f64), String> {
    let t0 = Instant::now();
    let out = Command::new(bin)
        .args(args)
        .output()
        .map_err(|e| e.to_string())?;
    let ms = t0.elapsed().as_secs_f64() * 1000.0;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).to_string());
    }
    Ok((String::from_utf8_lossy(&out.stdout).to_string(), ms))
}

fn main() {
    let raw_args = env::args().skip(1).collect::<Vec<_>>();
    if let Some(first) = raw_args.first() {
        if matches!(first.as_str(), "--help" | "-h" | "help") {
            eprintln!(
                "usage:
  nsq-profile <input.nsq> <workdir>"
            );
            std::process::exit(0);
        }
    }
    let mut args = raw_args.into_iter();
    let input = PathBuf::from(args.next().unwrap_or_else(|| usage()));
    let workdir = PathBuf::from(args.next().unwrap_or_else(|| usage()));

    std::fs::create_dir_all(&workdir).unwrap();

    let bin_dir = env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(format!(
                "{}/.cargo/target-cache/Braxon",
                env::var("HOME").unwrap()
            ))
        })
        .join("release");

    let nsq_source = bin_dir.join("nsq-source");

    let spine = workdir.join("profile.spine.nsq");
    let prime = workdir.join("profile.prime.json");

    let mut phases = Vec::<Phase>::new();

    let (spine_out, ms) = run_capture(
        nsq_source.to_str().unwrap(),
        &["spine".into(), input.display().to_string()],
    )
    .unwrap_or_else(|e| {
        eprintln!("spine failed: {}", e);
        std::process::exit(2);
    });

    let spine_body = spine_out
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    std::fs::write(&spine, spine_body).unwrap();
    phases.push(Phase {
        phase: "spine".into(),
        ms,
        ok: true,
        note: "nsq-source spine".into(),
    });

    let (prime_out, ms) = run_capture(
        nsq_source.to_str().unwrap(),
        &["prime".into(), input.display().to_string()],
    )
    .unwrap_or_else(|e| {
        eprintln!("prime failed: {}", e);
        std::process::exit(2);
    });
    std::fs::write(&prime, prime_out).unwrap();
    phases.push(Phase {
        phase: "prime".into(),
        ms,
        ok: true,
        note: "nsq-source prime".into(),
    });

    let report = ProfileReport {
        input: input.display().to_string(),
        spine_path: spine.display().to_string(),
        prime_path: prime.display().to_string(),
        phases,
    };

    println!("{}", serde_json::to_string_pretty(&report).unwrap());
}
