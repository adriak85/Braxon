use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sha2::{Digest as ShaDigest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CompressionArch {
    Nu256,
    Nu336,
    Nu369,
}

impl CompressionArch {
    pub fn metadata_headroom_percent(self) -> u8 {
        match self {
            Self::Nu256 => 20,
            Self::Nu336 => 35,
            Self::Nu369 => 50,
        }
    }
}

impl std::str::FromStr for CompressionArch {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "nu256" => Ok(Self::Nu256),
            "nu336" => Ok(Self::Nu336),
            "nu369" => Ok(Self::Nu369),
            other => Err(format!("unsupported architecture: {other}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StampRecord {
    pub stamp_id: String,
    pub source_path: String,
    pub bytes: u64,
    pub blake3: String,
    pub sha256: String,
    pub topology_class: String,
    pub integrity_witness: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SectionEncoding {
    pub source_path: String,
    pub stamp_reference: String,
    pub local_delta: String,
    pub exception_mask: String,
    pub integrity_witness: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PipelineManifest {
    pub architecture: CompressionArch,
    pub generated_by: String,
    pub root: String,
    pub stamp_records: Vec<StampRecord>,
    pub section_encodings: Vec<SectionEncoding>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RepeatGroup {
    pub bytes: u64,
    pub blake3: String,
    pub sha256: String,
    pub paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SeamCandidate {
    pub left_path: String,
    pub right_path: String,
    pub shared_chunks: u64,
    pub shared_bytes: u64,
    pub left_total_chunks: u64,
    pub right_total_chunks: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RepeatScanReport {
    pub root: String,
    pub chunk_bytes: usize,
    pub exact_repeat_groups: Vec<RepeatGroup>,
    pub seam_candidates: Vec<SeamCandidate>,
}

#[derive(Debug, Default)]
pub struct ModelCompressor {
    stamp_records: Vec<StampRecord>,
    section_encodings: Vec<SectionEncoding>,
}

impl ModelCompressor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn scan_root(&mut self, root: &Path) -> io::Result<()> {
        let mut files = Vec::new();
        collect_files(root, &mut files)?;
        files.sort();

        for path in files {
            let bytes = fs::read(&path)?;
            let blake3 = blake3_hex(&bytes);
            let sha256 = sha256_hex(&bytes);
            let topology_class = classify_path(&path);
            let stamp_id = format!("stamp:{blake3}");
            let source_path = path.to_string_lossy().to_string();
            let integrity_witness = format!("sha256:{sha256}");

            self.stamp_records.push(StampRecord {
                stamp_id: stamp_id.clone(),
                source_path: source_path.clone(),
                bytes: bytes.len() as u64,
                blake3,
                sha256,
                topology_class,
                integrity_witness: integrity_witness.clone(),
            });

            self.section_encodings.push(SectionEncoding {
                source_path,
                stamp_reference: stamp_id,
                local_delta: "delta:pending_exact_repeat_and_transform_law".to_string(),
                exception_mask: "mask:pending".to_string(),
                integrity_witness,
            });
        }

        Ok(())
    }

    pub fn manifest(self, arch: CompressionArch, root: &Path) -> PipelineManifest {
        PipelineManifest {
            architecture: arch,
            generated_by: "nsq-compress scaffold".to_string(),
            root: root.to_string_lossy().to_string(),
            stamp_records: self.stamp_records,
            section_encodings: self.section_encodings,
        }
    }
}

pub fn write_json<T: Serialize>(path: &Path, value: &T) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let body = serde_json::to_vec_pretty(value).map_err(|err| io::Error::other(err.to_string()))?;
    fs::write(path, body)
}

pub fn read_json<T: DeserializeOwned>(path: &Path) -> io::Result<T> {
    let body = fs::read(path)?;
    serde_json::from_slice(&body).map_err(|err| io::Error::other(err.to_string()))
}

pub fn verify_manifest(manifest: &PipelineManifest) -> io::Result<Vec<String>> {
    let mut notes = Vec::new();
    let mut stamp_ids = BTreeMap::<String, String>::new();

    for record in &manifest.stamp_records {
        if let Some(existing) =
            stamp_ids.insert(record.stamp_id.clone(), record.source_path.clone())
        {
            notes.push(format!(
                "duplicate stamp id {} for {} and {}",
                record.stamp_id, existing, record.source_path
            ));
        }
        if !record.integrity_witness.starts_with("sha256:") {
            notes.push(format!(
                "integrity witness missing sha256 prefix for {}",
                record.source_path
            ));
        }
    }

    Ok(notes)
}

pub fn scan_repeats(
    root: &Path,
    chunk_bytes: usize,
    min_shared_chunks: usize,
) -> io::Result<RepeatScanReport> {
    if chunk_bytes == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "chunk_bytes must be greater than zero",
        ));
    }

    let mut files = Vec::new();
    collect_files(root, &mut files)?;
    files.sort();

    let mut exact_map = BTreeMap::<(u64, String, String), Vec<String>>::new();
    let mut chunk_sets = Vec::<(String, BTreeSet<String>)>::new();

    for path in files {
        let bytes = fs::read(&path)?;
        let source_path = path.to_string_lossy().to_string();
        let blake3 = blake3_hex(&bytes);
        let sha256 = sha256_hex(&bytes);

        exact_map
            .entry((bytes.len() as u64, blake3.clone(), sha256.clone()))
            .or_default()
            .push(source_path.clone());

        if is_seam_candidate_file(&path) && !bytes.is_empty() {
            let mut set = BTreeSet::new();
            for chunk in bytes.chunks(chunk_bytes) {
                set.insert(blake3_hex(chunk));
            }
            chunk_sets.push((source_path, set));
        }
    }

    let mut exact_repeat_groups = exact_map
        .into_iter()
        .filter_map(|((bytes, blake3, sha256), mut paths)| {
            if paths.len() < 2 {
                return None;
            }
            paths.sort();
            Some(RepeatGroup {
                bytes,
                blake3,
                sha256,
                paths,
            })
        })
        .collect::<Vec<_>>();

    exact_repeat_groups.sort_by(|a, b| {
        b.paths
            .len()
            .cmp(&a.paths.len())
            .then_with(|| b.bytes.cmp(&a.bytes))
            .then_with(|| a.blake3.cmp(&b.blake3))
    });

    let mut seam_candidates = Vec::<SeamCandidate>::new();

    for left_idx in 0..chunk_sets.len() {
        for right_idx in (left_idx + 1)..chunk_sets.len() {
            let (left_path, left_chunks) = &chunk_sets[left_idx];
            let (right_path, right_chunks) = &chunk_sets[right_idx];

            let shared = left_chunks.intersection(right_chunks).count();
            if shared < min_shared_chunks {
                continue;
            }

            seam_candidates.push(SeamCandidate {
                left_path: left_path.clone(),
                right_path: right_path.clone(),
                shared_chunks: shared as u64,
                shared_bytes: (shared * chunk_bytes) as u64,
                left_total_chunks: left_chunks.len() as u64,
                right_total_chunks: right_chunks.len() as u64,
            });
        }
    }

    seam_candidates.sort_by(|a, b| {
        b.shared_chunks
            .cmp(&a.shared_chunks)
            .then_with(|| b.shared_bytes.cmp(&a.shared_bytes))
            .then_with(|| a.left_path.cmp(&b.left_path))
            .then_with(|| a.right_path.cmp(&b.right_path))
    });

    Ok(RepeatScanReport {
        root: root.to_string_lossy().to_string(),
        chunk_bytes,
        exact_repeat_groups,
        seam_candidates,
    })
}

fn collect_files(root: &Path, out: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();

        if should_skip_dir_name(&name) && entry.file_type()?.is_dir() {
            continue;
        }

        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            collect_files(&path, out)?;
        } else if file_type.is_file() {
            out.push(path);
        }
    }
    Ok(())
}

fn should_skip_dir_name(name: &str) -> bool {
    matches!(
        name,
        ".git" | "target" | "state" | "rustsec-advisory-db" | "node_modules"
    )
}

fn classify_path(path: &Path) -> String {
    let lower = path.to_string_lossy().to_ascii_lowercase();
    if lower.contains("tokenizer") {
        "tokenizer_surface".to_string()
    } else if lower.contains("weights") || lower.contains("model") {
        "weight_surface".to_string()
    } else if lower.ends_with(".json") {
        "json_surface".to_string()
    } else if lower.ends_with(".md") {
        "document_surface".to_string()
    } else {
        "generic_surface".to_string()
    }
}

fn is_seam_candidate_file(path: &Path) -> bool {
    let lower = path.to_string_lossy().to_ascii_lowercase();
    lower.ends_with(".md")
        || lower.ends_with(".txt")
        || lower.ends_with(".json")
        || lower.ends_with(".tsv")
        || lower.ends_with(".rs")
        || lower.ends_with(".toml")
        || lower.ends_with(".yaml")
        || lower.ends_with(".yml")
}

fn blake3_hex(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arch_headroom_is_stable() {
        assert_eq!(CompressionArch::Nu256.metadata_headroom_percent(), 20);
        assert_eq!(CompressionArch::Nu336.metadata_headroom_percent(), 35);
        assert_eq!(CompressionArch::Nu369.metadata_headroom_percent(), 50);
    }

    #[test]
    fn manifest_verifier_flags_duplicate_stamp_ids() {
        let manifest = PipelineManifest {
            architecture: CompressionArch::Nu336,
            generated_by: "test".into(),
            root: "/tmp".into(),
            stamp_records: vec![
                StampRecord {
                    stamp_id: "stamp:a".into(),
                    source_path: "a".into(),
                    bytes: 1,
                    blake3: "a".into(),
                    sha256: "a".into(),
                    topology_class: "generic_surface".into(),
                    integrity_witness: "sha256:a".into(),
                },
                StampRecord {
                    stamp_id: "stamp:a".into(),
                    source_path: "b".into(),
                    bytes: 1,
                    blake3: "b".into(),
                    sha256: "b".into(),
                    topology_class: "generic_surface".into(),
                    integrity_witness: "sha256:b".into(),
                },
            ],
            section_encodings: vec![],
        };

        let notes = verify_manifest(&manifest).unwrap();
        assert!(notes.iter().any(|note| note.contains("duplicate stamp id")));
    }
}
