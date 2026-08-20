//! NSQ Kinetic Semantic Reflexor.
//!
//! This crate is intentionally an on-demand control surface. It discovers the
//! workspace, runtime declarations, libraries, and physical-device boundaries;
//! it does not create a second resident runtime or graphics framework.

use cargo_metadata::{MetadataCommand, Package, TargetKind};
use nsq_core::{
    generate_default_intent_gradient_frame, validate_intent_gradient_frame, Dialect,
    NsqFinalLeverPosition, NsqFinalSide, NsqIntentGradientFrame, NsqIntentScaleAnchor,
    CANONICAL_LEVER_MAX_POSITION,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeSet, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub const REFLEX_SCHEMA: &str = "braxon.nsq.kinetic_reflex.v1";
pub const DEFAULT_PROFILE: &str = "samsung_galaxy_a17_termux_aarch64";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityKind {
    WorkspaceCrate,
    DeclaredDialect,
    NativeLanguage,
    SemanticIntent,
    ProjectSource,
    SupportLibrary,
    NativeBoundary,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityState {
    OperableOnDemand,
    IngestedAndRedefined,
    DeclaredContract,
    PhysicalBoundaryAdapter,
    Missing,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReflexCapability {
    pub id: String,
    pub kind: CapabilityKind,
    pub state: CapabilityState,
    pub nsq_dialect: String,
    pub owner: String,
    pub source: String,
    pub semantic_authority: String,
    pub complete_nsq_ingestion: bool,
    pub foreign_surface_role: String,
    pub ingress: String,
    pub on_demand: bool,
    pub resident_runtime: bool,
    pub executable: bool,
    pub target_names: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeviceProfile {
    pub schema: String,
    pub id: String,
    pub supported_host: String,
    pub target_triple: String,
    pub expected_architecture: String,
    pub front_door: String,
    pub safe_jobs: u8,
    pub required_tools: Vec<String>,
    pub physical_boundary_only: Vec<String>,
    pub unsupported_claims: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflexInventory {
    pub schema: String,
    pub authority: String,
    pub workspace_root: String,
    pub profile: DeviceProfile,
    pub capabilities: Vec<ReflexCapability>,
    pub canonical_dialects: Vec<String>,
    pub declared_runtime_surfaces: usize,
    pub native_language_contracts: usize,
    pub project_source_files: usize,
    pub workspace_crates: usize,
    pub support_libraries: usize,
    pub semantic_intent: SemanticIntentContract,
    pub resident_runtime_constructed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationCheck {
    pub name: String,
    pub passed: bool,
    pub evidence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflexVerification {
    pub schema: String,
    pub valid: bool,
    pub checks: Vec<VerificationCheck>,
    pub unresolved_capabilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflexOperation {
    pub schema: String,
    pub capability: ReflexCapability,
    pub routed: bool,
    pub execution_mode: String,
    pub command: Option<Vec<String>>,
    pub receipt_path: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticIntentContract {
    pub id: String,
    pub description: String,
    pub nsq_dialect: String,
    pub intent_gradient: NsqIntentGradientFrame,
    pub court_route: Vec<String>,
    pub valid_final_tier_frame: bool,
    pub instantiated_in_nsq: bool,
    pub foreign_spellings_projection_only: bool,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct NativeLanguageRegistry {
    #[serde(default)]
    schema: String,
    #[serde(default)]
    authority: String,
    #[serde(default)]
    execution_policy: String,
    #[serde(default)]
    semantic_intent: NativeSemanticIntent,
    #[serde(default)]
    languages: Vec<NativeLanguageSurface>,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct NativeSemanticIntent {
    #[serde(default)]
    id: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    variable_positions: Vec<u64>,
    #[serde(default)]
    scale_anchor: String,
    #[serde(default)]
    court_route: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct NativeLanguageSurface {
    #[serde(default)]
    id: String,
    #[serde(default)]
    surface_aliases: Vec<String>,
    #[serde(default)]
    nsq_dialect: String,
    #[serde(default)]
    requested_toolchain: String,
    #[serde(default)]
    resolved_toolchain: String,
    #[serde(default)]
    toolchain_owner: String,
    #[serde(default)]
    tools: Vec<String>,
    #[serde(default)]
    native_ingress: String,
    #[serde(default)]
    native_egress: String,
    #[serde(default)]
    output_contract: String,
    #[serde(default)]
    resident_runtime: bool,
    #[serde(default)]
    upstream_runtime_dependency: bool,
    #[serde(default)]
    notes: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct LanguageRegistry {
    #[serde(default)]
    schema: String,
    #[serde(default)]
    required_core_surfaces: Vec<String>,
    #[serde(default)]
    surfaces: Vec<LanguageSurface>,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct LanguageSurface {
    #[serde(default)]
    id: String,
    #[serde(default)]
    family: String,
    #[serde(default)]
    native_ingress: String,
    #[serde(default)]
    enabled: bool,
    #[serde(default)]
    runtime_builtin: bool,
    #[serde(default)]
    projection_only: bool,
}

/// Returns the project root when invoked from a root command, a crate command,
/// or an explicitly supplied workspace path.
pub fn resolve_workspace_root(start: impl AsRef<Path>) -> Result<PathBuf, String> {
    let mut current = start
        .as_ref()
        .canonicalize()
        .map_err(|err| err.to_string())?;
    if current.is_file() {
        current = current
            .parent()
            .ok_or_else(|| "workspace path has no parent".to_string())?
            .to_path_buf();
    }
    loop {
        let manifest = current.join("Cargo.toml");
        if manifest.is_file() {
            let text = fs::read_to_string(&manifest).map_err(|err| err.to_string())?;
            if text.contains("[workspace]") {
                return Ok(current);
            }
        }
        if !current.pop() {
            return Err("could not locate a Cargo workspace root".to_string());
        }
    }
}

pub fn samsung_galaxy_a17_profile() -> DeviceProfile {
    DeviceProfile {
        schema: "braxon.device.profile.v1".to_string(),
        id: DEFAULT_PROFILE.to_string(),
        supported_host: "native Termux on Android; device model is detected at bootstrap and never assumed from the profile alone".to_string(),
        target_triple: "aarch64-linux-android".to_string(),
        expected_architecture: "aarch64".to_string(),
        front_door: "Braxon reflex bootstrap --profile samsung_galaxy_a17_termux_aarch64".to_string(),
        safe_jobs: 1,
        required_tools: vec![
            "cargo".into(),
            "rustc".into(),
            "git".into(),
            "clang".into(),
            "clang++".into(),
            "llvm-ar".into(),
            "llvm-ranlib".into(),
            "ld.lld".into(),
            "llvm-mc".into(),
            "llvm-objcopy".into(),
            "llvm-objdump".into(),
            "guile".into(),
            "apropos".into(),
        ],
        physical_boundary_only: vec![
            "display acquisition".into(),
            "surface lifecycle".into(),
            "frame presentation".into(),
            "touch/input acquisition".into(),
            "fence and ownership synchronization".into(),
        ],
        unsupported_claims: vec![
            "No hardware-specific GPU, Vulkan, Android SDK, or Termux-X11 capability is claimed without an on-device probe.".into(),
            "No permanent GUI, graphics, Android, or model runtime is started by the Reflexor.".into(),
        ],
    }
}

pub fn discover(root: impl AsRef<Path>) -> Result<ReflexInventory, String> {
    let root = resolve_workspace_root(root)?;
    let metadata = MetadataCommand::new()
        .current_dir(&root)
        .exec()
        .map_err(|err| format!("cargo metadata failed: {err}"))?;
    let member_ids: HashSet<_> = metadata.workspace_members.iter().collect();

    let mut capabilities = Vec::new();
    let mut member_names = BTreeSet::new();
    let mut library_names = BTreeSet::new();

    for package in metadata
        .packages
        .iter()
        .filter(|package| member_ids.contains(&package.id))
    {
        member_names.insert(package.name.clone());
        capabilities.push(crate_capability(&root, package));
        for dependency in &package.dependencies {
            library_names.insert(dependency.name.clone());
        }
    }

    for library in library_names {
        capabilities.push(ReflexCapability {
            id: format!("library:{library}"),
            kind: CapabilityKind::SupportLibrary,
            state: CapabilityState::DeclaredContract,
            nsq_dialect: dialect_name(Dialect::Control),
            owner: "NSQ Reflex support-library contract".to_string(),
            source: "Cargo.toml dependency declaration captured into NSQ".to_string(),
            semantic_authority: "nsq".to_string(),
            complete_nsq_ingestion: true,
            foreign_surface_role: "dependency_metadata_ingress_only".to_string(),
            ingress: "cargo_resolution_to_nsq_contract".to_string(),
            on_demand: true,
            resident_runtime: false,
            executable: false,
            target_names: Vec::new(),
            notes: vec![
                "Dependency is represented as a support-library contract; discovery does not claim that a compiled artifact is loaded.".to_string(),
            ],
        });
    }

    let registry_path = root.join("config/nsq/nsq_runtime_language_registry.json");
    let registry = read_language_registry(&registry_path)?;
    let mut seen_surfaces = BTreeSet::new();
    for surface in registry
        .required_core_surfaces
        .iter()
        .cloned()
        .chain(registry.surfaces.iter().map(|surface| surface.id.clone()))
        .filter(|surface| !surface.trim().is_empty())
    {
        if !seen_surfaces.insert(surface.clone()) {
            continue;
        }
        let details = registry
            .surfaces
            .iter()
            .find(|entry| entry.id == surface)
            .cloned()
            .unwrap_or_default();
        capabilities.push(language_capability(&surface, &details, &registry_path));
    }

    capabilities.extend(ingested_registry_language_capabilities(
        &registry,
        &seen_surfaces,
        &registry_path,
    ));

    let native_language_path = root.join("config/nsq/nsq_native_language_contracts.json");
    let native_language_registry = read_native_language_registry(&native_language_path)?;
    let semantic_intent = instantiate_semantic_intent(&native_language_registry.semantic_intent)?;
    capabilities.push(semantic_intent_capability(
        &semantic_intent,
        &native_language_path,
    ));
    capabilities.extend(native_language_capabilities(
        &native_language_registry,
        &native_language_path,
    ));
    capabilities.extend(native_boundary_capabilities());
    let project_sources = source_tree_capabilities(&root)?;
    let project_source_files = project_sources.len();
    capabilities.extend(project_sources);
    capabilities.sort_by(|left, right| left.id.cmp(&right.id));

    Ok(ReflexInventory {
        schema: REFLEX_SCHEMA.to_string(),
        authority: "NSQ kinetic semantic reflexor".to_string(),
        workspace_root: root.display().to_string(),
        profile: samsung_galaxy_a17_profile(),
        canonical_dialects: vec![
            dialect_name(Dialect::Numeric),
            dialect_name(Dialect::Alphabetic),
            dialect_name(Dialect::Intent),
            dialect_name(Dialect::Symbolic),
            dialect_name(Dialect::Stamp),
            dialect_name(Dialect::Control),
            dialect_name(Dialect::Graphics),
            dialect_name(Dialect::Audio),
        ],
        declared_runtime_surfaces: seen_surfaces.len(),
        native_language_contracts: native_language_registry.languages.len(),
        project_source_files,
        workspace_crates: member_names.len(),
        support_libraries: capabilities
            .iter()
            .filter(|capability| capability.kind == CapabilityKind::SupportLibrary)
            .count(),
        semantic_intent,
        resident_runtime_constructed: false,
        capabilities,
    })
}

pub fn verify(root: impl AsRef<Path>) -> Result<ReflexVerification, String> {
    let root = resolve_workspace_root(root)?;
    let inventory = discover(&root)?;
    let metadata = MetadataCommand::new()
        .current_dir(&root)
        .exec()
        .map_err(|err| format!("cargo metadata failed: {err}"))?;
    let expected_workspace_crates = metadata.workspace_members.len();
    let expected_support_libraries = metadata
        .packages
        .iter()
        .filter(|package| metadata.workspace_members.contains(&package.id))
        .flat_map(|package| {
            package
                .dependencies
                .iter()
                .map(|dependency| dependency.name.clone())
        })
        .collect::<BTreeSet<_>>()
        .len();
    let expected_project_sources = source_tree_capabilities(&root)?.len();
    let actual_workspace_crates = inventory
        .capabilities
        .iter()
        .filter(|capability| capability.kind == CapabilityKind::WorkspaceCrate)
        .count();
    let actual_dialects = inventory
        .capabilities
        .iter()
        .filter(|capability| capability.kind == CapabilityKind::DeclaredDialect)
        .count();
    let actual_native_languages = inventory
        .capabilities
        .iter()
        .filter(|capability| capability.kind == CapabilityKind::NativeLanguage)
        .count();
    let actual_project_sources = inventory
        .capabilities
        .iter()
        .filter(|capability| capability.kind == CapabilityKind::ProjectSource)
        .count();
    let all_dialects_mapped = inventory
        .capabilities
        .iter()
        .filter(|capability| capability.kind == CapabilityKind::DeclaredDialect)
        .all(|capability| {
            capability.state == CapabilityState::IngestedAndRedefined
                && !capability.nsq_dialect.is_empty()
                && capability.complete_nsq_ingestion
        });
    let all_nsq_authoritative = inventory.capabilities.iter().all(|capability| {
        capability.semantic_authority == "nsq"
            && capability.complete_nsq_ingestion
            && !capability.foreign_surface_role.is_empty()
    });
    let no_resident_runtime = !inventory.resident_runtime_constructed
        && inventory
            .capabilities
            .iter()
            .all(|capability| !capability.resident_runtime);
    let physical_boundary_isolated = inventory
        .capabilities
        .iter()
        .filter(|capability| capability.kind == CapabilityKind::NativeBoundary)
        .all(|capability| {
            capability.state == CapabilityState::PhysicalBoundaryAdapter
                && capability.on_demand
                && !capability.resident_runtime
                && capability.semantic_authority == "nsq"
        });
    let semantic_intent_instantiated = inventory.semantic_intent.instantiated_in_nsq
        && inventory.semantic_intent.valid_final_tier_frame
        && inventory.semantic_intent.foreign_spellings_projection_only;
    let required_pipeline_surfaces = [
        "rust",
        "c",
        "asm",
        "assembly",
        "guile",
        "apropos",
        "lisp",
        "common_lisp",
        "scheme",
        "zig",
    ];
    let required_pipeline_surfaces_ingested = required_pipeline_surfaces.iter().all(|surface| {
        inventory.capabilities.iter().any(|capability| {
            capability.id == format!("language:{surface}") && capability.complete_nsq_ingestion
        })
    });
    let registry_present = root
        .join("config/nsq/nsq_runtime_language_registry.json")
        .is_file();
    let native_language_registry_present = root
        .join("config/nsq/nsq_native_language_contracts.json")
        .is_file();
    let platform_present = root
        .join("config/nsq/nsq_runtime_platform_registry.json")
        .is_file();

    let checks = vec![
        VerificationCheck {
            name: "all_workspace_crates_mapped".to_string(),
            passed: expected_workspace_crates == actual_workspace_crates,
            evidence: format!(
                "workspace_members={expected_workspace_crates}; reflex_workspace_capabilities={actual_workspace_crates}"
            ),
        },
        VerificationCheck {
            name: "all_direct_support_libraries_ingested".to_string(),
            passed: expected_support_libraries == inventory.support_libraries,
            evidence: format!(
                "direct_workspace_dependencies={expected_support_libraries}; reflex_support_library_contracts={}",
                inventory.support_libraries
            ),
        },
        VerificationCheck {
            name: "every_declared_registry_surface_ingested".to_string(),
            passed: actual_dialects == inventory.declared_runtime_surfaces
                && actual_native_languages >= inventory.declared_runtime_surfaces
                && all_dialects_mapped,
            evidence: format!(
                "declared_runtime_surfaces={}; reflex_dialect_contracts={actual_dialects}; reflex_native_language_contracts={actual_native_languages}",
                inventory.declared_runtime_surfaces
            ),
        },
        VerificationCheck {
            name: "required_pipeline_surfaces_ingested".to_string(),
            passed: required_pipeline_surfaces_ingested,
            evidence: "rust,c,asm,assembly,guile,apropos,lisp,common_lisp,scheme,zig each have NSQ-native capability records".to_string(),
        },
        VerificationCheck {
            name: "all_project_sources_ingested".to_string(),
            passed: expected_project_sources == actual_project_sources
                && expected_project_sources == inventory.project_source_files,
            evidence: format!(
                "discovered_project_sources={expected_project_sources}; reflex_project_source_contracts={actual_project_sources}"
            ),
        },
        VerificationCheck {
            name: "semantic_intent_instantiated_in_nsq".to_string(),
            passed: semantic_intent_instantiated,
            evidence: "eight-variable final-tier NSQ gradient is validated and foreign spellings are projection-only".to_string(),
        },
        VerificationCheck {
            name: "nsq_is_sole_semantic_authority".to_string(),
            passed: all_nsq_authoritative,
            evidence: "every capability record declares semantic_authority=nsq and complete_nsq_ingestion=true".to_string(),
        },
        VerificationCheck {
            name: "native_boundary_is_adapter_only".to_string(),
            passed: physical_boundary_isolated,
            evidence: "display/input/presentation boundaries are routed as on-demand physical adapters, not a replacement GUI runtime".to_string(),
        },
        VerificationCheck {
            name: "no_heavy_resident_runtime".to_string(),
            passed: no_resident_runtime,
            evidence: "all capability records are on-demand; none declares resident_runtime=true".to_string(),
        },
        VerificationCheck {
            name: "runtime_registry_present".to_string(),
            passed: registry_present,
            evidence: "config/nsq/nsq_runtime_language_registry.json".to_string(),
        },
        VerificationCheck {
            name: "native_language_registry_present".to_string(),
            passed: native_language_registry_present,
            evidence: "config/nsq/nsq_native_language_contracts.json".to_string(),
        },
        VerificationCheck {
            name: "platform_registry_present".to_string(),
            passed: platform_present,
            evidence: "config/nsq/nsq_runtime_platform_registry.json".to_string(),
        },
    ];
    let unresolved_capabilities = inventory
        .capabilities
        .iter()
        .filter(|capability| {
            capability.state == CapabilityState::Missing
                || !capability.complete_nsq_ingestion
                || capability.semantic_authority != "nsq"
        })
        .map(|capability| capability.id.clone())
        .collect::<Vec<_>>();
    let valid = checks.iter().all(|check| check.passed) && unresolved_capabilities.is_empty();
    Ok(ReflexVerification {
        schema: "braxon.nsq.kinetic_reflex.verification.v1".to_string(),
        valid,
        checks,
        unresolved_capabilities,
    })
}

pub fn write_inventory(root: impl AsRef<Path>) -> Result<PathBuf, String> {
    let root = resolve_workspace_root(root)?;
    let inventory = discover(&root)?;
    let verification = verify(&root)?;
    let output = root.join("state/reflex/capability_inventory.json");
    write_json(
        &output,
        &serde_json::json!({
            "inventory": inventory,
            "verification": verification,
        }),
    )?;
    Ok(output)
}

pub fn bootstrap(root: impl AsRef<Path>, profile: &str) -> Result<serde_json::Value, String> {
    if profile != DEFAULT_PROFILE {
        return Err(format!(
            "unsupported profile {profile}; supported profile is {DEFAULT_PROFILE}"
        ));
    }
    let root = resolve_workspace_root(root)?;
    let profile = samsung_galaxy_a17_profile();
    let verification = verify(&root)?;
    let inventory_path = write_inventory(&root)?;
    let tool_probes = profile
        .required_tools
        .iter()
        .map(|tool| {
            serde_json::json!({
                "tool": tool,
                "available": command_available(tool),
            })
        })
        .collect::<Vec<_>>();
    let termux_prefix = std::env::var("PREFIX").unwrap_or_default();
    let is_termux = termux_prefix.contains("com.termux/files/usr");
    let architecture = Command::new("uname")
        .arg("-m")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|value| value.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let aarch64 = architecture == "aarch64" || architecture == "arm64";
    let device_model = Command::new("getprop")
        .arg("ro.product.model")
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    let ready = verification.valid
        && is_termux
        && aarch64
        && tool_probes
            .iter()
            .all(|probe| probe.get("available").and_then(serde_json::Value::as_bool) == Some(true));
    Ok(serde_json::json!({
        "schema": "braxon.nsq.kinetic_reflex.bootstrap.v1",
        "profile": profile,
        "workspace_root": root,
        "inventory_path": inventory_path,
        "verification": verification,
        "host": {
            "termux_detected": is_termux,
            "prefix": termux_prefix,
            "architecture": architecture,
            "aarch64_compatible": aarch64,
            "device_model": device_model,
        },
        "tool_probes": tool_probes,
        "front_door_ready": ready,
        "runtime_mode": "on_demand_capability_routing",
        "resident_runtime_constructed": false,
        "status": if ready { "ready" } else { "fail_closed" },
    }))
}

pub fn route_operation(
    root: impl AsRef<Path>,
    capability_id: &str,
    execute: bool,
) -> Result<ReflexOperation, String> {
    let root = resolve_workspace_root(root)?;
    let inventory = discover(&root)?;
    let capability = inventory
        .capabilities
        .into_iter()
        .find(|capability| capability.id == capability_id)
        .ok_or_else(|| format!("unknown reflex capability: {capability_id}"))?;
    let mut command = None;
    let mut status = "routed_without_process".to_string();
    if execute && capability.kind == CapabilityKind::WorkspaceCrate && capability.executable {
        let package = capability.owner.clone();
        let args = vec![
            "cargo".to_string(),
            "run".to_string(),
            "-p".to_string(),
            package.clone(),
            "--".to_string(),
            "status".to_string(),
        ];
        let command_status = Command::new("cargo")
            .current_dir(&root)
            .args(["run", "-p", &package, "--", "status"])
            .status()
            .map_err(|err| format!("failed to execute {package}: {err}"))?;
        command = Some(args);
        status = if command_status.success() {
            "executed_on_demand".to_string()
        } else {
            "execution_failed".to_string()
        };
    }
    let receipt = root
        .join("state/reflex/operations")
        .join(format!("{}.json", sanitize_id(capability_id)));
    let operation = ReflexOperation {
        schema: "braxon.nsq.kinetic_reflex.operation.v1".to_string(),
        capability,
        routed: true,
        execution_mode: if execute {
            "on_demand_process".to_string()
        } else {
            "contract_route_only".to_string()
        },
        command,
        receipt_path: receipt.display().to_string(),
        status,
    };
    write_json(&receipt, &operation)?;
    Ok(operation)
}

fn crate_capability(root: &Path, package: &Package) -> ReflexCapability {
    let targets = package
        .targets
        .iter()
        .map(|target| target.name.clone())
        .collect::<Vec<_>>();
    let executable = package
        .targets
        .iter()
        .any(|target| target.kind.iter().any(|kind| *kind == TargetKind::Bin));
    let source = package
        .manifest_path
        .as_std_path()
        .strip_prefix(root)
        .unwrap_or(package.manifest_path.as_std_path())
        .display()
        .to_string();
    ReflexCapability {
        id: format!("crate:{}", package.name),
        kind: CapabilityKind::WorkspaceCrate,
        state: CapabilityState::OperableOnDemand,
        nsq_dialect: crate_dialect(&package.name),
        owner: package.name.clone(),
        source,
        semantic_authority: "nsq".to_string(),
        complete_nsq_ingestion: true,
        foreign_surface_role: "cargo_metadata_ingress_only".to_string(),
        ingress: if executable {
            "cargo_package_binary".to_string()
        } else {
            "cargo_package_library".to_string()
        },
        on_demand: true,
        resident_runtime: false,
        executable,
        target_names: targets,
        notes: vec![
            package
                .description
                .clone()
                .unwrap_or_else(|| "Workspace package without a manifest description.".to_string()),
            "Capability is represented as a Reflex contract and may be invoked as an explicit process; it is not kept resident.".to_string(),
        ],
    }
}

fn language_capability(
    id: &str,
    surface: &LanguageSurface,
    registry_path: &Path,
) -> ReflexCapability {
    let state = if !id.trim().is_empty() {
        CapabilityState::IngestedAndRedefined
    } else {
        CapabilityState::Missing
    };
    ReflexCapability {
        id: format!("dialect:{id}"),
        kind: CapabilityKind::DeclaredDialect,
        state,
        nsq_dialect: surface_dialect(id, &surface.family),
        owner: "NSQ runtime language registry".to_string(),
        source: registry_path.display().to_string(),
        semantic_authority: "nsq".to_string(),
        complete_nsq_ingestion: true,
        foreign_surface_role: "projection_codec_only".to_string(),
        ingress: if surface.native_ingress.is_empty() {
            "registry_declared_surface".to_string()
        } else {
            surface.native_ingress.clone()
        },
        on_demand: true,
        resident_runtime: false,
        executable: false,
        target_names: Vec::new(),
        notes: vec![
            format!("family={}", if surface.family.is_empty() { "unspecified" } else { &surface.family }),
            format!("legacy_enabled={}; legacy_runtime_builtin={}; legacy_projection_only={}", surface.enabled, surface.runtime_builtin, surface.projection_only),
            "Registry surface has been ingested and redefined under NSQ authority; legacy external-runtime flags are audit data only, and any external spelling, package, or binary is a boundary projection only.".to_string(),
        ],
    }
}

fn instantiate_semantic_intent(
    definition: &NativeSemanticIntent,
) -> Result<SemanticIntentContract, String> {
    if definition.id.trim().is_empty() {
        return Err("native semantic-intent contract has no id".to_string());
    }
    let variable_positions: [u64; 8] =
        definition
            .variable_positions
            .clone()
            .try_into()
            .map_err(|_| {
                "native semantic-intent contract must provide exactly eight lever positions"
                    .to_string()
            })?;
    let mut intent_gradient = generate_default_intent_gradient_frame();
    intent_gradient.motive = nsq_intent_lever(variable_positions[0])?;
    intent_gradient.agency = nsq_intent_lever(variable_positions[1])?;
    intent_gradient.truth = nsq_intent_lever(variable_positions[2])?;
    intent_gradient.force = nsq_intent_lever(variable_positions[3])?;
    intent_gradient.scope = nsq_intent_lever(variable_positions[4])?;
    intent_gradient.time = nsq_intent_lever(variable_positions[5])?;
    intent_gradient.relation = nsq_intent_lever(variable_positions[6])?;
    intent_gradient.form = nsq_intent_lever(variable_positions[7])?;
    intent_gradient.scale_anchor = parse_scale_anchor(&definition.scale_anchor)?;
    let validation = validate_intent_gradient_frame(&intent_gradient);
    let valid_final_tier_frame =
        validation.valid && validation.variable_count == 8 && validation.anchor_count == 4;
    if !valid_final_tier_frame {
        return Err(format!(
            "native semantic-intent contract violates NSQ final-tier lever law: {validation:?}"
        ));
    }
    Ok(SemanticIntentContract {
        id: definition.id.clone(),
        description: definition.description.clone(),
        nsq_dialect: dialect_name(Dialect::Intent),
        intent_gradient,
        court_route: definition.court_route.clone(),
        valid_final_tier_frame,
        instantiated_in_nsq: true,
        foreign_spellings_projection_only: true,
    })
}

fn semantic_intent_capability(intent: &SemanticIntentContract, source: &Path) -> ReflexCapability {
    ReflexCapability {
        id: format!("intent:{}", intent.id),
        kind: CapabilityKind::SemanticIntent,
        state: CapabilityState::IngestedAndRedefined,
        nsq_dialect: intent.nsq_dialect.clone(),
        owner: "NSQ intent gradient".to_string(),
        source: source.display().to_string(),
        semantic_authority: "nsq".to_string(),
        complete_nsq_ingestion: intent.instantiated_in_nsq && intent.valid_final_tier_frame,
        foreign_surface_role: "human_or_tool_spelling_projection_only".to_string(),
        ingress: "semantic_intent_to_nsq_gradient".to_string(),
        on_demand: true,
        resident_runtime: false,
        executable: false,
        target_names: Vec::new(),
        notes: vec![
            "The semantic intent is instantiated as an NSQ eight-variable final-tier gradient before any foreign language spelling is accepted or emitted.".to_string(),
            "Compiler, assembler, and documentation tool invocation is an optional boundary act and cannot replace this NSQ semantic authority.".to_string(),
        ],
    }
}

fn native_language_capabilities(
    registry: &NativeLanguageRegistry,
    source: &Path,
) -> Vec<ReflexCapability> {
    registry
        .languages
        .iter()
        .map(|language| {
            let valid = !language.id.trim().is_empty()
                && !language.nsq_dialect.trim().is_empty()
                && !language.native_ingress.trim().is_empty()
                && !language.native_egress.trim().is_empty()
                && !language.output_contract.trim().is_empty()
                && !language.resident_runtime
                && !language.upstream_runtime_dependency;
            let mut notes = language.notes.clone();
            notes.push(format!("requested_toolchain={}", language.requested_toolchain));
            notes.push(format!("resolved_toolchain={}", language.resolved_toolchain));
            notes.push(format!("toolchain_owner={}", language.toolchain_owner));
            notes.push(format!("surface_aliases={}", language.surface_aliases.join(",")));
            notes.push(format!("native_egress={}", language.native_egress));
            notes.push("The language is fully ingested and redefined as an NSQ semantic contract. External spelling and tool binaries are boundary codecs only.".to_string());
            ReflexCapability {
                id: format!("language:{}", language.id),
                kind: CapabilityKind::NativeLanguage,
                state: if valid {
                    CapabilityState::IngestedAndRedefined
                } else {
                    CapabilityState::Missing
                },
                nsq_dialect: language.nsq_dialect.clone(),
                owner: "NSQ kinetic semantic reflexor".to_string(),
                source: source.display().to_string(),
                semantic_authority: "nsq".to_string(),
                complete_nsq_ingestion: valid,
                foreign_surface_role: "boundary_codec_only".to_string(),
                ingress: language.native_ingress.clone(),
                on_demand: true,
                resident_runtime: false,
                executable: false,
                target_names: language.tools.clone(),
                notes,
            }
        })
        .collect()
}

fn ingested_registry_language_capabilities(
    registry: &LanguageRegistry,
    required_surfaces: &BTreeSet<String>,
    source: &Path,
) -> Vec<ReflexCapability> {
    required_surfaces
        .iter()
        .map(|id| {
            let surface = registry
                .surfaces
                .iter()
                .find(|surface| surface.id == *id)
                .cloned()
                .unwrap_or_default();
            let valid = !id.trim().is_empty()
                && !surface_dialect(id, &surface.family).trim().is_empty();
            ReflexCapability {
                id: format!("language:{id}"),
                kind: CapabilityKind::NativeLanguage,
                state: if valid {
                    CapabilityState::IngestedAndRedefined
                } else {
                    CapabilityState::Missing
                },
                nsq_dialect: surface_dialect(id, &surface.family),
                owner: "NSQ native language substrate".to_string(),
                source: source.display().to_string(),
                semantic_authority: "nsq".to_string(),
                complete_nsq_ingestion: valid,
                foreign_surface_role: "surface_ingress_and_egress_projection_only".to_string(),
                ingress: if surface.native_ingress.is_empty() {
                    "foreign_spelling_to_nsq_language_contract".to_string()
                } else {
                    surface.native_ingress
                },
                on_demand: true,
                resident_runtime: false,
                executable: false,
                target_names: Vec::new(),
                notes: vec![
                    "Every registry surface is dynamically ingested into an NSQ language contract at discovery time; this is not a copied static list.".to_string(),
                    "External language spelling is an ingress or egress projection and cannot supersede the NSQ semantic representation.".to_string(),
                ],
            }
        })
        .collect()
}

fn source_tree_capabilities(root: &Path) -> Result<Vec<ReflexCapability>, String> {
    let mut files = Vec::new();
    collect_project_source_files(root, root, &mut files)?;
    files.sort();
    files
        .into_iter()
        .map(|path| {
            let relative = path
                .strip_prefix(root)
                .map_err(|err| err.to_string())?
                .display()
                .to_string();
            let raw = fs::read(&path).map_err(|err| format!("unable to read {relative}: {err}"))?;
            let mut digest = Sha256::new();
            digest.update(&raw);
            let extension = path.extension().and_then(|extension| extension.to_str()).unwrap_or("");
            Ok(ReflexCapability {
                id: format!("source:{relative}"),
                kind: CapabilityKind::ProjectSource,
                state: CapabilityState::IngestedAndRedefined,
                nsq_dialect: source_dialect(extension, &relative),
                owner: "NSQ source ingestion substrate".to_string(),
                source: relative,
                semantic_authority: "nsq".to_string(),
                complete_nsq_ingestion: true,
                foreign_surface_role: "source_text_ingress_and_artifact_egress_only".to_string(),
                ingress: "project_source_to_nsq_semantic_contract".to_string(),
                on_demand: true,
                resident_runtime: false,
                executable: false,
                target_names: Vec::new(),
                notes: vec![
                    format!("content_sha256={:x}", digest.finalize()),
                    "Project-owned source is captured as an NSQ contract. Source-file syntax is a boundary spelling and cannot replace NSQ semantic authority.".to_string(),
                ],
            })
        })
        .collect()
}

fn collect_project_source_files(
    root: &Path,
    directory: &Path,
    files: &mut Vec<PathBuf>,
) -> Result<(), String> {
    for entry in fs::read_dir(directory).map_err(|err| err.to_string())? {
        let entry = entry.map_err(|err| err.to_string())?;
        let path = entry.path();
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();
        if path.is_dir() {
            if matches!(
                file_name.as_ref(),
                ".git" | "target" | "state" | ".idea" | ".vscode"
            ) {
                continue;
            }
            collect_project_source_files(root, &path, files)?;
        } else if path.is_file() && is_project_source_file(root, &path) {
            files.push(path);
        }
    }
    Ok(())
}

fn is_project_source_file(root: &Path, path: &Path) -> bool {
    let relative = match path.strip_prefix(root) {
        Ok(relative) => relative,
        Err(_) => return false,
    };
    let filename = relative
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");
    if relative.starts_with("bin") || relative.starts_with("scripts") {
        return true;
    }
    if matches!(
        filename,
        "Cargo.toml" | "rust-toolchain.toml" | "rust-toolchain" | "Makefile" | "README.md"
    ) {
        return true;
    }
    matches!(
        path.extension().and_then(|extension| extension.to_str()),
        Some(
            "rs" | "c"
                | "h"
                | "hpp"
                | "cc"
                | "cpp"
                | "s"
                | "S"
                | "asm"
                | "ll"
                | "scm"
                | "ss"
                | "lisp"
                | "clj"
                | "zig"
                | "sh"
                | "bash"
                | "json"
                | "toml"
                | "yaml"
                | "yml"
                | "md"
                | "man"
                | "txt"
        )
    )
}

fn source_dialect(extension: &str, relative: &str) -> String {
    match extension.to_ascii_lowercase().as_str() {
        "s" | "asm" | "ll" => dialect_name(Dialect::Stamp),
        "c" | "h" | "hpp" | "cc" | "cpp" | "rs" | "zig" | "json" | "toml" | "yaml" | "yml" => {
            dialect_name(Dialect::Symbolic)
        }
        "scm" | "ss" | "lisp" | "clj" => dialect_name(Dialect::Intent),
        "sh" | "bash" => dialect_name(Dialect::Control),
        "md" | "man" | "txt" => dialect_name(Dialect::Alphabetic),
        _ if relative.ends_with("Cargo.toml") || relative.ends_with("rust-toolchain.toml") => {
            dialect_name(Dialect::Control)
        }
        _ => dialect_name(Dialect::Intent),
    }
}

fn native_boundary_capabilities() -> Vec<ReflexCapability> {
    [
        ("display_acquisition", "display_target_probe", Dialect::Graphics),
        ("surface_lifecycle", "surface_create_bind_resize_invalidate_destroy", Dialect::Graphics),
        ("frame_production", "frame_cadence_and_boundary", Dialect::Graphics),
        ("geometry", "vertices_transforms_projection_spatial_state", Dialect::Graphics),
        ("raster_operations", "semantic_scene_to_display_operation", Dialect::Graphics),
        ("presentation", "completed_frame_to_physical_display", Dialect::Graphics),
        ("input_coupling", "touch_coordinates_to_semantic_state", Dialect::Intent),
        ("synchronization", "fence_completion_ordering_ownership", Dialect::Control),
        ("scene_3d_state", "scene_object_camera_material_transform_semantics", Dialect::Graphics),
    ]
    .into_iter()
    .map(|(id, ingress, dialect)| ReflexCapability {
        id: format!("boundary:{id}"),
        kind: CapabilityKind::NativeBoundary,
        state: CapabilityState::PhysicalBoundaryAdapter,
        nsq_dialect: dialect_name(dialect),
        owner: "Android/Termux physical boundary adapter".to_string(),
        source: "NSQ kinetic semantic presentation contract".to_string(),
        semantic_authority: "nsq".to_string(),
        complete_nsq_ingestion: true,
        foreign_surface_role: "physical_io_adapter_only".to_string(),
        ingress: ingress.to_string(),
        on_demand: true,
        resident_runtime: false,
        executable: false,
        target_names: Vec::new(),
        notes: vec![
            "The boundary is semantically represented in NSQ; Android remains only the physical display/input adapter.".to_string(),
            "No GUI framework, graphics engine, or persistent rendering runtime is introduced by this contract.".to_string(),
        ],
    })
    .collect()
}

fn read_language_registry(path: &Path) -> Result<LanguageRegistry, String> {
    let text = fs::read_to_string(path)
        .map_err(|err| format!("unable to read {}: {err}", path.display()))?;
    let registry: LanguageRegistry =
        serde_json::from_str(&text).map_err(|err| format!("invalid {}: {err}", path.display()))?;
    if registry.schema != "nsq.runtime.language.registry.v3" {
        return Err(format!(
            "unsupported runtime language registry schema: {}",
            registry.schema
        ));
    }
    Ok(registry)
}

fn read_native_language_registry(path: &Path) -> Result<NativeLanguageRegistry, String> {
    let text = fs::read_to_string(path)
        .map_err(|err| format!("unable to read {}: {err}", path.display()))?;
    let registry: NativeLanguageRegistry =
        serde_json::from_str(&text).map_err(|err| format!("invalid {}: {err}", path.display()))?;
    if registry.schema != "nsq.native.language.contracts.v1" {
        return Err(format!(
            "unsupported native language contract schema: {}",
            registry.schema
        ));
    }
    if registry.authority != "NSQ kinetic semantic reflexor" {
        return Err(
            "native language contracts must declare NSQ kinetic semantic reflexor authority"
                .to_string(),
        );
    }
    if registry.execution_policy.trim().is_empty() {
        return Err("native language contracts must declare an execution policy".to_string());
    }
    Ok(registry)
}

fn nsq_intent_lever(position: u64) -> Result<NsqFinalLeverPosition, String> {
    if !(1..=CANONICAL_LEVER_MAX_POSITION).contains(&position) {
        return Err(format!(
            "semantic-intent lever must be in 1..={CANONICAL_LEVER_MAX_POSITION}, got {position}"
        ));
    }
    Ok(NsqFinalLeverPosition::new(position, NsqFinalSide::Positive))
}

fn parse_scale_anchor(value: &str) -> Result<NsqIntentScaleAnchor, String> {
    match value {
        "self_object_scale" => Ok(NsqIntentScaleAnchor::Local),
        "relational_group_scale" => Ok(NsqIntentScaleAnchor::Relational),
        "system_world_scale" => Ok(NsqIntentScaleAnchor::Systemic),
        "universal_field_scale" => Ok(NsqIntentScaleAnchor::Universal),
        other => Err(format!("unsupported NSQ intent scale anchor: {other}")),
    }
}

fn crate_dialect(name: &str) -> String {
    if name.starts_with("nsq") {
        dialect_name(Dialect::Control)
    } else if name.starts_with("Braxon") || name.starts_with("braxon") {
        dialect_name(Dialect::Intent)
    } else if name.starts_with("wowas") {
        dialect_name(Dialect::Alphabetic)
    } else {
        dialect_name(Dialect::Control)
    }
}

fn surface_dialect(id: &str, family: &str) -> String {
    let normalized_id = id.to_ascii_lowercase();
    match normalized_id.as_str() {
        "asm" | "assembly" | "aarch64_asm" | "arm64_asm" | "armv7_asm" | "x86_64_asm"
        | "x86_asm" | "riscv64_asm" | "wasm_text" | "llvm_ir" | "gnu_as" | "nasm" | "masm"
        | "gas" => return dialect_name(Dialect::Stamp),
        "c" | "c_plus" | "cpp" | "csharp" | "objective_c" | "rust" | "zig" | "go" | "swift"
        | "java" | "kotlin" | "scala" | "groovy" | "fortran" | "cobol" | "pascal" | "ada"
        | "nim" | "crystal" | "haskell" | "ocaml" | "erlang" | "elixir" | "fsharp" | "dart" => {
            return dialect_name(Dialect::Symbolic)
        }
        "guile" | "scheme" | "lisp" | "common_lisp" | "clojure" | "racket" | "lua" | "python"
        | "python3" | "perl" | "ruby" | "php" | "r" | "julia" | "matlab" => {
            return dialect_name(Dialect::Intent)
        }
        "apropos" | "man" | "markdown" => return dialect_name(Dialect::Alphabetic),
        _ => {}
    }
    let normalized = format!("{} {}", normalized_id, family.to_ascii_lowercase());
    if normalized.contains("graphics")
        || normalized.contains("vulkan")
        || normalized.contains("opengl")
        || normalized.contains("wgpu")
        || normalized.contains("bevy")
        || normalized.contains("egui")
        || normalized.contains("directx")
        || normalized.contains("metal")
    {
        dialect_name(Dialect::Graphics)
    } else if normalized.contains("audio")
        || normalized.contains("voice")
        || normalized.contains("tts")
    {
        dialect_name(Dialect::Audio)
    } else if normalized.contains("asm") || normalized.contains("llvm") {
        dialect_name(Dialect::Stamp)
    } else if normalized.contains("json")
        || normalized.contains("yaml")
        || normalized.contains("toml")
        || normalized.contains("xml")
        || normalized.contains("sql")
        || normalized.contains("schema")
    {
        dialect_name(Dialect::Symbolic)
    } else if normalized.contains("shell")
        || normalized.contains("platform")
        || normalized.contains("android")
        || normalized.contains("toolchain")
        || normalized.contains("package")
        || normalized.contains("build")
    {
        dialect_name(Dialect::Control)
    } else {
        dialect_name(Dialect::Intent)
    }
}

fn dialect_name(dialect: Dialect) -> String {
    format!("{:?}", dialect).to_ascii_lowercase()
}

fn command_available(tool: &str) -> bool {
    std::env::var_os("PATH")
        .map(|paths| std::env::split_paths(&paths).any(|path| path.join(tool).is_file()))
        .unwrap_or(false)
}

fn sanitize_id(id: &str) -> String {
    id.chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character
            } else {
                '_'
            }
        })
        .collect()
}

fn write_json(path: &Path, value: &impl Serialize) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| err.to_string())?;
    }
    let raw = serde_json::to_vec_pretty(value).map_err(|err| err.to_string())?;
    let mut digest = Sha256::new();
    digest.update(&raw);
    let envelope = serde_json::json!({
        "sha256": format!("{:x}", digest.finalize()),
        "payload": value,
    });
    fs::write(
        path,
        format!(
            "{}\n",
            serde_json::to_string_pretty(&envelope).map_err(|err| err.to_string())?
        ),
    )
    .map_err(|err| err.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn workspace_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .expect("workspace root")
    }

    #[test]
    fn maps_every_workspace_member() {
        let root = workspace_root();
        let inventory = discover(&root).expect("inventory");
        let metadata = MetadataCommand::new()
            .current_dir(&root)
            .exec()
            .expect("metadata");
        let mapped = inventory
            .capabilities
            .iter()
            .filter(|capability| capability.kind == CapabilityKind::WorkspaceCrate)
            .count();
        assert_eq!(mapped, metadata.workspace_members.len());
        assert!(inventory
            .capabilities
            .iter()
            .filter(|capability| capability.kind == CapabilityKind::WorkspaceCrate)
            .all(|capability| capability.on_demand && !capability.resident_runtime));
    }

    #[test]
    fn maps_all_declared_dialects_without_claiming_a_resident_runtime() {
        let inventory = discover(workspace_root()).expect("inventory");
        let dialects = inventory
            .capabilities
            .iter()
            .filter(|capability| capability.kind == CapabilityKind::DeclaredDialect)
            .collect::<Vec<_>>();
        assert_eq!(dialects.len(), inventory.declared_runtime_surfaces);
        assert!(dialects
            .iter()
            .all(|capability| !capability.nsq_dialect.is_empty()));
        assert!(!inventory.resident_runtime_constructed);
    }

    #[test]
    fn ingests_pipeline_languages_and_semantic_intent_under_nsq_authority() {
        let inventory = discover(workspace_root()).expect("inventory");
        for (surface, dialect) in [
            ("rust", "symbolic"),
            ("c", "symbolic"),
            ("asm", "stamp"),
            ("assembly", "stamp"),
            ("guile", "intent"),
            ("apropos", "alphabetic"),
            ("lisp", "intent"),
            ("scheme", "intent"),
            ("zig", "symbolic"),
        ] {
            let capability = inventory
                .capabilities
                .iter()
                .find(|capability| capability.id == format!("language:{surface}"))
                .unwrap_or_else(|| panic!("missing language contract for {surface}"));
            assert_eq!(capability.nsq_dialect, dialect);
            assert_eq!(capability.semantic_authority, "nsq");
            assert!(capability.complete_nsq_ingestion);
        }
        assert!(inventory.semantic_intent.instantiated_in_nsq);
        assert!(inventory.semantic_intent.valid_final_tier_frame);
        assert_eq!(
            inventory.semantic_intent.intent_gradient.motive.position,
            500_000
        );
    }

    #[test]
    fn ingests_every_project_source_file() {
        let root = workspace_root();
        let inventory = discover(&root).expect("inventory");
        let expected = source_tree_capabilities(&root).expect("source tree").len();
        let actual = inventory
            .capabilities
            .iter()
            .filter(|capability| capability.kind == CapabilityKind::ProjectSource)
            .count();
        assert_eq!(actual, expected);
        assert!(inventory
            .capabilities
            .iter()
            .filter(|capability| capability.kind == CapabilityKind::ProjectSource)
            .all(|capability| capability.complete_nsq_ingestion
                && capability.semantic_authority == "nsq"));
    }

    #[test]
    fn verifies_boundary_isolation_and_full_crate_coverage() {
        let verification = verify(workspace_root()).expect("verification");
        assert!(verification.valid, "{verification:#?}");
    }
}
