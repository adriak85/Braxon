use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Read};
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
        Ok(Self {
            root: root.display().to_string(),
            artifacts,
        })
    }

    fn walk(root: &Path, dir: &Path, out: &mut Vec<SourceArtifact>) -> std::io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            if path.is_dir() {
                if name == ".git" {
                    continue;
                }
                Self::walk(root, &path, out)?;
                continue;
            }
            let metadata = entry.metadata()?;
            let rel = path
                .strip_prefix(root)
                .unwrap_or(&path)
                .display()
                .to_string();
            let lower = rel.to_ascii_lowercase();
            let historical = lower.contains("before_")
                || lower.contains("backup")
                || lower.contains("archive")
                || lower.contains("old_");
            let kind = if historical {
                SourceKind::HistoricalBackup
            } else if lower.ends_with(".rs") {
                SourceKind::Rust
            } else if lower.ends_with(".toml") || lower.ends_with(".lock") {
                SourceKind::Manifest
            } else if lower.ends_with(".md") || lower.ends_with(".txt") {
                SourceKind::Documentation
            } else if lower.ends_with(".sh") || lower.ends_with(".bash") {
                SourceKind::Script
            } else if lower.ends_with(".json")
                || lower.ends_with(".csv")
                || lower.ends_with(".yaml")
                || lower.ends_with(".yml")
            {
                SourceKind::Data
            } else if lower.contains("generated") {
                SourceKind::Generated
            } else {
                SourceKind::Other
            };
            let digest_hint = digest_hint(&path)?;
            out.push(SourceArtifact {
                path: rel,
                kind,
                bytes: metadata.len(),
                historical,
                digest_hint,
            });
        }
        Ok(())
    }

    pub fn rust_files(&self) -> impl Iterator<Item = &SourceArtifact> {
        self.artifacts.iter().filter(|a| a.kind == SourceKind::Rust)
    }
    pub fn historical_files(&self) -> impl Iterator<Item = &SourceArtifact> {
        self.artifacts.iter().filter(|a| a.historical)
    }

    pub fn absolute_path(&self, artifact: &SourceArtifact) -> PathBuf {
        Path::new(&self.root).join(&artifact.path)
    }
}

fn digest_hint(path: &Path) -> io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut buffer = [0_u8; 64 * 1024];
    let mut total = 0_u64;
    let mut hash = 0xcbf29ce484222325_u64;
    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        total = total.saturating_add(read as u64);
        for byte in &buffer[..read] {
            hash ^= *byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
    }
    Ok(format!("len:{total}:fnv1a:{hash:016x}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn scan_includes_hidden_generated_backup_and_binary_files_but_excludes_only_git_internals() {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!("braxon-source-tree-{suffix}"));
        fs::create_dir_all(root.join(".git/objects")).unwrap();
        fs::create_dir_all(root.join("target/debug")).unwrap();
        fs::create_dir_all(root.join("nested/.hidden")).unwrap();
        fs::write(root.join(".env"), b"hidden").unwrap();
        fs::write(root.join("target/debug/generated.bin"), [1_u8, 2, 3]).unwrap();
        fs::write(root.join("nested/.hidden/backup.before_test"), b"backup").unwrap();
        fs::write(root.join(".git/objects/ignored"), b"git internals").unwrap();

        let tree = SourceTree::scan(&root).unwrap();
        let paths: Vec<&str> = tree
            .artifacts
            .iter()
            .map(|artifact| artifact.path.as_str())
            .collect();
        assert!(paths.contains(&".env"));
        assert!(paths.contains(&"target/debug/generated.bin"));
        assert!(paths.contains(&"nested/.hidden/backup.before_test"));
        assert!(!paths.iter().any(|path| path.starts_with(".git/")));
        assert!(tree
            .artifacts
            .iter()
            .all(|artifact| !artifact.digest_hint.contains("unreadable")));
        fs::remove_dir_all(root).unwrap();
    }
}
