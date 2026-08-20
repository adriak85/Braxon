use crate::{
    load_braxon_chain_root_db, load_braxon_context_manifest, NativeNsqStack, SemanticLinkRequest,
    SemanticLinkSurface, TokenizerBridge,
};
use nsq_core::{Charge, Dialect, NSQLever, NSQSlot, NsqAddress};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub const LANGUAGE_ARTIFACT_CONTEXT_SCHEMA: &str = "braxon.nsq.language_artifact_context.v1";
pub const LANGUAGE_ARTIFACT_INDEX_RELATIVE_PATH: &str = "config/nsq/language_artifact_index.json";

#[derive(Debug, Clone, Deserialize)]
struct LanguageArtifactIndex {
    schema: String,
    documentation_path: String,
    records: Vec<LanguageArtifactRecord>,
}

#[derive(Debug, Clone, Deserialize)]
struct LanguageArtifactRecord {
    id: String,
    pointer_id: String,
    symbol: String,
    source_path: String,
    ast_identity: String,
    token: String,
    capability: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LanguageArtifactTraversal {
    pub id: String,
    pub pointer_id: String,
    pub symbol: String,
    pub source_path: String,
    pub ast_identity: String,
    pub documentation_path: String,
    pub token: String,
    pub native_token_id: Option<u64>,
    pub universal_token_id: Option<u64>,
    pub nsq_address: Option<String>,
    pub capability: String,
    pub source_symbol_resolved: bool,
    pub ast_identity_resolved: bool,
    pub documentation_link_resolved: bool,
    pub canonical_chain_address_resolved: bool,
    pub token_resolved: bool,
    pub capability_resolved: bool,
    pub runtime_lookup_accepted: bool,
    pub released: bool,
    pub resident_client_bytes: u64,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LanguageArtifactContextReport {
    pub schema: String,
    pub record_total: usize,
    pub verified_total: usize,
    pub all_passed: bool,
    pub traversals: Vec<LanguageArtifactTraversal>,
}

pub fn verify_language_artifact_context(
    root: &Path,
) -> Result<LanguageArtifactContextReport, String> {
    let index: LanguageArtifactIndex = read_json(root, LANGUAGE_ARTIFACT_INDEX_RELATIVE_PATH)?;
    if index.schema != "braxon.nsq.language_artifact_index.v1" {
        return Err("language artifact index schema mismatch".into());
    }
    if !root.join(&index.documentation_path).exists() {
        return Err(format!(
            "documentation index is unavailable: {}",
            index.documentation_path
        ));
    }
    let manifest = load_braxon_context_manifest(root)?;
    let chain = load_braxon_chain_root_db(root, &manifest)?;
    let bridge = TokenizerBridge::from_root(root, "braxon_native")?;
    let mut traversals = Vec::new();
    for record in index.records {
        traversals.push(traverse_record(
            root,
            &manifest,
            &chain,
            &bridge,
            &index.documentation_path,
            record,
        )?);
    }
    let verified_total = traversals
        .iter()
        .filter(|traversal| traversal_passes(traversal))
        .count();
    let record_total = traversals.len();
    Ok(LanguageArtifactContextReport {
        schema: LANGUAGE_ARTIFACT_CONTEXT_SCHEMA.into(),
        record_total,
        verified_total,
        all_passed: record_total > 0 && record_total == verified_total,
        traversals,
    })
}

fn traverse_record(
    root: &Path,
    manifest: &crate::BraxonContextManifest,
    chain: &crate::ContextChainRootDb,
    bridge: &TokenizerBridge,
    documentation_path: &str,
    record: LanguageArtifactRecord,
) -> Result<LanguageArtifactTraversal, String> {
    let source_contents = fs::read_to_string(root.join(&record.source_path))
        .map_err(|error| format!("failed to read '{}': {error}", record.source_path))?;
    let source_symbol_resolved = source_contents.contains(&record.symbol);
    let ast_identity_resolved = !record.ast_identity.trim().is_empty();
    let documentation_link_resolved = fs::read_to_string(root.join(documentation_path))
        .map(|contents| contents.contains(&record.symbol) || contents.contains(&record.pointer_id))
        .unwrap_or(false);
    let canonical_chain_address_resolved = chain.chain_records.iter().any(|chain_record| {
        chain_record.pointer_id == record.pointer_id && chain_record.path == record.source_path
    });
    let token_receipt = bridge.encode_translate_round_trip(&record.token);
    let projection = token_receipt.projections.first();
    let token_resolved = token_receipt.all_required_mappings_resolved() && projection.is_some();
    let mut stack = stack()?;
    let capability_resolved = !stack
        .discover_raw_capabilities(&record.capability)
        .is_empty();
    let request = SemanticLinkRequest {
        pointer_id: record.pointer_id.clone(),
        query: record.capability.clone(),
        input: [("input".into(), record.symbol.clone())]
            .into_iter()
            .collect(),
    };
    let mut surface = SemanticLinkSurface::new();
    let lookup = surface
        .resolve(manifest, root, &request, &stack)
        .and_then(|resolution| surface.invoke(&mut stack, resolution, request.input));
    let (runtime_lookup_accepted, released, resident_client_bytes, lookup_reason) = match lookup {
        Ok(receipt) => (
            true,
            receipt.released,
            receipt.resident_client_bytes,
            "runtime lookup accepted and receipt released".into(),
        ),
        Err(error) => (false, false, 0, format!("runtime lookup failed: {error}")),
    };
    let reason = if source_symbol_resolved
        && ast_identity_resolved
        && documentation_link_resolved
        && canonical_chain_address_resolved
        && token_resolved
        && capability_resolved
        && runtime_lookup_accepted
        && released
        && resident_client_bytes == 0
    {
        "symbol-to-AST-to-documentation-to-token-to-address-to-runtime traversal verified".into()
    } else {
        lookup_reason
    };
    Ok(LanguageArtifactTraversal {
        id: record.id,
        pointer_id: record.pointer_id,
        symbol: record.symbol,
        source_path: record.source_path,
        ast_identity: record.ast_identity,
        documentation_path: documentation_path.into(),
        token: record.token,
        native_token_id: projection.map(|projection| projection.native_id),
        universal_token_id: projection.map(|projection| projection.universal_id),
        nsq_address: projection.map(|projection| projection.nsq_address.clone()),
        capability: record.capability,
        source_symbol_resolved,
        ast_identity_resolved,
        documentation_link_resolved,
        canonical_chain_address_resolved,
        token_resolved,
        capability_resolved,
        runtime_lookup_accepted,
        released,
        resident_client_bytes,
        reason,
    })
}

fn traversal_passes(traversal: &LanguageArtifactTraversal) -> bool {
    traversal.source_symbol_resolved
        && traversal.ast_identity_resolved
        && traversal.documentation_link_resolved
        && traversal.canonical_chain_address_resolved
        && traversal.token_resolved
        && traversal.capability_resolved
        && traversal.runtime_lookup_accepted
        && traversal.released
        && traversal.resident_client_bytes == 0
}

fn stack() -> Result<NativeNsqStack, String> {
    let council = (1..=10).map(address).collect::<Vec<_>>();
    NativeNsqStack::new(council, address(20), slot(21), 1)
}

fn address(position: u64) -> NsqAddress {
    NsqAddress::root(NSQSlot::new(
        Dialect::Control,
        vec![NSQLever::new(Charge::Positive, position)
            .map_err(|error| error.to_string())
            .unwrap()],
    ))
}

fn slot(position: u64) -> NSQSlot {
    NSQSlot::new(
        Dialect::Intent,
        vec![NSQLever::new(Charge::Positive, position)
            .map_err(|error| error.to_string())
            .unwrap()],
    )
}

fn read_json<T: for<'de> Deserialize<'de>>(root: &Path, relative_path: &str) -> Result<T, String> {
    let path = root.join(relative_path);
    let raw = fs::read_to_string(&path)
        .map_err(|error| format!("failed to read '{}': {error}", path.display()))?;
    serde_json::from_str(&raw)
        .map_err(|error| format!("failed to parse '{}': {error}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_root() -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .unwrap()
    }

    #[test]
    fn language_artifacts_traverse_from_symbol_to_released_runtime_lookup_without_resident_state() {
        let report = verify_language_artifact_context(&repo_root()).unwrap();
        assert!(report.all_passed, "{report:?}");
        assert_eq!(report.verified_total, report.record_total);
        assert!(report
            .traversals
            .iter()
            .all(|traversal| traversal.released && traversal.resident_client_bytes == 0));
    }
}
