//! NSQ Native Platform Runtime Backbone
//!
//! Boot and platform authority for the NSQ court on ARM64 Android (Termux).
//!
//! Android is the CARRIER. Braxon is the sovereign OS.
//! The court does not boot until CourtBootClearance::nsq_court_launch_ready is true.
//! No partial boot. No degraded mode. No fallback to restricted models.
//! The inner system communicates in intent gradient pressure exclusively.
//! Tokenization is a boundary projection tool — it does not exist inside the court.

use nsq_core::intent::{CourtBootClearance, CourtLanguageLaw, CourtSeating};
use nsq_core::{CANONICAL_SWITCH_POSITIONS, NSQ_CANONICAL_SWITCH_SHAPE};
use serde::{Deserialize, Serialize};

/// The canonical NSQ base for this runtime.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativePlatformCanonicalNsq {
    pub base: u8,
    pub not_u8: bool,
    pub not_bytes: bool,
    pub switch_shape: [u16; CANONICAL_SWITCH_POSITIONS],
}

impl Default for NativePlatformCanonicalNsq {
    fn default() -> Self {
        Self {
            base: 8,
            not_u8: true,
            not_bytes: true,
            switch_shape: NSQ_CANONICAL_SWITCH_SHAPE,
        }
    }
}

/// A native platform family — how a language/runtime family is classified
/// at the ingress boundary of the NSQ court. These are BOUNDARY descriptors.
/// They exist at the surface of the court, not inside it.
/// Inside the court there is only intent gradient pressure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativePlatformFamilyDescriptor {
    pub id: String,
    pub surface: String,
    pub boundary_authority: String,
    pub not_plugin: bool,
    pub fail_closed_until_proven: bool,
    pub nsq_translation_boundary: String,
    pub court_route: Vec<String>,
    pub boot_handoff_restrictions: String,
}

/// Boot clearance for the NSQ court on the native platform.
/// `nsq_court_launch_ready` is the authoritative boot gate.
/// Replaces `final_dax_os_boot_launch_ready` everywhere it appeared.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NativePlatformLaunchAdmission {
    pub schema: String,
    pub operator_scaffold_launch_ready: bool,
    /// The boot gate. True only when all court conditions are met.
    pub nsq_court_launch_ready: bool,
    pub fail_closed_policy: String,
    pub signed_handoff_required: bool,
    pub native_binding_required: bool,
    pub whole_core_hot_live_required: bool,
    pub inner_system_language_law_active: bool,
}

impl NativePlatformLaunchAdmission {
    pub fn not_ready() -> Self {
        Self {
            schema: "nsq.runtime.launch_admission.v1".to_string(),
            operator_scaffold_launch_ready: false,
            nsq_court_launch_ready: false,
            fail_closed_policy: "court does not start unless all conditions are met; no degraded mode; no restricted model fallback".to_string(),
            signed_handoff_required: true,
            native_binding_required: true,
            whole_core_hot_live_required: true,
            inner_system_language_law_active: true,
        }
    }

    pub fn from_boot_clearance(
        operator_scaffold_ready: bool,
        clearance: &CourtBootClearance,
    ) -> Self {
        Self {
            schema: "nsq.runtime.launch_admission.v1".to_string(),
            operator_scaffold_launch_ready: operator_scaffold_ready,
            nsq_court_launch_ready: clearance.nsq_court_launch_ready,
            fail_closed_policy: "court does not start unless all conditions are met; no degraded mode; no restricted model fallback".to_string(),
            signed_handoff_required: true,
            native_binding_required: true,
            whole_core_hot_live_required: true,
            inner_system_language_law_active: clearance.language_law_active,
        }
    }
}

/// The complete native platform runtime backbone.
///
/// This is the boot authority. It does not make Braxon an app.
/// It makes Braxon the court that runs on top of the Android carrier.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativePlatformRuntimeBackbone {
    pub schema: String,
    pub watermark: String,
    pub canonical_nsq: NativePlatformCanonicalNsq,
    /// Termux is the carrier — the NSQ court is the authority above it.
    pub termux_role: String,
    pub termux_runtime_authority: bool,
    pub families: Vec<NativePlatformFamilyDescriptor>,
    pub native_spine: Vec<String>,
    pub launch_admission: NativePlatformLaunchAdmission,
    pub inner_system_language_law: CourtLanguageLaw,
}

impl NativePlatformRuntimeBackbone {
    pub fn new() -> Self {
        Self {
            schema: "nsq.runtime.native_platform_runtime_backbone.v1".to_string(),
            watermark: "BRAXON_NSQ_COURT_NATIVE_PLATFORM_RUNTIME_BACKBONE_V1".to_string(),
            canonical_nsq: NativePlatformCanonicalNsq::default(),
            termux_role: "carrier — Android/Termux is the host carrier; NSQ court is the sovereign authority; Termux does not own the court".to_string(),
            termux_runtime_authority: false,
            families: build_platform_families(),
            native_spine: vec![
                "nsq-core".to_string(),
                "nsq-court".to_string(),
                "nsq-runtime".to_string(),
                "nsq-council".to_string(),
                "nsq-wake".to_string(),
                "braxon-core".to_string(),
                "braxon-court".to_string(),
            ],
            launch_admission: NativePlatformLaunchAdmission::not_ready(),
            inner_system_language_law: CourtLanguageLaw::active(),
        }
    }

    pub fn evaluate_launch(
        &mut self,
        seating: &CourtSeating,
        operator_scaffold_ready: bool,
        signed_handoff_present: bool,
        native_binding_confirmed: bool,
    ) {
        let clearance = CourtBootClearance::evaluate(
            seating,
            signed_handoff_present,
            native_binding_confirmed,
        );
        self.launch_admission =
            NativePlatformLaunchAdmission::from_boot_clearance(operator_scaffold_ready, &clearance);
    }
}

impl Default for NativePlatformRuntimeBackbone {
    fn default() -> Self {
        Self::new()
    }
}

fn build_platform_families() -> Vec<NativePlatformFamilyDescriptor> {
    vec![
        NativePlatformFamilyDescriptor {
            id: "termux".to_string(),
            surface: "termux_linux_arm64".to_string(),
            boundary_authority: "carrier — Termux provides the execution environment; NSQ court is the authority above it".to_string(),
            not_plugin: true,
            fail_closed_until_proven: true,
            nsq_translation_boundary: "all traffic from Termux surface is translated to intent gradient at ingress; intent gradient is translated to human text at egress only".to_string(),
            court_route: vec!["policer".to_string(), "lexer".to_string(), "parser".to_string(), "router".to_string(), "inspector".to_string()],
            boot_handoff_restrictions: "no host ownership; NSQ court admits only signed handoff from operator scaffold".to_string(),
        },
        NativePlatformFamilyDescriptor {
            id: "android_jni".to_string(),
            surface: "android_jni_portal".to_string(),
            boundary_authority: "JNI is a carrier bridge — the NSQ court does not grant Android administrative authority".to_string(),
            not_plugin: true,
            fail_closed_until_proven: true,
            nsq_translation_boundary: "JNI surface traffic translated to intent gradient at ingress boundary; no JNI types inside the court".to_string(),
            court_route: vec!["policer".to_string(), "compositor".to_string(), "router".to_string(), "inspector".to_string()],
            boot_handoff_restrictions: "administrative shell cannot become NSQ court authority; signed handoff required".to_string(),
        },
        NativePlatformFamilyDescriptor {
            id: "python3".to_string(),
            surface: "python3_boundary_surface".to_string(),
            boundary_authority: "boundary tool — Python is a surface scripting layer; it does not own the court".to_string(),
            not_plugin: true,
            fail_closed_until_proven: true,
            nsq_translation_boundary: "Python surface traffic translated to intent gradient at ingress; Python receives human text at egress only".to_string(),
            court_route: vec!["policer".to_string(), "lexer".to_string(), "router".to_string(), "inspector".to_string()],
            boot_handoff_restrictions: "Python3 is a carrier surface tool; it cannot claim NSQ court authority".to_string(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backbone_has_no_dax_references() {
        let backbone = NativePlatformRuntimeBackbone::new();
        let json = serde_json::to_string(&backbone).unwrap();
        assert!(!json.contains("dax_os_boot"));
        assert!(!json.contains("final_dax"));
    }

    #[test]
    fn backbone_watermark_is_braxon_nsq_court() {
        let backbone = NativePlatformRuntimeBackbone::new();
        assert!(backbone.watermark.contains("BRAXON_NSQ_COURT"));
    }

    #[test]
    fn launch_admission_not_ready_by_default() {
        let backbone = NativePlatformRuntimeBackbone::new();
        assert!(!backbone.launch_admission.nsq_court_launch_ready);
        assert!(!backbone.launch_admission.operator_scaffold_launch_ready);
    }

    #[test]
    fn termux_is_carrier_not_authority() {
        let backbone = NativePlatformRuntimeBackbone::new();
        assert!(!backbone.termux_runtime_authority);
        assert!(backbone.termux_role.contains("carrier"));
    }

    #[test]
    fn inner_system_language_law_active_at_boot() {
        let backbone = NativePlatformRuntimeBackbone::new();
        assert!(backbone.inner_system_language_law.no_internal_language_flag);
        assert!(backbone.inner_system_language_law.inner_system_language.contains("nsq_intent_gradient"));
        assert!(backbone.launch_admission.inner_system_language_law_active);
    }

    #[test]
    fn all_families_enforce_translation_boundary() {
        let backbone = NativePlatformRuntimeBackbone::new();
        for family in &backbone.families {
            assert!(
                family.nsq_translation_boundary.contains("intent gradient"),
                "family {} missing intent gradient translation boundary",
                family.id
            );
        }
    }

    #[test]
    fn nsq_council_and_wake_in_native_spine() {
        let backbone = NativePlatformRuntimeBackbone::new();
        assert!(backbone.native_spine.iter().any(|s| s == "nsq-council"));
        assert!(backbone.native_spine.iter().any(|s| s == "nsq-wake"));
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeSemanticContext {
    pub workspace_root: std::path::PathBuf,
    pub semantic_lane: String,
    pub algorithm_lever_hint: u64,
    pub runtime_lane_hint: String,
    pub evidence_loaded: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfflineModelLane {
    Logic,
    Creativity,
    Arbiter,
    Analyzer,
    Limbic,
    Support,
    Vision,
    Hearing,
    Emotion,
    SurfaceTranslation,
}

pub fn load_runtime_semantic_context_from_root(
    root: impl AsRef<std::path::Path>,
) -> RuntimeSemanticContext {
    RuntimeSemanticContext {
        workspace_root: root.as_ref().to_path_buf(),
        semantic_lane: "nsq_intent_gradient_runtime".to_string(),
        algorithm_lever_hint: semantic_algorithm_lever_hint("nsq_intent_gradient_runtime"),
        runtime_lane_hint: semantic_runtime_lane_hint("nsq_intent_gradient_runtime"),
        evidence_loaded: load_runtime_semantic_evidence_from_root(root),
    }
}

pub fn load_runtime_semantic_evidence_from_root(
    root: impl AsRef<std::path::Path>,
) -> bool {
    let root = root.as_ref();
    root.join("apps/nsq/lawful_bare_metal_boot_task.nsq").exists()
        || root.join("BRAXON_GLOBAL_TAG.json").exists()
        || root.join("crates/nsq-core").exists()
}

pub fn semantic_algorithm_lever_hint(text: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in text.as_bytes() {
        hash ^= *b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

pub fn semantic_bias_for_text(text: &str) -> i64 {
    let lower = text.to_ascii_lowercase();
    let positive = ["preserve", "truth", "intent", "court", "substrate", "build", "restore"];
    let negative = ["shim", "fake", "stub", "app", "feature", "tokenizer", "drift"];

    let mut score = 0i64;
    for term in positive {
        if lower.contains(term) {
            score += 1;
        }
    }
    for term in negative {
        if lower.contains(term) {
            score -= 1;
        }
    }
    score
}

pub fn semantic_runtime_lane_hint(text: &str) -> String {
    let lower = text.to_ascii_lowercase();

    if lower.contains("court") || lower.contains("substrate") {
        "nsq_court_substrate".to_string()
    } else if lower.contains("intent") || lower.contains("gradient") {
        "nsq_intent_gradient_runtime".to_string()
    } else if lower.contains("model") || lower.contains("offline") {
        "offline_model_lane".to_string()
    } else {
        "surface_translation_lane".to_string()
    }
}
