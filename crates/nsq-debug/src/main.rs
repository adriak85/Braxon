use nsq_index::{build_index_from_text, normalize_canonical_text};
use serde_json::json;
use std::env;
use std::fs;

fn usage() -> ! {
    eprintln!("usage: nsq-debug <input.nsq>");
    std::process::exit(2);
}

fn main() {
    let input = env::args().nth(1).unwrap_or_else(|| usage());
    let text = fs::read_to_string(&input).unwrap_or_else(|e| {
        eprintln!("read error: {}", e);
        std::process::exit(2);
    });

    let (normalized, comment_lines_stripped, duplicate_lines_removed) =
        normalize_canonical_text(&text);
    let idx = build_index_from_text(&input, &text);

    let out = json!({
        "input": input,
        "trace": {
            "comment_lines_stripped": comment_lines_stripped,
            "duplicate_lines_removed": duplicate_lines_removed,
            "normalized_lines_preview": normalized.iter().take(32).cloned().collect::<Vec<_>>()
        },
        "phase_counters": {
            "input_lines": idx.stats.input_lines,
            "normalized_lines": idx.stats.normalized_lines,
            "comment_lines_stripped": idx.stats.comment_lines_stripped,
            "duplicate_lines_removed": idx.stats.duplicate_lines_removed
        },
        "summary": idx.stats
    });

    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}
