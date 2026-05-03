//! nsq-index — CLI: read corpus NSQ, write .idx.json and optionally .idx.bin
//!
//! Usage: nsq-index <corpus.nsq> <out.idx.json> [out.idx.bin]

use nsq_index::{build_index_from_text, write_index_binary, write_index_json};
use std::{env, fs, path::Path, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: nsq-index <corpus.nsq> <out.idx.json> [out.idx.bin]");
        process::exit(2);
    }

    let text = fs::read_to_string(&args[1]).unwrap_or_else(|e| {
        eprintln!("read {}: {e}", args[1]);
        process::exit(2);
    });

    let idx = build_index_from_text(&args[1], &text);

    write_index_json(&idx, Path::new(&args[2])).unwrap_or_else(|e| {
        eprintln!("write json: {e}");
        process::exit(2);
    });

    if let Some(bin_path) = args.get(3) {
        let n = write_index_binary(&idx, Path::new(bin_path)).unwrap_or_else(|e| {
            eprintln!("write binary: {e}");
            process::exit(2);
        });
        eprintln!("wrote {} bytes binary index", n);
    }

    eprintln!(
        "symbols={} edges={} states={}",
        idx.stats.symbols, idx.stats.edges, idx.stats.states
    );
}
