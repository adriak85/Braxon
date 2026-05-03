//! nsq-bench-split — three honest benchmark modes.
//!
//! Usage:
//!   nsq-bench-split core   <corpus.nsq> <queries.json> [iters]
//!   nsq-bench-split cold   <corpus.nsq> <index.idx.json> <queries.json> [iters]
//!   nsq-bench-split warm   <index.idx.bin> <queries.json> [iters]
//!
//! Mode: CORE
//!   What: pure in-process index build + query, no file I/O on the query side.
//!   Measures: How fast is the index data structure itself?
//!   Honest answer to: "what does NSQ's in-memory graph cost per query?"
//!
//! Mode: COLD
//!   What: full CLI-equivalent pipeline: read corpus → build index → write index
//!         → read index back from disk (JSON) → run queries.
//!   Measures: Full end-to-end cold-start latency including disk round-trip.
//!   Honest answer to: "what does a user actually experience?"
//!
//! Mode: WARM
//!   What: Load a pre-built binary index from disk → run queries.
//!   Measures: Binary format load speed + query speed with minimal startup cost.
//!   Honest answer to: "what does the system feel like after first-build warm cache?"

use nsq_index::{
    anchors_in_range, build_index_from_text, read_index_binary, read_index_json, write_index_json,
    ArtifactIndex,
};
use nsq_query::{edges_left, find_rel, find_symbol, neighbors, states_target};
use serde::Serialize;
use std::path::Path;
use std::time::Instant;
use std::{env, fs, process};

#[derive(Serialize)]
struct SplitResult {
    mode: String,
    corpus_bytes: Option<u64>,
    index_bytes_json: Option<u64>,
    index_bytes_bin: Option<u64>,
    iters: usize,
    // CORE timings (in-process)
    build_ms_mean: Option<f64>,
    query_ms_mean: Option<f64>,
    query_ms_total: Option<f64>,
    // COLD timings (disk round-trip)
    cold_build_ms: Option<f64>,
    cold_write_ms: Option<f64>,
    cold_read_ms: Option<f64>,
    cold_query_ms: Option<f64>,
    // WARM timings (binary load + query)
    warm_load_ms: Option<f64>,
    warm_query_ms: Option<f64>,
    // Query stats
    query_count: usize,
    symbols: usize,
    edges: usize,
    states: usize,
}

fn load_queries(path: &str) -> Vec<String> {
    let raw = fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("read queries {path}: {e}");
        process::exit(2);
    });
    serde_json::from_str::<Vec<String>>(&raw).unwrap_or_else(|e| {
        eprintln!("parse queries: {e}");
        process::exit(2);
    })
}

fn run_queries_inprocess(idx: &ArtifactIndex, queries: &[String]) {
    for q in queries {
        let toks: Vec<&str> = q.split_whitespace().collect();
        match (
            toks.first().copied(),
            toks.get(1).copied(),
            toks.get(2).copied(),
        ) {
            (Some("find"), Some("symbol"), Some(n)) => {
                let _ = find_symbol(idx, n);
            }
            (Some("find"), Some("rel"), Some(n)) => {
                let _ = find_rel(idx, n);
            }
            (Some("neighbors"), Some(n), _) => {
                let _ = neighbors(idx, n);
            }
            (Some("edges"), Some(arg), _) => {
                if let Some(n) = arg.strip_prefix("left=") {
                    let _ = edges_left(idx, n);
                } else if let Some(n) = arg.strip_prefix("rel=") {
                    let _ = find_rel(idx, n);
                }
            }
            (Some("states"), Some(arg), _) => {
                if let Some(n) = arg.strip_prefix("target=") {
                    let _ = states_target(idx, n);
                }
            }
            (Some("anchors"), ..) => {
                let min = toks
                    .iter()
                    .find_map(|t| t.strip_prefix("min="))
                    .unwrap_or("0");
                let max = toks
                    .iter()
                    .find_map(|t| t.strip_prefix("max="))
                    .unwrap_or("");
                let _ = anchors_in_range(idx, min, max);
            }
            _ => {}
        }
    }
}

fn bench_core(corpus_path: &str, queries_path: &str, iters: usize) -> SplitResult {
    let corpus = fs::read_to_string(corpus_path).unwrap_or_else(|e| {
        eprintln!("read corpus: {e}");
        process::exit(2);
    });
    let corpus_bytes = corpus.len() as u64;
    let queries = load_queries(queries_path);

    // Warmup
    let idx0 = build_index_from_text(corpus_path, &corpus);
    run_queries_inprocess(&idx0, &queries);

    // Time build
    let t_build = Instant::now();
    let mut last_idx = idx0.clone();
    for _ in 0..iters {
        last_idx = build_index_from_text(corpus_path, &corpus);
        std::hint::black_box(&last_idx);
    }
    let build_ms = t_build.elapsed().as_secs_f64() * 1000.0 / iters as f64;

    // Time queries (all in-process, no I/O)
    let t_q = Instant::now();
    for _ in 0..iters {
        run_queries_inprocess(&last_idx, &queries);
    }
    let query_ms_total = t_q.elapsed().as_secs_f64() * 1000.0 / iters as f64;
    let query_ms_mean = if queries.is_empty() {
        0.0
    } else {
        query_ms_total / queries.len() as f64
    };

    SplitResult {
        mode: "core".to_string(),
        corpus_bytes: Some(corpus_bytes),
        index_bytes_json: None,
        index_bytes_bin: None,
        iters,
        build_ms_mean: Some(build_ms),
        query_ms_mean: Some(query_ms_mean),
        query_ms_total: Some(query_ms_total),
        cold_build_ms: None,
        cold_write_ms: None,
        cold_read_ms: None,
        cold_query_ms: None,
        warm_load_ms: None,
        warm_query_ms: None,
        query_count: queries.len(),
        symbols: last_idx.stats.symbols,
        edges: last_idx.stats.edges,
        states: last_idx.stats.states,
    }
}

fn bench_cold(
    corpus_path: &str,
    index_json_path: &str,
    queries_path: &str,
    iters: usize,
) -> SplitResult {
    let corpus = fs::read_to_string(corpus_path).unwrap_or_else(|e| {
        eprintln!("read corpus: {e}");
        process::exit(2);
    });
    let corpus_bytes = corpus.len() as u64;
    let queries = load_queries(queries_path);
    let json_path = Path::new(index_json_path);

    let mut cold_build = 0.0f64;
    let mut cold_write = 0.0f64;
    let mut cold_read = 0.0f64;
    let mut cold_query = 0.0f64;
    let mut last_symbols = 0;
    let mut last_edges = 0;
    let mut last_states = 0;
    let mut json_bytes = 0u64;

    for _ in 0..iters {
        let t = Instant::now();
        let idx = build_index_from_text(corpus_path, &corpus);
        cold_build += t.elapsed().as_secs_f64() * 1000.0;

        let t = Instant::now();
        write_index_json(&idx, json_path).unwrap_or_else(|e| {
            eprintln!("write index: {e}");
            process::exit(2);
        });
        cold_write += t.elapsed().as_secs_f64() * 1000.0;
        json_bytes = fs::metadata(json_path).map(|m| m.len()).unwrap_or(0);

        let t = Instant::now();
        let idx2 = read_index_json(json_path).unwrap_or_else(|e| {
            eprintln!("read index: {e}");
            process::exit(2);
        });
        cold_read += t.elapsed().as_secs_f64() * 1000.0;

        let t = Instant::now();
        run_queries_inprocess(&idx2, &queries);
        cold_query += t.elapsed().as_secs_f64() * 1000.0;

        last_symbols = idx2.stats.symbols;
        last_edges = idx2.stats.edges;
        last_states = idx2.stats.states;
    }

    SplitResult {
        mode: "cold".to_string(),
        corpus_bytes: Some(corpus_bytes),
        index_bytes_json: Some(json_bytes),
        index_bytes_bin: None,
        iters,
        build_ms_mean: None,
        query_ms_mean: None,
        query_ms_total: None,
        cold_build_ms: Some(cold_build / iters as f64),
        cold_write_ms: Some(cold_write / iters as f64),
        cold_read_ms: Some(cold_read / iters as f64),
        cold_query_ms: Some(cold_query / iters as f64),
        warm_load_ms: None,
        warm_query_ms: None,
        query_count: queries.len(),
        symbols: last_symbols,
        edges: last_edges,
        states: last_states,
    }
}

fn bench_warm(index_bin_path: &str, queries_path: &str, iters: usize) -> SplitResult {
    let queries = load_queries(queries_path);
    let bin_path = Path::new(index_bin_path);
    let bin_bytes = fs::metadata(bin_path).map(|m| m.len()).unwrap_or(0);

    let mut warm_load = 0.0f64;
    let mut warm_query = 0.0f64;
    let mut last_symbols = 0;
    let mut last_edges = 0;
    let mut last_states = 0;

    for _ in 0..iters {
        let t = Instant::now();
        let idx = read_index_binary(bin_path).unwrap_or_else(|e| {
            eprintln!("load binary: {e}");
            process::exit(2);
        });
        warm_load += t.elapsed().as_secs_f64() * 1000.0;

        let t = Instant::now();
        run_queries_inprocess(&idx, &queries);
        warm_query += t.elapsed().as_secs_f64() * 1000.0;

        last_symbols = idx.stats.symbols;
        last_edges = idx.stats.edges;
        last_states = idx.stats.states;
    }

    SplitResult {
        mode: "warm".to_string(),
        corpus_bytes: None,
        index_bytes_json: None,
        index_bytes_bin: Some(bin_bytes),
        iters,
        build_ms_mean: None,
        query_ms_mean: None,
        query_ms_total: None,
        cold_build_ms: None,
        cold_write_ms: None,
        cold_read_ms: None,
        cold_query_ms: None,
        warm_load_ms: Some(warm_load / iters as f64),
        warm_query_ms: Some(warm_query / iters as f64),
        query_count: queries.len(),
        symbols: last_symbols,
        edges: last_edges,
        states: last_states,
    }
}

fn usage() -> ! {
    eprintln!("nsq-bench-split core  <corpus.nsq> <queries.json> [iters=10]");
    eprintln!("nsq-bench-split cold  <corpus.nsq> <index.idx.json> <queries.json> [iters=5]");
    eprintln!("nsq-bench-split warm  <index.idx.bin> <queries.json> [iters=10]");
    process::exit(2);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage();
    }

    let result = match args[1].as_str() {
        "core" => {
            if args.len() < 4 {
                usage();
            }
            let iters = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(10);
            bench_core(&args[2], &args[3], iters)
        }
        "cold" => {
            if args.len() < 5 {
                usage();
            }
            let iters = args.get(5).and_then(|s| s.parse().ok()).unwrap_or(5);
            bench_cold(&args[2], &args[3], &args[4], iters)
        }
        "warm" => {
            if args.len() < 4 {
                usage();
            }
            let iters = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(10);
            bench_warm(&args[2], &args[3], iters)
        }
        _ => usage(),
    };

    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
