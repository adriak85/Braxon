//! COURT PROJECTION / SEED READER
//! This surface currently reads configured court seeds and emits reports.
//! It should not be mistaken for the final primitive/base-native court operation path.

use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::process::exit;

#[derive(Deserialize)]
struct CourtConfig {
    offices: BTreeMap<String, Office>,
    escalation: Escalation,
    laws: Laws,
    required_ledgers: Vec<String>,
}

#[derive(Deserialize)]
struct Office {
    title: String,
    class: String,
    authority_domain: Vec<String>,
}

#[derive(Deserialize)]
struct Escalation {
    default_chain: Vec<String>,
    keeper_final_if_ace_trumps_jack: bool,
}

#[derive(Deserialize)]
struct Laws {
    promoted_agents_have_authority: bool,
    pages_persist_but_do_not_rule: bool,
    court_seats_crash_guarded: bool,
    only_disposable_agents_may_die_without_inheritance: bool,
}

fn main() {
    let path = "config/braxon_court.json";
    let raw = fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("read error {path}: {e}");
        exit(2);
    });

    let cfg: CourtConfig = serde_json::from_str(&raw).unwrap_or_else(|e| {
        eprintln!("json error {path}: {e}");
        exit(2);
    });

    println!("court_config={path}");
    println!("offices={}", cfg.offices.len());
    println!("ledgers={}", cfg.required_ledgers.len());
    println!(
        "escalation_chain={}",
        cfg.escalation.default_chain.join(" -> ")
    );
    println!(
        "keeper_final={}",
        cfg.escalation.keeper_final_if_ace_trumps_jack
    );
    println!(
        "pages_persist_but_do_not_rule={}",
        cfg.laws.pages_persist_but_do_not_rule
    );
    println!(
        "promoted_agents_have_authority={}",
        cfg.laws.promoted_agents_have_authority
    );
    println!(
        "court_seats_crash_guarded={}",
        cfg.laws.court_seats_crash_guarded
    );
    println!(
        "only_disposable_agents_may_die_without_inheritance={}",
        cfg.laws.only_disposable_agents_may_die_without_inheritance
    );

    for (id, office) in cfg.offices {
        println!(
            "office={} title={} class={} domains={}",
            id,
            office.title,
            office.class,
            office.authority_domain.join("|")
        );
    }
}

#[allow(dead_code)]
mod native_wiring;
