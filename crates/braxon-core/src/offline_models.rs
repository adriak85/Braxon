use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub const OFFLINE_MODEL_REGISTRY_RELATIVE_PATH: &str = "state/braxon/offline_model_registry.json";
pub const TOKENIZER_BRIDGE_STAMP: &str = "nsq.runtime.native.tokenizer.bridge.v2";
pub const OVERHEAD_COMPENSATION_STAMP: &str = "nsq.runtime.native.overhead.compensation.v1";
pub const BRAXON_FEATURE_ATTACHMENT_STAMP: &str = "nsq.runtime.native.Braxon.feature.attach.v1";
pub const WHOLE_PARAMETER_STAMP: &str = "nsq.runtime.native.model.parameter.whole.v1";
pub const WHOLE_PARAMETER_PROJECTION_MODE: &str = "single_bit_factor_shim";
pub const ENV_PARAMETER_COPY_MODE: &str = "lazy_load";
pub const BRAXON_TOKENIZER_BINDING_STATE: &str = "semantic_feed_bound_not_runtime_unified";
pub const BRAXON_PARAMETER_BINDING_STATE: &str = "direct_source_materialization_required";
const LEGACY_BRAXON_TOKENIZER_BOUND_STATE: &str = concat!("BRAXON_core_", "tokenizer_bound");
const LEGACY_BRAXON_PARAMETER_BOUND_STATE: &str = concat!("BRAXON_core_", "parameter_set_bound");
pub const CAPABILITY_LATTICE_STAMP: &str = "nsq.runtime.native.Braxon.capability.lattice.v1";
pub const ZLM_SESSION_SURFACE: &str = "zlm_native_runtime_surface";
pub const PERSISTENT_SESSION_MODE: &str = "persistent_agentic_conversation";
pub const FULL_AGENTIC_CAPABILITY: &str = "full_agentic_conversation";
pub const BRAXON_LEGACY_CAPABILITY_PROFILE: &str = "BRAXON_native_capability_lattice";
pub const BRAXON_CORE_BINDING_RELATIVE_PATH: &str = "state/braxon/braxon_binding.json";
pub const BRAXON_CORE_PRIMARY_MODEL: &str = "BRAXON_core_primary_model";
pub const BRAXON_SOURCE_INGEST_DIRECTORY: &str =
    "assets/braxon_core/source_ingest/braxon_transport";
pub const BRAXON_NSQ_WEIGHTS_DIRECTORY: &str = "assets/braxon_core/weights/nsq";
pub const BRAXON_NSQ_REWRITE_ARTIFACT: &str =
    "assets/braxon_core/weights/nsq/Braxon-27B_extended.nsqb";
pub const BRAXON_NSQ_ENVELOPE_ARTIFACT: &str =
    "assets/braxon_core/weights/nsq/Braxon-27B_extended.nsqb.meta";
pub const BRAXON_NSQ_REWRITE_EXTENSION: &str = "nsqb";
pub const BRAXON_NSQ_REWRITE_MODE: &str = "structure_preserving_base8_transform";
pub const BRAXON_STATUS_STATE_PATH: &str = "state/braxon/braxon_nsq_pipeline.status";
pub const BRAXON_DEFAULT_SOURCE_INGEST_STATUS: &str = "missing";
pub const BRAXON_DEFAULT_NSQ_ENVELOPE_STATUS: &str = "missing";
pub const BRAXON_DEFAULT_NSQ_REWRITE_STATUS: &str = "not_started";
pub const BRAXON_DEFAULT_WHOLE_CORE_RUNTIME_STATUS: &str = "not_ready";
pub const BRAXON_RUNTIME_LOAD_POLICY: &str = "whole_core_only";
pub const BRAXON_LAUNCH_FORM: &str = "hot_whole_core";
pub const BRAXON_ZLM_BINDING_MODE: &str = "whole_core_session_surface";
pub const BRAXON_GRID_26D_MODE: &str = "sealed_reference_structure";
pub const BRAXON_GRID_26D_ACTIVATION_MODE: &str = "semantic_score_alignment";
pub const BRAXON_SUPERMODEL_EXTENSION_MODE: &str = "sealed_reference_structure";
pub const BRAXON_SUPERMODEL_EXTENSION_ACTIVATION_MODE: &str = "semantic_score_alignment";
pub const BRAXON_DELTA_EXTENSION_MODE: &str = "sealed_reference_structure";
pub const BRAXON_DELTA_EXTENSION_ACTIVATION_MODE: &str = "semantic_score_alignment";
pub const REQUIRED_BRAXON_FEATURE_ATTACHMENTS: [&str; 4] =
    ["status", "agent", "runtime_models", "runtime_infer"];
pub const REQUIRED_BRAXON_LEGACY_CAPABILITIES: [&str; 8] = [
    "conversation_continuity",
    "planning_memory_lane",
    "court_tool_routing",
    "task_queue_carry",
    "session_persistence",
    "authoring_bias_bridge",
    "operator_shell_ingress",
    "memory_guard_window",
];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OfflineModelAssetRecord {
    pub id: String,
    pub label: String,
    pub provider_family: String,
    pub manifest_path: String,
    pub weights_directory: String,
    #[serde(default)]
    pub source_weights_directory: String,
    #[serde(default)]
    pub nsq_envelope_artifact: String,
    #[serde(default)]
    pub status_state_path: String,
    #[serde(default)]
    pub source_ingest_status: String,
    #[serde(default)]
    pub source_authority_lane: String,
    #[serde(default)]
    pub source_authority_state: String,
    #[serde(default)]
    pub nsq_envelope_status: String,
    #[serde(default)]
    pub nsq_rewrite_artifact: String,
    #[serde(default)]
    pub nsq_rewrite_extension: String,
    #[serde(default)]
    pub nsq_rewrite_mode: String,
    #[serde(default)]
    pub nsq_rewrite_status: String,
    #[serde(default)]
    pub runtime_load_policy: String,
    #[serde(default)]
    pub launch_form: String,
    #[serde(default)]
    pub zlm_binding_mode: String,
    #[serde(default)]
    pub grid_26d_mode: String,
    #[serde(default)]
    pub grid_26d_activation_mode: String,
    #[serde(default)]
    pub supermodel_extension_mode: String,
    #[serde(default)]
    pub supermodel_extension_activation_mode: String,
    #[serde(default)]
    pub delta_extension_mode: String,
    #[serde(default)]
    pub delta_extension_activation_mode: String,
    #[serde(default)]
    pub whole_core_runtime_status: String,
    #[serde(default)]
    pub nsq_artifact_state: String,
    #[serde(default)]
    pub runtime_authority_lane: String,
    #[serde(default)]
    pub runtime_authority_state: String,
    #[serde(default)]
    pub runtime_authority_bound: bool,
    #[serde(default)]
    pub live_grid_loading: bool,
    #[serde(default)]
    pub live_delta_loading: bool,
    pub authority_lane: String,
    pub runtime_authority: String,
    pub offline_only: bool,
    pub cxx_runtime_authority: bool,
    pub external_tool_host: String,
    pub status: String,
    #[serde(default)]
    pub representation_mode: String,
    #[serde(default)]
    pub runtime_mass_profile: String,
    #[serde(default)]
    pub tokenizer_bridge_stamp: String,
    #[serde(default)]
    pub overhead_compensation_stamp: String,
    #[serde(default)]
    pub BRAXON_feature_attachment_stamp: String,
    #[serde(default)]
    pub whole_parameter_stamp: String,
    #[serde(default)]
    pub parameter_projection_mode: String,
    #[serde(default)]
    pub env_parameter_copy_mode: String,
    #[serde(default)]
    pub stamp_bundle: Vec<String>,
    #[serde(default)]
    pub BRAXON_feature_attachments: Vec<String>,
    #[serde(default)]
    pub session_surface: String,
    #[serde(default)]
    pub session_mode: String,
    #[serde(default)]
    pub agentic_capability: String,
    #[serde(default)]
    pub capability_lattice_stamp: String,
    #[serde(default)]
    pub legacy_capability_profile: String,
    #[serde(default)]
    pub legacy_capabilities: Vec<String>,
    #[serde(default)]
    pub legacy_capabilities_status: String,
    #[serde(default)]
    pub BRAXON_core_identity: String,
    #[serde(default)]
    pub core_binding_path: String,
    #[serde(default)]
    pub tokenizer_binding_state: String,
    #[serde(default)]
    pub parameter_binding_state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OfflineModelRegistryState {
    pub lane_surface: String,
    pub runtime_authority: String,
    pub canonical_semantics: String,
    pub assets: Vec<OfflineModelAssetRecord>,
}

impl OfflineModelRegistryState {
    pub fn default_registry() -> Self {
        Self {
            lane_surface: "offline_model_native_runtime_lane".to_string(),
            runtime_authority: "rust_native_offline_model_lane".to_string(),
            canonical_semantics: "base8_switch_topology".to_string(),
            assets: vec![OfflineModelAssetRecord {
                id: "Braxon".to_string(),
                label: "BRAXON".to_string(),
                provider_family: "Braxon".to_string(),
                manifest_path: "models/braxon/manifest.json".to_string(),
                weights_directory: BRAXON_NSQ_WEIGHTS_DIRECTORY.to_string(),
                source_weights_directory: BRAXON_SOURCE_INGEST_DIRECTORY.to_string(),
                nsq_envelope_artifact: BRAXON_NSQ_ENVELOPE_ARTIFACT.to_string(),
                status_state_path: BRAXON_STATUS_STATE_PATH.to_string(),
                source_ingest_status: BRAXON_DEFAULT_SOURCE_INGEST_STATUS.to_string(),
                source_authority_lane: BRAXON_SOURCE_INGEST_DIRECTORY.to_string(),
                source_authority_state: BRAXON_DEFAULT_SOURCE_INGEST_STATUS.to_string(),
                nsq_envelope_status: BRAXON_DEFAULT_NSQ_ENVELOPE_STATUS.to_string(),
                nsq_rewrite_artifact: BRAXON_NSQ_REWRITE_ARTIFACT.to_string(),
                nsq_rewrite_extension: BRAXON_NSQ_REWRITE_EXTENSION.to_string(),
                nsq_rewrite_mode: BRAXON_NSQ_REWRITE_MODE.to_string(),
                nsq_rewrite_status: BRAXON_DEFAULT_NSQ_REWRITE_STATUS.to_string(),
                runtime_load_policy: BRAXON_RUNTIME_LOAD_POLICY.to_string(),
                launch_form: BRAXON_LAUNCH_FORM.to_string(),
                zlm_binding_mode: BRAXON_ZLM_BINDING_MODE.to_string(),
                grid_26d_mode: BRAXON_GRID_26D_MODE.to_string(),
                grid_26d_activation_mode: BRAXON_GRID_26D_ACTIVATION_MODE.to_string(),
                supermodel_extension_mode: BRAXON_SUPERMODEL_EXTENSION_MODE.to_string(),
                supermodel_extension_activation_mode: BRAXON_SUPERMODEL_EXTENSION_ACTIVATION_MODE
                    .to_string(),
                delta_extension_mode: BRAXON_DELTA_EXTENSION_MODE.to_string(),
                delta_extension_activation_mode: BRAXON_DELTA_EXTENSION_ACTIVATION_MODE.to_string(),
                whole_core_runtime_status: BRAXON_DEFAULT_WHOLE_CORE_RUNTIME_STATUS.to_string(),
                nsq_artifact_state: "absent_checkout".to_string(),
                runtime_authority_lane: "none_bound".to_string(),
                runtime_authority_state: "unbound".to_string(),
                runtime_authority_bound: false,
                live_grid_loading: false,
                live_delta_loading: false,
                authority_lane: "offline_model_native_runtime_lane".to_string(),
                runtime_authority: "rust_native_offline_model_lane".to_string(),
                offline_only: true,
                cxx_runtime_authority: false,
                external_tool_host: "none".to_string(),
                status: "stamp_bound_manifest_registered_core".to_string(),
                representation_mode: "stamp_bound_manifest".to_string(),
                runtime_mass_profile: "manifest_and_stamps_only".to_string(),
                tokenizer_bridge_stamp: TOKENIZER_BRIDGE_STAMP.to_string(),
                overhead_compensation_stamp: OVERHEAD_COMPENSATION_STAMP.to_string(),
                BRAXON_feature_attachment_stamp: BRAXON_FEATURE_ATTACHMENT_STAMP.to_string(),
                whole_parameter_stamp: WHOLE_PARAMETER_STAMP.to_string(),
                parameter_projection_mode: WHOLE_PARAMETER_PROJECTION_MODE.to_string(),
                env_parameter_copy_mode: ENV_PARAMETER_COPY_MODE.to_string(),
                stamp_bundle: default_stamp_bundle_for("Braxon"),
                BRAXON_feature_attachments: REQUIRED_BRAXON_FEATURE_ATTACHMENTS
                    .iter()
                    .map(|value| (*value).to_string())
                    .collect(),
                session_surface: ZLM_SESSION_SURFACE.to_string(),
                session_mode: PERSISTENT_SESSION_MODE.to_string(),
                agentic_capability: FULL_AGENTIC_CAPABILITY.to_string(),
                capability_lattice_stamp: CAPABILITY_LATTICE_STAMP.to_string(),
                legacy_capability_profile: BRAXON_LEGACY_CAPABILITY_PROFILE.to_string(),
                legacy_capabilities: default_legacy_capabilities_for("Braxon"),
                legacy_capabilities_status: "capability_lattice_bound".to_string(),
                BRAXON_core_identity: BRAXON_CORE_PRIMARY_MODEL.to_string(),
                core_binding_path: BRAXON_CORE_BINDING_RELATIVE_PATH.to_string(),
                tokenizer_binding_state: BRAXON_TOKENIZER_BINDING_STATE.to_string(),
                parameter_binding_state: BRAXON_PARAMETER_BINDING_STATE.to_string(),
            }],
        }
    }

    pub fn asset(&self, id: &str) -> Option<&OfflineModelAssetRecord> {
        let canonical_id = canonical_asset_key(id);
        self.assets.iter().find(|asset| asset.id == canonical_id)
    }

    pub fn stamp_bundle_ready(&self, id: &str) -> bool {
        self.asset(id).is_some_and(|asset| {
            asset.representation_mode == "stamp_bound_manifest"
                && asset.runtime_mass_profile == "manifest_and_stamps_only"
                && asset.tokenizer_bridge_stamp == TOKENIZER_BRIDGE_STAMP
                && asset.overhead_compensation_stamp == OVERHEAD_COMPENSATION_STAMP
                && asset.BRAXON_feature_attachment_stamp == BRAXON_FEATURE_ATTACHMENT_STAMP
                && asset.whole_parameter_stamp == WHOLE_PARAMETER_STAMP
                && asset.parameter_projection_mode == WHOLE_PARAMETER_PROJECTION_MODE
                && asset.env_parameter_copy_mode == ENV_PARAMETER_COPY_MODE
                && asset.stamp_bundle == default_stamp_bundle_for(id)
        })
    }

    pub fn BRAXON_feature_attachment_ready(&self, id: &str) -> bool {
        self.asset(id).is_some_and(|asset| {
            asset.BRAXON_feature_attachments
                == REQUIRED_BRAXON_FEATURE_ATTACHMENTS
                    .iter()
                    .map(|value| (*value).to_string())
                    .collect::<Vec<_>>()
        })
    }

    pub fn persistent_session_ready(&self, id: &str) -> bool {
        self.asset(id).is_some_and(|asset| {
            asset.session_surface == ZLM_SESSION_SURFACE
                && asset.session_mode == PERSISTENT_SESSION_MODE
                && asset.agentic_capability == FULL_AGENTIC_CAPABILITY
                && asset.zlm_binding_mode == BRAXON_ZLM_BINDING_MODE
                && asset.runtime_authority == "rust_native_offline_model_lane"
                && asset.external_tool_host == "none"
        })
    }

    pub fn legacy_capabilities_ready(&self, id: &str) -> bool {
        self.asset(id).is_some_and(|asset| {
            asset.capability_lattice_stamp == CAPABILITY_LATTICE_STAMP
                && asset.legacy_capability_profile == BRAXON_LEGACY_CAPABILITY_PROFILE
                && asset.legacy_capabilities == default_legacy_capabilities_for(id)
                && asset.legacy_capabilities_status == "capability_lattice_bound"
        })
    }

    pub fn BRAXON_core_binding_ready(&self, id: &str) -> bool {
        let canonical_id = canonical_asset_key(id);
        self.asset(id).is_some_and(|asset| match canonical_id {
            "Braxon" | "BRAXON_core" => {
                asset.BRAXON_core_identity == BRAXON_CORE_PRIMARY_MODEL
                    && asset.core_binding_path == BRAXON_CORE_BINDING_RELATIVE_PATH
                    && asset.tokenizer_binding_state == BRAXON_TOKENIZER_BINDING_STATE
                    && asset.parameter_binding_state == BRAXON_PARAMETER_BINDING_STATE
                    && asset.whole_parameter_stamp == WHOLE_PARAMETER_STAMP
                    && asset.parameter_projection_mode == WHOLE_PARAMETER_PROJECTION_MODE
                    && asset.env_parameter_copy_mode == ENV_PARAMETER_COPY_MODE
            }
            _ => true,
        })
    }

    pub fn whole_core_policy_ready(&self, id: &str) -> bool {
        let canonical_id = canonical_asset_key(id);
        self.asset(id).is_some_and(|asset| match canonical_id {
            "Braxon" | "BRAXON_core" => {
                asset.weights_directory == BRAXON_NSQ_WEIGHTS_DIRECTORY
                    && asset.source_weights_directory == BRAXON_SOURCE_INGEST_DIRECTORY
                    && asset.nsq_envelope_artifact == BRAXON_NSQ_ENVELOPE_ARTIFACT
                    && asset.nsq_rewrite_artifact == BRAXON_NSQ_REWRITE_ARTIFACT
                    && asset.nsq_rewrite_extension == BRAXON_NSQ_REWRITE_EXTENSION
                    && asset.nsq_rewrite_mode == BRAXON_NSQ_REWRITE_MODE
                    && asset.status_state_path == BRAXON_STATUS_STATE_PATH
                    && asset.runtime_load_policy == BRAXON_RUNTIME_LOAD_POLICY
                    && asset.launch_form == BRAXON_LAUNCH_FORM
                    && asset.zlm_binding_mode == BRAXON_ZLM_BINDING_MODE
                    && asset.grid_26d_mode == BRAXON_GRID_26D_MODE
                    && asset.grid_26d_activation_mode == BRAXON_GRID_26D_ACTIVATION_MODE
                    && asset.supermodel_extension_mode == BRAXON_SUPERMODEL_EXTENSION_MODE
                    && asset.supermodel_extension_activation_mode
                        == BRAXON_SUPERMODEL_EXTENSION_ACTIVATION_MODE
                    && asset.delta_extension_mode == BRAXON_DELTA_EXTENSION_MODE
                    && asset.delta_extension_activation_mode
                        == BRAXON_DELTA_EXTENSION_ACTIVATION_MODE
                    && !asset.live_grid_loading
                    && !asset.live_delta_loading
            }
            _ => true,
        })
    }

    fn repair_defaults(&mut self) -> bool {
        let mut changed = false;
        if self.lane_surface.is_empty() {
            self.lane_surface = "offline_model_native_runtime_lane".to_string();
            changed = true;
        }
        if self.runtime_authority.is_empty() {
            self.runtime_authority = "rust_native_offline_model_lane".to_string();
            changed = true;
        }
        if self.canonical_semantics.is_empty() {
            self.canonical_semantics = "base8_switch_topology".to_string();
            changed = true;
        }
        for asset in &mut self.assets {
            changed |= asset.repair_defaults();
        }
        changed
    }
}

impl OfflineModelAssetRecord {
    fn repair_defaults(&mut self) -> bool {
        let mut changed = false;
        let _canonical_id = canonical_asset_key(&self.id);
        let expected_status = crate::OfflineTaskStatus::Done;
        if self.status.is_empty()
            || self.status == "manifest_registered"
            || self.status == "stamp_bound_manifest_registered"
        {
            self.status = match expected_status {
                crate::OfflineTaskStatus::Pending => "Pending".to_string(),
                crate::OfflineTaskStatus::InProgress => "InProgress".to_string(),
                crate::OfflineTaskStatus::Done => "Done".to_string(),
                crate::OfflineTaskStatus::Blocked => "Blocked".to_string(),
            };
            changed = true;
        }
        if self.representation_mode.is_empty() {
            self.representation_mode = "stamp_bound_manifest".to_string();
            changed = true;
        }
        if self.runtime_mass_profile.is_empty() {
            self.runtime_mass_profile = "manifest_and_stamps_only".to_string();
            changed = true;
        }
        if self.tokenizer_bridge_stamp.is_empty() {
            self.tokenizer_bridge_stamp = TOKENIZER_BRIDGE_STAMP.to_string();
            changed = true;
        }
        if self.overhead_compensation_stamp.is_empty() {
            self.overhead_compensation_stamp = OVERHEAD_COMPENSATION_STAMP.to_string();
            changed = true;
        }
        if self.BRAXON_feature_attachment_stamp.is_empty() {
            self.BRAXON_feature_attachment_stamp = BRAXON_FEATURE_ATTACHMENT_STAMP.to_string();
            changed = true;
        }
        if self.whole_parameter_stamp.is_empty() {
            self.whole_parameter_stamp = WHOLE_PARAMETER_STAMP.to_string();
            changed = true;
        }
        if self.parameter_projection_mode.is_empty() {
            self.parameter_projection_mode = WHOLE_PARAMETER_PROJECTION_MODE.to_string();
            changed = true;
        }
        if self.env_parameter_copy_mode.is_empty() {
            self.env_parameter_copy_mode = ENV_PARAMETER_COPY_MODE.to_string();
            changed = true;
        }

        let expected_bundle = default_stamp_bundle_for(&self.id);
        if self.stamp_bundle != expected_bundle {
            self.stamp_bundle = expected_bundle;
            changed = true;
        }

        let expected_attachments = REQUIRED_BRAXON_FEATURE_ATTACHMENTS
            .iter()
            .map(|value| (*value).to_string())
            .collect::<Vec<_>>();
        if self.BRAXON_feature_attachments != expected_attachments {
            self.BRAXON_feature_attachments = expected_attachments;
            changed = true;
        }

        if self.session_surface.is_empty() {
            self.session_surface = ZLM_SESSION_SURFACE.to_string();
            changed = true;
        }
        if self.session_mode.is_empty() {
            self.session_mode = PERSISTENT_SESSION_MODE.to_string();
            changed = true;
        }
        if self.agentic_capability.is_empty() {
            self.agentic_capability = FULL_AGENTIC_CAPABILITY.to_string();
            changed = true;
        }
        if self.capability_lattice_stamp.is_empty() {
            self.capability_lattice_stamp = CAPABILITY_LATTICE_STAMP.to_string();
            changed = true;
        }
        if self.legacy_capability_profile.is_empty() {
            self.legacy_capability_profile = BRAXON_LEGACY_CAPABILITY_PROFILE.to_string();
            changed = true;
        }
        let expected_legacy_capabilities = default_legacy_capabilities_for(&self.id);
        if self.legacy_capabilities != expected_legacy_capabilities {
            self.legacy_capabilities = expected_legacy_capabilities;
            changed = true;
        }
        if self.legacy_capabilities_status.is_empty() {
            self.legacy_capabilities_status = "capability_lattice_bound".to_string();
            changed = true;
        }
        if matches!(self.id.as_str(), "Braxon" | "BRAXON_core") {
            if self.weights_directory != BRAXON_NSQ_WEIGHTS_DIRECTORY {
                self.weights_directory = BRAXON_NSQ_WEIGHTS_DIRECTORY.to_string();
                changed = true;
            }
            if self.source_weights_directory.is_empty() {
                self.source_weights_directory = BRAXON_SOURCE_INGEST_DIRECTORY.to_string();
                changed = true;
            }
            if self.nsq_envelope_artifact.is_empty() {
                self.nsq_envelope_artifact = BRAXON_NSQ_ENVELOPE_ARTIFACT.to_string();
                changed = true;
            }
            if self.nsq_rewrite_artifact.is_empty() {
                self.nsq_rewrite_artifact = BRAXON_NSQ_REWRITE_ARTIFACT.to_string();
                changed = true;
            }
            if self.status_state_path.is_empty() {
                self.status_state_path = BRAXON_STATUS_STATE_PATH.to_string();
                changed = true;
            }
            if self.source_ingest_status.is_empty() {
                self.source_ingest_status = BRAXON_DEFAULT_SOURCE_INGEST_STATUS.to_string();
                changed = true;
            }
            if self.source_authority_lane.is_empty() {
                self.source_authority_lane = BRAXON_SOURCE_INGEST_DIRECTORY.to_string();
                changed = true;
            }
            if self.source_authority_state.is_empty() {
                self.source_authority_state = BRAXON_DEFAULT_SOURCE_INGEST_STATUS.to_string();
                changed = true;
            }
            if self.nsq_envelope_status.is_empty() {
                self.nsq_envelope_status = BRAXON_DEFAULT_NSQ_ENVELOPE_STATUS.to_string();
                changed = true;
            }
            if self.nsq_rewrite_extension.is_empty() {
                self.nsq_rewrite_extension = BRAXON_NSQ_REWRITE_EXTENSION.to_string();
                changed = true;
            }
            if self.nsq_rewrite_mode.is_empty() {
                self.nsq_rewrite_mode = BRAXON_NSQ_REWRITE_MODE.to_string();
                changed = true;
            }
            if self.nsq_rewrite_status.is_empty() {
                self.nsq_rewrite_status = BRAXON_DEFAULT_NSQ_REWRITE_STATUS.to_string();
                changed = true;
            }
            if self.runtime_load_policy.is_empty() {
                self.runtime_load_policy = BRAXON_RUNTIME_LOAD_POLICY.to_string();
                changed = true;
            }
            if self.launch_form.is_empty() {
                self.launch_form = BRAXON_LAUNCH_FORM.to_string();
                changed = true;
            }
            if self.zlm_binding_mode.is_empty() {
                self.zlm_binding_mode = BRAXON_ZLM_BINDING_MODE.to_string();
                changed = true;
            }
            if self.grid_26d_mode.is_empty() {
                self.grid_26d_mode = BRAXON_GRID_26D_MODE.to_string();
                changed = true;
            }
            if self.grid_26d_activation_mode.is_empty() {
                self.grid_26d_activation_mode = BRAXON_GRID_26D_ACTIVATION_MODE.to_string();
                changed = true;
            }
            if self.supermodel_extension_mode.is_empty() {
                self.supermodel_extension_mode = BRAXON_SUPERMODEL_EXTENSION_MODE.to_string();
                changed = true;
            }
            if self.supermodel_extension_activation_mode.is_empty() {
                self.supermodel_extension_activation_mode =
                    BRAXON_SUPERMODEL_EXTENSION_ACTIVATION_MODE.to_string();
                changed = true;
            }
            if self.delta_extension_mode.is_empty() {
                self.delta_extension_mode = BRAXON_DELTA_EXTENSION_MODE.to_string();
                changed = true;
            }
            if self.delta_extension_activation_mode.is_empty() {
                self.delta_extension_activation_mode =
                    BRAXON_DELTA_EXTENSION_ACTIVATION_MODE.to_string();
                changed = true;
            }
            if self.whole_core_runtime_status.is_empty() {
                self.whole_core_runtime_status =
                    BRAXON_DEFAULT_WHOLE_CORE_RUNTIME_STATUS.to_string();
                changed = true;
            }
            if self.nsq_artifact_state.is_empty() {
                self.nsq_artifact_state = "absent_checkout".to_string();
                changed = true;
            }
            if self.runtime_authority_lane.is_empty() {
                self.runtime_authority_lane = "none_bound".to_string();
                changed = true;
            }
            if self.runtime_authority_state.is_empty() {
                self.runtime_authority_state = "unbound".to_string();
                changed = true;
            }
            if self.BRAXON_core_identity.is_empty() {
                self.BRAXON_core_identity = BRAXON_CORE_PRIMARY_MODEL.to_string();
                changed = true;
            }
            if self.core_binding_path.is_empty() {
                self.core_binding_path = BRAXON_CORE_BINDING_RELATIVE_PATH.to_string();
                changed = true;
            }
            if self.tokenizer_binding_state.is_empty()
                || self.tokenizer_binding_state == LEGACY_BRAXON_TOKENIZER_BOUND_STATE
            {
                self.tokenizer_binding_state = BRAXON_TOKENIZER_BINDING_STATE.to_string();
                changed = true;
            }
            if self.parameter_binding_state.is_empty()
                || self.parameter_binding_state == LEGACY_BRAXON_PARAMETER_BOUND_STATE
            {
                self.parameter_binding_state = BRAXON_PARAMETER_BINDING_STATE.to_string();
                changed = true;
            }
            if self.live_grid_loading {
                self.live_grid_loading = false;
                changed = true;
            }
            if self.live_delta_loading {
                self.live_delta_loading = false;
                changed = true;
            }
        } else {
            if self.BRAXON_core_identity.is_empty() {
                self.BRAXON_core_identity = "secondary_model_surface".to_string();
                changed = true;
            }
            if self.tokenizer_binding_state.is_empty() {
                self.tokenizer_binding_state = "external_or_secondary".to_string();
                changed = true;
            }
            if self.parameter_binding_state.is_empty() {
                self.parameter_binding_state = "external_or_secondary".to_string();
                changed = true;
            }
        }
        changed
    }
}

fn default_model_bundle_stamp(asset_id: &str) -> &'static str {
    match canonical_asset_key(asset_id) {
        "Braxon" | "BRAXON_core" => "nsq.runtime.native.model.bundle.Braxon.v1",
        _ => "nsq.runtime.native.model.bundle.generic.v1",
    }
}

fn default_stamp_bundle_for(asset_id: &str) -> Vec<String> {
    vec![
        default_model_bundle_stamp(asset_id).to_string(),
        WHOLE_PARAMETER_STAMP.to_string(),
        OVERHEAD_COMPENSATION_STAMP.to_string(),
        TOKENIZER_BRIDGE_STAMP.to_string(),
        BRAXON_FEATURE_ATTACHMENT_STAMP.to_string(),
    ]
}

fn default_legacy_capabilities_for(asset_id: &str) -> Vec<String> {
    match canonical_asset_key(asset_id) {
        "Braxon" | "BRAXON_core" => REQUIRED_BRAXON_LEGACY_CAPABILITIES
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        _ => vec!["conversation_continuity".to_string()],
    }
}

fn canonical_asset_key(asset_id: &str) -> &str {
    match asset_id {
        "Braxon" => "Braxon",
        other => other,
    }
}

pub fn offline_model_registry_path(root: &Path) -> PathBuf {
    root.join(OFFLINE_MODEL_REGISTRY_RELATIVE_PATH)
}

pub fn load_or_initialize_offline_model_registry(
    root: &Path,
) -> Result<OfflineModelRegistryState, String> {
    let path = offline_model_registry_path(root);
    if path.exists() {
        let raw = fs::read_to_string(&path).map_err(|err| {
            format!(
                "failed to read offline model registry '{}': {err}",
                path.display()
            )
        })?;
        let mut registry: OfflineModelRegistryState =
            serde_json::from_str(&raw).map_err(|err| {
                format!(
                    "failed to parse offline model registry '{}': {err}",
                    path.display()
                )
            })?;
        if registry.repair_defaults() {
            save_offline_model_registry(root, &registry)?;
        }
        Ok(registry)
    } else {
        let registry = OfflineModelRegistryState::default_registry();
        save_offline_model_registry(root, &registry)?;
        Ok(registry)
    }
}

pub fn save_offline_model_registry(
    root: &Path,
    registry: &OfflineModelRegistryState,
) -> Result<(), String> {
    let path = offline_model_registry_path(root);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| {
            format!(
                "failed to create offline model registry directory '{}': {err}",
                parent.display()
            )
        })?;
    }

    let body = serde_json::to_string_pretty(registry)
        .map_err(|err| format!("failed to serialize offline model registry: {err}"))?;
    fs::write(&path, body).map_err(|err| {
        format!(
            "failed to write offline model registry '{}': {err}",
            path.display()
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_registry_keeps_offline_only_assets() {
        let registry = OfflineModelRegistryState::default_registry();
        assert_eq!(registry.assets.len(), 1);
        assert!(registry.assets.iter().all(|asset| asset.offline_only));
        assert!(registry
            .assets
            .iter()
            .all(|asset| !asset.cxx_runtime_authority && asset.external_tool_host == "none"));
        assert!(registry.stamp_bundle_ready("Braxon"));
        assert!(registry.BRAXON_feature_attachment_ready("Braxon"));
        assert!(registry.persistent_session_ready("Braxon"));
        assert!(registry.legacy_capabilities_ready("Braxon"));
        assert!(registry.BRAXON_core_binding_ready("Braxon"));
    }

    #[test]
    fn registry_can_find_BRAXON() {
        let registry = OfflineModelRegistryState::default_registry();
        assert!(registry.asset("Braxon").is_some());
        assert!(registry.asset("Braxon").is_some());
    }

    #[test]
    fn legacy_registry_records_are_repaired_with_stamp_defaults() {
        let mut registry = OfflineModelRegistryState {
            lane_surface: String::new(),
            runtime_authority: String::new(),
            canonical_semantics: String::new(),
            assets: vec![OfflineModelAssetRecord {
                id: "Braxon".to_string(),
                label: "BRAXON".to_string(),
                provider_family: "Braxon".to_string(),
                manifest_path: "models/braxon/manifest.json".to_string(),
                weights_directory: "models/braxon/weights".to_string(),
                authority_lane: "offline_model_native_runtime_lane".to_string(),
                runtime_authority: "rust_native_offline_model_lane".to_string(),
                offline_only: true,
                cxx_runtime_authority: false,
                external_tool_host: "none".to_string(),
                status: "manifest_registered".to_string(),
                representation_mode: String::new(),
                runtime_mass_profile: String::new(),
                tokenizer_bridge_stamp: String::new(),
                overhead_compensation_stamp: String::new(),
                BRAXON_feature_attachment_stamp: String::new(),
                whole_parameter_stamp: String::new(),
                parameter_projection_mode: String::new(),
                env_parameter_copy_mode: String::new(),
                stamp_bundle: Vec::new(),
                BRAXON_feature_attachments: Vec::new(),
                session_surface: String::new(),
                session_mode: String::new(),
                agentic_capability: String::new(),
                capability_lattice_stamp: String::new(),
                legacy_capability_profile: String::new(),
                legacy_capabilities: Vec::new(),
                legacy_capabilities_status: String::new(),
                BRAXON_core_identity: String::new(),
                core_binding_path: String::new(),
                tokenizer_binding_state: String::new(),
                parameter_binding_state: String::new(),
                source_weights_directory: String::new(),
                nsq_envelope_artifact: String::new(),
                status_state_path: String::new(),
                source_ingest_status: String::new(),
                source_authority_lane: String::new(),
                source_authority_state: String::new(),
                nsq_envelope_status: String::new(),
                nsq_rewrite_artifact: String::new(),
                nsq_rewrite_extension: String::new(),
                nsq_rewrite_mode: String::new(),
                nsq_rewrite_status: String::new(),
                runtime_load_policy: String::new(),
                launch_form: String::new(),
                zlm_binding_mode: String::new(),
                grid_26d_mode: String::new(),
                grid_26d_activation_mode: String::new(),
                supermodel_extension_mode: String::new(),
                supermodel_extension_activation_mode: String::new(),
                delta_extension_mode: String::new(),
                delta_extension_activation_mode: String::new(),
                whole_core_runtime_status: String::new(),
                nsq_artifact_state: String::new(),
                runtime_authority_lane: String::new(),
                runtime_authority_state: String::new(),
                runtime_authority_bound: false,
                live_grid_loading: false,
                live_delta_loading: false,
            }],
        };

        assert!(registry.repair_defaults());
        assert!(registry.stamp_bundle_ready("Braxon"));
        assert!(registry.BRAXON_feature_attachment_ready("Braxon"));
        assert!(registry.persistent_session_ready("Braxon"));
        assert!(registry.legacy_capabilities_ready("Braxon"));
        assert!(registry.BRAXON_core_binding_ready("Braxon"));
    }
}
