use clap::{Parser, Subcommand};
use nsq_compress::{
    read_json, scan_repeats, verify_manifest, write_json, CompressionArch, ModelCompressor,
    PipelineManifest,
};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(author, version, about = "NSQ stamp/lever/delta scaffold pipeline")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Scan {
        #[arg(long, default_value = "nu336")]
        arch: CompressionArch,
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    Verify {
        #[arg(long)]
        manifest: PathBuf,
    },
    RepeatScan {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
        #[arg(long, default_value_t = 4096)]
        chunk_bytes: usize,
        #[arg(long, default_value_t = 2)]
        min_shared_chunks: usize,
    },
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan {
            arch,
            input,
            output,
        } => {
            let mut compressor = ModelCompressor::new();
            compressor
                .scan_root(&input)
                .map_err(|err| format!("scan failed: {err}"))?;
            let manifest = compressor.manifest(arch, &input);
            write_json(&output, &manifest)
                .map_err(|err| format!("write manifest failed: {err}"))?;
            println!("wrote manifest: {}", output.display());
        }
        Commands::Verify { manifest } => {
            let manifest: PipelineManifest =
                read_json(&manifest).map_err(|err| format!("read manifest failed: {err}"))?;
            let notes =
                verify_manifest(&manifest).map_err(|err| format!("verify failed: {err}"))?;
            if notes.is_empty() {
                println!("manifest verification passed");
            } else {
                for note in notes {
                    println!("verify_note={note}");
                }
            }
        }
        Commands::RepeatScan {
            input,
            output,
            chunk_bytes,
            min_shared_chunks,
        } => {
            let report = scan_repeats(&input, chunk_bytes, min_shared_chunks)
                .map_err(|err| format!("repeat scan failed: {err}"))?;
            write_json(&output, &report)
                .map_err(|err| format!("write repeat report failed: {err}"))?;
            println!("wrote repeat report: {}", output.display());
        }
    }

    Ok(())
}
