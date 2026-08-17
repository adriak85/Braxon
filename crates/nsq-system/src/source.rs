use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceKind {
    Rust,
    Manifest,
    Documentation,
    Script,
    Data,
    Generated,
    HistoricalBackup,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceArtifact {
    pub path: String,
    pub kind: SourceKind,
    pub bytes: u64,
    pub historical: bool,
    pub digest_hint: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SourceTree {
    pub root: String,
    pub artifacts: Vec<SourceArtifact>,
}

impl SourceTree {
    /// Walk the entire repository. Only .git internals are excluded to avoid
    /// traversing Git's object database. Dotfiles, generated files, build output,
    /// backups, and uncommon extensions remain visible as source artifacts.
    pub fn scan(root: impl AsRef<Path>) -> std::io::Result<Self> {
        let root = root.as_ref().canonicalize()?;
        let mut artifacts = Vec::new();
        Self::walk(&root, &root, &mut artifacts)?;
        artifacts.sort_by(|a, b| a.path.cmp(&b.path));
        Ok(Self { root: root.display().to_string(), artifacts })
    }

    fn walk(root: &Path, dir: &Path, out: &mut Vec<SourceArtifact>) -> std::io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            if path.is_dir() {
                if name == ".git" { continue; }
                Self::walk(root, &path, out)?;
                continue;
            }
            let metadata = entry.metadata()?;
            let rel = path.strip_prefix(root).unwrap_or(&path).display().to_string();
            let lower = rel.to_ascii_lowercase();
            let historical = lower.contains("before_") || lower.contains("backup") || lower.contains("archive") || lower.contains("old_");
            let kind = if historical { SourceKind::HistoricalBackup }
                else if lower.ends_with(".rs") { SourceKind::Rust }
                else if lower.ends_with(".toml") || lower.ends_with(".lock") { SourceKind::Manifest }
                else if lower.ends_with(".md") || lower.ends_with(".txt") { SourceKind::Documentation }
                else if lower.ends_with(".sh") || lower.ends_with(".bash") { SourceKind::Script }
                else if lower.ends_with(".json") || lower.ends_with(".csv") || lower.ends_with(".yaml") || lower.ends_with(".yml") { SourceKind::Data }
                else if lower.contains("generated") { SourceKind::Generated }
                else { SourceKind::Other };
            let digest_hint = match fs::read(&path) {
                Ok(bytes) => format!("len:{}", bytes.len()),
                Err(_) => "unreadable".to_string(),
            };
            out.push(SourceArtifact { path: rel, kind, bytes: metadata.len(), historical, digest_hint });
        }
        Ok(())
    }

    pub fn rust_files(&self) -> impl Iterator<Item = &SourceArtifact> { self.artifacts.iter().filter(|a| a.kind == SourceKind::Rust) }
    pub fn historical_files(&self) -> impl Iterator<Item = &SourceArtifact> { self.artifacts.iter().filter(|a| a.historical) }

    pub fn absolute_path(&self, artifact: &SourceArtifact) -> PathBuf {
        Path::new(&self.root).join(&artifact.path)
    }
}
