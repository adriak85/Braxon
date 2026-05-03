use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::io::{self, Read};

#[derive(Serialize)]
struct Report {
    mode: String,
    bytes: usize,
    lines: usize,
    tokens: usize,
    byte_sum: u64,
    hash64: String,
    unique_tokens: Option<usize>,
    concept_edges: Option<usize>,
    token_classes: Option<BTreeMap<String, usize>>,
    path: Option<String>,
}

fn hash64(bytes: &[u8]) -> u64 {
    let mut h: u64 = 1469598103934665603;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(1099511628211);
    }
    h
}

fn line_count(s: &str) -> usize {
    if s.is_empty() {
        0
    } else {
        s.lines().count()
    }
}

fn token_vec(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}

fn byte_sum(bytes: &[u8]) -> u64 {
    bytes.iter().map(|&b| b as u64).sum()
}

fn classify(tok: &str) -> &'static str {
    if tok.chars().all(|c| c.is_ascii_digit()) {
        "int"
    } else if tok.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        "symbol"
    } else {
        "mixed"
    }
}

fn report_basic(mode: &str, path: Option<String>, bytes: &[u8], text: &str) -> Report {
    let toks = token_vec(text);
    Report {
        mode: mode.to_string(),
        bytes: bytes.len(),
        lines: line_count(text),
        tokens: toks.len(),
        byte_sum: byte_sum(bytes),
        hash64: format!("{:016x}", hash64(bytes)),
        unique_tokens: None,
        concept_edges: None,
        token_classes: None,
        path,
    }
}

fn report_prime(mode: &str, path: Option<String>, bytes: &[u8], text: &str) -> Report {
    let toks = token_vec(text);
    let mut uniq = BTreeSet::new();
    let mut classes: BTreeMap<String, usize> = BTreeMap::new();
    let mut edges = 0usize;

    for (i, tok) in toks.iter().enumerate() {
        uniq.insert((*tok).to_string());
        *classes.entry(classify(tok).to_string()).or_insert(0) += 1;
        if i > 0 {
            edges += 1;
        }
    }

    Report {
        mode: mode.to_string(),
        bytes: bytes.len(),
        lines: line_count(text),
        tokens: toks.len(),
        byte_sum: byte_sum(bytes),
        hash64: format!("{:016x}", hash64(bytes)),
        unique_tokens: Some(uniq.len()),
        concept_edges: Some(edges),
        token_classes: Some(classes),
        path,
    }
}

fn print_json(rep: &Report) {
    println!("{}", serde_json::to_string_pretty(rep).unwrap());
}

fn main() {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("parity-text") => {
            let text = args.collect::<Vec<_>>().join(" ");
            let rep = report_basic("parity-text", None, text.as_bytes(), &text);
            print_json(&rep);
        }
        Some("parity-file") => {
            let path = args.next().unwrap_or_else(|| {
                eprintln!("usage: nsq-bench parity-file <path>");
                std::process::exit(2);
            });
            let data = fs::read(&path).unwrap_or_else(|e| {
                eprintln!("read error: {}", e);
                std::process::exit(2);
            });
            let text = String::from_utf8_lossy(&data);
            let rep = report_basic("parity-file", Some(path), &data, &text);
            print_json(&rep);
        }
        Some("prime-text") => {
            let text = args.collect::<Vec<_>>().join(" ");
            let rep = report_prime("prime-text", None, text.as_bytes(), &text);
            print_json(&rep);
        }
        Some("prime-file") => {
            let path = args.next().unwrap_or_else(|| {
                eprintln!("usage: nsq-bench prime-file <path>");
                std::process::exit(2);
            });
            let data = fs::read(&path).unwrap_or_else(|e| {
                eprintln!("read error: {}", e);
                std::process::exit(2);
            });
            let text = String::from_utf8_lossy(&data);
            let rep = report_prime("prime-file", Some(path), &data, &text);
            print_json(&rep);
        }
        Some("stdin") => {
            let mut buf = Vec::new();
            io::stdin().read_to_end(&mut buf).unwrap();
            let text = String::from_utf8_lossy(&buf);
            let rep = report_prime("stdin", None, &buf, &text);
            print_json(&rep);
        }
        _ => {
            eprintln!("usage:");
            eprintln!("  nsq-bench parity-text <text...>");
            eprintln!("  nsq-bench parity-file <path>");
            eprintln!("  nsq-bench prime-text <text...>");
            eprintln!("  nsq-bench prime-file <path>");
            eprintln!("  nsq-bench stdin");
            std::process::exit(2);
        }
    }
}
