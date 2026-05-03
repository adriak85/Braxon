use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

const NATIVE_MARKER: &[u8; 9] = b"NSQPACK01";

#[derive(Serialize, Deserialize)]
pub struct InspectReport {
    pub path: String,
    pub native_marker: String,
    pub marker_ok: bool,
    pub artifact_carrier_units: usize,
    pub payload_carrier_units: usize,
}

pub fn inspect_file(path: &str) -> io::Result<InspectReport> {
    let data = fs::read(path)?;
    let marker_ok =
        data.len() >= NATIVE_MARKER.len() && &data[..NATIVE_MARKER.len()] == NATIVE_MARKER;
    let payload_carrier_units = if marker_ok && data.len() > NATIVE_MARKER.len() {
        data.len().saturating_sub(NATIVE_MARKER.len() + 1)
    } else {
        data.len()
    };

    Ok(InspectReport {
        path: path.to_string(),
        native_marker: String::from_utf8_lossy(NATIVE_MARKER).into_owned(),
        marker_ok,
        artifact_carrier_units: data.len(),
        payload_carrier_units,
    })
}
