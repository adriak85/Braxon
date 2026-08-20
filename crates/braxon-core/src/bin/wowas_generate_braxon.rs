use std::env;
use std::fs;
use std::path::PathBuf;
use BRAXON_core::{prepare_generation_run, WowasRealization};

fn main() {
    if let Err(error) = run() {
        eprintln!("wowas_generate_braxon: ERROR: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let root = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    let manifest_path = root.join("config/wowas/ordered_stretched_spine_manifest.json");
    let raw = fs::read_to_string(&manifest_path)
        .map_err(|error| format!("cannot read {}: {error}", manifest_path.display()))?;
    let realization = WowasRealization::from_ordered_manifest(&raw)?;
    let run = prepare_generation_run(&realization, &root)?;
    let output = root.join("state/wowas/generation_run.json");
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("cannot create {}: {error}", parent.display()))?;
    }
    fs::write(
        &output,
        serde_json::to_string_pretty(&run)
            .map_err(|error| format!("cannot serialize generation run: {error}"))?,
    )
    .map_err(|error| format!("cannot write {}: {error}", output.display()))?;
    println!("schema={}", run.schema);
    println!("requests={}", run.request_count);
    println!("request_hash={}", run.request_hash);
    match run.readiness {
        BRAXON_core::WowasGenerationReadiness::Ready { seated_poles } => {
            println!("readiness=ready seated_poles={seated_poles}")
        }
        BRAXON_core::WowasGenerationReadiness::Blocked { reasons } => {
            println!("readiness=blocked");
            for reason in reasons {
                println!("blocked_reason={reason}");
            }
            println!("request_plan={}", output.display());
            return Err(
                "real model lane is not ready; request plan persisted but no prose was generated"
                    .into(),
            );
        }
    }
    Ok(())
}
