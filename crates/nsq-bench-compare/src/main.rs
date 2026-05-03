use nsq_index::build_index_from_text;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::time::Instant;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct NeutralTaskSpec {
    name: String,
    corpus_path: String,
    queries: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CompareRow {
    name: String,
    nsq_index_ms: f64,
    baseline_index_ms: f64,
    nsq_query_ms: f64,
    baseline_query_ms: f64,
    nsq_symbols: usize,
    baseline_symbols: usize,
    nsq_edges: usize,
    baseline_edges: usize,
}

fn usage() -> ! {
    eprintln!("usage: nsq-bench-compare <task_spec.json>");
    std::process::exit(2);
}

fn baseline_parse(text: &str) -> (usize, usize, BTreeMap<String, Vec<String>>) {
    let mut syms = BTreeSet::<String>::new();
    let mut edges = BTreeMap::<String, ()>::new();
    let mut left_adj = BTreeMap::<String, Vec<String>>::new();

    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let toks: Vec<&str> = line.split_whitespace().collect();
        if toks.first() == Some(&"triple") && toks.len() >= 6 {
            syms.insert(toks[1].to_string());
            syms.insert(toks[5].to_string());
            edges.insert(format!("{}|{}|{}", toks[1], toks[3], toks[5]), ());
            left_adj
                .entry(toks[1].to_string())
                .or_default()
                .push(toks[5].to_string());
        } else if toks.first() == Some(&"membrane") && toks.len() >= 2 {
            syms.insert(toks[1].to_string());
        }
    }

    for v in left_adj.values_mut() {
        v.sort();
        v.dedup();
    }

    (syms.len(), edges.len(), left_adj)
}

fn run_baseline_queries(adj: &BTreeMap<String, Vec<String>>) -> usize {
    let mut total = 0usize;
    if let Some(v) = adj.get("node.0") {
        total += v.len();
    }
    if let Some(v) = adj.get("hub.root") {
        total += v.len();
    }
    if let Some(v) = adj.get("hot.root") {
        total += v.len();
    }
    total
}

fn main() {
    let spec_path = env::args().nth(1).unwrap_or_else(|| usage());
    let tasks: Vec<NeutralTaskSpec> =
        serde_json::from_str(&fs::read_to_string(&spec_path).unwrap_or_else(|e| {
            eprintln!("read spec error: {}", e);
            std::process::exit(2);
        }))
        .unwrap_or_else(|e| {
            eprintln!("parse spec error: {}", e);
            std::process::exit(2);
        });

    let mut rows = Vec::<CompareRow>::new();

    for task in tasks {
        let text = fs::read_to_string(&task.corpus_path).unwrap();

        let t0 = Instant::now();
        let idx = build_index_from_text(&task.corpus_path, &text);
        let nsq_index_ms = t0.elapsed().as_secs_f64() * 1000.0;

        let t0 = Instant::now();
        let (baseline_symbols, baseline_edges, baseline_adj) = baseline_parse(&text);
        let baseline_index_ms = t0.elapsed().as_secs_f64() * 1000.0;

        let t0 = Instant::now();
        let mut nsq_total = 0usize;
        if let Some(v) = idx.left_to_edges.get("node.0") {
            nsq_total += v.len();
        }
        if let Some(v) = idx.left_to_edges.get("hub.root") {
            nsq_total += v.len();
        }
        if let Some(v) = idx.left_to_edges.get("hot.root") {
            nsq_total += v.len();
        }
        let _ = nsq_total;
        let nsq_query_ms = t0.elapsed().as_secs_f64() * 1000.0;

        let t0 = Instant::now();
        let _ = run_baseline_queries(&baseline_adj);
        let baseline_query_ms = t0.elapsed().as_secs_f64() * 1000.0;

        rows.push(CompareRow {
            name: task.name,
            nsq_index_ms,
            baseline_index_ms,
            nsq_query_ms,
            baseline_query_ms,
            nsq_symbols: idx.stats.symbols,
            baseline_symbols,
            nsq_edges: idx.stats.edges,
            baseline_edges,
        });
    }

    println!("{}", serde_json::to_string_pretty(&rows).unwrap());
}
