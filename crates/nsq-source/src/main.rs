use nsq_source::{build_prime_representation, sanitize_source_ingress, spine_source};
use std::env;
use std::fs;

fn main() {
    let raw_args = env::args().skip(1).collect::<Vec<_>>();
    if let Some(first) = raw_args.first() {
        if matches!(first.as_str(), "--help" | "-h" | "help") {
            eprintln!(
                "usage:
  nsq-source spine <input>
  nsq-source sanitize <input>
  nsq-source prime <input>"
            );
            std::process::exit(0);
        }
    }
    let mut args = raw_args.into_iter();
    let mode = args.next().unwrap_or_else(|| "spine".to_string());
    let path = args.next().unwrap_or_else(|| {
        eprintln!("usage:");
        eprintln!("  nsq-source spine <input>");
        eprintln!("  nsq-source sanitize <input>");
        eprintln!("  nsq-source prime <input>");
        std::process::exit(2);
    });

    let text = fs::read_to_string(&path).unwrap_or_else(|e| {
        eprintln!("read error: {}", e);
        std::process::exit(2);
    });

    match mode.as_str() {
        "spine" => {
            let (source_form, spine_lines) = spine_source(&text);
            println!("# source_form={}", source_form.as_str());
            for line in spine_lines {
                println!("{}", line);
            }
        }
        "sanitize" => {
            let sanitized = sanitize_source_ingress(&text);
            println!("{}", serde_json::to_string_pretty(&sanitized).unwrap());
        }
        "prime" => {
            let rep = build_prime_representation(&text);
            println!("{}", serde_json::to_string_pretty(&rep).unwrap());
        }
        _ => {
            eprintln!("unknown mode: {}", mode);
            std::process::exit(2);
        }
    }
}
