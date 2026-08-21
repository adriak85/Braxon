use crate::tokenizer_bridge::{TokenizerBridge, TokenizerBridgeReceipt};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub const BIONIC_COMPATIBILITY_SCHEMA: &str = "braxon.nsq.bionic_compatibility_report.v1";
const MATRIX_RELATIVE_PATH: &str = "config/nsq/bionic_gnu_compatibility_matrix.json";
const OVERLAY_PROOF_RELATIVE_PATH: &str = "state/full_android_language_toolchain/native/android_libc_extensions/proofs/unified_android_libc_contracts_symbol_proof.txt";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BionicInterfaceReport {
    pub symbol: String,
    pub header: String,
    pub universal_lexical: String,
    pub source_present: bool,
    pub source_implementation_state: String,
    pub target_proof_required: bool,
    pub tokenizer_receipt: TokenizerBridgeReceipt,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BionicCompatibilityReport {
    pub schema: String,
    pub target_abi: String,
    pub bionic_remains_platform_libc: bool,
    pub glibc_replacement_claimed: bool,
    pub overlay_source_present: bool,
    pub target_proof_present: bool,
    pub structural_contract_valid: bool,
    pub interfaces: Vec<BionicInterfaceReport>,
    pub exact_target_materialization_action: String,
}

#[derive(Debug, Deserialize)]
struct CompatibilityMatrix {
    schema: String,
    target: CompatibilityTarget,
    policy: CompatibilityPolicy,
    overlay: OverlayContract,
    #[serde(default)]
    interfaces: Vec<CompatibilityInterface>,
    dialect_projection: DialectProjection,
    #[serde(default)]
    required_documents: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct CompatibilityTarget {
    abi: String,
    libc: String,
    glibc_replacement_claimed: bool,
}

#[derive(Debug, Deserialize)]
struct CompatibilityPolicy {
    bionic_remains_the_platform_libc: bool,
    unknown_or_unimplemented_interfaces_must_fail_closed: bool,
    universal_tokenization_uses_existing_braxon_native_tokenizer: bool,
    all_dialect_mapping_requires_semantic_role_not_decorative_label: bool,
}

#[derive(Debug, Deserialize)]
struct OverlayContract {
    source: String,
    header_generator: String,
    build_model: String,
}

#[derive(Debug, Deserialize)]
struct CompatibilityInterface {
    symbol: String,
    header: String,
    signature_class: String,
    aarch64_syscall_or_bridge: String,
    universal_lexical: String,
    provenance: String,
    proof_state: String,
}

#[derive(Debug, Deserialize)]
struct DialectProjection {
    alphabetic: String,
    numeric: String,
    intent: String,
    symbolic: String,
    stamp: String,
    control: String,
    graphics: String,
    audio: String,
}

fn root_from(start: impl AsRef<Path>) -> Result<PathBuf, String> {
    let canonical = start
        .as_ref()
        .canonicalize()
        .map_err(|error| format!("unable to resolve Bionic compatibility root: {error}"))?;
    canonical
        .ancestors()
        .find(|candidate| candidate.join(MATRIX_RELATIVE_PATH).is_file())
        .map(Path::to_path_buf)
        .ok_or_else(|| "unable to locate repository Bionic compatibility matrix".to_string())
}

pub fn verify_bionic_compatibility(
    start: impl AsRef<Path>,
) -> Result<BionicCompatibilityReport, String> {
    let root = root_from(start)?;
    let matrix_path = root.join(MATRIX_RELATIVE_PATH);
    let matrix_raw = fs::read_to_string(&matrix_path)
        .map_err(|error| format!("unable to read {}: {error}", matrix_path.display()))?;
    let matrix: CompatibilityMatrix = serde_json::from_str(&matrix_raw)
        .map_err(|error| format!("invalid {}: {error}", matrix_path.display()))?;
    if matrix.schema != "braxon.nsq.bionic_gnu_compatibility.v1" {
        return Err("unsupported Bionic compatibility matrix schema".to_string());
    }
    if matrix.target.abi != "aarch64-linux-android" || matrix.target.libc != "Bionic" {
        return Err(
            "Bionic compatibility matrix target must be aarch64-linux-android with Bionic"
                .to_string(),
        );
    }
    if matrix.interfaces.is_empty() {
        return Err("Bionic compatibility matrix must declare at least one interface".to_string());
    }
    let overlay_source_present = root.join(&matrix.overlay.source).is_file();
    let overlay_generator_present = root.join(&matrix.overlay.header_generator).is_file();
    let documentation_present = matrix
        .required_documents
        .iter()
        .all(|document| root.join(document).is_file());
    let dialects_meaningful = [
        &matrix.dialect_projection.alphabetic,
        &matrix.dialect_projection.numeric,
        &matrix.dialect_projection.intent,
        &matrix.dialect_projection.symbolic,
        &matrix.dialect_projection.stamp,
        &matrix.dialect_projection.control,
        &matrix.dialect_projection.graphics,
        &matrix.dialect_projection.audio,
    ]
    .iter()
    .all(|value| !value.trim().is_empty());
    let bridge = TokenizerBridge::from_root(&root, "braxon_native")?;
    let interfaces = matrix
        .interfaces
        .iter()
        .map(|interface| BionicInterfaceReport {
            symbol: interface.symbol.clone(),
            header: interface.header.clone(),
            universal_lexical: interface.universal_lexical.clone(),
            source_present: overlay_source_present,
            source_implementation_state: if overlay_source_present {
                "first_party_overlay_source_present".to_string()
            } else {
                "overlay_source_missing".to_string()
            },
            target_proof_required: interface.proof_state == "target_compile_link_run_required",
            tokenizer_receipt: bridge.encode_translate_round_trip(&interface.universal_lexical),
        })
        .collect::<Vec<_>>();
    let interface_contracts_valid = matrix.interfaces.iter().all(|interface| {
        !interface.symbol.trim().is_empty()
            && !interface.header.trim().is_empty()
            && !interface.signature_class.trim().is_empty()
            && !interface.aarch64_syscall_or_bridge.trim().is_empty()
            && !interface.universal_lexical.trim().is_empty()
            && !interface.provenance.trim().is_empty()
            && interface.proof_state == "target_compile_link_run_required"
    });
    let tokenization_valid = interfaces
        .iter()
        .all(|interface| interface.tokenizer_receipt.all_required_mappings_resolved());
    let target_proof_present = root.join(OVERLAY_PROOF_RELATIVE_PATH).is_file();
    let structural_contract_valid = overlay_source_present
        && overlay_generator_present
        && documentation_present
        && interface_contracts_valid
        && tokenization_valid
        && dialects_meaningful
        && matrix.policy.bionic_remains_the_platform_libc
        && matrix
            .policy
            .unknown_or_unimplemented_interfaces_must_fail_closed
        && matrix
            .policy
            .universal_tokenization_uses_existing_braxon_native_tokenizer
        && matrix
            .policy
            .all_dialect_mapping_requires_semantic_role_not_decorative_label
        && !matrix.target.glibc_replacement_claimed;
    Ok(BionicCompatibilityReport {
        schema: BIONIC_COMPATIBILITY_SCHEMA.to_string(),
        target_abi: matrix.target.abi,
        bionic_remains_platform_libc: matrix.policy.bionic_remains_the_platform_libc,
        glibc_replacement_claimed: matrix.target.glibc_replacement_claimed,
        overlay_source_present,
        target_proof_present,
        structural_contract_valid,
        interfaces,
        exact_target_materialization_action: format!(
            "Use declared build model '{}' by running '{}' from a target-capacity-approved executable workspace, then retain '{}'.",
            matrix.overlay.build_model,
            matrix.overlay.header_generator,
            OVERLAY_PROOF_RELATIVE_PATH
        ),
    })
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
    fn every_retained_bionic_overlay_interface_has_universal_tokenization() {
        let report =
            verify_bionic_compatibility(repository_root()).expect("Bionic compatibility report");
        assert!(report.structural_contract_valid, "{report:#?}");
        assert_eq!(report.interfaces.len(), 13);
        assert!(report.bionic_remains_platform_libc);
        assert!(!report.glibc_replacement_claimed);
        assert!(report
            .interfaces
            .iter()
            .all(|interface| { interface.tokenizer_receipt.all_required_mappings_resolved() }));
    }
}
