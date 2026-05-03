use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::process::exit;

#[derive(Serialize, Deserialize)]
struct OptimizeReport {
    threshold_macro_promotion: usize,
    threshold_expansion: usize,
    live_selection: LiveSelection,
    range_inference: RangeInference,
    macro_suggestions: Vec<MacroSuggestion>,
    expansion_suggestions: Vec<ExpansionSuggestion>,
    counts: Counts,
}

#[derive(Serialize, Deserialize)]
struct LiveSelection {
    selected_profile: String,
    dense_small_score: i64,
    balanced_score: i64,
    decode_favoring_score: i64,
    throughput_parallel_score: i64,
}

#[derive(Serialize, Deserialize)]
struct RangeInference {
    derived_symbol_boundary_carrier: String,
    derived_macro_boundary_carrier: String,
    anchor_boundary_projection: String,
    gain_boundary_projection: String,
    window_boundary_projection: String,
}

#[derive(Serialize, Deserialize)]
struct MacroSuggestion {
    candidate: String,
    from_relation: String,
    count: usize,
    promoted: bool,
}

#[derive(Serialize, Deserialize)]
struct ExpansionSuggestion {
    target: String,
    reason: String,
    count: usize,
}

#[derive(Serialize, Deserialize)]
struct Counts {
    noise: usize,
    triple: usize,
    membrane: usize,
    calibrate: usize,
    dominant_family: String,
    recommendation: String,
}

#[derive(Serialize)]
struct CalibrationLock {
    selected_profile: String,
    promoted_macros: Vec<String>,
    hot_targets: Vec<String>,
    threshold_macro_promotion: usize,
    threshold_expansion: usize,
    representation_lock: RepresentationLock,
    rebalance_actions: Vec<String>,
}

#[derive(Serialize)]
struct RepresentationLock {
    derived_symbol_boundary_carrier: String,
    derived_macro_boundary_carrier: String,
    anchor_boundary_projection: String,
    gain_boundary_projection: String,
    window_boundary_projection: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: nsq-calibrate <optimizer_report.json> <calibration_lock.json>");
        exit(2);
    }

    let input = &args[1];
    let output = &args[2];

    let raw = fs::read_to_string(input).unwrap_or_else(|e| {
        eprintln!("read error: {e}");
        exit(2);
    });

    let rep: OptimizeReport = serde_json::from_str(&raw).unwrap_or_else(|e| {
        eprintln!("json error: {e}");
        exit(2);
    });

    let promoted_macros = rep
        .macro_suggestions
        .into_iter()
        .filter(|m| m.promoted)
        .map(|m| m.candidate)
        .collect::<Vec<_>>();

    let hot_targets = rep
        .expansion_suggestions
        .into_iter()
        .map(|e| e.target)
        .collect::<Vec<_>>();

    let mut rebalance_actions = Vec::<String>::new();
    if rep.counts.noise > rep.counts.triple * 2 {
        rebalance_actions.push("promote repeated noise lanes into triples".into());
    }
    if rep.counts.triple > rep.counts.membrane * 2 {
        rebalance_actions.push("increase membrane transitions near dense triple corridors".into());
    }
    if rep.counts.calibrate == 0 {
        rebalance_actions.push("add calibration records".into());
    }
    if rebalance_actions.is_empty() {
        rebalance_actions.push("no rebalance action required for current proof surface".into());
    }

    let lock = CalibrationLock {
        selected_profile: rep.live_selection.selected_profile,
        promoted_macros,
        hot_targets,
        threshold_macro_promotion: rep.threshold_macro_promotion,
        threshold_expansion: rep.threshold_expansion,
        representation_lock: RepresentationLock {
            derived_symbol_boundary_carrier: rep.range_inference.derived_symbol_boundary_carrier,
            derived_macro_boundary_carrier: rep.range_inference.derived_macro_boundary_carrier,
            anchor_boundary_projection: rep.range_inference.anchor_boundary_projection,
            gain_boundary_projection: rep.range_inference.gain_boundary_projection,
            window_boundary_projection: rep.range_inference.window_boundary_projection,
        },
        rebalance_actions,
    };

    fs::write(output, serde_json::to_string_pretty(&lock).unwrap()).unwrap_or_else(|e| {
        eprintln!("write error: {e}");
        exit(2);
    });

    println!("{}", serde_json::to_string_pretty(&lock).unwrap());
}
