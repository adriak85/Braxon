//! BASE READ
//! full on / full off binary state is read first.
//! then the graded multipositional lever on the first switch+lever pair locks the native ingress surface.
//! the rest of the bit is operating code interpreted inside that native surface.
//! language and family registries are native NSQ runtime hosting maps; foreign spellings exist only at the boundary.

mod bit_circulation;
mod braxon_runtime;
mod runtime_contract;

use nsq_core::{
    CourtSurface, FullBinaryAnchor, MultipositionalLever, NuCell, NuCellRole, NuPair, NuWord,
    CANONICAL_SWITCH_POSITIONS,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub use bit_circulation::{simulate_bit_job_board, BitJobBoardReport};
pub use braxon_runtime::{
    audit_BRAXON_runtime_materials, audit_sovereign_lifecycle, preferred_donor_tensor_root,
    DeltaOverlayMetadata, DeltaOverlayPlan, DeltaSourceClassification, DeltaSourceStatus,
    DonorTensorRuntimeProof, NsqRuntimeArtifactProof, RuntimeAuthorityProof, RuntimeMaterialAudit,
    RuntimeTokenizerProof, SovereignLifecycleAudit,
};
pub use runtime_contract::{
    assistant_reply, persistent_client_profile, secure_transit_plan, PersistentClientProfile,
    SecureTransitPlan,
};

const PYTHON3_COURT_ROUTE: [CourtSurface; 5] = [
    CourtSurface::Policer,
    CourtSurface::Lexer,
    CourtSurface::Parser,
    CourtSurface::Router,
    CourtSurface::Inspector,
];

const LUA_COURT_ROUTE: [CourtSurface; 5] = [
    CourtSurface::Policer,
    CourtSurface::Lexer,
    CourtSurface::Parser,
    CourtSurface::Router,
    CourtSurface::Inspector,
];

const LISP_FAMILY_COURT_ROUTE: [CourtSurface; 5] = [
    CourtSurface::Policer,
    CourtSurface::Lexer,
    CourtSurface::Parser,
    CourtSurface::Router,
    CourtSurface::Inspector,
];

const C_FAMILY_COURT_ROUTE: [CourtSurface; 5] = [
    CourtSurface::Policer,
    CourtSurface::Parser,
    CourtSurface::Optimizer,
    CourtSurface::Router,
    CourtSurface::Inspector,
];

const JAVA_KOTLIN_COURT_ROUTE: [CourtSurface; 5] = [
    CourtSurface::Policer,
    CourtSurface::Parser,
    CourtSurface::Router,
    CourtSurface::Scheduler,
    CourtSurface::Inspector,
];

const RUST_COURT_ROUTE: [CourtSurface; 5] = [
    CourtSurface::Policer,
    CourtSurface::Parser,
    CourtSurface::Optimizer,
    CourtSurface::Router,
    CourtSurface::Inspector,
];

const ANDROID_INTERFACE_COURT_ROUTE: [CourtSurface; 4] = [
    CourtSurface::Policer,
    CourtSurface::Router,
    CourtSurface::Scheduler,
    CourtSurface::Inspector,
];

const OFFLINE_MODEL_COURT_ROUTE: [CourtSurface; 4] = [
    CourtSurface::Policer,
    CourtSurface::Router,
    CourtSurface::Scheduler,
    CourtSurface::Inspector,
];

const ZLM_SESSION_COURT_ROUTE: [CourtSurface; 4] = [
    CourtSurface::Policer,
    CourtSurface::Router,
    CourtSurface::Scheduler,
    CourtSurface::Inspector,
];

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NativeRuntimeClass {
    Language,
    RuntimeInterface,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct NativeRuntimeLaneDescriptor {
    pub surface: &'static str,
    pub class: NativeRuntimeClass,
    pub runtime_model: &'static str,
    pub canonical_semantics: &'static str,
    pub ingress_rule: &'static str,
    pub status: &'static str,
    pub court_route: Vec<&'static str>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Default)]
pub struct NativeRuntimeRegistry {
    pub lanes: Vec<NativeRuntimeLaneDescriptor>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct NativeLanguageIngressPlan {
    pub language_id: String,
    pub surface: String,
    pub action: &'static str,
    pub existing_native_lane: bool,
    pub runtime_model: &'static str,
    pub canonical_semantics: &'static str,
    pub ingress_rule: &'static str,
    pub status: &'static str,
    pub full_ingress_required: bool,
    pub fail_closed: bool,
    pub parallel_runtime_allowed: bool,
    pub ported_runtime_allowed: bool,
    pub shim_runtime_allowed: bool,
    pub court_route: Vec<&'static str>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Python3RuntimeLane {
    pub surface: &'static str,
    pub canonical_semantics: &'static str,
}

impl Default for Python3RuntimeLane {
    fn default() -> Self {
        Self {
            surface: "python3_native_runtime_lane",
            canonical_semantics: "base8_switch_topology",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Python3Ingress {
    pub symbol: String,
    pub args: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeSliceReport {
    pub lane: String,
    pub symbol: String,
    pub arg_count: usize,
    pub canonical_cells: usize,
    pub canonical_semantics: String,
    pub court_route: Vec<String>,
    pub lever_positions: Vec<MultipositionalLever>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OfflineModelLane {
    pub surface: &'static str,
    pub runtime_authority: &'static str,
    pub canonical_semantics: &'static str,
}

impl Default for OfflineModelLane {
    fn default() -> Self {
        Self {
            surface: "offline_model_native_runtime_lane",
            runtime_authority: "rust_native_offline_model_lane",
            canonical_semantics: "base8_switch_topology",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OfflineInferenceReport {
    pub lane: String,
    pub model_id: String,
    pub offline_only: bool,
    pub runtime_authority: String,
    pub canonical_semantics: String,
    pub canonical_cells: usize,
    pub court_route: Vec<String>,
    pub lever_positions: Vec<MultipositionalLever>,
    pub representation_mode: String,
    pub runtime_mass_profile: String,
    pub tokenizer_bridge_stamp: String,
    pub runtime_semantic_consumers_ready: bool,
    pub runtime_semantic_feed_entries: usize,
    pub runtime_compass_seed_tokens: usize,
    pub runtime_semantic_patch_anchor_count: usize,
    pub runtime_semantic_tests_present: bool,
    pub required_donor_shard_count: usize,
    pub present_donor_shard_count: usize,
    pub materialized_donor_shard_count: usize,
    pub pointer_donor_shard_count: usize,
    pub donor_index_discovered: bool,
    pub donor_shard_resolved: bool,
    pub donor_shard_opened: bool,
    pub donor_shard_payload_materialized: bool,
    pub donor_shard_is_lfs_pointer: bool,
    pub donor_shard_size_bytes: u64,
    pub donor_tensor_addressed: bool,
    pub donor_tensor_activated: bool,
    pub donor_index_path: String,
    pub donor_shard_path: String,
    pub donor_tensor_name: String,
    pub nsq_artifact_present: bool,
    pub nsq_envelope_present: bool,
    pub nsq_artifact_size_bytes: u64,
    pub nsq_artifact_is_lfs_pointer: bool,
    pub nsq_artifact_is_text_manifest: bool,
    pub nsq_runtime_mass_profile: String,
    pub nsq_hot_live_parameter_embodiment: bool,
    pub nsq_verification_state: String,
    pub nsq_whole_core_runtime_status: String,
    pub donor_source_lane: String,
    pub donor_source_state: String,
    pub nsq_artifact_lane: String,
    pub nsq_artifact_state: String,
    pub runtime_authority_lane: String,
    pub runtime_authority_state: String,
    pub runtime_authority_bound: bool,
    pub pointer_free_runtime_ready: bool,
    pub tokenizer_candidate_discovered: bool,
    pub tokenizer_runtime_selected: bool,
    pub tokenizer_runtime_bound: bool,
    pub tokenizer_selection_mode: String,
    pub tokenizer_selected_path: String,
    pub delta_source_count: usize,
    pub delta_live_source_count: usize,
    pub delta_bridge_source_count: usize,
    pub delta_stale_source_count: usize,
    pub delta_absent_source_count: usize,
    pub overlay_planner_ready: bool,
    pub overlay_runtime_bound: bool,
    pub overlay_live_parameter_application: bool,
    pub overlay_metadata_entries: usize,
    pub secure_transit_duty_cycle_ready: bool,
    pub secure_transit_instruction_data_separated: bool,
    pub secure_transit_switch_release_verified: bool,
    pub secure_transit_self_transform_stage: String,
    pub secure_transit_picker_binding_state: String,
    pub secure_transit_phase_count: usize,
    pub bit_job_board_ready: bool,
    pub bit_object_non_consumptive: bool,
    pub bit_object_migration_enabled: bool,
    pub bit_scan_communicate_only: bool,
    pub bit_life_extension_total: usize,
    pub persistent_client_pipe_binding_state: String,
    pub persistent_client_reasoning_state: String,
    pub persistent_client_comprehension_mode: String,
    pub persistent_client_fuzzy_match_enabled: bool,
    pub persistent_client_context_awareness_mode: String,
    pub persistent_client_self_expression_state: String,
    pub persistent_client_session_continuity_ready: bool,
    pub assistant_reply: String,
    pub delta_sources: Vec<DeltaSourceStatus>,
    pub overhead_compensation_stamp: String,
    pub BRAXON_feature_attachment_stamp: String,
    pub whole_parameter_stamp: String,
    pub parameter_projection_mode: String,
    pub env_parameter_copy_mode: String,
    pub stamp_bundle: Vec<String>,
    pub BRAXON_feature_attachments: Vec<String>,
    pub session_surface: String,
    pub session_mode: String,
    pub agentic_capability: String,
    pub capability_lattice_stamp: String,
    pub legacy_capability_profile: String,
    pub legacy_capabilities: Vec<String>,
    pub legacy_capabilities_status: String,
    pub result_summary: String,
}

#[derive(Debug, Clone, Copy)]
struct OfflineModelStampProfile {
    representation_mode: &'static str,
    runtime_mass_profile: &'static str,
    tokenizer_bridge_stamp: &'static str,
    overhead_compensation_stamp: &'static str,
    BRAXON_feature_attachment_stamp: &'static str,
    whole_parameter_stamp: &'static str,
    parameter_projection_mode: &'static str,
    env_parameter_copy_mode: &'static str,
    model_bundle_stamp: &'static str,
    session_surface: &'static str,
    session_mode: &'static str,
    agentic_capability: &'static str,
    BRAXON_feature_attachments: [&'static str; 4],
    capability_lattice_stamp: &'static str,
    legacy_capability_profile: &'static str,
    legacy_capabilities: &'static [&'static str],
    legacy_capabilities_status: &'static str,
}

impl NativeRuntimeRegistry {
    pub fn lane(&self, surface: &str) -> Option<&NativeRuntimeLaneDescriptor> {
        // BRAXON_runtime_semantic_patch::lane
        let _runtime_semantic_bias = crate::semantic_context::semantic_bias_for_text(
            crate::semantic_context::runtime_semantic_context(),
            surface,
        );
        self.lanes.iter().find(|lane| lane.surface == surface)
    }

    pub fn resolve_language_ingress(&self, language_id: &str) -> NativeLanguageIngressPlan {
        let canonical_id = canonical_language_id(language_id);
        let surface = native_surface_for_language_id(&canonical_id);

        if let Some(lane) = self.lane(&surface) {
            return NativeLanguageIngressPlan {
                language_id: canonical_id,
                surface,
                action: "use_existing_native_full_ingress",
                existing_native_lane: true,
                runtime_model: lane.runtime_model,
                canonical_semantics: lane.canonical_semantics,
                ingress_rule: lane.ingress_rule,
                status: lane.status,
                full_ingress_required: true,
                fail_closed: false,
                parallel_runtime_allowed: false,
                ported_runtime_allowed: false,
                shim_runtime_allowed: false,
                court_route: lane.court_route.clone(),
            };
        }

        NativeLanguageIngressPlan {
            language_id: canonical_id,
            surface,
            action: "bootstrap_native_full_ingress",
            existing_native_lane: false,
            runtime_model: "native_runtime_incorporation",
            canonical_semantics: "base8_switch_topology",
            ingress_rule: "native_language_full_ingress",
            status: "bootstrap_required",
            full_ingress_required: true,
            fail_closed: false,
            parallel_runtime_allowed: false,
            ported_runtime_allowed: false,
            shim_runtime_allowed: false,
            court_route: court_route_names(&PYTHON3_COURT_ROUTE),
        }
    }
}

fn canonical_language_id(language_id: &str) -> String {
    let mut out = String::new();
    let mut last_was_separator = false;

    for ch in language_id.trim().chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            last_was_separator = false;
        } else if !last_was_separator && !out.is_empty() {
            out.push('_');
            last_was_separator = true;
        }
    }

    let canonical = out.trim_matches('_');
    if canonical.is_empty() {
        "unknown_language".to_string()
    } else {
        canonical.to_string()
    }
}

fn native_surface_for_language_id(canonical_id: &str) -> String {
    match canonical_id {
        "python" | "python3" => "python3_native_runtime_lane".to_string(),
        "lua" => "lua_native_runtime_lane".to_string(),
        "lisp" | "common_lisp" | "scheme" | "guile" => {
            "lisp_family_native_runtime_lane".to_string()
        }
        "c" | "cpp" | "c_plus" | "objective_c" => "c_family_native_runtime_lane".to_string(),
        "java" => "java_native_runtime_lane".to_string(),
        "kotlin" => "kotlin_native_runtime_lane".to_string(),
        "rust" => "rust_native_runtime_lane".to_string(),
        other => format!("{other}_native_runtime_lane"),
    }
}

pub fn native_runtime_registry() -> NativeRuntimeRegistry {
    NativeRuntimeRegistry {
        lanes: vec![
            NativeRuntimeLaneDescriptor {
                surface: "python3_native_runtime_lane",
                class: NativeRuntimeClass::Language,
                runtime_model: "native_runtime_incorporation",
                canonical_semantics: "base8_switch_topology",
                ingress_rule: "native_call_ingress",
                status: "active_first_slice",
                court_route: court_route_names(&PYTHON3_COURT_ROUTE),
            },
            NativeRuntimeLaneDescriptor {
                surface: "lua_native_runtime_lane",
                class: NativeRuntimeClass::Language,
                runtime_model: "native_runtime_incorporation",
                canonical_semantics: "base8_switch_topology",
                ingress_rule: "native_expression_ingress",
                status: "lane_defined_next_slice",
                court_route: court_route_names(&LUA_COURT_ROUTE),
            },
            NativeRuntimeLaneDescriptor {
                surface: "lisp_family_native_runtime_lane",
                class: NativeRuntimeClass::Language,
                runtime_model: "native_runtime_incorporation",
                canonical_semantics: "base8_switch_topology",
                ingress_rule: "native_form_ingress",
                status: "lane_defined_next_slice",
                court_route: court_route_names(&LISP_FAMILY_COURT_ROUTE),
            },
            NativeRuntimeLaneDescriptor {
                surface: "c_family_native_runtime_lane",
                class: NativeRuntimeClass::Language,
                runtime_model: "native_runtime_incorporation",
                canonical_semantics: "base8_switch_topology",
                ingress_rule: "native_unit_ingress",
                status: "lane_defined_next_slice",
                court_route: court_route_names(&C_FAMILY_COURT_ROUTE),
            },
            NativeRuntimeLaneDescriptor {
                surface: "java_native_runtime_lane",
                class: NativeRuntimeClass::Language,
                runtime_model: "native_runtime_incorporation",
                canonical_semantics: "base8_switch_topology",
                ingress_rule: "native_class_ingress",
                status: "lane_defined_next_slice",
                court_route: court_route_names(&JAVA_KOTLIN_COURT_ROUTE),
            },
            NativeRuntimeLaneDescriptor {
                surface: "kotlin_native_runtime_lane",
                class: NativeRuntimeClass::Language,
                runtime_model: "native_runtime_incorporation",
                canonical_semantics: "base8_switch_topology",
                ingress_rule: "native_class_ingress",
                status: "lane_defined_next_slice",
                court_route: court_route_names(&JAVA_KOTLIN_COURT_ROUTE),
            },
            NativeRuntimeLaneDescriptor {
                surface: "rust_native_runtime_lane",
                class: NativeRuntimeClass::Language,
                runtime_model: "native_runtime_incorporation",
                canonical_semantics: "base8_switch_topology",
                ingress_rule: "native_module_ingress",
                status: "active_existing_lane_family",
                court_route: court_route_names(&RUST_COURT_ROUTE),
            },
            NativeRuntimeLaneDescriptor {
                surface: "jni_native_runtime_interface",
                class: NativeRuntimeClass::RuntimeInterface,
                runtime_model: "native_boundary_translation_only",
                canonical_semantics: "boundary_export_only",
                ingress_rule: "court_routed_boundary_ingress",
                status: "lane_defined_boundary_only",
                court_route: court_route_names(&ANDROID_INTERFACE_COURT_ROUTE),
            },
            NativeRuntimeLaneDescriptor {
                surface: "adb_native_runtime_interface",
                class: NativeRuntimeClass::RuntimeInterface,
                runtime_model: "native_boundary_translation_only",
                canonical_semantics: "boundary_export_only",
                ingress_rule: "court_routed_boundary_ingress",
                status: "lane_defined_boundary_only",
                court_route: court_route_names(&ANDROID_INTERFACE_COURT_ROUTE),
            },
            NativeRuntimeLaneDescriptor {
                surface: "agdk_native_runtime_interface",
                class: NativeRuntimeClass::RuntimeInterface,
                runtime_model: "native_boundary_translation_only",
                canonical_semantics: "boundary_export_only",
                ingress_rule: "court_routed_boundary_ingress",
                status: "lane_defined_boundary_only",
                court_route: court_route_names(&ANDROID_INTERFACE_COURT_ROUTE),
            },
            NativeRuntimeLaneDescriptor {
                surface: "offline_model_native_runtime_lane",
                class: NativeRuntimeClass::RuntimeInterface,
                runtime_model: "rust_native_offline_inference_stamp_lane",
                canonical_semantics: "base8_switch_topology",
                ingress_rule: "native_prompt_ingress",
                status: "active_phase2_slice",
                court_route: court_route_names(&OFFLINE_MODEL_COURT_ROUTE),
            },
            NativeRuntimeLaneDescriptor {
                surface: "zlm_native_runtime_surface",
                class: NativeRuntimeClass::RuntimeInterface,
                runtime_model: "native_persistent_session_orchestrator",
                canonical_semantics: "base8_switch_topology",
                ingress_rule: "persistent_conversation_ingress",
                status: "active_phase2_slice",
                court_route: court_route_names(&ZLM_SESSION_COURT_ROUTE),
            },
        ],
    }
}

impl Python3RuntimeLane {
    pub fn ingest(&self, source: &str) -> Result<Python3Ingress, String> {
        let source = source.trim();
        let open = source
            .find('(')
            .ok_or_else(|| "python3 ingress requires call syntax with '('".to_string())?;
        let close = source
            .rfind(')')
            .ok_or_else(|| "python3 ingress requires call syntax with ')'".to_string())?;
        if close <= open {
            return Err("python3 ingress has invalid call shape".into());
        }

        let symbol = source[..open].trim();
        if symbol.is_empty() {
            return Err("python3 ingress symbol is empty".into());
        }

        let body = source[open + 1..close].trim();
        let mut args = Vec::new();
        if !body.is_empty() {
            for seg in body.split(',') {
                let kv = seg.trim();
                let (k, v) = kv
                    .split_once('=')
                    .ok_or_else(|| format!("python3 ingress arg '{kv}' must use key=value"))?;
                let key = k.trim();
                let value = v.trim();
                if key.is_empty() || value.is_empty() {
                    return Err(format!(
                        "python3 ingress arg '{kv}' contains empty key or value"
                    ));
                }
                args.push((key.to_string(), value.to_string()));
            }
        }

        Ok(Python3Ingress {
            symbol: symbol.to_string(),
            args,
        })
    }

    pub fn encode_switch_faithful(&self, ingress: &Python3Ingress) -> Result<NuWord, String> {
        let mut cells = Vec::new();
        cells.push(self.make_cell(
            NuCellRole::Language,
            FullBinaryAnchor::on(),
            Self::attune_base_identity("python3")?,
        ));
        cells.push(self.make_cell(
            NuCellRole::Symbol,
            FullBinaryAnchor::off(),
            Self::attune_base_identity(&ingress.symbol)?,
        ));

        for (k, v) in &ingress.args {
            let kv = format!("{k}={v}");
            cells.push(self.make_cell(
                NuCellRole::Macro,
                FullBinaryAnchor::on(),
                Self::algorithm_lever_from_semantic_text(&kv)?,
            ));
        }

        let algorithm_footprint = format!("arity:{}", ingress.args.len());
        cells.push(self.make_cell(
            NuCellRole::Algorithm,
            FullBinaryAnchor::off(),
            Self::algorithm_lever_from_semantic_text(&algorithm_footprint)?,
        ));

        let word = NuWord { cells };
        word.validate()?;
        Ok(word)
    }

    pub fn execute_slice(&self, source: &str) -> Result<RuntimeSliceReport, String> {
        // BRAXON_runtime_semantic_patch::execute_slice
        let _runtime_semantic_bias = crate::semantic_context::semantic_bias_for_text(
            crate::semantic_context::runtime_semantic_context(),
            source,
        );
        let ingress = self.ingest(source)?;
        let word = self.encode_switch_faithful(&ingress)?;
        let lever_positions = word
            .cells
            .iter()
            .map(|cell| cell.pair.lever.clone())
            .collect();
        Ok(RuntimeSliceReport {
            lane: self.surface.to_string(),
            symbol: ingress.symbol,
            arg_count: ingress.args.len(),
            canonical_cells: word.cells.len(),
            canonical_semantics: self.canonical_semantics.to_string(),
            court_route: court_route_names(&PYTHON3_COURT_ROUTE)
                .into_iter()
                .map(str::to_string)
                .collect(),
            lever_positions,
        })
    }

    fn make_cell(
        &self,
        role: NuCellRole,
        switch: FullBinaryAnchor,
        lever: MultipositionalLever,
    ) -> NuCell {
        NuCell {
            role,
            pair: NuPair { switch, lever },
        }
    }

    fn attune_base_identity(text: &str) -> Result<MultipositionalLever, String> {
        attune_base_identity(text)
    }

    fn algorithm_lever_from_semantic_text(text: &str) -> Result<MultipositionalLever, String> {
        // BRAXON_runtime_semantic_patch::algorithm_lever_from_semantic_text
        let _runtime_semantic_bias = crate::semantic_context::semantic_bias_for_text(
            crate::semantic_context::runtime_semantic_context(),
            text,
        );
        algorithm_lever_from_semantic_text(text)
    }

    pub fn canonical_positions(&self) -> usize {
        CANONICAL_SWITCH_POSITIONS
    }
}

impl OfflineModelLane {
    pub fn execute_request(
        &self,
        model_id: &str,
        prompt: &str,
    ) -> Result<OfflineInferenceReport, String> {
        // BRAXON_runtime_semantic_patch::execute_request
        let _runtime_semantic_bias = crate::semantic_context::semantic_bias_for_text(
            crate::semantic_context::runtime_semantic_context(),
            prompt,
        );
        let model_id = model_id.trim();
        if model_id.is_empty() {
            return Err("offline model lane requires a model id".into());
        }
        let asset_key = runtime_asset_key(model_id);
        let public_model_id = public_model_name(asset_key);
        let profile = supported_profile(asset_key)
            .ok_or_else(|| format!("unsupported offline model asset '{model_id}'"))?;
        let prompt = prompt.trim();
        if prompt.is_empty() {
            return Err("offline model lane requires a non-empty prompt".into());
        }
        let runtime_semantic_context =
            crate::semantic_context::load_runtime_semantic_context_default();
        let runtime_semantic_evidence =
            crate::semantic_context::runtime_semantic_evidence(&runtime_semantic_context);
        let secure_transit = secure_transit_plan(
            prompt,
            "compositor::storage::offline_model_native_runtime_lane",
        );
        let bit_job_board = simulate_bit_job_board(prompt);
        let runtime_materials = resolve_runtime_root()
            .map(|root| audit_BRAXON_runtime_materials(&root))
            .unwrap_or_default();
        let delta_live_source_count = runtime_materials
            .delta_sources
            .iter()
            .filter(|source| {
                matches!(
                    source.classification,
                    DeltaSourceClassification::PresentLive
                )
            })
            .count();
        let delta_bridge_source_count = runtime_materials
            .delta_sources
            .iter()
            .filter(|source| {
                matches!(
                    source.classification,
                    DeltaSourceClassification::PresentBridge
                )
            })
            .count();
        let delta_stale_source_count = runtime_materials
            .delta_sources
            .iter()
            .filter(|source| matches!(source.classification, DeltaSourceClassification::Stale))
            .count();
        let delta_absent_source_count = runtime_materials
            .delta_sources
            .iter()
            .filter(|source| {
                matches!(
                    source.classification,
                    DeltaSourceClassification::AbsentCheckout
                )
            })
            .count();
        let legacy_capabilities = profile
            .legacy_capabilities
            .iter()
            .map(|value| (*value).to_string())
            .collect::<Vec<_>>();
        let persistent_client = persistent_client_profile(
            profile.session_surface,
            profile.session_mode,
            runtime_semantic_evidence.consumers_ready,
            runtime_semantic_evidence.feed_entries,
            runtime_semantic_evidence.tests_present,
            &legacy_capabilities,
        );
        let assistant_reply = assistant_reply(
            prompt,
            &persistent_client,
            &secure_transit,
            &runtime_materials.authority.donor_source_state,
            &runtime_materials.authority.runtime_authority_lane,
            runtime_materials.tokenizer.tokenizer_runtime_bound,
            runtime_materials.overlay.planner_ready,
        );

        let cells = vec![
            self.make_cell(
                NuCellRole::Language,
                FullBinaryAnchor::on(),
                attune_base_identity(self.surface)?,
            ),
            self.make_cell(
                NuCellRole::Symbol,
                FullBinaryAnchor::off(),
                algorithm_lever_from_semantic_text(asset_key)?,
            ),
            self.make_cell(
                NuCellRole::Macro,
                FullBinaryAnchor::on(),
                algorithm_lever_from_semantic_text(prompt)?,
            ),
            self.make_cell(
                NuCellRole::Algorithm,
                FullBinaryAnchor::off(),
                algorithm_lever_from_semantic_text(self.runtime_authority)?,
            ),
            self.make_cell(
                NuCellRole::Macro,
                FullBinaryAnchor::on(),
                algorithm_lever_from_semantic_text(profile.overhead_compensation_stamp)?,
            ),
            self.make_cell(
                NuCellRole::Algorithm,
                FullBinaryAnchor::off(),
                algorithm_lever_from_semantic_text(profile.session_surface)?,
            ),
            self.make_cell(
                NuCellRole::Macro,
                FullBinaryAnchor::on(),
                algorithm_lever_from_semantic_text(profile.legacy_capability_profile)?,
            ),
        ];

        let word = NuWord { cells };
        word.validate()?;
        let lever_positions = word
            .cells
            .iter()
            .map(|cell| cell.pair.lever.clone())
            .collect::<Vec<_>>();

        Ok(OfflineInferenceReport {
            lane: self.surface.to_string(),
            model_id: public_model_id.to_string(),
            offline_only: true,
            runtime_authority: self.runtime_authority.to_string(),
            canonical_semantics: self.canonical_semantics.to_string(),
            canonical_cells: word.cells.len(),
            court_route: court_route_names(&OFFLINE_MODEL_COURT_ROUTE)
                .into_iter()
                .map(str::to_string)
                .collect(),
            lever_positions,
            representation_mode: profile.representation_mode.to_string(),
            runtime_mass_profile: profile.runtime_mass_profile.to_string(),
            tokenizer_bridge_stamp: profile.tokenizer_bridge_stamp.to_string(),
            runtime_semantic_consumers_ready: runtime_semantic_evidence.consumers_ready,
            runtime_semantic_feed_entries: runtime_semantic_evidence.feed_entries,
            runtime_compass_seed_tokens: runtime_semantic_evidence.compass_seed_tokens,
            runtime_semantic_patch_anchor_count: runtime_semantic_evidence.patch_anchor_count,
            runtime_semantic_tests_present: runtime_semantic_evidence.tests_present,
            required_donor_shard_count: runtime_materials.donor.required_shard_count,
            present_donor_shard_count: runtime_materials.donor.present_shard_count,
            materialized_donor_shard_count: runtime_materials.donor.materialized_shard_count,
            pointer_donor_shard_count: runtime_materials.donor.pointer_shard_count,
            donor_index_discovered: runtime_materials.donor.donor_index_discovered,
            donor_shard_resolved: runtime_materials.donor.donor_shard_resolved,
            donor_shard_opened: runtime_materials.donor.donor_shard_opened,
            donor_shard_payload_materialized: runtime_materials
                .donor
                .donor_shard_payload_materialized,
            donor_shard_is_lfs_pointer: runtime_materials.donor.donor_shard_is_lfs_pointer,
            donor_shard_size_bytes: runtime_materials.donor.donor_shard_size_bytes,
            donor_tensor_addressed: runtime_materials.donor.donor_tensor_addressed,
            donor_tensor_activated: runtime_materials.donor.donor_tensor_activated,
            donor_index_path: runtime_materials.donor.donor_index_path.clone(),
            donor_shard_path: runtime_materials.donor.resolved_shard_path.clone(),
            donor_tensor_name: runtime_materials.donor.resolved_tensor_name.clone(),
            nsq_artifact_present: runtime_materials.nsq.nsq_artifact_present,
            nsq_envelope_present: runtime_materials.nsq.nsq_envelope_present,
            nsq_artifact_size_bytes: runtime_materials.nsq.nsq_artifact_size_bytes,
            nsq_artifact_is_lfs_pointer: runtime_materials.nsq.nsq_artifact_is_lfs_pointer,
            nsq_artifact_is_text_manifest: runtime_materials.nsq.nsq_artifact_is_text_manifest,
            nsq_runtime_mass_profile: runtime_materials.nsq.nsq_runtime_mass_profile.clone(),
            nsq_hot_live_parameter_embodiment: runtime_materials
                .nsq
                .nsq_hot_live_parameter_embodiment,
            nsq_verification_state: runtime_materials.nsq.nsq_verification_state.clone(),
            nsq_whole_core_runtime_status: runtime_materials
                .nsq
                .nsq_whole_core_runtime_status
                .clone(),
            donor_source_lane: runtime_materials.authority.donor_source_lane.clone(),
            donor_source_state: runtime_materials.authority.donor_source_state.clone(),
            nsq_artifact_lane: runtime_materials.authority.nsq_artifact_lane.clone(),
            nsq_artifact_state: runtime_materials.authority.nsq_artifact_state.clone(),
            runtime_authority_lane: runtime_materials.authority.runtime_authority_lane.clone(),
            runtime_authority_state: runtime_materials.authority.runtime_authority_state.clone(),
            runtime_authority_bound: runtime_materials.authority.runtime_authority_bound,
            pointer_free_runtime_ready: runtime_materials.authority.pointer_free_runtime_ready,
            tokenizer_candidate_discovered: runtime_materials
                .tokenizer
                .tokenizer_candidate_discovered,
            tokenizer_runtime_selected: runtime_materials.tokenizer.tokenizer_runtime_selected,
            tokenizer_runtime_bound: runtime_materials.tokenizer.tokenizer_runtime_bound,
            tokenizer_selection_mode: runtime_materials.tokenizer.selection_mode.clone(),
            tokenizer_selected_path: runtime_materials.tokenizer.selected_tokenizer_path.clone(),
            delta_source_count: runtime_materials.delta_sources.len(),
            delta_live_source_count,
            delta_bridge_source_count,
            delta_stale_source_count,
            delta_absent_source_count,
            overlay_planner_ready: runtime_materials.overlay.planner_ready,
            overlay_runtime_bound: runtime_materials.overlay.runtime_overlay_bound,
            overlay_live_parameter_application: runtime_materials
                .overlay
                .live_parameter_application,
            overlay_metadata_entries: runtime_materials.overlay.metadata_entries,
            secure_transit_duty_cycle_ready: secure_transit.duty_cycle_ready,
            secure_transit_instruction_data_separated: secure_transit
                .instruction_data_separated,
            secure_transit_switch_release_verified: secure_transit
                .transient_switch_release_verified,
            secure_transit_self_transform_stage: "post_linter_pre_compositor".to_string(),
            secure_transit_picker_binding_state: secure_transit.picker_binding_state.clone(),
            secure_transit_phase_count: secure_transit.phase_count,
            bit_job_board_ready: bit_job_board.strict_lane_controls
                && bit_job_board.non_consumptive_cycles
                && bit_job_board.idle_pool_ready,
            bit_object_non_consumptive: bit_job_board.non_consumptive_cycles,
            bit_object_migration_enabled: bit_job_board.object_migration_enabled,
            bit_scan_communicate_only: bit_job_board.scan_or_communicate_only,
            bit_life_extension_total: bit_job_board.life_extension_total,
            persistent_client_pipe_binding_state: persistent_client
                .pipe_binding_state
                .clone(),
            persistent_client_reasoning_state: persistent_client.reasoning_state.clone(),
            persistent_client_comprehension_mode: persistent_client
                .comprehension_mode
                .clone(),
            persistent_client_fuzzy_match_enabled: persistent_client.fuzzy_match_enabled,
            persistent_client_context_awareness_mode: persistent_client
                .context_awareness_mode
                .clone(),
            persistent_client_self_expression_state: persistent_client
                .self_expression_state
                .clone(),
            persistent_client_session_continuity_ready: persistent_client
                .session_continuity_ready,
            assistant_reply: assistant_reply.clone(),
            delta_sources: runtime_materials.delta_sources.clone(),
            overhead_compensation_stamp: profile.overhead_compensation_stamp.to_string(),
            BRAXON_feature_attachment_stamp: profile.BRAXON_feature_attachment_stamp.to_string(),
            whole_parameter_stamp: profile.whole_parameter_stamp.to_string(),
            parameter_projection_mode: profile.parameter_projection_mode.to_string(),
            env_parameter_copy_mode: profile.env_parameter_copy_mode.to_string(),
            stamp_bundle: vec![
                profile.model_bundle_stamp.to_string(),
                profile.whole_parameter_stamp.to_string(),
                profile.overhead_compensation_stamp.to_string(),
                profile.tokenizer_bridge_stamp.to_string(),
                profile.BRAXON_feature_attachment_stamp.to_string(),
            ],
            BRAXON_feature_attachments: profile
                .BRAXON_feature_attachments
                .into_iter()
                .map(str::to_string)
                .collect(),
            session_surface: profile.session_surface.to_string(),
            session_mode: profile.session_mode.to_string(),
            agentic_capability: profile.agentic_capability.to_string(),
            capability_lattice_stamp: profile.capability_lattice_stamp.to_string(),
            legacy_capability_profile: profile.legacy_capability_profile.to_string(),
            legacy_capabilities,
            legacy_capabilities_status: profile.legacy_capabilities_status.to_string(),
            result_summary: format!(
                "offline_request_bound(model={public_model_id}, prompt_chars={}, representation={}, session_surface={}, capability_profile={}, donor_source_state={}, runtime_authority_lane={}, donor_payload_materialized={}, tokenizer_bound={}, overlay_planner_ready={}, transit_ready={}, pipe_binding_state={})",
                prompt.chars().count(),
                profile.representation_mode,
                profile.session_surface,
                profile.legacy_capability_profile,
                runtime_materials.authority.donor_source_state,
                runtime_materials.authority.runtime_authority_lane,
                runtime_materials.donor.donor_shard_payload_materialized,
                runtime_materials.tokenizer.tokenizer_runtime_bound,
                runtime_materials.overlay.planner_ready,
                secure_transit.duty_cycle_ready,
                persistent_client.pipe_binding_state
            ),
        })
    }

    fn make_cell(
        &self,
        role: NuCellRole,
        switch: FullBinaryAnchor,
        lever: MultipositionalLever,
    ) -> NuCell {
        NuCell {
            role,
            pair: NuPair { switch, lever },
        }
    }
}

fn supported_profile(model_id: &str) -> Option<OfflineModelStampProfile> {
    match model_id {
        "Braxon" => Some(OfflineModelStampProfile {
            representation_mode: "stamp_bound_manifest",
            runtime_mass_profile: "manifest_and_stamps_only",
            tokenizer_bridge_stamp: "nsq.runtime.native.tokenizer.bridge.v2",
            overhead_compensation_stamp: "nsq.runtime.native.overhead.compensation.v1",
            BRAXON_feature_attachment_stamp: "nsq.runtime.native.Braxon.feature.attach.v1",
            whole_parameter_stamp: "nsq.runtime.native.model.parameter.whole.v1",
            parameter_projection_mode: "single_bit_factor_shim",
            env_parameter_copy_mode: "lazy_load",
            model_bundle_stamp: "nsq.runtime.native.model.bundle.Braxon.v1",
            session_surface: "zlm_native_runtime_surface",
            session_mode: "persistent_agentic_conversation",
            agentic_capability: "full_agentic_conversation",
            BRAXON_feature_attachments: ["status", "agent", "runtime_models", "runtime_infer"],
            capability_lattice_stamp: "nsq.runtime.native.Braxon.capability.lattice.v1",
            legacy_capability_profile: "BRAXON_native_capability_lattice",
            legacy_capabilities: &[
                "conversation_continuity",
                "planning_memory_lane",
                "court_tool_routing",
                "task_queue_carry",
                "session_persistence",
                "authoring_bias_bridge",
                "operator_shell_ingress",
                "memory_guard_window",
            ],
            legacy_capabilities_status: "capability_lattice_bound",
        }),
        _ => None,
    }
}

fn runtime_asset_key(model_id: &str) -> &str {
    match model_id {
        "Braxon" => "Braxon",
        other => other,
    }
}

fn public_model_name(model_id: &str) -> &str {
    match runtime_asset_key(model_id) {
        "BRAXON_core" => "Braxon",
        other => other,
    }
}

fn resolve_runtime_root() -> Option<PathBuf> {
    for key in ["ROOT", "BRAXON_ROOT"] {
        if let Ok(value) = std::env::var(key) {
            return Some(PathBuf::from(value));
        }
    }

    if let Ok(cwd) = std::env::current_dir() {
        for ancestor in cwd.ancestors() {
            let candidate = ancestor.to_path_buf();
            if candidate
                .join("assets/braxon_core/source_ingest/braxon_transport/model.safetensors.index.json")
                .exists()
            {
                return Some(candidate);
            }
        }
    }

    let home_default =
        PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| String::from("."))).join("Braxon");
    if home_default
        .join("assets/braxon_core/source_ingest/braxon_transport/model.safetensors.index.json")
        .exists()
    {
        return Some(home_default);
    }

    None
}

fn attune_base_identity(text: &str) -> Result<MultipositionalLever, String> {
    let lower = text.trim().to_ascii_lowercase();
    let carrier = match lower.as_str() {
        "nsq" => 1126,
        "rust" => 920,
        "cargo" => 910,
        "python" | "python3" => 900,
        "lua" => 880,
        "bash" | "sh" | "shell" => 860,
        "sql" | "sqlite" => 840,
        "html" => 820,
        "css" => 800,
        "javascript" | "js" => 780,
        "typescript" | "ts" => 760,
        "json" => 740,
        "xml" => 720,
        "toml" => 700,
        "yaml" | "yml" => 680,
        "c" => 660,
        "c++" | "cpp" => 640,
        "c#" | "csharp" => 620,
        "java" => 600,
        "kotlin" => 580,
        "perl" => 560,
        "ruby" => 540,
        "guile" | "scheme" | "lisp" => 520,
        "bevy" => 500,
        "wgpu" => 480,
        "egui" => 460,
        "adb" => 440,
        "ndk" => 420,
        "sdk" => 400,
        "jni" => 380,
        _ => return algorithm_lever_from_semantic_text(text),
    };
    MultipositionalLever::new(carrier.to_string())
}

fn algorithm_lever_from_semantic_text(text: &str) -> Result<MultipositionalLever, String> {
    let samples = semantic_hertz_samples(text)?;
    MultipositionalLever::stabilize_from_hertz_samples(&samples)
}

fn semantic_hertz_samples(text: &str) -> Result<Vec<f32>, String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Err("native semantic sampling requires non-empty text".into());
    }

    let mut samples = Vec::new();
    for (index, glyph) in trimmed.chars().enumerate() {
        let scalar = glyph as usize;
        let glyph_band = (scalar % 1126) as f32 / 1125.0;
        let ordinal_band = ((index + 1) % 1126) as f32 / 1125.0;
        let sample = ((glyph_band * 0.75) + (ordinal_band * 0.25)).clamp(0.0, 1.0);
        samples.push(sample);
    }

    Ok(samples)
}

fn court_route_names(route: &[CourtSurface]) -> Vec<&'static str> {
    route.iter().map(|surface| surface.as_str()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lever_position(lever: &MultipositionalLever) -> usize {
        lever.as_canonical_text().parse::<usize>().unwrap()
    }

    #[test]
    fn python3_ingress_parses_symbol_and_args() {
        let lane = Python3RuntimeLane::default();
        let ingress = lane.ingest("vectorize(axis=2, mode='fast')").unwrap();
        assert_eq!(ingress.symbol, "vectorize");
        assert_eq!(ingress.args.len(), 2);
        assert_eq!(ingress.args[0].0, "axis");
    }

    #[test]
    fn runtime_slice_emits_canonical_word() {
        let lane = Python3RuntimeLane::default();
        let report = lane
            .execute_slice("score(task='alpha', retries=3)")
            .unwrap();
        assert_eq!(report.lane, "python3_native_runtime_lane");
        assert_eq!(report.symbol, "score");
        assert_eq!(report.arg_count, 2);
        assert!(report.canonical_cells >= 4);
        assert_eq!(
            report.court_route,
            vec!["policer", "lexer", "parser", "router", "inspector"]
        );
        assert_eq!(report.canonical_semantics, "base8_switch_topology");
    }

    #[test]
    fn canonical_word_starts_with_language_cell() {
        let lane = Python3RuntimeLane::default();
        let ingress = lane.ingest("emit()").unwrap();
        let word = lane.encode_switch_faithful(&ingress).unwrap();
        assert!(matches!(word.cells[0].role, NuCellRole::Language));
    }

    #[test]
    fn semantic_sampling_uses_hertz_stabilization_not_bytes() {
        let lane = Python3RuntimeLane::default();
        let ingress = lane.ingest("emit(topic='orbit')").unwrap();
        let word = lane.encode_switch_faithful(&ingress).unwrap();
        assert!(word
            .cells
            .iter()
            .all(|cell| (1..=1126).contains(&lever_position(&cell.pair.lever))));
    }

    #[test]
    fn base_identity_attunement_is_stable_for_language_surface() {
        let rust = lever_position(&attune_base_identity("rust").unwrap());
        let python = lever_position(&attune_base_identity("python3").unwrap());
        let nsq = lever_position(&attune_base_identity("nsq").unwrap());

        assert!((1..=1126).contains(&rust));
        assert!((1..=1126).contains(&python));
        assert_eq!(nsq, 1126);
        assert_ne!(rust, python);
    }

    #[test]
    fn algorithmic_supporting_cells_still_use_hertz_path() {
        let a = lever_position(&algorithm_lever_from_semantic_text("macro.fold").unwrap());
        let b = lever_position(&algorithm_lever_from_semantic_text("macro.fold").unwrap());
        assert_eq!(a, b);
        assert!((1..=1126).contains(&a));
    }

    #[test]
    fn runtime_registry_rejects_hook_and_wrapper_models() {
        let registry = native_runtime_registry();
        assert!(registry
            .lanes
            .iter()
            .all(|lane| !lane.runtime_model.contains("hook")));
        assert!(registry
            .lanes
            .iter()
            .all(|lane| !lane.runtime_model.contains("plugin")));
        assert!(registry
            .lanes
            .iter()
            .all(|lane| !lane.runtime_model.contains("wrapper")));
        assert!(registry
            .lanes
            .iter()
            .all(|lane| !lane.runtime_model.contains("sidecar")));
    }

    #[test]
    fn offline_model_lane_binds_supported_assets_without_cpp_authority() {
        let report = OfflineModelLane::default()
            .execute_request("Braxon", "repair phase 2 model lane")
            .unwrap();
        assert_eq!(report.lane, "offline_model_native_runtime_lane");
        assert_eq!(report.model_id, "Braxon");
        assert!(report.offline_only);
        assert_eq!(report.runtime_authority, "rust_native_offline_model_lane");
        assert_eq!(report.representation_mode, "stamp_bound_manifest");
        assert_eq!(report.runtime_mass_profile, "manifest_and_stamps_only");
        assert_eq!(report.session_surface, "zlm_native_runtime_surface");
        assert_eq!(report.session_mode, "persistent_agentic_conversation");
        assert_eq!(report.agentic_capability, "full_agentic_conversation");
        assert_eq!(
            report.capability_lattice_stamp,
            "nsq.runtime.native.Braxon.capability.lattice.v1"
        );
        assert_eq!(
            report.legacy_capability_profile,
            "BRAXON_native_capability_lattice"
        );
        assert!(report
            .legacy_capabilities
            .contains(&"conversation_continuity".to_string()));
        assert!(report
            .legacy_capabilities
            .contains(&"operator_shell_ingress".to_string()));
        assert!(report
            .stamp_bundle
            .contains(&"nsq.runtime.native.model.bundle.Braxon.v1".to_string()));
        assert_eq!(
            report.court_route,
            vec!["policer", "router", "scheduler", "inspector"]
        );
        assert!(report.secure_transit_duty_cycle_ready);
        assert!(report.secure_transit_instruction_data_separated);
        assert!(report.secure_transit_switch_release_verified);
        assert_eq!(
            report.secure_transit_self_transform_stage,
            "post_linter_pre_compositor"
        );
        assert_eq!(
            report.persistent_client_pipe_binding_state,
            "native_session_pipe_bound"
        );
        assert_eq!(
            report.persistent_client_comprehension_mode,
            "fuzzy_logic_context_match"
        );
        assert!(report.persistent_client_fuzzy_match_enabled);
        assert!(report.persistent_client_session_continuity_ready);
        assert!(report.bit_job_board_ready);
        assert!(report.bit_object_non_consumptive);
        assert!(report.bit_object_migration_enabled);
        assert!(report.bit_scan_communicate_only);
        assert!(report.bit_life_extension_total >= 6);
        assert!(report.assistant_reply.starts_with("I'm Braxon"));
    }

    #[test]
    fn runtime_registry_contains_offline_model_lane() {
        let registry = native_runtime_registry();
        let lane = registry.lane("offline_model_native_runtime_lane").unwrap();
        assert_eq!(
            lane.runtime_model,
            "rust_native_offline_inference_stamp_lane"
        );
        assert!(!lane.runtime_model.contains("cpp"));
        assert!(!lane.runtime_model.contains("foreign_tool_host"));
        let zlm = registry.lane("zlm_native_runtime_surface").unwrap();
        assert_eq!(zlm.runtime_model, "native_persistent_session_orchestrator");
        assert_eq!(zlm.canonical_semantics, "base8_switch_topology");
    }

    #[test]
    fn missing_language_triggers_native_bootstrap_full_ingress() {
        let registry = native_runtime_registry();
        let plan = registry.resolve_language_ingress("New Lang++");

        assert_eq!(plan.language_id, "new_lang");
        assert_eq!(plan.surface, "new_lang_native_runtime_lane");
        assert_eq!(plan.action, "bootstrap_native_full_ingress");
        assert!(!plan.existing_native_lane);
        assert!(plan.full_ingress_required);
        assert!(!plan.fail_closed);
        assert!(!plan.parallel_runtime_allowed);
        assert!(!plan.ported_runtime_allowed);
        assert!(!plan.shim_runtime_allowed);
        assert_eq!(plan.runtime_model, "native_runtime_incorporation");
        assert_eq!(plan.canonical_semantics, "base8_switch_topology");
    }

    #[test]
    fn known_language_uses_existing_native_full_ingress() {
        let registry = native_runtime_registry();
        let plan = registry.resolve_language_ingress("python3");

        assert_eq!(plan.surface, "python3_native_runtime_lane");
        assert_eq!(plan.action, "use_existing_native_full_ingress");
        assert!(plan.existing_native_lane);
        assert!(plan.full_ingress_required);
        assert!(!plan.fail_closed);
        assert!(!plan.parallel_runtime_allowed);
        assert!(!plan.ported_runtime_allowed);
        assert!(!plan.shim_runtime_allowed);
    }
}

#[allow(dead_code)]
pub mod runtime_native_wiring;
mod semantic_context;
pub use semantic_context::{
    load_runtime_semantic_context_from_root, load_runtime_semantic_evidence_from_root,
    runtime_semantic_context, runtime_semantic_evidence, semantic_algorithm_lever_hint,
    semantic_bias_for_text, semantic_runtime_lane_hint, RuntimeSemanticContext,
    RuntimeSemanticEvidence, SemanticBias,
};
