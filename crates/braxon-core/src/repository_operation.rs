use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub const REPOSITORY_OPERATION_SCHEMA: &str = "braxon.nsq.repository_operation.v1";
const REPOSITORY_MANIFEST_RELATIVE_PATH: &str =
    "config/toolchains/extended_repository_integration_manifest.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RepositoryOperationReport {
    pub schema: String,
    pub repository_id: String,
    pub source_url: String,
    pub revision: String,
    pub visibility: String,
    pub license_class: String,
    pub android_disposition: String,
    pub nsq_capability: String,
    pub source_edge_path: String,
    pub source_edge_present: bool,
    pub revision_matches: bool,
    pub legal_build_authorized: bool,
    pub target_build_proven: bool,
    pub operation_ready: bool,
    pub exact_next_action: String,
}

#[derive(Debug, Deserialize)]
struct RepositoryManifest {
    schema: String,
    repository_total: usize,
    #[serde(default)]
    repositories: Vec<RepositoryRecord>,
}

#[derive(Debug, Deserialize)]
struct RepositoryRecord {
    id: String,
    source_url: String,
    revision: String,
    visibility: String,
    license_class: String,
    android_disposition: String,
    nsq_capability: String,
    materialization: String,
    validation: String,
}

pub fn evaluate_repository_operation(
    start: impl AsRef<Path>,
    repository_id: &str,
) -> Result<RepositoryOperationReport, String> {
    let root = resolve_root(start)?;
    let manifest: RepositoryManifest = read_json(&root.join(REPOSITORY_MANIFEST_RELATIVE_PATH))?;
    if manifest.schema != "braxon.toolchain.extended_repository_integration.v1"
        || manifest.repository_total != manifest.repositories.len()
    {
        return Err("invalid extended repository integration manifest".to_string());
    }
    let record = manifest
        .repositories
        .into_iter()
        .find(|record| record.id == repository_id)
        .ok_or_else(|| format!("repository '{repository_id}' is not declared"))?;
    if record.nsq_capability != format!("repository:{}", record.id) {
        return Err(format!(
            "repository '{}' has a noncanonical NSQ capability",
            record.id
        ));
    }
    let source_edge_path = source_edge_path(&root, &record.id);
    let source_edge_present = if record.id == "braxon" {
        root.join("Cargo.toml").is_file()
    } else {
        source_edge_path.join(".git").is_dir()
    };
    let revision_matches = if record.id == "braxon" {
        root.join(".git").is_dir()
    } else {
        source_edge_present && git_head_matches(&source_edge_path, &record.revision)
    };
    let legal_build_authorized = matches!(
        record.id.as_str(),
        "braxon" | "dax_autonomous_system" | "termux_packages"
    );
    let target_build_proven = false;
    let operation_ready =
        source_edge_present && revision_matches && legal_build_authorized && target_build_proven;
    let exact_next_action = if record.id == "braxon" {
        "Run the Braxon offline workspace and closure gates; Android-native toolchain promotion remains capacity and target-probe gated.".to_string()
    } else if !legal_build_authorized {
        format!(
            "Build is blocked by the recorded legal boundary. Required source handling is '{}'. Satisfy this condition before any execution: {}",
            record.materialization,
            record.validation
        )
    } else if !source_edge_present {
        format!(
            "Acquire the pinned source edge without executing it: scripts/braxon_repository_lane.sh acquire {}. Declared source handling: {}",
            record.id,
            record.materialization
        )
    } else if !revision_matches {
        format!(
            "Reject the source edge because its revision differs from {}; reacquire it with scripts/braxon_repository_lane.sh acquire {}.",
            record.revision, record.id
        )
    } else {
        format!(
            "Pinned source edge is present and eligible for controlled preparation. Run scripts/braxon_repository_lane.sh prepare {} and then create a source-only Android build receipt; no target build proof exists yet.",
            record.id
        )
    };
    Ok(RepositoryOperationReport {
        schema: REPOSITORY_OPERATION_SCHEMA.to_string(),
        repository_id: record.id,
        source_url: record.source_url,
        revision: record.revision,
        visibility: record.visibility,
        license_class: record.license_class,
        android_disposition: record.android_disposition,
        nsq_capability: record.nsq_capability,
        source_edge_path: source_edge_path.display().to_string(),
        source_edge_present,
        revision_matches,
        legal_build_authorized,
        target_build_proven,
        operation_ready,
        exact_next_action,
    })
}

fn resolve_root(start: impl AsRef<Path>) -> Result<PathBuf, String> {
    let canonical = start
        .as_ref()
        .canonicalize()
        .map_err(|error| format!("unable to resolve repository operation start: {error}"))?;
    canonical
        .ancestors()
        .find(|candidate| candidate.join(REPOSITORY_MANIFEST_RELATIVE_PATH).is_file())
        .map(Path::to_path_buf)
        .ok_or_else(|| "unable to locate extended repository integration manifest".to_string())
}

fn source_edge_path(root: &Path, id: &str) -> PathBuf {
    env::var_os("BRAXON_REPOSITORY_EDGE_ROOT")
        .map(PathBuf::from)
        .or_else(|| {
            env::var_os("XDG_CACHE_HOME")
                .map(|value| PathBuf::from(value).join("braxon/repository_edges"))
        })
        .or_else(|| {
            env::var_os("HOME")
                .map(|value| PathBuf::from(value).join(".cache/braxon/repository_edges"))
        })
        .unwrap_or_else(|| root.join("state/repository_edges"))
        .join(id)
}

fn git_head_matches(edge: &Path, expected: &str) -> bool {
    fs::read_to_string(edge.join(".git/HEAD"))
        .ok()
        .map(|head| head.contains(expected))
        .unwrap_or(false)
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, String> {
    let raw = fs::read_to_string(path)
        .map_err(|error| format!("failed to read '{}': {error}", path.display()))?;
    serde_json::from_str(&raw)
        .map_err(|error| format!("failed to parse '{}': {error}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repository_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .expect("repository root")
    }

    #[test]
    fn primary_repository_is_an_addressable_nsq_operation_without_false_target_build_claim() {
        let report = evaluate_repository_operation(repository_root(), "braxon").unwrap();
        assert_eq!(report.nsq_capability, "repository:braxon");
        assert!(report.source_edge_present);
        assert!(report.legal_build_authorized);
        assert!(!report.target_build_proven);
        assert!(!report.operation_ready);
    }

    #[test]
    fn restricted_repository_remains_a_fail_closed_addressable_boundary() {
        let report = evaluate_repository_operation(repository_root(), "papi").unwrap();
        assert_eq!(report.nsq_capability, "repository:papi");
        assert!(!report.legal_build_authorized);
        assert!(!report.operation_ready);
        assert!(report.exact_next_action.contains("legal boundary"));
    }
}
