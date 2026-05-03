use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

#[derive(Serialize)]
struct Report {
    lane: String,
    mode: String,
    bytes: usize,
    lines: usize,
    tokens: usize,
    byte_sum: u64,
    hash64: String,
    unique_tokens: usize,
    transitions: usize,
    classes: BTreeMap<String, usize>,
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

fn byte_sum(bytes: &[u8]) -> u64 {
    bytes.iter().map(|&b| b as u64).sum()
}

fn line_count(s: &str) -> usize {
    if s.is_empty() {
        0
    } else {
        s.lines().count()
    }
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

fn nsq_report(mode: &str, path: Option<String>, bytes: &[u8], text: &str) -> Report {
    let toks: Vec<&str> = text.split_whitespace().collect();
    let mut uniq = BTreeSet::new();
    let mut classes: BTreeMap<String, usize> = BTreeMap::new();

    for tok in &toks {
        uniq.insert((*tok).to_string());
        *classes.entry(classify(tok).to_string()).or_insert(0) += 1;
    }

    Report {
        lane: "nsq-native".to_string(),
        mode: mode.to_string(),
        bytes: bytes.len(),
        lines: line_count(text),
        tokens: toks.len(),
        byte_sum: byte_sum(bytes),
        hash64: format!("{:016x}", hash64(bytes)),
        unique_tokens: uniq.len(),
        transitions: toks.len().saturating_sub(1),
        classes,
        path,
    }
}

fn emit(rep: &Report) {
    println!("{}", serde_json::to_string_pretty(rep).unwrap());
}

fn walk_files(root: &Path, out: &mut Vec<PathBuf>) -> io::Result<()> {
    let mut entries = fs::read_dir(root)?.collect::<Result<Vec<_>, io::Error>>()?;
    entries.sort_by_key(|e| e.path());

    for entry in entries {
        let path = entry.path();
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            walk_files(&path, out)?;
        } else if file_type.is_file() {
            out.push(path);
        }
    }

    Ok(())
}

fn should_include(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|s| s.to_str()) else {
        return false;
    };

    if name.starts_with('.') || name.ends_with('~') {
        return false;
    }

    if path.components().any(|c| {
        let s = c.as_os_str().to_string_lossy();
        s == ".git" || s == "target" || s == "node_modules" || s == ".cargo"
    }) {
        return false;
    }

    match path
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_ascii_lowercase())
    {
        Some(ext) => matches!(
            ext.as_str(),
            "c" | "h"
                | "rs"
                | "json"
                | "toml"
                | "md"
                | "txt"
                | "sql"
                | "yaml"
                | "yml"
                | "log"
                | "diff"
                | "code"
        ),
        None => true,
    }
}

fn read_dir_corpus(root: &Path) -> io::Result<(Vec<u8>, String)> {
    let mut files = Vec::new();
    walk_files(root, &mut files)?;

    let mut corpus_bytes = Vec::new();
    let mut corpus_text = String::new();

    for path in files {
        if !should_include(&path) {
            continue;
        }

        let rel = path.strip_prefix(root).unwrap_or(&path);
        let data = match fs::read(&path) {
            Ok(d) => d,
            Err(_) => continue,
        };
        let text = String::from_utf8_lossy(&data);

        corpus_text.push_str("\n=== FILE: ");
        corpus_text.push_str(&rel.to_string_lossy());
        corpus_text.push_str(" ===\n");
        corpus_text.push_str(&text);
        corpus_text.push('\n');

        corpus_bytes.extend_from_slice(rel.to_string_lossy().as_bytes());
        corpus_bytes.push(b'\n');
        corpus_bytes.extend_from_slice(&data);
        corpus_bytes.push(b'\n');
    }

    Ok((corpus_bytes, corpus_text))
}

fn main() {
    let raw_args = env::args().skip(1).collect::<Vec<_>>();
    if let Some(first) = raw_args.first() {
        if matches!(first.as_str(), "--help" | "-h" | "help") {
            eprintln!(
                "usage:
  nsq-native-bench [options]"
            );
            std::process::exit(0);
        }
    }
    let mut args = raw_args.into_iter();
    match args.next().as_deref() {
        Some("text") => {
            let text = args.collect::<Vec<_>>().join(" ");
            emit(&nsq_report("text", None, text.as_bytes(), &text));
        }
        Some("file") => {
            let path = args.next().unwrap_or_else(|| {
                eprintln!("usage: nsq-native-bench file <path>");
                std::process::exit(2);
            });

            let p = Path::new(&path);
            if p.is_dir() {
                eprintln!("read error: path is a directory; use 'dir <path>' or 'stdin'");
                std::process::exit(2);
            }

            let data = fs::read(&path).unwrap_or_else(|e| {
                eprintln!("read error: {}", e);
                std::process::exit(2);
            });
            let text = String::from_utf8_lossy(&data);
            emit(&nsq_report("file", Some(path), &data, &text));
        }
        Some("dir") => {
            let path = args.next().unwrap_or_else(|| {
                eprintln!("usage: nsq-native-bench dir <path>");
                std::process::exit(2);
            });

            let root = Path::new(&path);
            if !root.is_dir() {
                eprintln!("read error: not a directory: {}", path);
                std::process::exit(2);
            }

            let (data, text) = read_dir_corpus(root).unwrap_or_else(|e| {
                eprintln!("dir read error: {}", e);
                std::process::exit(2);
            });

            emit(&nsq_report("dir", Some(path), &data, &text));
        }
        Some("stdin") => {
            let mut buf = Vec::new();
            io::stdin().read_to_end(&mut buf).unwrap();
            let text = String::from_utf8_lossy(&buf);
            emit(&nsq_report("stdin", None, &buf, &text));
        }
        _ => {
            eprintln!("usage:");
            eprintln!("  nsq-native-bench text <text...>");
            eprintln!("  nsq-native-bench file <path>");
            eprintln!("  nsq-native-bench dir <path>");
            eprintln!("  nsq-native-bench stdin");
            std::process::exit(2);
        }
    }
}
