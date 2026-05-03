use serde::Serialize;
use std::path::Path;

#[derive(Serialize)]
struct Check {
    name: String,
    ok: bool,
    note: String,
}

#[derive(Serialize)]
struct Showdown {
    ready: bool,
    checks: Vec<Check>,
}

fn main() {
    let root = std::env::var("BRAXON_HOME")
        .unwrap_or_else(|_| format!("{}/Braxon", std::env::var("HOME").unwrap()));
    let rootp = Path::new(&root);

    let checks = vec![
        (
            "canonical court",
            rootp.join("config/kingdom/court_canonical.json"),
        ),
        (
            "constitution",
            rootp.join("specs/court/COURT_CONSTITUTION.md"),
        ),
        (
            "nsq court spec",
            rootp.join("specs/nsq/court_of_archons.md"),
        ),
        (
            "Braxon court config",
            rootp.join("config/braxon_court.json"),
        ),
        ("nsq court config", rootp.join("config/nsq_court.json")),
        ("nsq court seed", rootp.join("config/nsq/court_seed.json")),
        (
            "identity ledger",
            rootp.join("runtime/kingdom/ledgers/identity_ledger.json"),
        ),
        (
            "ticket ledger",
            rootp.join("runtime/kingdom/ledgers/ticket_ledger.json"),
        ),
        (
            "recovery ledger",
            rootp.join("runtime/kingdom/ledgers/recovery_ledger.json"),
        ),
        (
            "capital ledger",
            rootp.join("runtime/kingdom/ledgers/capital_ledger.json"),
        ),
        (
            "hound ledger",
            rootp.join("runtime/kingdom/ledgers/hound_ledger.json"),
        ),
    ];

    let mut out = Vec::new();
    for (name, path) in checks {
        let ok = path.exists();
        out.push(Check {
            name: name.to_string(),
            ok,
            note: path.display().to_string(),
        });
    }

    let ready = out.iter().all(|c| c.ok);
    let showdown = Showdown { ready, checks: out };
    println!("{}", serde_json::to_string_pretty(&showdown).unwrap());

    if !ready {
        std::process::exit(1);
    }
}
