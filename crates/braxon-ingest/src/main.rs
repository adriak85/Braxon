use clap::{Parser, Subcommand};
use BRAXON_ingest::{BRAXON_ingest_status, workspace_root};

#[derive(Parser, Debug)]
#[command(name = "Braxon-ingest")]
#[command(version)]
#[command(about = "BRAXON chunk-governed model ingress surface")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    Status,
    Json,
}

fn main() {
    let cli = Cli::parse();
    let root = workspace_root();
    let status = BRAXON_ingest_status(&root);

    match cli.command.unwrap_or(Command::Status) {
        Command::Status => print_status(&status),
        Command::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&status)
                    .expect("serialize Braxon-ingest status to json")
            );
        }
    }
}

fn print_status(status: &BRAXON_ingest::BRAXONIngestStatus) {
    println!("target_lineage={}", status.target_lineage);
    println!("canonical_semantics={}", status.canonical_semantics);
    println!("target_source_variant_gb={}", status.target_source_variant_gb);
    println!("nsq_storage_target_gb={}", status.nsq_storage_target_gb);
    println!("nsq_hot_memory_target_gb={}", status.nsq_hot_memory_target_gb);
    println!(
        "nsq_hot_residency_surface={}",
        status.nsq_hot_residency_surface
    );
    println!("active_source_lane={}", status.active_source_lane);
    println!("active_source_state={}", status.active_source_state);
    println!("active_source_family={}", status.active_source_family);
    println!(
        "target_lineage_bound_to_active_source={}",
        status.target_lineage_bound_to_active_source
    );
    println!("visible_source_host_bytes={}", status.visible_source_host_bytes);
    println!(
        "visible_source_within_chunk_window={}",
        status.visible_source_within_chunk_window
    );
    println!("max_chunk_size_gb={}", status.max_chunk_size_gb);
    println!("max_live_downloads={}", status.max_live_downloads);
    println!(
        "current_materialized_shards={}",
        status.current_materialized_shards
    );
    println!("required_shards={}", status.required_shards);
    println!("pointer_shards={}", status.pointer_shards);
    println!(
        "direct_source_path_ready={}",
        status.direct_source_path_ready
    );
    println!(
        "runtime_authority_bound={}",
        status.runtime_authority_bound
    );
    println!("next_chunk_allowed={}", status.next_chunk_allowed);
    println!("target_manifest_bound={}", status.target_manifest_bound);
    println!("target_manifest_state={}", status.target_manifest_state);
    println!("next_action={}", status.next_action);
}
