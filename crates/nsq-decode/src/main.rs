//! DERIVED DECODE ONLY
//! This crate is not canonical NSQ truth.
//! It decodes derived artifacts and readable projections from local NSQ-owned
//! packages or packed derivatives. It must not become semantic authority.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::env;
use std::fs;

#[derive(Debug, Deserialize)]
struct PackedNsq {
    symbols: Vec<String>,
    #[serde(default)]
    derived_symbol_carriers: Vec<usize>,
    #[serde(default)]
    derived_edges: Vec<(usize, usize)>,
    #[serde(default)]
    classes: BTreeMap<String, usize>,
    #[serde(default)]
    bytes: usize,
    #[serde(default)]
    lines: usize,
    #[serde(default)]
    tokens: usize,
    #[serde(default)]
    byte_sum: u64,
    #[serde(default)]
    hash64: u64,
}

#[derive(Debug, Serialize)]
struct DecodeReport {
    lane: &'static str,
    mode: &'static str,
    unique_symbols: usize,
    emitted_symbols: usize,
    transitions: usize,
    bytes: usize,
    lines: usize,
    tokens: usize,
    byte_sum: u64,
    hash64: String,
    classes: BTreeMap<String, usize>,
}

fn render_text(p: &PackedNsq) -> String {
    let mut out = String::new();
    for id in &p.derived_symbol_carriers {
        if let Some(sym) = p.symbols.get(*id) {
            if !out.is_empty() {
                out.push(' ');
            }
            out.push_str(sym);
        }
    }
    out
}

fn main() {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("decode") => {
            let path = args.next().expect("usage: nsq-decode decode <artifact>");
            let raw = fs::read(&path).expect("read artifact");
            let packed: PackedNsq = serde_json::from_slice(&raw).expect("json artifact");
            println!("{}", render_text(&packed));
        }
        Some("report") => {
            let path = args.next().expect("usage: nsq-decode report <artifact>");
            let raw = fs::read(&path).expect("read artifact");
            let packed: PackedNsq = serde_json::from_slice(&raw).expect("json artifact");
            let rep = DecodeReport {
                lane: "nsq-derived-decode",
                mode: "report",
                unique_symbols: packed.symbols.len(),
                emitted_symbols: packed.derived_symbol_carriers.len(),
                transitions: packed.derived_edges.len(),
                bytes: packed.bytes,
                lines: packed.lines,
                tokens: packed.tokens,
                byte_sum: packed.byte_sum,
                hash64: format!("{:016x}", packed.hash64),
                classes: packed.classes,
            };
            println!("{}", serde_json::to_string_pretty(&rep).unwrap());
        }
        _ => {
            eprintln!("usage:");
            eprintln!("  nsq-decode decode <artifact>");
            eprintln!("  nsq-decode report <artifact>");
            std::process::exit(2);
        }
    }
}
