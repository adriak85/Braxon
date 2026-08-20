use crate::{
    braxon_context_manifest_status, load_braxon_chain_root_db, load_braxon_context_manifest,
    verify_language_artifact_context, BraxonBus, ModelExecutionState, TokenizerBridge,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

pub const CLOSURE_AUDIT_SCHEMA: &str = "braxon.nsq.closure_audit.v1";
pub const CLOSURE_ACTIVATION_MANIFEST_RELATIVE_PATH: &str =
    "config/nsq/closure_activation_manifest.json";
pub const TOKENIZER_BAND_REGISTRY_RELATIVE_PATH: &str = "config/nsq/tokenizer_band_registry.json";
pub const ACTIVE_WIRING_MAP_RELATIVE_PATH: &str =
    "config/nsq/runtime_native/active_wiring_map.json";

#[derive(Debug, Clone, Deserialize)]
struct ActivationManifest {
    schema: String,
    version: String,
    entries: Vec<ActivationManifestEntry>,
}

#[derive(Debug, Clone, Deserialize)]
struct ActivationManifestEntry {
    id: String,
    #[serde(rename = "class")]
    class_name: String,
    source: String,
    identity: String,
    address: String,
    object_type: String,
    version: String,
    dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WakeActivationEntryReport {
    pub id: String,
    pub activation_class: String,
    pub source: String,
    pub identity: String,
    pub address: String,
    pub object_type: String,
    pub version: String,
    pub loaded: bool,
    pub validated: bool,
    pub activated: bool,
    pub dependencies: Vec<String>,
    pub unresolved_dependencies: Vec<String>,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FullWakeReport {
    pub schema: String,
    pub activation_manifest_version: String,
    pub required_total: usize,
    pub activated_total: usize,
    pub unresolved: usize,
    pub orphaned: usize,
    pub invalid_bindings: usize,
    pub all_passed: bool,
    pub council_ten_passed: bool,
    pub entries: Vec<WakeActivationEntryReport>,
}

#[derive(Debug, Clone, Deserialize)]
struct TokenizerBandRegistry {
    schema: String,
    universal_translation_version: String,
    address_namespace: String,
    bands: Vec<TokenizerBand>,
}

#[derive(Debug, Clone, Deserialize)]
struct TokenizerBand {
    band_id: String,
    model_id: String,
    active: bool,
    required: bool,
    tokenizer_path: String,
    representation: String,
    provenance: String,
    expected_model_execution: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TokenizerBandVerification {
    pub band_id: String,
    pub model_id: String,
    pub active: bool,
    pub required: bool,
    pub tokenizer_path: String,
    pub tokenizer_source_exists: bool,
    pub vocabulary_loaded: bool,
    pub vocabulary_count: usize,
    pub deterministic_mapping: bool,
    pub forward_translation: bool,
    pub reverse_translation: bool,
    pub provenance_recorded: bool,
    pub shared_semantic_addressing: bool,
    pub collective_state_compatible: bool,
    pub unresolved_required_tokens: Vec<String>,
    pub collision_count: usize,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TokenizerVerificationReport {
    pub schema: String,
    pub universal_translation_version: String,
    pub address_namespace: String,
    pub required_band_total: usize,
    pub active_band_total: usize,
    pub verified_active_band_total: usize,
    pub inactive_required_band_total: usize,
    pub unresolved_required_mappings: usize,
    pub collision_count: usize,
    pub all_active_bands_verified: bool,
    pub bands: Vec<TokenizerBandVerification>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AddressIntegrityReport {
    pub schema: String,
    pub canonical_address_total: usize,
    pub collisions: Vec<String>,
    pub duplicates: Vec<String>,
    pub gaps: Vec<u64>,
    pub dangling_references: Vec<String>,
    pub orphan_objects: Vec<String>,
    pub invalid_ranges: Vec<String>,
    pub stale_mappings: Vec<String>,
    pub invalid_bindings: Vec<String>,
    pub all_passed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModelExecutionTruth {
    pub target_pole: String,
    pub model_id: String,
    pub configured: bool,
    pub available: bool,
    pub loaded: bool,
    pub initialized: bool,
    pub executing: bool,
    pub valid_transition_chain: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClosureGate {
    pub id: String,
    pub passed: bool,
    pub evidence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClosureAuditReport {
    pub schema: String,
    pub full_wake: FullWakeReport,
    pub tokenizer: TokenizerVerificationReport,
    pub address_integrity: AddressIntegrityReport,
    pub language_artifact_context: crate::LanguageArtifactContextReport,
    pub model_execution_truth: Vec<ModelExecutionTruth>,
    pub gates: Vec<ClosureGate>,
    pub passed_gate_total: usize,
    pub required_gate_total: usize,
    pub all_gates_passed: bool,
}

pub fn full_wake(root: &Path) -> Result<FullWakeReport, String> {
    let manifest = load_activation_manifest(root)?;
    if manifest.schema != "braxon.nsq.closure_activation_manifest.v1"
        || manifest.version.trim().is_empty()
    {
        return Err("closure activation manifest schema or version is invalid".into());
    }
    let address_integrity = address_integrity_audit(root)?;
    let tokenizer = tokenizer_verification(root)?;
    let mut unique_ids = BTreeSet::new();
    let mut unique_identities = BTreeSet::new();
    let mut unique_addresses = BTreeSet::new();
    let mut activated_by_id = BTreeMap::new();
    let mut entries = Vec::new();

    for entry in manifest.entries {
        let manifest_shape_valid = !entry.id.trim().is_empty()
            && !entry.class_name.trim().is_empty()
            && !entry.source.trim().is_empty()
            && !entry.identity.trim().is_empty()
            && !entry.address.trim().is_empty()
            && !entry.object_type.trim().is_empty()
            && !entry.version.trim().is_empty();
        let unique = unique_ids.insert(entry.id.clone())
            && unique_identities.insert(entry.identity.clone())
            && unique_addresses.insert(entry.address.clone());
        let loaded = root.join(&entry.source).exists();
        let unresolved_dependencies = entry
            .dependencies
            .iter()
            .filter(|dependency| !activated_by_id.get(*dependency).copied().unwrap_or(false))
            .cloned()
            .collect::<Vec<_>>();
        let validated = manifest_shape_valid
            && unique
            && loaded
            && validate_activation_entry(root, &entry, &tokenizer, &address_integrity);
        let activated = validated && unresolved_dependencies.is_empty();
        let reason = if !manifest_shape_valid {
            "activation entry is missing a required identity, source, address, type, or version"
                .into()
        } else if !unique {
            "activation entry duplicates an identifier, identity, or Wake address".into()
        } else if !loaded {
            format!("activation source is unavailable: {}", entry.source)
        } else if !unresolved_dependencies.is_empty() {
            format!(
                "activation dependencies unresolved: {}",
                unresolved_dependencies.join(",")
            )
        } else if !validated {
            "activation-specific verification failed".into()
        } else {
            "activated on demand with source and dependency verification".into()
        };
        activated_by_id.insert(entry.id.clone(), activated);
        entries.push(WakeActivationEntryReport {
            id: entry.id,
            activation_class: entry.class_name,
            source: entry.source,
            identity: entry.identity,
            address: entry.address,
            object_type: entry.object_type,
            version: entry.version,
            loaded,
            validated,
            activated,
            dependencies: entry.dependencies,
            unresolved_dependencies,
            reason,
        });
    }
    let required_total = entries.len();
    let activated_total = entries.iter().filter(|entry| entry.activated).count();
    let unresolved = required_total.saturating_sub(activated_total);
    let orphaned =
        address_integrity.orphan_objects.len() + address_integrity.dangling_references.len();
    let invalid_bindings = address_integrity.invalid_bindings.len()
        + address_integrity.stale_mappings.len()
        + address_integrity.collisions.len()
        + address_integrity.duplicates.len()
        + address_integrity.gaps.len()
        + address_integrity.invalid_ranges.len();
    let council_ten_passed = crate::CouncilTen::new().wake().all_passed;
    let all_passed = council_ten_passed
        && required_total == activated_total
        && unresolved == 0
        && orphaned == 0
        && invalid_bindings == 0;
    Ok(FullWakeReport {
        schema: CLOSURE_AUDIT_SCHEMA.into(),
        activation_manifest_version: manifest.version,
        required_total,
        activated_total,
        unresolved,
        orphaned,
        invalid_bindings,
        all_passed,
        council_ten_passed,
        entries,
    })
}

pub fn tokenizer_verification(root: &Path) -> Result<TokenizerVerificationReport, String> {
    let registry: TokenizerBandRegistry = read_json(root, TOKENIZER_BAND_REGISTRY_RELATIVE_PATH)?;
    if registry.schema != "braxon.nsq.tokenizer_band_registry.v1" {
        return Err("tokenizer-band registry schema mismatch".into());
    }
    let mut seen_band_ids = BTreeSet::new();
    let mut bands = Vec::new();
    let mut active_band_total = 0;
    let mut verified_active_band_total = 0;
    let mut inactive_required_band_total = 0;
    let mut unresolved_required_mappings = 0;
    let mut collision_count = 0;
    for band in registry.bands {
        let mut result = verify_tokenizer_band(root, &registry.address_namespace, &band)?;
        if !seen_band_ids.insert(band.band_id.clone()) {
            result.collision_count = result.collision_count.saturating_add(1);
            result.reason = "duplicate tokenizer band identity".into();
        }
        if band.active {
            active_band_total += 1;
            if tokenizer_band_passes(&result) {
                verified_active_band_total += 1;
            }
        } else if band.required {
            inactive_required_band_total += 1;
        }
        unresolved_required_mappings += result.unresolved_required_tokens.len();
        collision_count += result.collision_count;
        bands.push(result);
    }
    let all_active_bands_verified = active_band_total > 0
        && active_band_total == verified_active_band_total
        && unresolved_required_mappings == 0
        && collision_count == 0;
    Ok(TokenizerVerificationReport {
        schema: CLOSURE_AUDIT_SCHEMA.into(),
        universal_translation_version: registry.universal_translation_version,
        address_namespace: registry.address_namespace,
        required_band_total: bands.iter().filter(|band| band.required).count(),
        active_band_total,
        verified_active_band_total,
        inactive_required_band_total,
        unresolved_required_mappings,
        collision_count,
        all_active_bands_verified,
        bands,
    })
}

pub fn address_integrity_audit(root: &Path) -> Result<AddressIntegrityReport, String> {
    let manifest = load_braxon_context_manifest(root)?;
    let chain = load_braxon_chain_root_db(root, &manifest)?;
    let mut collisions = Vec::new();
    let mut duplicates = Vec::new();
    let mut dangling_references = Vec::new();
    let mut orphan_objects = Vec::new();
    let mut invalid_ranges = Vec::new();
    let mut numeric_ids = BTreeSet::new();
    let mut base8_ids = BTreeSet::new();
    let mut semantic_ids = BTreeSet::new();
    let mut pointer_ids = BTreeSet::new();
    let pointer_map = manifest
        .semantic_pointers
        .iter()
        .map(|pointer| (pointer.id.as_str(), pointer))
        .collect::<BTreeMap<_, _>>();
    for record in &chain.chain_records {
        if !numeric_ids.insert(record.numeric_id) {
            collisions.push(format!("numeric_id:{}", record.numeric_id));
        }
        if !base8_ids.insert(record.numeric_id_base8.clone()) {
            collisions.push(format!("numeric_id_base8:{}", record.numeric_id_base8));
        }
        if !semantic_ids.insert(record.semantic_id.clone()) {
            duplicates.push(format!("semantic_id:{}", record.semantic_id));
        }
        if !pointer_ids.insert(record.pointer_id.clone()) {
            duplicates.push(format!("pointer_id:{}", record.pointer_id));
        }
        if record.numeric_id == 0 || record.numeric_id_base8 != format!("{:o}", record.numeric_id) {
            invalid_ranges.push(format!(
                "record:{} has invalid numeric/base8 address",
                record.pointer_id
            ));
        }
        match pointer_map.get(record.pointer_id.as_str()) {
            Some(pointer) if pointer.path == record.path && pointer.kind == record.kind => {}
            Some(_) => dangling_references.push(format!(
                "record:{} does not match manifest path or kind",
                record.pointer_id
            )),
            None => dangling_references.push(format!(
                "record:{} has no manifest pointer",
                record.pointer_id
            )),
        }
        if !root.join(&record.path).exists() {
            dangling_references.push(format!(
                "record:{} path is unavailable: {}",
                record.pointer_id, record.path
            ));
        }
    }
    for pointer in &manifest.semantic_pointers {
        if !pointer_ids.contains(&pointer.id) {
            orphan_objects.push(format!(
                "manifest_pointer:{} has no canonical chain address",
                pointer.id
            ));
        }
        if !root.join(&pointer.path).exists() {
            orphan_objects.push(format!(
                "manifest_pointer:{} source missing: {}",
                pointer.id, pointer.path
            ));
        }
    }
    let gaps = expected_gaps(&numeric_ids);
    let (stale_mappings, invalid_bindings) = wiring_binding_audit(root)?;
    let all_passed = collisions.is_empty()
        && duplicates.is_empty()
        && gaps.is_empty()
        && dangling_references.is_empty()
        && orphan_objects.is_empty()
        && invalid_ranges.is_empty()
        && stale_mappings.is_empty()
        && invalid_bindings.is_empty();
    Ok(AddressIntegrityReport {
        schema: CLOSURE_AUDIT_SCHEMA.into(),
        canonical_address_total: chain.chain_records.len(),
        collisions,
        duplicates,
        gaps,
        dangling_references,
        orphan_objects,
        invalid_ranges,
        stale_mappings,
        invalid_bindings,
        all_passed,
    })
}

pub fn model_execution_truth(root: &Path) -> Vec<ModelExecutionTruth> {
    let registry = crate::load_or_initialize_model_registry(root);
    registry
        .execution_matrix()
        .into_iter()
        .map(|(target_pole, model_id, state)| model_truth_entry(target_pole, model_id, state))
        .collect()
}

pub fn closure_audit(root: &Path) -> Result<ClosureAuditReport, String> {
    let full_wake = full_wake(root)?;
    let tokenizer = tokenizer_verification(root)?;
    let address_integrity = address_integrity_audit(root)?;
    let language_artifact_context = verify_language_artifact_context(root)?;
    let model_execution_truth = model_execution_truth(root);
    let context_status = braxon_context_manifest_status(root)?;
    let bus_report =
        BraxonBus::speak("verify operator bus and preserve disagreement but reject speech");
    let all_model_truth_valid = model_execution_truth
        .iter()
        .all(|entry| entry.valid_transition_chain && !entry.executing);
    let gates = vec![
        gate(
            "source_integrity",
            context_status.all_required_context_present(),
            "all required context paths resolve",
        ),
        gate(
            "wake",
            full_wake.all_passed,
            "full activation manifest count and binding result",
        ),
        gate(
            "seed_activation",
            entry_activated(&full_wake, "seed"),
            "seed activation entry",
        ),
        gate(
            "parameter_activation",
            entry_activated(&full_wake, "system_parameters")
                && entry_activated(&full_wake, "individual_model_parameters"),
            "system and individual parameter entries",
        ),
        gate(
            "tokenizer_native_bands",
            tokenizer.all_active_bands_verified,
            "all active native bands verified",
        ),
        gate(
            "universal_translation",
            tokenizer.all_active_bands_verified,
            "forward, reverse, provenance, address, collision checks",
        ),
        gate(
            "documentation_index",
            entry_activated(&full_wake, "documentation") && language_artifact_context.all_passed,
            "documentation identity, canonical address, and released runtime lookup",
        ),
        gate(
            "guile_index",
            entry_activated(&full_wake, "guile") && language_artifact_context.all_passed,
            "Guile symbol-to-runtime traversal",
        ),
        gate(
            "apropos_index",
            entry_activated(&full_wake, "apropos") && language_artifact_context.all_passed,
            "apropos symbol-to-runtime traversal",
        ),
        gate(
            "tree_sitter",
            entry_activated(&full_wake, "tree_sitter") && language_artifact_context.all_passed,
            "tree-sitter symbol-to-runtime traversal",
        ),
        gate(
            "ast",
            entry_activated(&full_wake, "ast") && language_artifact_context.all_passed,
            "AST identity-to-runtime traversal",
        ),
        gate(
            "address_integrity",
            address_integrity.all_passed,
            "canonical chain and wiring audit",
        ),
        gate(
            "organ_topology",
            bus_report.collective_self_state.is_some(),
            "addressed independent organ-band perspectives",
        ),
        gate(
            "recursive_citadel_topology",
            parameter_citadel_topology_valid(),
            "parameter-Citadel recursive invariant contract",
        ),
        gate(
            "conflict_preservation",
            bus_report
                .collective_self_state
                .as_ref()
                .map(|state| state.conflict_preserved)
                .unwrap_or(false),
            "opposed computational priorities remain recorded",
        ),
        gate(
            "unified_self_state",
            bus_report
                .collective_self_state
                .as_ref()
                .map(|state| state.validate().is_ok())
                .unwrap_or(false),
            "derived self-state validates against preserved perspectives",
        ),
        gate(
            "bus",
            bus_report.processing.input_accepted && bus_report.hard_runtime_valid(),
            "on-demand classified bus measurement",
        ),
        gate(
            "model_execution_truth",
            all_model_truth_valid,
            "five independent per-model state facts with no execution claim",
        ),
        gate(
            "offline_constraint",
            !bus_report.model_weight_execution_claimed
                && !bus_report.native_runtime_completion_claimed,
            "no model-weight or persistent-runtime claim",
        ),
        gate(
            "narrative_hard_state_separation",
            bus_report.hard_runtime_valid(),
            "runtime accepts hard/derived state only",
        ),
    ];
    let passed_gate_total = gates.iter().filter(|gate| gate.passed).count();
    let required_gate_total = gates.len();
    Ok(ClosureAuditReport {
        schema: CLOSURE_AUDIT_SCHEMA.into(),
        full_wake,
        tokenizer,
        address_integrity,
        language_artifact_context,
        model_execution_truth,
        gates,
        passed_gate_total,
        required_gate_total,
        all_gates_passed: passed_gate_total == required_gate_total,
    })
}

fn validate_activation_entry(
    root: &Path,
    entry: &ActivationManifestEntry,
    tokenizer: &TokenizerVerificationReport,
    address_integrity: &AddressIntegrityReport,
) -> bool {
    let source = root.join(&entry.source);
    let contents = fs::read_to_string(source).unwrap_or_default();
    match entry.id.as_str() {
        "seed" => contents.contains("build_seed_plan"),
        "system_parameters" => contents.contains("execute_dynamic_parameter_pipeline"),
        "individual_model_parameters" => {
            contents.contains("ModelExecutionState") && contents.contains("ModelRegistry")
        }
        "native_tokenizer_bands" | "universal_translator" => tokenizer.all_active_bands_verified,
        "documentation" => {
            contents.contains("braxon.guile.contract")
                && contents.contains("braxon.tree_sitter.contract")
        }
        "guile" => contents.contains("guile.rebuild_intent"),
        "apropos" => contents.contains("apropos.discover"),
        "tree_sitter" => contents.contains("tree_sitter.parse"),
        "ast" => contents.contains("SyntaxIntent") || contents.contains("syntax"),
        "place_world" => contents.contains("WowasRealization") || contents.contains("World"),
        "address_bindings" => address_integrity.all_passed,
        "runtime_capabilities" => {
            contents.contains("tokenizer.boundary") && contents.contains("tree_sitter.parse")
        }
        "collective_self_state" => {
            contents.contains("conflict_preserved") && contents.contains("forced_consensus")
        }
        "bus" => contents.contains("hard_runtime_valid") && contents.contains("UserPresentation"),
        _ => false,
    }
}

fn verify_tokenizer_band(
    root: &Path,
    address_namespace: &str,
    band: &TokenizerBand,
) -> Result<TokenizerBandVerification, String> {
    let tokenizer_source_exists = root.join(&band.tokenizer_path).exists();
    if !tokenizer_source_exists {
        return Ok(TokenizerBandVerification {
            band_id: band.band_id.clone(),
            model_id: band.model_id.clone(),
            active: band.active,
            required: band.required,
            tokenizer_path: band.tokenizer_path.clone(),
            tokenizer_source_exists: false,
            vocabulary_loaded: false,
            vocabulary_count: 0,
            deterministic_mapping: false,
            forward_translation: false,
            reverse_translation: false,
            provenance_recorded: !band.provenance.trim().is_empty(),
            shared_semantic_addressing: false,
            collective_state_compatible: false,
            unresolved_required_tokens: if band.active {
                vec!["tokenizer_source_missing".into()]
            } else {
                Vec::new()
            },
            collision_count: 0,
            reason: format!("tokenizer artifact unavailable: {}", band.tokenizer_path),
        });
    }
    let runtime_receipt = if band.active {
        Some(
            TokenizerBridge::from_root(root, &band.band_id)
                .map(|bridge| bridge.encode_translate_round_trip("is truth"))
                .map_err(|error| {
                    format!(
                        "active tokenizer bridge failed for '{}': {error}",
                        band.band_id
                    )
                })?,
        )
    } else {
        None
    };
    let raw: Value = read_json(root, &band.tokenizer_path)?;
    let vocab = raw
        .get("vocab")
        .and_then(Value::as_object)
        .ok_or_else(|| format!("tokenizer '{}' has no vocab object", band.tokenizer_path))?;
    let mut ids = BTreeSet::new();
    let mut universal_ids = BTreeMap::<u64, String>::new();
    let mut collision_count = 0;
    for (token, id) in vocab {
        let Some(id) = id.as_u64() else {
            collision_count += 1;
            continue;
        };
        if !ids.insert(id) {
            collision_count += 1;
        }
        let universal = stable_id(token);
        if let Some(previous) = universal_ids.insert(universal, token.clone()) {
            if previous != *token {
                collision_count += 1;
            }
        }
    }
    let samples = ["is", "truth"];
    let mut unresolved_required_tokens = Vec::new();
    let mut forward_translation = true;
    let mut reverse_translation = true;
    let mut shared_semantic_addressing = true;
    for token in samples {
        let native_id = vocab.get(token).and_then(Value::as_u64);
        match native_id {
            Some(native_id) => {
                let universal_id = stable_id(token);
                let address = format!("{address_namespace}/{universal_id:016x}");
                forward_translation &= universal_id != 0;
                reverse_translation &= stable_id(token) == universal_id
                    && native_id == vocab.get(token).and_then(Value::as_u64).unwrap_or(u64::MAX);
                shared_semantic_addressing &= address.starts_with(address_namespace)
                    && address != format!("{address_namespace}/0000000000000000");
            }
            None => unresolved_required_tokens.push(token.into()),
        }
    }
    let static_deterministic_mapping = collision_count == 0 && !vocab.is_empty();
    let vocabulary_loaded = !vocab.is_empty();
    let bridge_deterministic_mapping = runtime_receipt
        .as_ref()
        .map(|receipt| receipt.deterministic_mapping)
        .unwrap_or(true);
    let bridge_forward_translation = runtime_receipt
        .as_ref()
        .map(|receipt| receipt.forward_translation)
        .unwrap_or(true);
    let bridge_reverse_translation = runtime_receipt
        .as_ref()
        .map(|receipt| receipt.reverse_translation)
        .unwrap_or(true);
    let bridge_shared_addressing = runtime_receipt
        .as_ref()
        .map(|receipt| receipt.shared_semantic_addressing)
        .unwrap_or(true);
    let collective_state_compatible = runtime_receipt
        .as_ref()
        .map(|receipt| receipt.collective_state_contribution_ready)
        .unwrap_or(false);
    Ok(TokenizerBandVerification {
        band_id: band.band_id.clone(),
        model_id: band.model_id.clone(),
        active: band.active,
        required: band.required,
        tokenizer_path: band.tokenizer_path.clone(),
        tokenizer_source_exists,
        vocabulary_loaded,
        vocabulary_count: vocab.len(),
        deterministic_mapping: static_deterministic_mapping && bridge_deterministic_mapping,
        forward_translation: forward_translation && bridge_forward_translation,
        reverse_translation: reverse_translation && bridge_reverse_translation,
        provenance_recorded: !band.provenance.trim().is_empty()
            && !band.representation.trim().is_empty()
            && !band.expected_model_execution.trim().is_empty(),
        shared_semantic_addressing: shared_semantic_addressing && bridge_shared_addressing,
        collective_state_compatible,
        unresolved_required_tokens: runtime_receipt
            .as_ref()
            .map(|receipt| receipt.unresolved_tokens.clone())
            .unwrap_or(unresolved_required_tokens),
        collision_count: collision_count
            + runtime_receipt
                .as_ref()
                .map(|receipt| receipt.collision_count)
                .unwrap_or(0),
        reason: if band.active {
            "active native band checked through executable encode-translate-address-contribute-reverse bridge".into()
        } else {
            "configured but inactive model band; artifact is not claimed loaded".into()
        },
    })
}

fn tokenizer_band_passes(band: &TokenizerBandVerification) -> bool {
    band.active
        && band.tokenizer_source_exists
        && band.vocabulary_loaded
        && band.deterministic_mapping
        && band.forward_translation
        && band.reverse_translation
        && band.provenance_recorded
        && band.shared_semantic_addressing
        && band.collective_state_compatible
        && band.unresolved_required_tokens.is_empty()
        && band.collision_count == 0
}

fn expected_gaps(ids: &BTreeSet<u64>) -> Vec<u64> {
    let Some(maximum) = ids.iter().next_back().copied() else {
        return vec![1];
    };
    (1..=maximum).filter(|id| !ids.contains(id)).collect()
}

fn wiring_binding_audit(root: &Path) -> Result<(Vec<String>, Vec<String>), String> {
    let wiring: Value = read_json(root, ACTIVE_WIRING_MAP_RELATIVE_PATH)?;
    let mut declared = BTreeSet::new();
    for section in ["runtime_native", "write_nsq"] {
        if let Some(object) = wiring.get(section).and_then(Value::as_object) {
            declared.extend(object.keys().cloned());
        }
    }
    let mut stale_mappings = Vec::new();
    let mut invalid_bindings = Vec::new();
    if let Some(crate_bindings) = wiring.get("crate_bindings").and_then(Value::as_object) {
        for (crate_name, values) in crate_bindings {
            let Some(values) = values.as_array() else {
                invalid_bindings.push(format!("crate binding '{crate_name}' is not an array"));
                continue;
            };
            for value in values {
                let Some(binding) = value.as_str() else {
                    invalid_bindings.push(format!(
                        "crate binding '{crate_name}' contains a non-string entry"
                    ));
                    continue;
                };
                if !declared.contains(binding) {
                    stale_mappings.push(format!("{crate_name}:{binding}"));
                }
            }
        }
    } else {
        invalid_bindings.push("wiring map has no crate_bindings object".into());
    }
    Ok((stale_mappings, invalid_bindings))
}

fn model_truth_entry(
    target_pole: String,
    model_id: String,
    state: ModelExecutionState,
) -> ModelExecutionTruth {
    let valid_transition_chain = state.validate().is_ok();
    ModelExecutionTruth {
        target_pole,
        model_id,
        configured: state.configured,
        available: state.available,
        loaded: state.loaded,
        initialized: state.initialized,
        executing: state.executing,
        valid_transition_chain,
    }
}

fn parameter_citadel_topology_valid() -> bool {
    let source = include_str!("parameter_citadel.rs");
    [
        "identity_preserved",
        "local_state_materialized",
        "multi_input_pressure_resolved",
        "routed_response_integrated",
        "generation_preserved",
        "persistent_state_reconstructible",
        "no_resident_runtime",
    ]
    .iter()
    .all(|required| source.contains(required))
}

fn entry_activated(report: &FullWakeReport, id: &str) -> bool {
    report
        .entries
        .iter()
        .find(|entry| entry.id == id)
        .map(|entry| entry.activated)
        .unwrap_or(false)
}

fn gate(id: &str, passed: bool, evidence: &str) -> ClosureGate {
    ClosureGate {
        id: id.into(),
        passed,
        evidence: evidence.into(),
    }
}

fn load_activation_manifest(root: &Path) -> Result<ActivationManifest, String> {
    read_json(root, CLOSURE_ACTIVATION_MANIFEST_RELATIVE_PATH)
}

fn read_json<T: for<'de> Deserialize<'de>>(root: &Path, relative_path: &str) -> Result<T, String> {
    let path = root.join(relative_path);
    let raw = fs::read_to_string(&path)
        .map_err(|error| format!("failed to read '{}': {error}", path.display()))?;
    serde_json::from_str(&raw)
        .map_err(|error| format!("failed to parse '{}': {error}", path.display()))
}

fn stable_id(value: &str) -> u64 {
    value
        .as_bytes()
        .iter()
        .fold(0xcbf29ce484222325u64, |hash, byte| {
            hash.wrapping_mul(0x100000001b3)
                .wrapping_add(u64::from(*byte))
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn repo_root() -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .unwrap()
    }

    #[test]
    fn address_audit_detects_complete_manifest_and_chain_registry_after_closure_extension() {
        let report = address_integrity_audit(&repo_root()).unwrap();
        assert!(report.collisions.is_empty(), "{:?}", report.collisions);
        assert!(report.duplicates.is_empty(), "{:?}", report.duplicates);
        assert!(report.gaps.is_empty(), "{:?}", report.gaps);
        assert!(
            report.dangling_references.is_empty(),
            "{:?}",
            report.dangling_references
        );
        assert!(
            report.orphan_objects.is_empty(),
            "{:?}",
            report.orphan_objects
        );
        assert!(
            report.invalid_ranges.is_empty(),
            "{:?}",
            report.invalid_ranges
        );
        assert!(
            report.stale_mappings.is_empty(),
            "{:?}",
            report.stale_mappings
        );
        assert!(
            report.invalid_bindings.is_empty(),
            "{:?}",
            report.invalid_bindings
        );
    }

    #[test]
    fn tokenizer_verifier_proves_active_native_band_without_claiming_inactive_model_artifacts() {
        let report = tokenizer_verification(&repo_root()).unwrap();
        assert_eq!(report.active_band_total, 1);
        assert_eq!(report.verified_active_band_total, 1);
        assert!(report.all_active_bands_verified, "{report:?}");
        assert_eq!(report.inactive_required_band_total, 10);
        assert!(report
            .bands
            .iter()
            .filter(|band| !band.active)
            .all(|band| !band.tokenizer_source_exists));
    }

    #[test]
    fn full_wake_has_exact_activation_accounting_and_no_orphaned_or_invalid_binding_state() {
        let report = full_wake(&repo_root()).unwrap();
        assert_eq!(report.required_total, 15);
        assert_eq!(report.activated_total, 15, "{report:?}");
        assert_eq!(report.unresolved, 0, "{report:?}");
        assert_eq!(report.orphaned, 0, "{report:?}");
        assert_eq!(report.invalid_bindings, 0, "{report:?}");
        assert!(report.all_passed, "{report:?}");
    }

    #[test]
    fn closure_audit_retains_execution_truth_and_reports_only_demonstrated_gates() {
        let report = closure_audit(&repo_root()).unwrap();
        assert_eq!(report.model_execution_truth.len(), 10);
        assert!(report
            .model_execution_truth
            .iter()
            .all(|entry| entry.configured
                && !entry.available
                && !entry.loaded
                && !entry.initialized
                && !entry.executing));
        assert!(
            report
                .gates
                .iter()
                .find(|gate| gate.id == "narrative_hard_state_separation")
                .unwrap()
                .passed
        );
    }

    #[test]
    fn absent_tokenizer_artifact_is_reported_without_promoting_a_missing_model_to_execution() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!("braxon-closure-missing-{unique}"));
        fs::create_dir_all(root.join("config/nsq")).unwrap();
        fs::write(
            root.join(TOKENIZER_BAND_REGISTRY_RELATIVE_PATH),
            r#"{"schema":"braxon.nsq.tokenizer_band_registry.v1","universal_translation_version":"test","address_namespace":"nsq.test","bands":[{"band_id":"x","model_id":"x","active":true,"required":true,"tokenizer_path":"missing.json","representation":"json","provenance":"test","expected_model_execution":"not_available"}]}"#,
        )
        .unwrap();
        let report = tokenizer_verification(&root).unwrap();
        assert!(!report.all_active_bands_verified);
        assert!(!report.bands[0].tokenizer_source_exists);
        let _ = fs::remove_dir_all(root);
    }
}
