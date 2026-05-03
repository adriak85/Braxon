use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::process::exit;

#[derive(Serialize, Deserialize)]
struct OptimizeReport {
    threshold_macro_promotion: Option<usize>,
    threshold_expansion: Option<usize>,
    counts: Counts,
    live_selection: Option<LiveSelection>,
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

#[derive(Serialize, Deserialize)]
struct LiveSelection {
    selected_profile: String,
    dense_small_score: i64,
    balanced_score: i64,
    decode_favoring_score: i64,
}

#[derive(Serialize)]
struct ArchonGateReport {
    input_report: String,
    selected_mode: String,
    intake_pressure: String,
    parallel_hint: usize,
    membrane_warning: bool,
    notices_for_linter: Vec<String>,
    notices_for_picker: Vec<String>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: nsq-archon <optimizer_report.json> <archon_report.json>");
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

    let total = rep.counts.noise + rep.counts.triple + rep.counts.membrane + rep.counts.calibrate;
    let membrane_warning = rep.counts.triple > rep.counts.membrane * 3;

    let intake_pressure = if total >= 50000 {
        "extreme".to_string()
    } else if total >= 20000 {
        "high".to_string()
    } else if total >= 5000 {
        "medium".to_string()
    } else {
        "low".to_string()
    };

    let selected_mode = if membrane_warning {
        "throughput_parallel".to_string()
    } else if rep.counts.dominant_family == "membrane" {
        "continuity_bias".to_string()
    } else {
        rep.live_selection
            .map(|x| x.selected_profile)
            .unwrap_or_else(|| "balanced".to_string())
    };

    let parallel_hint = if total >= 50000 {
        6
    } else if total >= 20000 {
        4
    } else {
        3
    };

    let mut notices_for_linter = Vec::<String>::new();
    let mut notices_for_picker = Vec::<String>::new();

    if membrane_warning {
        notices_for_linter
            .push("heads_up: triple density materially exceeds membrane continuity".into());
        notices_for_picker
            .push("prefer continuity-supportive selections near dense graph corridors".into());
    }

    notices_for_linter.push("heads_up: macro promotion threshold should be surfaced inline".into());
    notices_for_picker
        .push("heads_up: picker should honor court-selected mode and archon pressure".into());

    let out = ArchonGateReport {
        input_report: input.clone(),
        selected_mode,
        intake_pressure,
        parallel_hint,
        membrane_warning,
        notices_for_linter,
        notices_for_picker,
    };

    fs::write(output, serde_json::to_string_pretty(&out).unwrap()).unwrap_or_else(|e| {
        eprintln!("write error: {e}");
        exit(2);
    });

    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}
