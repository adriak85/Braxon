use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::io;

#[derive(Serialize)]
struct Score {
    artifact_path: String,
    inspect_path: String,
    artifact_bytes: u64,
    decoded_bytes: u64,
    decoded_records: usize,
    unique_symbols: usize,
    structural_edges: usize,
    information_density: f64,
    decoded_bytes_per_artifact_byte: f64,
    replay_sha256: String,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("usage: nsq-proof <artifact.nsqb> <inspect.txt> <score.json>");
        std::process::exit(2);
    }

    let artifact_path = &args[1];
    let inspect_path = &args[2];
    let score_path = &args[3];

    let artifact = fs::read(artifact_path)?;
    let inspect = fs::read_to_string(inspect_path)?;

    let artifact_bytes = artifact.len() as u64;
    let decoded_bytes = inspect.len() as u64;

    let mut decoded_records = 0usize;
    let mut structural_edges = 0usize;
    let mut unique = BTreeSet::<String>::new();

    for line in inspect.lines() {
        if line.starts_with("noise ")
            || line.starts_with("triple ")
            || line.starts_with("membrane ")
        {
            decoded_records += 1;
        }
        if line.starts_with("triple ") {
            structural_edges += 1;
        }

        for part in line.split_whitespace() {
            if let Some(v) = part.strip_prefix("sym=") {
                unique.insert(v.to_string());
            }
            if let Some(v) = part.strip_prefix("subject=") {
                unique.insert(v.to_string());
            }
            if let Some(v) = part.strip_prefix("object=") {
                unique.insert(v.to_string());
            }
            if let Some(v) = part.strip_prefix("cell=") {
                unique.insert(v.to_string());
            }
            if let Some(v) = part.strip_prefix("state=") {
                unique.insert(v.to_string());
            }
        }
    }

    let mut hasher = Sha256::new();
    hasher.update(&artifact);
    let replay_sha256 = format!("{:x}", hasher.finalize());

    let score = Score {
        artifact_path: artifact_path.to_string(),
        inspect_path: inspect_path.to_string(),
        artifact_bytes,
        decoded_bytes,
        decoded_records,
        unique_symbols: unique.len(),
        structural_edges,
        information_density: if artifact_bytes == 0 {
            0.0
        } else {
            decoded_records as f64 / artifact_bytes as f64
        },
        decoded_bytes_per_artifact_byte: if artifact_bytes == 0 {
            0.0
        } else {
            decoded_bytes as f64 / artifact_bytes as f64
        },
        replay_sha256,
    };

    fs::write(score_path, serde_json::to_string_pretty(&score).unwrap())?;
    println!("{}", serde_json::to_string_pretty(&score).unwrap());
    Ok(())
}

#[allow(dead_code)]
mod native_wiring;
