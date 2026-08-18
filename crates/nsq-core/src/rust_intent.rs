use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

pub const RUST_INTENT_SCHEMA: &str = "braxon.nsq.rust_intent.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RustSemanticRecord {
    pub schema: String,
    pub record_id: String,
    pub source_path: String,
    pub line: usize,
    pub surface_kind: String,
    pub symbol: String,
    pub visibility: String,
    pub semantic_intent: String,
    pub nsq_capability: String,
    pub source_hash: String,
}

pub fn extract_rust_intent(source_path: impl AsRef<Path>, source: &str) -> Vec<RustSemanticRecord> {
    let source_path = source_path.as_ref().display().to_string();
    let mut records = Vec::new();
    for (line_index, line) in source.lines().enumerate() {
        if let Some((surface_kind, symbol, visibility)) = parse_surface(line) {
            let line_number = line_index + 1;
            let semantic_intent = format!("rust.{surface_kind}.{symbol}");
            let nsq_capability = format!("native.{surface_kind}.{symbol}");
            let source_hash = stable_hash(line.trim());
            let record_id = stable_hash(&format!(
                "{source_path}:{line_number}:{surface_kind}:{symbol}"
            ));
            records.push(RustSemanticRecord {
                schema: RUST_INTENT_SCHEMA.into(),
                record_id,
                source_path: source_path.clone(),
                line: line_number,
                surface_kind: surface_kind.into(),
                symbol,
                visibility: visibility.into(),
                semantic_intent,
                nsq_capability,
                source_hash,
            });
        }
    }
    records
}

pub fn extract_rust_tree(root: impl AsRef<Path>) -> Result<Vec<RustSemanticRecord>, String> {
    let mut paths = Vec::new();
    collect_rust_files(root.as_ref(), &mut paths)?;
    paths.sort();
    let mut records = Vec::new();
    for path in paths {
        let source = fs::read_to_string(&path)
            .map_err(|error| format!("failed reading {}: {error}", path.display()))?;
        records.extend(extract_rust_intent(path, &source));
    }
    Ok(records)
}

fn collect_rust_files(root: &Path, paths: &mut Vec<PathBuf>) -> Result<(), String> {
    let metadata = fs::metadata(root)
        .map_err(|error| format!("failed stating {}: {error}", root.display()))?;
    if metadata.is_file() {
        if root.extension().and_then(|extension| extension.to_str()) == Some("rs") {
            paths.push(root.to_path_buf());
        }
        return Ok(());
    }
    let mut entries = fs::read_dir(root)
        .map_err(|error| format!("failed reading directory {}: {error}", root.display()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("failed enumerating {}: {error}", root.display()))?;
    entries.sort_by_key(|entry| entry.path());
    for entry in entries {
        let path = entry.path();
        if path.file_name().and_then(|name| name.to_str()) == Some(".git") {
            continue;
        }
        collect_rust_files(&path, paths)?;
    }
    Ok(())
}

fn parse_surface(line: &str) -> Option<(&'static str, String, &'static str)> {
    let trimmed = line.trim_start();
    let (visibility, rest) = if let Some(rest) = trimmed.strip_prefix("pub ") {
        ("pub", rest)
    } else {
        ("private", trimmed)
    };
    let rest = rest.strip_prefix("async ").unwrap_or(rest);
    for (keyword, kind) in [
        ("fn ", "function"),
        ("struct ", "struct"),
        ("enum ", "enum"),
        ("trait ", "trait"),
        ("const ", "constant"),
        ("static ", "static"),
        ("mod ", "module"),
        ("type ", "type"),
        ("impl ", "impl"),
    ] {
        if let Some(rest) = rest.strip_prefix(keyword) {
            let symbol = rest
                .split(|character: char| {
                    character.is_whitespace()
                        || matches!(character, '<' | '{' | '(' | ':' | '=' | ';')
                })
                .find(|token| !token.is_empty())?
                .to_string();
            return Some((kind, symbol, visibility));
        }
    }
    None
}

fn stable_hash(value: &str) -> String {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_all_supported_rust_surfaces_as_native_intent() {
        let source = "pub mod example;\npub struct State { value: i64 }\nimpl State {\n    pub fn step(&self) {}\n}\nfn hidden() {}\npub const LIMIT: usize = 10;";
        let records = extract_rust_intent("src/example.rs", source);
        assert_eq!(records.len(), 6);
        assert!(records
            .iter()
            .all(|record| record.schema == RUST_INTENT_SCHEMA));
        assert!(records
            .iter()
            .any(|record| record.nsq_capability == "native.function.step"));
        assert!(records
            .iter()
            .any(|record| record.semantic_intent == "rust.constant.LIMIT"));
    }

    #[test]
    fn record_identity_is_path_and_line_stable() {
        let a = extract_rust_intent("src/a.rs", "pub fn run() {}");
        let b = extract_rust_intent("src/a.rs", "pub fn run() {}\n");
        assert_eq!(a[0].record_id, b[0].record_id);
        assert_ne!(a[0].source_hash, stable_hash("pub fn other() {}"));
    }
}
