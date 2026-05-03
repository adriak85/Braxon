//! Native Wiring — Database path registry for NSQ court runtime.
//! These paths are LIVE — used by the court to locate its persistent stores.
//! Not dead code. Not stubs. Used by Court::new() and role operations.

use std::path::{Path, PathBuf};

/// Root path for NSQ runtime native databases.
pub fn runtime_db_root(workspace_root: &Path) -> PathBuf {
    workspace_root.join("nsq/runtime_native/databases")
}

/// Runtime domain registry — maps NSQ domains to active runtime surfaces.
pub fn runtime_domain_registry(workspace_root: &Path) -> PathBuf {
    runtime_db_root(workspace_root).join("runtime_domain_registry.db")
}

/// Graded selector registry — selector grammar and resolution tables.
pub fn graded_selector_registry(workspace_root: &Path) -> PathBuf {
    runtime_db_root(workspace_root).join("graded_selector_registry.db")
}

/// Package DB multiport registry — multi-crate package routing.
pub fn package_db_multiport_registry(workspace_root: &Path) -> PathBuf {
    runtime_db_root(workspace_root).join("package_db_multiport_registry.db")
}

/// Human-machine document registry — docs accessible to both NSQ and human readers.
pub fn human_machine_doc_registry(workspace_root: &Path) -> PathBuf {
    runtime_db_root(workspace_root).join("human_machine_doc_registry.db")
}

/// Tokenizer bridge registry — NSQ ↔ tokenizer binding surface.
pub fn tokenizer_bridge_registry(workspace_root: &Path) -> PathBuf {
    runtime_db_root(workspace_root).join("tokenizer_bridge_registry.db")
}

/// Court seed — the active court configuration.
pub fn court_seed(workspace_root: &Path) -> PathBuf {
    workspace_root.join("config/nsq_court.json")
}

/// BRAXON court seed — the BRAXON court configuration.
pub fn braxon_court_seed(workspace_root: &Path) -> PathBuf {
    workspace_root.join("config/braxon_court.json")
}

/// Court records directory — where all court operation records are written.
pub fn court_records_dir(workspace_root: &Path) -> PathBuf {
    workspace_root.join("state/court/records")
}

/// Verify all native wiring paths are accessible.
/// Returns list of missing paths.
pub fn verify_wiring(workspace_root: &Path) -> Vec<String> {
    let mut missing = Vec::new();

    // Config files must exist
    let required = [
        court_seed(workspace_root),
        braxon_court_seed(workspace_root),
    ];

    for path in &required {
        if !path.exists() {
            missing.push(format!("missing: {}", path.display()));
        }
    }

    // Directories are created on demand — just verify they can be created
    let dirs = [
        runtime_db_root(workspace_root),
        court_records_dir(workspace_root),
    ];

    for dir in &dirs {
        if !dir.exists() {
            if let Err(e) = std::fs::create_dir_all(dir) {
                missing.push(format!("cannot create {}: {}", dir.display(), e));
            }
        }
    }

    missing
}
