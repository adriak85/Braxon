//! nsq-query — single query or batch mode.
//!
//! Usage:
//!   nsq-query <index.idx.json|index.idx.bin> <query>
//!   nsq-query <index.idx.json|index.idx.bin> --batch <queries.txt>
//!   nsq-query <index.idx.json|index.idx.bin> --batch-json <queries.json>
//!
//! --batch:      one query per line in a plain text file, results as JSON array
//! --batch-json: JSON array of query strings, results as JSON array
//!
//! The key benefit of batch mode: the index is loaded exactly once.
//! All queries run in-process before the process exits.
//! This eliminates the cold-start overhead that was dominating query_ms_mean.

use nsq_index::{
    anchors_in_range, read_index_binary, read_index_json, shortest_path, ArtifactIndex,
};
use nsq_query::{
    edges_left, edges_rel, edges_right, find_rel, find_symbol, neighbors, states_target,
    QueryResult,
};
use serde::Serialize;
use serde_json::json;
use std::path::Path;
use std::time::Instant;
use std::{env, fs, process};

#[derive(Serialize)]
struct BatchReport {
    index_path: String,
    index_load_ms: f64,
    query_count: usize,
    total_query_ms: f64,
    mean_query_ms: f64,
    results: Vec<QueryResult>,
}

fn load_index(path: &str) -> ArtifactIndex {
    let p = Path::new(path);
    if path.ends_with(".bin") {
        read_index_binary(p).unwrap_or_else(|e| {
            eprintln!("load binary index error: {e}");
            process::exit(2);
        })
    } else {
        read_index_json(p).unwrap_or_else(|e| {
            eprintln!("load json index error: {e}");
            process::exit(2);
        })
    }
}

fn run_query(idx: &ArtifactIndex, q: &str) -> QueryResult {
    let toks: Vec<&str> = q.split_whitespace().collect();
    if toks.is_empty() {
        return QueryResult {
            command: q.to_string(),
            matches: json!(null),
        };
    }
    match (toks[0], toks.get(1).copied(), toks.get(2).copied()) {
        ("find", Some("symbol"), Some(name)) => find_symbol(idx, name),
        ("find", Some("rel"), Some(name)) => find_rel(idx, name),
        ("neighbors", Some(name), _) => neighbors(idx, name),
        ("edges", ..) => {
            // "edges left=X" or "edges right=X" or "edges rel=X"
            if let Some(arg) = toks.get(1) {
                if let Some(name) = arg.strip_prefix("left=") {
                    return edges_left(idx, name);
                }
                if let Some(name) = arg.strip_prefix("right=") {
                    return edges_right(idx, name);
                }
                if let Some(name) = arg.strip_prefix("rel=") {
                    return edges_rel(idx, name);
                }
            }
            QueryResult {
                command: q.to_string(),
                matches: json!({ "error": "bad edges syntax" }),
            }
        }
        ("states", ..) => {
            if let Some(arg) = toks.get(1) {
                if let Some(name) = arg.strip_prefix("target=") {
                    return states_target(idx, name);
                }
            }
            QueryResult {
                command: q.to_string(),
                matches: json!({ "error": "bad states syntax" }),
            }
        }
        ("anchors", ..) => {
            let min = toks
                .iter()
                .find_map(|t| t.strip_prefix("min="))
                .unwrap_or("0");
            let max = toks
                .iter()
                .find_map(|t| t.strip_prefix("max="))
                .unwrap_or("");
            let rows = anchors_in_range(idx, min, max);
            QueryResult {
                command: q.to_string(),
                matches: serde_json::to_value(rows).unwrap(),
            }
        }
        ("path", Some(src), Some(dst)) => {
            let depth = toks
                .iter()
                .find_map(|t| t.strip_prefix("depth="))
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(6);
            let path = shortest_path(idx, src, dst, depth);
            QueryResult {
                command: q.to_string(),
                matches: json!({ "path": path }),
            }
        }
        _ => QueryResult {
            command: q.to_string(),
            matches: json!({ "error": "unknown query" }),
        },
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: nsq-query <index_path> <query>");
        eprintln!("       nsq-query <index_path> --batch <queries.txt>");
        eprintln!("       nsq-query <index_path> --batch-json <queries.json>");
        process::exit(2);
    }

    let index_path = &args[1];
    let t_load = Instant::now();
    let idx = load_index(index_path);
    let load_ms = t_load.elapsed().as_secs_f64() * 1000.0;

    match args[2].as_str() {
        "--batch" | "--batch-json" => {
            let queries: Vec<String> = if args[2] == "--batch-json" {
                let raw = fs::read_to_string(&args[3]).unwrap_or_else(|e| {
                    eprintln!("read {}: {e}", args[3]);
                    process::exit(2);
                });
                serde_json::from_str::<Vec<String>>(&raw).unwrap_or_else(|e| {
                    eprintln!("parse json queries: {e}");
                    process::exit(2);
                })
            } else {
                let raw = fs::read_to_string(&args[3]).unwrap_or_else(|e| {
                    eprintln!("read {}: {e}", args[3]);
                    process::exit(2);
                });
                raw.lines()
                    .filter(|l| !l.trim().is_empty())
                    .map(|l| l.to_string())
                    .collect()
            };

            let t_queries = Instant::now();
            let results: Vec<QueryResult> = queries.iter().map(|q| run_query(&idx, q)).collect();
            let total_ms = t_queries.elapsed().as_secs_f64() * 1000.0;

            let report = BatchReport {
                index_path: index_path.clone(),
                index_load_ms: load_ms,
                query_count: results.len(),
                total_query_ms: total_ms,
                mean_query_ms: if results.is_empty() {
                    0.0
                } else {
                    total_ms / results.len() as f64
                },
                results,
            };
            println!("{}", serde_json::to_string_pretty(&report).unwrap());
        }
        query => {
            let r = run_query(&idx, query);
            println!("{}", serde_json::to_string_pretty(&r).unwrap());
        }
    }
}
