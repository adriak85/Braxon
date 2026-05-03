use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::process::exit;

#[derive(Default)]
struct Stats {
    noise: usize,
    triple: usize,
    membrane: usize,
    calibrate: usize,
    symbols: BTreeSet<String>,
    macros: BTreeMap<String, usize>,
    relations: BTreeMap<String, usize>,
    targets: BTreeMap<String, usize>,
    relation_family_counts: BTreeMap<String, usize>,
    target_family_counts: BTreeMap<String, usize>,
    max_anchor: usize,
    max_gain: usize,
    max_window: usize,
}

#[derive(Serialize)]
struct RangeInference {
    derived_symbol_boundary_carrier: String,
    derived_macro_boundary_carrier: String,
    anchor_boundary_projection: String,
    gain_boundary_projection: String,
    window_boundary_projection: String,
}

#[derive(Serialize)]
struct MacroSuggestion {
    candidate: String,
    from_relation: String,
    count: usize,
    promoted: bool,
}

#[derive(Serialize)]
struct ExpansionSuggestion {
    target: String,
    reason: String,
    count: usize,
}

#[derive(Serialize)]
struct FamilyCluster {
    family: String,
    count: usize,
}

#[derive(Serialize)]
struct BalanceReport {
    noise: usize,
    triple: usize,
    membrane: usize,
    calibrate: usize,
    dominant_family: String,
    recommendation: String,
}

#[derive(Serialize)]
struct LiveSelection {
    selected_profile: String,
    dense_small_score: i64,
    balanced_score: i64,
    decode_favoring_score: i64,
    throughput_parallel_score: i64,
}

#[derive(Serialize)]
struct OptimizeReport {
    file: String,
    lint_required: bool,
    threshold_macro_promotion: usize,
    threshold_expansion: usize,
    counts: BalanceReport,
    range_inference: RangeInference,
    macro_suggestions: Vec<MacroSuggestion>,
    expansion_suggestions: Vec<ExpansionSuggestion>,
    relation_family_clusters: Vec<FamilyCluster>,
    target_family_clusters: Vec<FamilyCluster>,
    live_selection: LiveSelection,
    symbol_count: usize,
    macro_count: usize,
}

fn boundary_carrier_for(n: usize) -> String {
    if n <= 0o377 {
        "boundary-single-octet-carrier".into()
    } else if n <= 0o177777 {
        "boundary-dual-octet-carrier".into()
    } else {
        "boundary-quad-octet-carrier".into()
    }
}

fn boundary_projection_for(n: usize, lane: &str) -> String {
    let carrier = if n <= 0o377 {
        "boundary-single-octet"
    } else if n <= 0o177777 {
        "boundary-dual-octet"
    } else {
        "boundary-quad-octet"
    };
    format!("{carrier}-{lane}-projection")
}

fn boundary_delta_projection_for(n: usize, lane: &str) -> String {
    let carrier = if n <= 0o377 {
        "boundary-single-octet"
    } else if n <= 0o177777 {
        "boundary-dual-octet"
    } else {
        "boundary-quad-octet"
    };
    format!("{carrier}-{lane}-delta-projection")
}

fn bump(map: &mut BTreeMap<String, usize>, key: &str) {
    *map.entry(key.to_string()).or_insert(0) += 1;
}

fn family_of(s: &str) -> String {
    s.split(':').next().unwrap_or("root").to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: nsq-optimize <input.nsq> <report.json>");
        exit(2);
    }

    let input = &args[1];
    let output = &args[2];
    let text = fs::read_to_string(input).unwrap_or_else(|e| {
        eprintln!("read error: {e}");
        exit(2);
    });

    let mut stats = Stats::default();

    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let toks: Vec<&str> = line.split_whitespace().collect();
        if toks.is_empty() {
            continue;
        }

        match toks[0] {
            "noise" => {
                stats.noise += 1;
                if let Some(sym) = toks.get(1) {
                    stats.symbols.insert((*sym).to_string());
                    bump(&mut stats.targets, sym);
                    bump(&mut stats.target_family_counts, &family_of(sym));
                }
                let mut i = 2usize;
                while i + 1 < toks.len() {
                    match toks[i] {
                        ":macro" => {
                            bump(&mut stats.macros, toks[i + 1]);
                            bump(&mut stats.relation_family_counts, &family_of(toks[i + 1]));
                        }
                        ":pos" => {
                            if let Ok(v) = toks[i + 1].parse::<usize>() {
                                stats.max_anchor = stats.max_anchor.max(v);
                            }
                        }
                        _ => {}
                    }
                    i += 2;
                }
            }
            "triple" => {
                stats.triple += 1;
                if let Some(subject) = toks.get(1) {
                    stats.symbols.insert((*subject).to_string());
                    bump(&mut stats.targets, subject);
                    bump(&mut stats.target_family_counts, &family_of(subject));
                }
                if let Some(relation) = toks.get(3) {
                    bump(&mut stats.relations, relation);
                    bump(&mut stats.macros, relation);
                    bump(&mut stats.relation_family_counts, &family_of(relation));
                }
                if let Some(object) = toks.get(5) {
                    stats.symbols.insert((*object).to_string());
                    bump(&mut stats.targets, object);
                    bump(&mut stats.target_family_counts, &family_of(object));
                }
                let mut i = 6usize;
                while i + 1 < toks.len() {
                    if toks[i] == ":anchor" {
                        if let Ok(v) = toks[i + 1].parse::<usize>() {
                            stats.max_anchor = stats.max_anchor.max(v);
                        }
                    }
                    i += 2;
                }
            }
            "membrane" => {
                stats.membrane += 1;
                if let Some(cell) = toks.get(1) {
                    stats.symbols.insert((*cell).to_string());
                    bump(&mut stats.targets, cell);
                    bump(&mut stats.target_family_counts, &family_of(cell));
                }
                let mut i = 2usize;
                while i + 1 < toks.len() {
                    if toks[i] == ":state" {
                        stats.symbols.insert(toks[i + 1].to_string());
                        bump(&mut stats.targets, toks[i + 1]);
                        bump(&mut stats.target_family_counts, &family_of(toks[i + 1]));
                    }
                    i += 2;
                }
            }
            "calibrate" => {
                stats.calibrate += 1;
                if let Some(target) = toks.get(1) {
                    stats.symbols.insert((*target).to_string());
                    bump(&mut stats.targets, target);
                    bump(&mut stats.target_family_counts, &family_of(target));
                    bump(&mut stats.relation_family_counts, &family_of(target));
                }
                let mut i = 2usize;
                while i + 1 < toks.len() {
                    match toks[i] {
                        ":basis" => {
                            stats.symbols.insert(toks[i + 1].to_string());
                            bump(&mut stats.targets, toks[i + 1]);
                            bump(&mut stats.target_family_counts, &family_of(toks[i + 1]));
                        }
                        ":gain" => {
                            if let Ok(v) = toks[i + 1].parse::<usize>() {
                                stats.max_gain = stats.max_gain.max(v);
                            }
                        }
                        ":window" => {
                            if let Ok(v) = toks[i + 1].parse::<usize>() {
                                stats.max_window = stats.max_window.max(v);
                            }
                        }
                        _ => {}
                    }
                    i += 2;
                }
            }
            _ => {}
        }
    }

    let threshold_macro_promotion = if stats.triple + stats.noise >= 512 {
        8
    } else if stats.triple + stats.noise >= 128 {
        4
    } else {
        2
    };
    let threshold_expansion = if stats.symbols.len() >= 64 {
        6
    } else if stats.symbols.len() >= 24 {
        4
    } else {
        2
    };

    let mut macro_suggestions = Vec::<MacroSuggestion>::new();
    for (rel, count) in &stats.relations {
        let promoted = *count >= threshold_macro_promotion;
        macro_suggestions.push(MacroSuggestion {
            candidate: format!("auto:{}", rel.replace(':', "_")),
            from_relation: rel.clone(),
            count: *count,
            promoted,
        });
    }

    let mut expansion_suggestions = Vec::<ExpansionSuggestion>::new();
    for (target, count) in &stats.targets {
        if *count >= threshold_expansion {
            expansion_suggestions.push(ExpansionSuggestion {
                target: target.clone(),
                reason: "high reuse above expansion threshold".into(),
                count: *count,
            });
        }
    }

    let mut relation_family_clusters = stats
        .relation_family_counts
        .iter()
        .map(|(family, count)| FamilyCluster {
            family: family.clone(),
            count: *count,
        })
        .collect::<Vec<_>>();
    relation_family_clusters
        .sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.family.cmp(&b.family)));

    let mut target_family_clusters = stats
        .target_family_counts
        .iter()
        .map(|(family, count)| FamilyCluster {
            family: family.clone(),
            count: *count,
        })
        .collect::<Vec<_>>();
    target_family_clusters
        .sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.family.cmp(&b.family)));

    let dominant_family = {
        let pairs = [
            ("noise", stats.noise),
            ("triple", stats.triple),
            ("membrane", stats.membrane),
            ("calibrate", stats.calibrate),
        ];
        pairs
            .iter()
            .max_by_key(|(_, v)| *v)
            .map(|(k, _)| (*k).to_string())
            .unwrap_or_else(|| "none".into())
    };

    let recommendation = if stats.triple == 0 {
        "add more structured triples if semantic graph density matters".to_string()
    } else if stats.calibrate == 0 {
        "add calibration records if adaptive behavior is intended".to_string()
    } else if stats.noise > stats.triple * 3 {
        "noise-heavy surface; promote repeated lanes into triples and promoted macros".to_string()
    } else if stats.triple > stats.membrane * 3 {
        "graph-heavy surface; increase membrane transitions to support state continuity".to_string()
    } else {
        "distribution is acceptable for current proof surface".to_string()
    };

    let promoted_macro_count = macro_suggestions.iter().filter(|m| m.promoted).count() as i64;
    let high_reuse_targets = expansion_suggestions.len() as i64;

    let dense_small_score = 3 * stats.symbols.len() as i64 + 2 * stats.macros.len() as i64
        - 2 * (stats.max_anchor as i64 / 256);

    let balanced_score = -((stats.noise as i64 - stats.triple as i64).abs())
        - ((stats.triple as i64 - stats.membrane as i64).abs())
        - ((stats.membrane as i64 - stats.calibrate as i64).abs());

    let decode_favoring_score = 4 * stats.triple as i64
        + 3 * promoted_macro_count
        + 2 * high_reuse_targets
        + stats.symbols.len() as i64;

    let throughput_parallel_score = 2 * (stats.noise + stats.triple + stats.membrane) as i64
        + 5 * promoted_macro_count
        + 2 * high_reuse_targets
        - (stats.max_anchor as i64 / 512);

    let selected_profile = {
        let mut pairs = [
            ("dense_small", dense_small_score),
            ("balanced", balanced_score),
            ("decode_favoring", decode_favoring_score),
            ("throughput_parallel", throughput_parallel_score),
        ];
        pairs.sort_by_key(|(_, s)| -*s);
        pairs[0].0.to_string()
    };

    let report = OptimizeReport {
        file: input.clone(),
        lint_required: true,
        threshold_macro_promotion,
        threshold_expansion,
        counts: BalanceReport {
            noise: stats.noise,
            triple: stats.triple,
            membrane: stats.membrane,
            calibrate: stats.calibrate,
            dominant_family,
            recommendation,
        },
        range_inference: RangeInference {
            derived_symbol_boundary_carrier: boundary_carrier_for(stats.symbols.len()),
            derived_macro_boundary_carrier: boundary_carrier_for(stats.macros.len()),
            anchor_boundary_projection: boundary_delta_projection_for(stats.max_anchor, "anchor"),
            gain_boundary_projection: boundary_projection_for(stats.max_gain, "gain"),
            window_boundary_projection: boundary_projection_for(stats.max_window, "window"),
        },
        macro_suggestions,
        expansion_suggestions,
        relation_family_clusters,
        target_family_clusters,
        live_selection: LiveSelection {
            selected_profile,
            dense_small_score,
            balanced_score,
            decode_favoring_score,
            throughput_parallel_score,
        },
        symbol_count: stats.symbols.len(),
        macro_count: stats.macros.len(),
    };

    fs::write(output, serde_json::to_string_pretty(&report).unwrap()).unwrap_or_else(|e| {
        eprintln!("write error: {e}");
        exit(2);
    });

    println!("{}", serde_json::to_string_pretty(&report).unwrap());
}
