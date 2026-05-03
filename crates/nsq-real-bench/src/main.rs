//! DERIVED TRANSPORT / READABLE PROJECTION ONLY
//! This crate may pack, query, and report through host-carrier shapes.
//! It must not become canonical NSQ truth.

//! DERIVED ARTIFACT ONLY
//! This crate is not canonical NSQ truth.
//! Integer lanes, packed transport, and benchmark/index layouts here are
//! disposable derivatives regenerated from preserved canonical NSQ artifacts.

use std::collections::{BTreeMap, HashMap};
use std::env;
use std::fs;
use std::io::{Read, Write};

#[derive(serde::Serialize, serde::Deserialize)]
struct PackedNsq {
    symbols: Vec<String>,
    derived_symbol_carriers: Vec<usize>,
    derived_edges: Vec<(usize, usize)>,
    classes: BTreeMap<String, usize>,
    bytes: usize,
    lines: usize,
    tokens: usize,
    byte_sum: u64,
    hash64: u64,
}

#[derive(serde::Serialize)]
struct QueryReport {
    lane: String,
    mode: String,
    bytes: usize,
    lines: usize,
    tokens: usize,
    byte_sum: u64,
    hash64: String,
    unique_symbols: usize,
    transitions: usize,
    classes: BTreeMap<String, usize>,
    artifact_bytes: Option<u64>,
    binary_bytes: Option<u64>,
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

fn tokenize(text: &str) -> Vec<&str> {
    text.split_whitespace().collect()
}

fn build_packed(derived_transport_bytes: &[u8], readable_projection_text: &str) -> PackedNsq {
    let toks = tokenize(readable_projection_text);
    let mut derived_symbol_transport_id: HashMap<String, usize> = HashMap::new();
    let mut symbols = Vec::new();
    let mut derived_symbol_carriers = Vec::with_capacity(toks.len());
    let mut derived_edges = Vec::with_capacity(toks.len().saturating_sub(1));
    let mut classes: BTreeMap<String, usize> = BTreeMap::new();

    for tok in &toks {
        let id = if let Some(id) = derived_symbol_transport_id.get(*tok) {
            *id
        } else {
            let id = symbols.len();
            symbols.push((*tok).to_string());
            derived_symbol_transport_id.insert((*tok).to_string(), id);
            id
        };
        derived_symbol_carriers.push(id);
        *classes.entry(classify(tok).to_string()).or_insert(0) += 1;
    }

    for pair in derived_symbol_carriers.windows(2) {
        derived_edges.push((pair[0], pair[1]));
    }

    PackedNsq {
        symbols,
        derived_symbol_carriers,
        derived_edges,
        classes,
        bytes: derived_transport_bytes.len(),
        lines: line_count(readable_projection_text),
        tokens: toks.len(),
        byte_sum: byte_sum(derived_transport_bytes),
        hash64: hash64(derived_transport_bytes),
    }
}

fn artifact_size(path: &str) -> Option<u64> {
    fs::metadata(path).ok().map(|m| m.len())
}

fn current_binary_size() -> Option<u64> {
    env::current_exe()
        .ok()
        .and_then(|p| fs::metadata(p).ok().map(|m| m.len()))
}

fn emit_report(mode: &str, packed: &PackedNsq, path: Option<String>, artifact_path: Option<&str>) {
    let rep = QueryReport {
        lane: "nsq-native-packed".to_string(),
        mode: mode.to_string(),
        bytes: packed.bytes,
        lines: packed.lines,
        tokens: packed.tokens,
        byte_sum: packed.byte_sum,
        hash64: format!("{:016x}", packed.hash64),
        unique_symbols: packed.symbols.len(),
        transitions: packed.derived_edges.len(),
        classes: packed.classes.clone(),
        artifact_bytes: artifact_path.and_then(artifact_size),
        binary_bytes: current_binary_size(),
        path,
    };
    println!("{}", serde_json::to_string_pretty(&rep).unwrap());
}

fn main() {
    let raw_args = env::args().skip(1).collect::<Vec<_>>();
    if let Some(first) = raw_args.first() {
        if matches!(first.as_str(), "--help" | "-h" | "help") {
            eprintln!(
                "usage:
  nsq-real-bench [options]"
            );
            std::process::exit(0);
        }
    }
    let mut args = raw_args.into_iter();
    match args.next().as_deref() {
        Some("build-text") => {
            let out = args.next().expect("usage: build-text <artifact> <text...>");
            let text = args.collect::<Vec<_>>().join(" ");
            let packed = build_packed(text.as_bytes(), &text); // derived transport only; not canonical truth
            let mut f = fs::File::create(&out).unwrap();
            serde_json::to_writer(&mut f, &packed).unwrap();
            f.flush().unwrap();
            emit_report("build-text", &packed, None, Some(&out));
        }
        Some("build-file") => {
            let out = args.next().expect("usage: build-file <artifact> <path>");
            let path = args.next().expect("usage: build-file <artifact> <path>");
            let data = fs::read(&path).unwrap();
            let text = String::from_utf8_lossy(&data);
            let packed = build_packed(&data, &text);
            let mut f = fs::File::create(&out).unwrap();
            serde_json::to_writer(&mut f, &packed).unwrap();
            f.flush().unwrap();
            emit_report("build-file", &packed, Some(path), Some(&out));
        }
        Some("query") => {
            let artifact = args.next().expect("usage: query <artifact>");
            let mut f = fs::File::open(&artifact).unwrap();
            let mut buf = Vec::new();
            f.read_to_end(&mut buf).unwrap();
            let packed: PackedNsq = serde_json::from_slice(&buf).unwrap();
            emit_report("query", &packed, None, Some(&artifact));
        }
        _ => {
            eprintln!("usage:");
            eprintln!("  nsq-real-bench build-text <artifact> <text...>");
            eprintln!("  nsq-real-bench build-file <artifact> <path>");
            eprintln!("  nsq-real-bench query <artifact>");
            std::process::exit(2);
        }
    }
}
