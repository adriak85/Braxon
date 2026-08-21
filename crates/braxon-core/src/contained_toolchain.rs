use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

pub const CONTAINED_TOOLCHAIN_SCHEMA: &str = "braxon.contained_toolchain.verification.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToolchainVerificationCheck {
    pub id: String,
    pub passed: bool,
    pub evidence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContainedToolchainReport {
    pub schema: String,
    pub workspace_root: String,
    pub target_environment: String,
    pub structural_verification_valid: bool,
    pub full_source_reconstruction_ready: bool,
    pub release_ready: bool,
    pub declared_language_total: usize,
    pub functional_language_total: usize,
    pub release_blockers: Vec<String>,
    pub checks: Vec<ToolchainVerificationCheck>,
}

#[derive(Debug, Deserialize)]
struct RuntimeLanguageRegistry {
    schema: String,
    #[serde(default)]
    required_core_surfaces: Vec<String>,
    #[serde(default)]
    surfaces: Vec<RuntimeLanguageSurface>,
}

#[derive(Debug, Deserialize)]
struct RuntimeLanguageSurface {
    id: String,
}

#[derive(Debug, Deserialize)]
struct FunctionalLanguageMatrix {
    schema: String,
    target_environment: String,
    language_total: usize,
    #[serde(default)]
    languages: Vec<FunctionalLanguageSurface>,
}

#[derive(Debug, Deserialize)]
struct FunctionalLanguageSurface {
    id: String,
    target_environment: String,
    semantic_contract: FunctionalSemanticContract,
    target_materialization: FunctionalTargetMaterialization,
}

#[derive(Debug, Deserialize)]
struct FunctionalSemanticContract {
    nsq_capability: String,
    kinetic_reflexor_route: String,
    semantic_operation_state: String,
    resident_runtime: bool,
}

#[derive(Debug, Deserialize)]
struct FunctionalTargetMaterialization {
    #[serde(default)]
    required_local_tools: Vec<String>,
    hidden_download_allowed: bool,
}

#[derive(Debug, Deserialize)]
struct SourceAvailabilityManifest {
    target_environment: String,
    #[serde(default)]
    sources: Vec<SourceAvailabilityRecord>,
    #[serde(default)]
    explicit_release_blockers: Vec<ReleaseBlocker>,
}

#[derive(Debug, Deserialize)]
struct SourceAvailabilityRecord {
    id: String,
    #[serde(default)]
    source_path: String,
    #[serde(default)]
    source_url: String,
    #[serde(default)]
    source_status: String,
    #[serde(default)]
    upstream_commit_verified: bool,
}

#[derive(Debug, Deserialize)]
struct ReleaseBlocker {
    id: String,
    #[serde(default)]
    severity: String,
    #[serde(default)]
    required_action: String,
}

#[derive(Debug, Deserialize)]
struct RustBootstrapChain {
    schema: String,
    target_environment: String,
    normal_runtime_policy: RustNormalRuntimePolicy,
    truth_rules: RustBootstrapTruthRules,
    #[serde(default)]
    lanes: Vec<RustBootstrapLane>,
    #[serde(default)]
    promotion_order: Vec<String>,
    current_release_state: RustCurrentReleaseState,
}

#[derive(Debug, Deserialize)]
struct RustNormalRuntimePolicy {
    network_required: bool,
    default_workspace_channel: String,
}

#[derive(Debug, Deserialize)]
struct RustBootstrapTruthRules {
    unmaterialized_source_must_not_be_presented_as_buildable: bool,
    edge_descriptor_is_not_an_activated_toolchain: bool,
    stage_promotion_requires_target_compile_link_run_and_equivalence_evidence: bool,
    absolute_termux_paths_are_not_authoritative_reconstruction_contracts: bool,
}

#[derive(Debug, Deserialize)]
struct RustBootstrapLane {
    id: String,
    role: String,
    #[serde(default)]
    materialization_status: String,
}

#[derive(Debug, Deserialize)]
struct RustCurrentReleaseState {
    workspace_offline_build_validated: bool,
    full_contained_rust_source_reconstruction_ready: bool,
    default_edge_nightly_activated: bool,
    #[serde(default)]
    release_blockers: Vec<String>,
}

fn read_json<T: for<'a> Deserialize<'a>>(path: &Path) -> Result<T, String> {
    let text = fs::read_to_string(path)
        .map_err(|error| format!("unable to read {}: {error}", path.display()))?;
    serde_json::from_str(&text).map_err(|error| format!("invalid {}: {error}", path.display()))
}

fn resolve_workspace_root(start: impl AsRef<Path>) -> Result<PathBuf, String> {
    let mut current = start
        .as_ref()
        .canonicalize()
        .map_err(|error| format!("unable to resolve workspace start: {error}"))?;
    if current.is_file() {
        current = current
            .parent()
            .ok_or("workspace start file has no parent")?
            .to_path_buf();
    }
    loop {
        if current.join("Cargo.toml").is_file() && current.join("crates").is_dir() {
            return Ok(current);
        }
        if !current.pop() {
            return Err("unable to locate Braxon workspace root".to_string());
        }
    }
}

fn language_ids(registry: &RuntimeLanguageRegistry) -> BTreeSet<String> {
    registry
        .required_core_surfaces
        .iter()
        .cloned()
        .chain(registry.surfaces.iter().map(|surface| surface.id.clone()))
        .filter(|id| !id.trim().is_empty())
        .collect()
}

fn source_is_materialized(root: &Path, record: &SourceAvailabilityRecord) -> bool {
    let path = root.join(&record.source_path);
    path.is_dir()
        && fs::read_dir(path)
            .map(|mut entries| entries.next().is_some())
            .unwrap_or(false)
}

pub fn verify_contained_toolchain(
    start: impl AsRef<Path>,
) -> Result<ContainedToolchainReport, String> {
    let root = resolve_workspace_root(start)?;
    let language_registry: RuntimeLanguageRegistry =
        read_json(&root.join("config/nsq/nsq_runtime_language_registry.json"))?;
    if language_registry.schema != "nsq.runtime.language.registry.v3" {
        return Err("unsupported NSQ runtime language registry schema".to_string());
    }
    let matrix: FunctionalLanguageMatrix =
        read_json(&root.join("config/nsq/language_functional_ingestion_matrix.json"))?;
    if matrix.schema != "braxon.language_functional_ingestion_matrix.v1" {
        return Err("unsupported functional language-ingestion matrix schema".to_string());
    }
    let sources: SourceAvailabilityManifest =
        read_json(&root.join("config/toolchains/source_availability_manifest.json"))?;
    let toolchain_value: serde_json::Value =
        read_json(&root.join("config/toolchains/contained_semantic_toolchain_inventory.json"))?;
    let rust_chain: RustBootstrapChain =
        read_json(&root.join("config/toolchains/rust_bootstrap_chain.json"))?;
    let source_built_graph: serde_json::Value =
        read_json(&root.join("config/toolchains/source_built_build_graph.json"))?;

    let declared = language_ids(&language_registry);
    let functional = matrix
        .languages
        .iter()
        .map(|language| language.id.clone())
        .collect::<BTreeSet<_>>();
    let language_matrix_complete = matrix.language_total == matrix.languages.len()
        && declared == functional
        && matrix.languages.iter().all(|language| {
            language.target_environment == matrix.target_environment
                && language.semantic_contract.nsq_capability == format!("language:{}", language.id)
                && !language
                    .semantic_contract
                    .kinetic_reflexor_route
                    .trim()
                    .is_empty()
                && language.semantic_contract.semantic_operation_state == "operable_on_demand"
                && !language.semantic_contract.resident_runtime
                && !language.target_materialization.hidden_download_allowed
                && !language
                    .target_materialization
                    .required_local_tools
                    .is_empty()
        });

    let cargo_config = fs::read_to_string(root.join(".cargo/config.toml")).unwrap_or_default();
    let vendor_path = root.join("vendor");
    let vendored_dependency_contained = cargo_config
        .contains("replace-with = \"vendored-sources\"")
        && cargo_config.contains("directory = \"vendor\"")
        && vendor_path.is_dir()
        && fs::read_dir(&vendor_path)
            .map(|mut entries| entries.next().is_some())
            .unwrap_or(false);

    let normal_runtime_policy = toolchain_value
        .get("normal_runtime_policy")
        .and_then(|value| value.as_object());
    let no_hidden_runtime_dependency = normal_runtime_policy
        .map(|policy| {
            policy
                .get("network_required")
                .and_then(|value| value.as_bool())
                == Some(false)
                && policy
                    .get("external_api_required")
                    .and_then(|value| value.as_bool())
                    == Some(false)
                && policy
                    .get("cloud_inference_required")
                    .and_then(|value| value.as_bool())
                    == Some(false)
                && policy
                    .get("cargo_registry_source")
                    .and_then(|value| value.as_str())
                    == Some("vendor")
        })
        .unwrap_or(false);

    let machine_proofs = toolchain_value
        .get("machine_boundary")
        .and_then(|value| value.get("active_proofs"))
        .and_then(|value| value.as_array())
        .map(|proofs| {
            !proofs.is_empty()
                && proofs.iter().all(|proof| {
                    proof
                        .as_str()
                        .map(|path| root.join(path).is_file())
                        .unwrap_or(false)
                })
        })
        .unwrap_or(false);

    let source_record_ids = sources
        .sources
        .iter()
        .map(|record| record.id.trim().to_string())
        .collect::<BTreeSet<_>>();
    let source_record_identity_valid = !sources.sources.is_empty()
        && source_record_ids.len() == sources.sources.len()
        && source_record_ids.iter().all(|id| !id.is_empty());
    let source_records_truthful = source_record_identity_valid
        && sources
            .sources
            .iter()
            .all(|record| match record.source_status.as_str() {
                "materialized_local_source_tree" => source_is_materialized(&root, record),
                "unmaterialized_orphaned_gitlink" => {
                    !source_is_materialized(&root, record)
                        && !record.source_url.is_empty()
                        && record.upstream_commit_verified
                }
                "local_built_payload_present" | "contained_vendored_source" => {
                    root.join(&record.source_path).exists()
                }
                _ => false,
            });
    let source_metadata_declared = sources
        .sources
        .iter()
        .filter(|record| record.source_status == "unmaterialized_orphaned_gitlink")
        .all(|record| {
            !record.source_url.is_empty()
                && fs::read_to_string(root.join(".gitmodules"))
                    .map(|modules| {
                        modules.contains(&record.source_path)
                            && modules.contains(&record.source_url)
                    })
                    .unwrap_or(false)
        });
    let full_source_reconstruction_ready = sources.sources.iter().all(|record| {
        record.source_status != "unmaterialized_orphaned_gitlink"
            || source_is_materialized(&root, record)
    });

    let rust_chain_ids = rust_chain
        .lanes
        .iter()
        .map(|lane| lane.id.as_str())
        .collect::<BTreeSet<_>>();
    let rust_chain_stage_roles_valid = rust_chain.lanes.iter().all(|lane| {
        !lane.id.trim().is_empty()
            && !lane.role.trim().is_empty()
            && (lane.id == "historical_phone_nightly_1_96"
                || !lane.materialization_status.trim().is_empty())
    });
    let rust_bootstrap_chain_valid = rust_chain.schema
        == "braxon.toolchain.rust_bootstrap_chain.v1"
        && rust_chain.target_environment == "aarch64-linux-android"
        && !rust_chain.normal_runtime_policy.network_required
        && rust_chain.normal_runtime_policy.default_workspace_channel == "1.97.0"
        && rust_chain
            .truth_rules
            .unmaterialized_source_must_not_be_presented_as_buildable
        && rust_chain
            .truth_rules
            .edge_descriptor_is_not_an_activated_toolchain
        && rust_chain
            .truth_rules
            .stage_promotion_requires_target_compile_link_run_and_equivalence_evidence
        && rust_chain
            .truth_rules
            .absolute_termux_paths_are_not_authoritative_reconstruction_contracts
        && rust_chain_ids
            == BTreeSet::from([
                "historical_phone_nightly_1_96",
                "bootstrap_termux_1_97_1",
                "stage1_pinned_rust_source",
                "stage2_pinned_rust_source",
                "workspace_known_good_1_97_0",
                "edge_candidate_1_100_0_nightly",
            ])
        && rust_chain.promotion_order
            == vec![
                "bootstrap_termux_1_97_1",
                "stage1_pinned_rust_source",
                "stage2_pinned_rust_source",
                "workspace_known_good_1_97_0",
                "edge_candidate_1_100_0_nightly",
            ]
        && rust_chain_stage_roles_valid
        && rust_chain
            .current_release_state
            .workspace_offline_build_validated
        && !rust_chain
            .current_release_state
            .full_contained_rust_source_reconstruction_ready
        && !rust_chain
            .current_release_state
            .default_edge_nightly_activated
        && !rust_chain.current_release_state.release_blockers.is_empty();
    let source_built_graph_valid = source_built_graph
        .get("schema")
        .and_then(|value| value.as_str())
        == Some("braxon.toolchain.source_built_build_graph.v1")
        && source_built_graph
            .get("nodes")
            .and_then(|value| value.as_array())
            .map(|nodes| {
                let ids = nodes
                    .iter()
                    .filter_map(|node| node.get("id").and_then(|id| id.as_str()))
                    .collect::<BTreeSet<_>>();
                [
                    "llvm_local_source_identity",
                    "rust_and_cpython_pinned_source_edges",
                    "termux_stage0_bootstrap",
                    "llvm_clang_lld_source_build",
                    "bionic_compatibility_overlay",
                    "rust_stage1_and_stage2_source_build",
                    "termux_nsq_calibration_and_recovery",
                    "complete_language_semantic_proofs",
                    "closure_release",
                ]
                .iter()
                .all(|required| ids.contains(required))
            })
            .unwrap_or(false);
    let target_consistent = sources.target_environment == matrix.target_environment
        && matrix.target_environment == "aarch64-linux-android";
    let release_blockers = sources
        .explicit_release_blockers
        .iter()
        .map(|blocker| {
            format!(
                "{}:{}:{}",
                blocker.id, blocker.severity, blocker.required_action
            )
        })
        .collect::<Vec<_>>();

    let checks = vec![
        ToolchainVerificationCheck {
            id: "all_declared_languages_have_functional_nsq_reflexor_contracts".to_string(),
            passed: language_matrix_complete,
            evidence: format!(
                "declared={}; matrix={}; target={}",
                declared.len(),
                functional.len(),
                matrix.target_environment
            ),
        },
        ToolchainVerificationCheck {
            id: "locked_rust_dependencies_are_contained".to_string(),
            passed: vendored_dependency_contained,
            evidence: "Cargo source replacement points to a populated local vendor directory".to_string(),
        },
        ToolchainVerificationCheck {
            id: "normal_runtime_has_no_hidden_network_or_registry_requirement".to_string(),
            passed: no_hidden_runtime_dependency,
            evidence: "contained semantic toolchain normal_runtime_policy".to_string(),
        },
        ToolchainVerificationCheck {
            id: "machine_semantic_proof_surfaces_are_declared_and_local".to_string(),
            passed: machine_proofs,
            evidence: "declared assembly, ABI, instruction-runtime, and semantic-benchmark proof paths".to_string(),
        },
        ToolchainVerificationCheck {
            id: "source_availability_is_truthful".to_string(),
            passed: source_records_truthful,
            evidence: "source records have unique IDs; materialized sources are present; unmaterialized sources are explicitly recorded as verified source-edge boundaries".to_string(),
        },
        ToolchainVerificationCheck {
            id: "source_edge_metadata_resolves_all_recorded_gitlinks".to_string(),
            passed: source_metadata_declared,
            evidence: ".gitmodules names each unmaterialized verified source path and upstream URL".to_string(),
        },
        ToolchainVerificationCheck {
            id: "rust_bootstrap_chain_is_explicit_and_non_promotional".to_string(),
            passed: rust_bootstrap_chain_valid,
            evidence: "Termux 1.97.1 bootstrap, stage1, stage2, workspace 1.97.0, and unactivated 1.100.0-nightly lanes are ordered with source and target proof blockers retained".to_string(),
        },
        ToolchainVerificationCheck {
            id: "source_built_graph_declares_compiler_semantic_and_recovery_order".to_string(),
            passed: source_built_graph_valid,
            evidence: "source-built graph declares LLVM/Clang/LLD, Bionic overlay, Rust stages, NSQ semantic proofs, calibration/recovery, and closure in explicit order".to_string(),
        },
        ToolchainVerificationCheck {
            id: "target_environment_is_consistent".to_string(),
            passed: target_consistent,
            evidence: format!("source_manifest={}; language_matrix={}", sources.target_environment, matrix.target_environment),
        },
    ];
    let structural_verification_valid = checks.iter().all(|check| check.passed);
    let release_ready = structural_verification_valid
        && full_source_reconstruction_ready
        && release_blockers.is_empty();

    Ok(ContainedToolchainReport {
        schema: CONTAINED_TOOLCHAIN_SCHEMA.to_string(),
        workspace_root: root.display().to_string(),
        target_environment: matrix.target_environment,
        structural_verification_valid,
        full_source_reconstruction_ready,
        release_ready,
        declared_language_total: declared.len(),
        functional_language_total: functional.len(),
        release_blockers,
        checks,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn workspace_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("workspace root")
            .to_path_buf()
    }

    #[test]
    fn contained_toolchain_report_preserves_language_coverage_and_source_truth() {
        let report = verify_contained_toolchain(workspace_root()).expect("toolchain report");
        assert!(report.structural_verification_valid, "{report:#?}");
        assert_eq!(
            report.declared_language_total,
            report.functional_language_total
        );
        assert!(report
            .release_blockers
            .iter()
            .any(|blocker| blocker.contains("orphaned_rust_gitlink")));
        assert!(!report.full_source_reconstruction_ready);
        assert!(!report.release_ready);
    }
}
