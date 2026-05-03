use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;

const NATIVE_MARKER: &[u8; 9] = b"NSQPACK01";

#[derive(Serialize, Deserialize)]
pub struct PackManifest {
    pub version: u32,
    pub artifacts: Vec<String>,
    pub native_marker: String,
    pub source_carrier_units: u64,
    pub payload_carrier_units: u64,
    pub artifact_carrier_units: u64,
}

pub fn pack_files(inputs: &[String], out_path: &str) -> io::Result<PackManifest> {
    let mut total = 0u64;
    let mut joined = Vec::<u8>::new();

    joined.extend_from_slice(NATIVE_MARKER);
    joined.push(b'\n');

    for item in inputs {
        let data = fs::read(item)?;
        total += data.len() as u64;
        joined.extend_from_slice(format!("FILE {}\n", item).as_bytes());
        joined.extend_from_slice(&data);
        joined.extend_from_slice(b"\n");
    }

    if let Some(parent) = Path::new(out_path).parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(out_path, &joined)?;

    Ok(PackManifest {
        version: 1,
        artifacts: inputs.to_vec(),
        native_marker: String::from_utf8_lossy(NATIVE_MARKER).into_owned(),
        source_carrier_units: total,
        payload_carrier_units: joined.len().saturating_sub(NATIVE_MARKER.len() + 1) as u64,
        artifact_carrier_units: joined.len() as u64,
    })
}
