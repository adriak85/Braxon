use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::exit;

#[derive(Deserialize)]
struct Canonical {
    version: u32,
    kingdom: String,
    classes: BTreeMap<String, ClassDef>,
    seats: Vec<Seat>,
    hounds: Vec<Hound>,
    escalation: Escalation,
    laws: BTreeMap<String, bool>,
    capitals: Vec<String>,
    required_ledgers: Vec<String>,
    hooks: BTreeMap<String, Vec<String>>,
}

#[derive(Deserialize)]
struct ClassDef {
    persistent: bool,
    authority: bool,
    crash_guarded: bool,
    journal_required: bool,
    recoverable: bool,
}

#[derive(Deserialize, Clone)]
struct Seat {
    id: String,
    title: String,
    class: String,
    aliases: Vec<String>,
    authority_domain: Vec<String>,
    advisory_domain: Vec<String>,
    forbidden: Vec<String>,
}

#[derive(Deserialize, Clone)]
struct Hound {
    id: String,
    title: String,
    class: String,
    authority_domain: Vec<String>,
    advisory_domain: Vec<String>,
}

#[derive(Deserialize)]
struct Escalation {
    default_chain: Vec<String>,
    keeper_final_if_ace_trumps_jack: bool,
}

#[derive(Serialize)]
struct BRAXONCourtConfig {
    version: u32,
    durability_classes: BTreeMap<String, ClassOut>,
    offices: BTreeMap<String, OfficeOut>,
    escalation: EscOut,
    laws: BTreeMap<String, bool>,
    required_ledgers: Vec<String>,
}

#[derive(Serialize)]
struct ClassOut {
    persistent: bool,
    authority: bool,
    crash_guarded: bool,
}

#[derive(Serialize)]
struct OfficeOut {
    title: String,
    class: String,
    authority_domain: Vec<String>,
}

#[derive(Serialize)]
struct EscOut {
    default_chain: Vec<String>,
    keeper_final_if_ace_trumps_jack: bool,
}

#[derive(Serialize)]
struct NsqSeed {
    version: u32,
    roles: Vec<SeedRole>,
    promotion_ladder: Vec<String>,
    deadlock_chain: Vec<String>,
}

#[derive(Serialize)]
struct SeedRole {
    name: String,
    title: String,
    class: String,
    authority: bool,
    crash_guarded: bool,
    persistence: String,
}

#[derive(Serialize)]
struct NsqCourtConfig {
    court: BTreeMap<String, NsqCourtRole>,
    hounds: HoundsOut,
    rules: BTreeMap<String, bool>,
    capitals: Vec<String>,
    seize_all_caps_flow: StepFlow,
    arrest_flow: ArrestFlow,
}

#[derive(Serialize)]
struct NsqCourtRole {
    title: String,
    authority: bool,
    domain: Vec<String>,
}

#[derive(Serialize)]
struct HoundsOut {
    authority: bool,
    call_phrase: String,
    purpose: Vec<String>,
    classes: BTreeMap<String, HoundClassOut>,
    escalation: Vec<String>,
}

#[derive(Serialize)]
struct HoundClassOut {
    domain: Vec<String>,
}

#[derive(Serialize)]
struct StepFlow {
    steps: Vec<String>,
}

#[derive(Serialize)]
struct ArrestFlow {
    steps: Vec<String>,
    required_fields: Vec<String>,
}

fn write(path: &Path, body: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, body).unwrap();
}

fn o8_count(value: usize) -> String {
    format!("{value:o}")
}

fn build_kingdom_surface_metrics(c: &Canonical) -> serde_json::Value {
    let hook_total: usize = c.hooks.values().map(|v| v.len()).sum();
    let seat_alias_total: usize = c.seats.iter().map(|seat| seat.aliases.len()).sum();
    let hound_authority_total: usize = c
        .hounds
        .iter()
        .map(|hound| hound.authority_domain.len())
        .sum();

    let hook_modules = c
        .hooks
        .iter()
        .map(|(module, hooks)| {
            (
                module.clone(),
                serde_json::json!({
                    "hook_count_o8": o8_count(hooks.len()),
                    "hooks": hooks.clone(),
                }),
            )
        })
        .collect::<serde_json::Map<String, serde_json::Value>>();

    let seat_alias_matrix = c
        .seats
        .iter()
        .enumerate()
        .map(|(idx, seat)| {
            serde_json::json!({
                "seat_index_o8": o8_count(idx),
                "alias_count_o8": o8_count(seat.aliases.len()),
                "aliases": seat.aliases.clone(),
            })
        })
        .collect::<Vec<serde_json::Value>>();

    let hound_authority_matrix = c
        .hounds
        .iter()
        .enumerate()
        .map(|(idx, hound)| {
            serde_json::json!({
                "hound_index_o8": o8_count(idx),
                "authority_count_o8": o8_count(hound.authority_domain.len()),
                "authority_domain": hound.authority_domain.clone(),
            })
        })
        .collect::<Vec<serde_json::Value>>();

    serde_json::json!({
        "surface_family": "canonical_base8_kingdom_metrics",
        "flattened": false,
        "hook_module_count_o8": o8_count(c.hooks.len()),
        "hook_total_o8": o8_count(hook_total),
        "seat_count_o8": o8_count(c.seats.len()),
        "seat_alias_total_o8": o8_count(seat_alias_total),
        "hound_count_o8": o8_count(c.hounds.len()),
        "hound_authority_total_o8": o8_count(hound_authority_total),
        "hook_modules": hook_modules,
        "seat_alias_matrix": seat_alias_matrix,
        "hound_authority_matrix": hound_authority_matrix
    })
}

fn main() {
    let root = std::env::var("BRAXON_HOME")
        .unwrap_or_else(|_| format!("{}/Braxon", std::env::var("HOME").unwrap()));
    let canonical_path = Path::new(&root).join("config/kingdom/court_canonical.json");
    let raw = fs::read_to_string(&canonical_path).unwrap_or_else(|e| {
        eprintln!("read error {}: {}", canonical_path.display(), e);
        exit(2);
    });
    let c: Canonical = serde_json::from_str(&raw).unwrap_or_else(|e| {
        eprintln!("json error {}: {}", canonical_path.display(), e);
        exit(2);
    });

    let kingdom_surface_metrics = build_kingdom_surface_metrics(&c);
    std::fs::create_dir_all("generated").unwrap_or_else(|e| {
        panic!("failed to create generated kingdom surface directory: {e}");
    });
    std::fs::write(
        "generated/kingdom_surface_metrics.json",
        serde_json::to_string_pretty(&kingdom_surface_metrics).unwrap(),
    )
    .unwrap_or_else(|e| {
        panic!("failed to write generated/kingdom_surface_metrics.json: {e}");
    });

    let mut md = String::new();
    md.push_str("# BRAXON Court Constitution\n\n");
    md.push_str(&format!("Kingdom: {}\n\n", c.kingdom));
    md.push_str("## Durability classes\n");
    for (name, class) in &c.classes {
        md.push_str(&format!(
            "- {}: persistent={} authority={} crash_guarded={} journal_required={} recoverable={}\n",
            name, class.persistent, class.authority, class.crash_guarded, class.journal_required, class.recoverable
        ));
    }
    md.push_str("\n## Seats\n");
    for seat in &c.seats {
        md.push_str(&format!(
            "- {} = {} [{}] domains={} advisory={} forbidden={}\n",
            seat.id,
            seat.title,
            seat.class,
            seat.authority_domain.join("|"),
            seat.advisory_domain.join("|"),
            seat.forbidden.join("|")
        ));
    }
    md.push_str("\n## Hounds\n");
    for hound in &c.hounds {
        md.push_str(&format!(
            "- {} = {} [{}] advisory={}\n",
            hound.id,
            hound.title,
            hound.class,
            hound.advisory_domain.join("|")
        ));
    }
    md.push_str("\n## Escalation\n");
    md.push_str(&format!("- {}\n", c.escalation.default_chain.join(" -> ")));
    md.push_str("\n## Capitals\n");
    for cap in &c.capitals {
        md.push_str(&format!("- {}\n", cap));
    }

    write(
        &Path::new(&root).join("specs/court/COURT_CONSTITUTION.md"),
        &md,
    );
    write(&Path::new(&root).join("specs/nsq/court_of_archons.md"), &md);

    let mut dur = BTreeMap::new();
    for (k, v) in &c.classes {
        dur.insert(
            k.clone(),
            ClassOut {
                persistent: v.persistent,
                authority: v.authority,
                crash_guarded: v.crash_guarded,
            },
        );
    }

    let mut offices = BTreeMap::new();
    for seat in &c.seats {
        offices.insert(
            seat.id.clone(),
            OfficeOut {
                title: seat.title.clone(),
                class: seat.class.clone(),
                authority_domain: seat.authority_domain.clone(),
            },
        );
    }

    let BRAXON_cfg = BRAXONCourtConfig {
        version: c.version,
        durability_classes: dur,
        offices,
        escalation: EscOut {
            default_chain: c.escalation.default_chain.clone(),
            keeper_final_if_ace_trumps_jack: c.escalation.keeper_final_if_ace_trumps_jack,
        },
        laws: c.laws.clone(),
        required_ledgers: c.required_ledgers.clone(),
    };
    write(
        &Path::new(&root).join("config/braxon_court.json"),
        &serde_json::to_string_pretty(&BRAXON_cfg).unwrap(),
    );

    let mut roles = Vec::new();
    for seat in &c.seats {
        let class = c.classes.get(&seat.class).unwrap();
        roles.push(SeedRole {
            name: seat.id.clone(),
            title: seat.id.clone(),
            class: "court_seat".to_string(),
            authority: class.authority,
            crash_guarded: class.crash_guarded,
            persistence: "seat_bound".to_string(),
        });
    }
    let nsq_seed = NsqSeed {
        version: c.version,
        roles,
        promotion_ladder: vec![
            "disposable_agent".into(),
            "page".into(),
            "recoverable_page".into(),
            "court_seat".into(),
        ],
        deadlock_chain: vec!["bard".into(), "jack".into(), "ace".into(), "keeper".into()],
    };
    write(
        &Path::new(&root).join("config/nsq/court_seed.json"),
        &serde_json::to_string_pretty(&nsq_seed).unwrap(),
    );

    let mut court = BTreeMap::new();
    for seat in &c.seats {
        court.insert(
            seat.id.clone(),
            NsqCourtRole {
                title: seat.title.clone(),
                authority: true,
                domain: {
                    let mut d = seat.authority_domain.clone();
                    d.extend(seat.advisory_domain.clone());
                    d
                },
            },
        );
    }

    let mut hound_classes = BTreeMap::new();
    let mut purpose = Vec::new();
    for h in &c.hounds {
        purpose.extend(h.advisory_domain.clone());
        hound_classes.insert(
            h.id.clone(),
            HoundClassOut {
                domain: h.advisory_domain.clone(),
            },
        );
    }

    let nsq_cfg = NsqCourtConfig {
        court,
        hounds: HoundsOut {
            authority: false,
            call_phrase: "call the hounds".into(),
            purpose,
            classes: hound_classes,
            escalation: vec![
                "hounds pursue".into(),
                "detective interprets".into(),
                "manager/director route response".into(),
                "jack breaks deadlock".into(),
                "ace may trump".into(),
                "keeper cleans final aftermath".into(),
            ],
        },
        rules: c.laws.clone(),
        capitals: c.capitals.clone(),
        seize_all_caps_flow: StepFlow {
            steps: vec![
                "identify_capitals".into(),
                "guard_seize_capitals".into(),
                "detective_verify".into(),
                "seer_reveal_hidden_pattern".into(),
                "oracle_project_consequence".into(),
                "crier_publish_state".into(),
                "keeper_finalize".into(),
            ],
        },
        arrest_flow: ArrestFlow {
            steps: vec![
                "guard_seize".into(),
                "ticketmaster_open_custody".into(),
                "detective_review".into(),
                "jack_review_if_contested".into(),
                "ace_override_if_needed".into(),
                "keeper_finalize".into(),
            ],
            required_fields: vec![
                "target_identity".into(),
                "cause".into(),
                "custodian".into(),
                "ticket_id".into(),
                "review_authority".into(),
                "release_path".into(),
            ],
        },
    };
    write(
        &Path::new(&root).join("config/nsq_court.json"),
        &serde_json::to_string_pretty(&nsq_cfg).unwrap(),
    );

    for ledger in &c.required_ledgers {
        let p = Path::new(&root).join(format!("runtime/kingdom/ledgers/{}.json", ledger));
        write(&p, "{ \"version\": 1, \"entries\": [] }\n");
    }

    println!("canonical={}", canonical_path.display());
    println!("generated=specs/court/COURT_CONSTITUTION.md");
    println!("generated=specs/nsq/court_of_archons.md");
    println!("generated=config/braxon_court.json");
    println!("generated=config/nsq/court_seed.json");
    println!("generated=config/nsq_court.json");
    println!("generated_ledgers={}", c.required_ledgers.len());
}

#[cfg(test)]
mod kingdom_surface_metrics_tests {
    use super::*;

    #[test]
    fn base8_metrics_are_not_flattened() {
        let c = Canonical {
            hooks: std::collections::BTreeMap::from([(
                "lexor".to_string(),
                vec!["court.open".to_string(), "court.route".to_string()],
            )]),
            seats: vec![Seat {
                id: "queen".to_string(),
                title: "Queen".to_string(),
                aliases: vec!["queen".to_string(), "linter".to_string()],
                authority_domain: vec!["validation".to_string()],
                advisory_domain: vec![],
                forbidden: vec![],

                class: String::new(),
            }],
            hounds: vec![Hound {
                id: "proof_hound".to_string(),
                title: "Proof Hound".to_string(),
                authority_domain: vec!["proof mismatch".to_string()],
                advisory_domain: vec!["inspect divergence".to_string()],

                class: String::new(),
            }],

            version: 0,
            kingdom: String::new(),
            classes: std::collections::BTreeMap::new(),
            escalation: Escalation {
                default_chain: vec![],
                keeper_final_if_ace_trumps_jack: false,
            },
            laws: std::collections::BTreeMap::new(),
            capitals: vec![],
            required_ledgers: vec![],
        };

        let v = build_kingdom_surface_metrics(&c);
        assert_eq!(v["flattened"], false);
        assert_eq!(v["hook_total_o8"], "2");
        assert_eq!(v["seat_alias_total_o8"], "2");
        assert_eq!(v["hound_authority_total_o8"], "1");
    }
}
