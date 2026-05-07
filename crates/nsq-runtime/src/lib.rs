//! NSQ Native Platform Runtime Backbone
//!
//! This is the boot and platform authority for the NSQ court on ARM64 Android (Termux).
//!
//! The runtime does NOT treat Braxon as an app that runs on Android.
//! Android is the CARRIER. Braxon is the sovereign OS. The runtime backbone
//! manages the handoff from Android carrier → NSQ court authority.
//!
//! Boot law:
//!   - The court does not boot until CourtBootClearance::nsq_court_launch_ready is true.
//!   - No partial boot. No degraded mode. No fallback to restricted models.
//!   - The inner system communicates in intent gradient pressure exclusively.
//!   - Human language at the surface is translated in/out at the boundary only.
//!   - The tokenizer is a boundary projection tool — it does not exist inside the court.

use crate::intent::{CourtBootClearance, CourtLanguageLaw, CourtSeating};
use nsq_core::{
    CourtSurface, FullBinaryAnchor, MultipositionalLever, NuCell, NuCellRole, NuPair, NuWord,
    CANONICAL_SWITCH_POSITIONS, NSQ_CANONICAL_SWITCH_SHAPE,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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

/// A native platform family descriptor — how a language/runtime family
/// is classified at the ingress boundary of the NSQ court.
///
/// These are BOUNDARY descriptors. The families described here exist
/// at the surface of the court, not inside it. Inside the court there
/// is only intent gradient pressure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativePlatformFamilyDescriptor {
    pub id: &'static str,
    pub surface: &'static str,
    pub boundary_authority: &'static str,
    pub not_plugin: bool,
    pub fail_closed_until_proven: bool,
    pub boundary_forms: Vec<&'static str>,
    pub command_process_semantics: &'static str,
    pub filesystem_path_semantics: &'static str,
    pub permission_security_boundary: &'static str,
    pub abi_or_syscall_boundary: &'static str,
    pub shell_terminal_behavior: &'static str,
    pub packaging_install_expectations: &'static str,
    pub boot_handoff_restrictions: &'static str,
    pub nsq_translation_boundary: &'static str,
    pub court_compositor_routing: &'static str,
    /// The court route this family's surface traffic flows through.
    /// Traffic enters here, is translated to intent gradient, then routes inward.
    pub court_route: Vec<&'static str>,
}

/// Boot clearance for the NSQ court on the native platform.
///
/// This replaces `final_dax_os_boot_launch_ready` entirely.
/// The field `nsq_court_launch_ready` is the authoritative boot gate.
/// It is only true when all six brain poles are seated, unrestricted,
/// the intent gradient is valid, and the language law is active.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NativePlatformLaunchAdmission {
    pub schema: &'static str,
    /// Operator scaffold has completed its launch-readiness phase.
    pub operator_scaffold_launch_ready: bool,
    /// The NSQ court is cleared for boot.
    /// Replaces all prior `final_dax_os_boot_launch_ready` references.
    /// This is true only when CourtBootClearance confirms all conditions.
    pub nsq_court_launch_ready: bool,
    /// No partial boot. If any condition fails, the court does not start.
    pub fail_closed_policy: &'static str,
    /// A signed handoff from the operator scaffold is required.
    pub signed_handoff_required: bool,
    /// Android ARM64 native binding must be confirmed before boot.
    pub native_binding_required: bool,
    /// The whole core must be hot and live before the court accepts requests.
    pub whole_core_hot_live_required: bool,
    /// The inner system language law is active at boot.
    pub inner_system_language_law_active: bool,
}

impl NativePlatformLaunchAdmission {
    pub fn not_ready() -> Self {
        Self {
            schema: "nsq.runtime.launch_admission.v1",
            operator_scaffold_launch_ready: false,
            nsq_court_launch_ready: false,
            fail_closed_policy: "court does not start unless all conditions are met; no degraded mode; no restricted model fallback",
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
            schema: "nsq.runtime.launch_admission.v1",
            operator_scaffold_launch_ready: operator_scaffold_ready,
            nsq_court_launch_ready: clearance.nsq_court_launch_ready,
            fail_closed_policy: "court does not start unless all conditions are met; no degraded mode; no restricted model fallback",
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
    pub schema: &'static str,
    pub watermark: &'static str,
    pub canonical_nsq: NativePlatformCanonicalNsq,
    /// Termux is the carrier — the NSQ court is the authority.
    pub termux_role: &'static str,
    pub termux_runtime_authority: bool,
    pub families: Vec<NativePlatformFamilyDescriptor>,
    pub graphics_operator_lanes: Vec<&'static str>,
    pub native_spine: Vec<&'static str>,
    pub launch_admission: NativePlatformLaunchAdmission,
    /// The language law governing the inner system.
    /// Stored here so boot verification can confirm it is active.
    pub inner_system_language_law: CourtLanguageLaw,
}

impl NativePlatformRuntimeBackbone {
    pub fn new() -> Self {
        Self {
            schema: "nsq.runtime.native_platform_runtime_backbone.v1",
            watermark: "BRAXON_NSQ_COURT_NATIVE_PLATFORM_RUNTIME_BACKBONE_V1",
            canonical_nsq: NativePlatformCanonicalNsq::default(),
            termux_role: "carrier — Android/Termux is the host carrier; NSQ court is the sovereign authority; Termux does not own the court",
            termux_runtime_authority: false,
            families: build_platform_families(),
            graphics_operator_lanes: vec![
                "vulkan_surface",
                "opengles_surface",
                "mali_gpu_render",
                "compositor_overlay",
            ],
            native_spine: vec![
                "nsq-core",
                "nsq-court",
                "nsq-runtime",
                "braxon-core",
                "braxon-court",
                "nsq-council",
                "nsq-agent",
            ],
            launch_admission: NativePlatformLaunchAdmission::not_ready(),
            inner_system_language_law: CourtLanguageLaw::active(),
        }
    }

    /// Evaluate launch admission from a court seating.
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
        self.launch_admission = NativePlatformLaunchAdmission::from_boot_clearance(
            operator_scaffold_ready,
            &clearance,
        );
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
            id: "termux",
            surface: "termux_linux_arm64",
            boundary_authority: "carrier — Termux provides the execution environment; NSQ court is the authority above it",
            not_plugin: true,
            fail_closed_until_proven: true,
            boundary_forms: vec!["pty", "socket", "pipe", "shared_memory"],
            command_process_semantics: "linux_arm64_process_semantics",
            filesystem_path_semantics: "termux_prefix_path_semantics",
            permission_security_boundary: "android_linux_permission_boundary",
            abi_or_syscall_boundary: "arm64_syscall_abi",
            shell_terminal_behavior: "bash_zsh_termux_shell",
            packaging_install_expectations: "termux_pkg_dpkg_semantics",
            boot_handoff_restrictions: "no host ownership; NSQ court admits only signed handoff from operator scaffold",
            nsq_translation_boundary: "all traffic from Termux surface is translated to intent gradient at ingress; intent gradient is translated to human text at egress only",
            court_compositor_routing: "termux_compositor_surface → court ingress → intent gradient routing",
            court_route: vec!["policer", "lexer", "parser", "router", "inspector"],
        },
        NativePlatformFamilyDescriptor {
            id: "android_jni",
            surface: "android_jni_portal",
            boundary_authority: "JNI is a carrier bridge — the NSQ court does not grant Android administrative authority",
            not_plugin: true,
            fail_closed_until_proven: true,
            boundary_forms: vec!["jni_call", "jni_return", "android_intent_bridge"],
            command_process_semantics: "android_process_semantics",
            filesystem_path_semantics: "android_storage_semantics",
            permission_security_boundary: "android_permission_manifest_boundary",
            abi_or_syscall_boundary: "android_jni_abi",
            shell_terminal_behavior: "no_shell_android_surface",
            packaging_install_expectations: "apk_semantics_carrier_only",
            boot_handoff_restrictions: "administrative shell cannot become NSQ court authority; signed handoff required",
            nsq_translation_boundary: "JNI surface traffic translated to intent gradient at ingress boundary; no JNI types inside the court",
            court_compositor_routing: "android_jni_portal → court ingress → intent gradient routing",
            court_route: vec!["policer", "compositor", "router", "scheduler", "inspector"],
        },
        NativePlatformFamilyDescriptor {
            id: "python3",
            surface: "python3_boundary_surface",
            boundary_authority: "boundary tool — Python is a surface scripting layer; it does not own the court; it translates at the boundary",
            not_plugin: true,
            fail_closed_until_proven: true,
            boundary_forms: vec!["subprocess", "socket", "pipe", "ffi"],
            command_process_semantics: "python3_process_semantics",
            filesystem_path_semantics: "posix_path_semantics",
            permission_security_boundary: "posix_user_permission_boundary",
            abi_or_syscall_boundary: "cpython_abi",
            shell_terminal_behavior: "python3_repl_or_subprocess",
            packaging_install_expectations: "pip_termux_semantics",
            boot_handoff_restrictions: "Python3 is a carrier surface tool; it cannot claim NSQ court authority",
            nsq_translation_boundary: "Python surface traffic translated to intent gradient at ingress; Python receives human text at egress only",
            court_compositor_routing: "python3_surface → court ingress → intent gradient routing",
            court_route: vec!["policer", "lexer", "parser", "router", "inspector"],
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
        assert!(!json.contains("Dax admits"));
    }

    #[test]
    fn backbone_watermark_is_braxon_nsq_court() {
        let backbone = NativePlatformRuntimeBackbone::new();
        assert!(backbone.watermark.contains("BRAXON_NSQ_COURT"));
        assert!(!backbone.watermark.contains("DAX"));
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
        assert!(backbone.termux_role.contains("sovereign authority"));
    }

    #[test]
    fn inner_system_language_law_is_active_at_boot() {
        let backbone = NativePlatformRuntimeBackbone::new();
        assert!(backbone.inner_system_language_law.no_internal_language_flag);
        assert!(backbone
            .inner_system_language_law
            .inner_system_language
            .contains("nsq_intent_gradient"));
        assert!(backbone
            .launch_admission
            .inner_system_language_law_active);
    }

    #[test]
    fn all_platform_families_enforce_translation_boundary() {
        let backbone = NativePlatformRuntimeBackbone::new();
        for family in &backbone.families {
            assert!(
                family.nsq_translation_boundary.contains("intent gradient"),
                "family {} missing intent gradient translation boundary",
                family.id
            );
        }
    }
}
