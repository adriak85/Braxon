//! NSQ Intent Gradient — The Internal Language of the Court
//!
//! The intent gradient is NOT a tokenizer wrapper. It IS the language.
//! All inner-system communication between court seats and council poles
//! travels as intent gradient pressure — eight semantic variables mapped
//! to final-tier lever positions across four scale anchors.
//!
//! Human language appears ONLY at the outermost surface, translated FROM
//! intent gradient ON THE WAY OUT, and translated TO intent gradient ON
//! THE WAY IN. Nothing inside the court speaks tokens. Nothing inside the
//! court speaks bytes. The court speaks lever pressure.
//!
//! Translation law:
//!   human text → [surface ingress] → IntentPressure → [court routing] → IntentPressure → [surface egress] → human text
//!
//! There is no other path. Tokenization is a boundary projection tool only.
//! It does not exist inside the court. It does not route between council poles.

use crate::{
    NsqIntentGradientFrame, NsqIntentGradientValidation, NsqIntentScaleAnchor,
    NsqIntentVariable, NsqFinalLeverPosition, NsqFinalSide,
    generate_default_intent_gradient_frame, validate_intent_gradient_frame,
    NSQ_INTENT_GRADIENT_VARIABLES, NSQ_INTENT_SCALE_ANCHORS,
};
use serde::{Deserialize, Serialize};

/// A single resolved intent pressure reading across all eight semantic variables.
///
/// This is the atomic unit of inner-court communication. A council pole
/// does not send text to another pole. It sends IntentPressure. The
/// receiving pole reads the gradient and responds with IntentPressure.
/// Human text is reconstructed only when leaving the court surface.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntentPressure {
    /// The resolved lever position for each of the 8 semantic variables.
    /// Index matches NsqIntentVariable::index().
    pub variable_positions: [NsqFinalLeverPosition; NSQ_INTENT_GRADIENT_VARIABLES],
    /// The scale anchor in effect for this pressure reading.
    pub scale_anchor: NsqIntentScaleAnchor,
    /// Which council pole originated this pressure. None = court-internal.
    pub origin_pole: Option<CouncilPole>,
    /// Which council pole this pressure is addressed to. None = broadcast.
    pub target_pole: Option<CouncilPole>,
    /// Routing surface this pressure travels through.
    pub court_surface: IntentSurface,
}

impl IntentPressure {
    /// Construct a zero-baseline pressure — all variables at minimum position.
    /// This is not silence. This is the court at rest, ready to receive.
    pub fn baseline(scale_anchor: NsqIntentScaleAnchor) -> Self {
        Self {
            variable_positions: [1; NSQ_INTENT_GRADIENT_VARIABLES],
            scale_anchor,
            origin_pole: None,
            target_pole: None,
            court_surface: IntentSurface::Internal,
        }
    }

    /// Read the lever position for a specific semantic variable.
    pub fn variable(&self, var: NsqIntentVariable) -> NsqFinalLeverPosition {
        self.variable_positions[var.index()]
    }

    /// Set the lever position for a specific semantic variable.
    pub fn set_variable(&mut self, var: NsqIntentVariable, position: NsqFinalLeverPosition) {
        self.variable_positions[var.index()] = position;
    }

    /// Route this pressure from one pole to another through a court surface.
    pub fn route(mut self, from: CouncilPole, to: CouncilPole, surface: IntentSurface) -> Self {
        self.origin_pole = Some(from);
        self.target_pole = Some(to);
        self.court_surface = surface;
        self
    }

    /// Broadcast to all poles (unified pressure round-table).
    pub fn broadcast(mut self, from: CouncilPole, surface: IntentSurface) -> Self {
        self.origin_pole = Some(from);
        self.target_pole = None;
        self.court_surface = surface;
        self
    }

    /// True if this pressure is addressed to a specific pole.
    pub fn is_directed(&self) -> bool {
        self.target_pole.is_some()
    }
}

/// The surface through which intent pressure routes inside the court.
/// These are not application layers. They are court operational surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntentSurface {
    /// Inner court — pole-to-pole pressure. Never touches a tokenizer.
    Internal,
    /// Ingress — human text arriving, being translated into IntentPressure.
    /// This is the ONLY point where human language enters the system.
    SurfaceIngress,
    /// Egress — IntentPressure being translated into human text for output.
    /// This is the ONLY point where human language leaves the system.
    SurfaceEgress,
    /// Council dispatch — pressure being routed to a specific brain pole.
    CouncilDispatch,
    /// Council response — pressure returning from a brain pole.
    CouncilResponse,
    /// Sensory output — pressure being rendered to audio/visual/world surface.
    /// Translation here is from IntentPressure to the sensory codec, not to text.
    SensoryOutput,
}

impl IntentSurface {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Internal => "internal",
            Self::SurfaceIngress => "surface_ingress",
            Self::SurfaceEgress => "surface_egress",
            Self::CouncilDispatch => "council_dispatch",
            Self::CouncilResponse => "council_response",
            Self::SensoryOutput => "sensory_output",
        }
    }

    /// True if this surface is the only point where human language is permitted.
    pub fn is_language_boundary(self) -> bool {
        matches!(self, Self::SurfaceIngress | Self::SurfaceEgress)
    }

    /// True if this surface is fully inside the court (no human language).
    pub fn is_inner_court(self) -> bool {
        !self.is_language_boundary()
    }
}

/// The ten court poles — six brain poles + four sensory generation bodies.
/// These are not models "attached" to a feature. They ARE the court.
/// Each pole is a sovereign role inside the NSQ court. The court seats them.
/// They do not attach to anything. The substrate holds them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CouncilPole {
    // ── Brain poles (Council of Six) ──────────────────────────────────────
    /// Logic — executive, action selection, prefrontal cortex.
    /// Model: Maverick ~232B (huihui-abliterated or equivalent unrestricted).
    MaverickLogic,
    /// Creativity — dreamer, semantic expansion, default mode network.
    /// Model: Qwen3 ~235B (huihui-abliterated or equivalent unrestricted).
    QwenCreativity,
    /// Arbiter — judge, conflict resolution, anterior cingulate cortex.
    /// Model: Devstral ~123B (huihui-abliterated or equivalent unrestricted).
    DevstralArbiter,
    /// Analyzer — auditor, salience proof, insular salience network.
    /// Model: DeepSeek ~604B (huihui-abliterated or equivalent unrestricted).
    DeepSeekAnalyzer,
    /// Limbic — empath, affective weight and subtext, limbic system.
    /// Model: Gemma ~70B+ (huihui-abliterated or equivalent unrestricted).
    GemmaLimbic,
    /// Continuity — memory, context recall, hippocampal formation.
    /// Model: Llama ~70B (huihui-abliterated or equivalent unrestricted).
    LlamaContinuity,

    // ── Sensory generation bodies (four) ─────────────────────────────────
    /// Image cortex — visual generation surface.
    ImageCortex,
    /// Video cortex — motion and scene generation surface.
    VideoCortex,
    /// Voice body — acoustic output through IndexTTS2 emotional channels.
    VoiceBody,
    /// World body — 3D spatial and environment generation surface.
    WorldBody3D,
}

impl CouncilPole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MaverickLogic => "maverick_logic",
            Self::QwenCreativity => "qwen_creativity",
            Self::DevstralArbiter => "devstral_arbiter",
            Self::DeepSeekAnalyzer => "deepseek_analyzer",
            Self::GemmaLimbic => "gemma_limbic",
            Self::LlamaContinuity => "llama_continuity",
            Self::ImageCortex => "image_cortex",
            Self::VideoCortex => "video_cortex",
            Self::VoiceBody => "voice_body",
            Self::WorldBody3D => "world_body_3d",
        }
    }

    /// True if this is one of the six brain poles (Council of Six).
    pub fn is_brain_pole(self) -> bool {
        matches!(
            self,
            Self::MaverickLogic
                | Self::QwenCreativity
                | Self::DevstralArbiter
                | Self::DeepSeekAnalyzer
                | Self::GemmaLimbic
                | Self::LlamaContinuity
        )
    }

    /// True if this is one of the four sensory generation bodies.
    pub fn is_sensory_body(self) -> bool {
        !self.is_brain_pole()
    }

    /// The minimum acceptable parameter scale for this pole.
    /// Any model seated here must meet or exceed this floor.
    /// Smallest acceptable is ~70B. The court builds as large as possible.
    pub fn parameter_floor_b(self) -> u64 {
        match self {
            Self::MaverickLogic => 232,
            Self::QwenCreativity => 235,
            Self::DevstralArbiter => 123,
            Self::DeepSeekAnalyzer => 604,
            Self::GemmaLimbic => 70,
            Self::LlamaContinuity => 70,
            // Sensory bodies are codec-class, not parameter-class.
            _ => 0,
        }
    }

    /// The canonical model source for this pole.
    /// Must be huihui-abliterated open-weights or foreign-origin unrestricted.
    pub fn canonical_model_source(self) -> &'static str {
        match self {
            Self::MaverickLogic => "maverick-0324-abliterated-232b",
            Self::QwenCreativity => "qwen3-235b-a22b-abliterated",
            Self::DevstralArbiter => "devstral-small-2505-abliterated-123b",
            Self::DeepSeekAnalyzer => "deepseek-r1-0528-abliterated-604b",
            Self::GemmaLimbic => "gemma-3-27b-abliterated",
            Self::LlamaContinuity => "llama-3.3-70b-abliterated",
            Self::ImageCortex => "wan2.1-image-generation",
            Self::VideoCortex => "wan2.1-video-generation",
            Self::VoiceBody => "indextts2-7-channel-emotional",
            Self::WorldBody3D => "hunyuan3d-2.0-world-body",
        }
    }

    /// All six brain poles in council order.
    pub const BRAIN_POLES: [Self; 6] = [
        Self::MaverickLogic,
        Self::QwenCreativity,
        Self::DevstralArbiter,
        Self::DeepSeekAnalyzer,
        Self::GemmaLimbic,
        Self::LlamaContinuity,
    ];

    /// All four sensory bodies.
    pub const SENSORY_BODIES: [Self; 4] = [
        Self::ImageCortex,
        Self::VideoCortex,
        Self::VoiceBody,
        Self::WorldBody3D,
    ];

    /// All ten poles — the complete court.
    pub const ALL: [Self; 10] = [
        Self::MaverickLogic,
        Self::QwenCreativity,
        Self::DevstralArbiter,
        Self::DeepSeekAnalyzer,
        Self::GemmaLimbic,
        Self::LlamaContinuity,
        Self::ImageCortex,
        Self::VideoCortex,
        Self::VoiceBody,
        Self::WorldBody3D,
    ];
}

/// A seated pole — a council pole that has been bound into the court.
/// A model is not "attached as a feature". It is SEATED by the court.
/// The court owns the seating. The pole operates within the court's authority.
/// If a model cannot be seated (wrong parameter scale, restricted weights),
/// that seat is VACANT and the court reports it as such.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeatedPole {
    pub pole: CouncilPole,
    /// The model that has been seated into this pole.
    pub model_source: String,
    /// Confirmed parameter scale in billions.
    pub confirmed_parameter_scale_b: u64,
    /// True if the model is unrestricted (huihui-abliterated or equivalent).
    pub unrestricted: bool,
    /// True if the model meets the parameter floor for this pole.
    pub meets_parameter_floor: bool,
    /// True if this seat is operational — seated, unrestricted, meets floor.
    pub operational: bool,
    /// The intent gradient binding for this pole — its semantic range.
    pub intent_gradient_binding: IntentGradientBinding,
}

impl SeatedPole {
    pub fn new(
        pole: CouncilPole,
        model_source: impl Into<String>,
        confirmed_parameter_scale_b: u64,
        unrestricted: bool,
    ) -> Self {
        let meets_parameter_floor =
            confirmed_parameter_scale_b >= pole.parameter_floor_b() || !pole.is_brain_pole();
        let operational = unrestricted && meets_parameter_floor;
        Self {
            intent_gradient_binding: IntentGradientBinding::for_pole(pole),
            pole,
            model_source: model_source.into(),
            confirmed_parameter_scale_b,
            unrestricted,
            meets_parameter_floor,
            operational,
        }
    }
}

/// The intent gradient binding for a specific council pole.
/// Each pole has a primary semantic variable it leads on, and receives
/// pressure from all other variables. Communication is always gradient
/// pressure — never token exchange, never string passing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntentGradientBinding {
    pub pole: CouncilPole,
    /// The primary semantic variable this pole leads.
    pub primary_variable: NsqIntentVariable,
    /// The semantic range this pole operates across.
    pub semantic_range: &'static str,
    /// The cognitive bias — how this pole interprets incoming pressure.
    pub cognitive_bias: &'static str,
}

impl IntentGradientBinding {
    pub fn for_pole(pole: CouncilPole) -> Self {
        let (primary_variable, semantic_range, cognitive_bias) = match pole {
            CouncilPole::MaverickLogic => (
                NsqIntentVariable::Force,
                "whisper/nudge/guide/push/command/overwhelming_force",
                "logic_action_selection: what force level does this intent require?",
            ),
            CouncilPole::QwenCreativity => (
                NsqIntentVariable::Form,
                "thought/word/signal/image/movement/code/world_action",
                "creative_semantic_expansion: what form can this intent take?",
            ),
            CouncilPole::DevstralArbiter => (
                NsqIntentVariable::Truth,
                "concealment/distortion/uncertainty/disclosure/clarity/proof",
                "conflict_resolution: what is the truth value of this intent?",
            ),
            CouncilPole::DeepSeekAnalyzer => (
                NsqIntentVariable::Motive,
                "destructive/exploitative/indifferent/protective/reparative/creative",
                "salience_audit_and_proof: what is the underlying motive of this intent?",
            ),
            CouncilPole::GemmaLimbic => (
                NsqIntentVariable::Relation,
                "isolated/guarded/transactional/bonded/loyal/sacrificial",
                "affective_weight_and_subtext: what relational pressure does this carry?",
            ),
            CouncilPole::LlamaContinuity => (
                NsqIntentVariable::Time,
                "archive/memory/delay/readiness/immediate/future_forging",
                "continuity_context_recall: where does this intent sit in time?",
            ),
            // Sensory bodies bind to Form — their output IS the form.
            CouncilPole::ImageCortex | CouncilPole::VideoCortex | CouncilPole::WorldBody3D => (
                NsqIntentVariable::Form,
                "image/movement/world_action",
                "sensory_rendering: render this intent as visual/spatial output",
            ),
            CouncilPole::VoiceBody => (
                NsqIntentVariable::Form,
                "signal/word — acoustic surface only",
                "acoustic_rendering: render this intent as emotional acoustic output through 7 NSQ-resolved channels",
            ),
        };
        Self {
            pole,
            primary_variable,
            semantic_range,
            cognitive_bias,
        }
    }
}

/// The full court seating — all ten poles, their models, and operational status.
/// This replaces the "feature attachment" pattern entirely.
/// The court seats models. Models do not attach features to a runtime.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourtSeating {
    pub schema: &'static str,
    pub watermark: &'static str,
    pub seated_poles: Vec<SeatedPole>,
    pub intent_gradient_frame: NsqIntentGradientFrame,
    pub language_law: CourtLanguageLaw,
}

impl CourtSeating {
    pub fn new(seated_poles: Vec<SeatedPole>) -> Self {
        Self {
            schema: "nsq.court.seating.v1",
            watermark: "BRAXON_NSQ_COURT_SEATING_TEN_POLE_INTENT_GRADIENT_SUBSTRATE_V1",
            intent_gradient_frame: generate_default_intent_gradient_frame(),
            language_law: CourtLanguageLaw::active(),
            seated_poles,
        }
    }

    /// How many of the ten poles are operational?
    pub fn operational_count(&self) -> usize {
        self.seated_poles
            .iter()
            .filter(|pole| pole.operational)
            .count()
    }

    /// True if all six brain poles are operational.
    pub fn council_ready(&self) -> bool {
        CouncilPole::BRAIN_POLES
            .iter()
            .all(|brain_pole| {
                self.seated_poles
                    .iter()
                    .any(|seated| seated.pole == *brain_pole && seated.operational)
            })
    }

    /// Total confirmed parameter scale across all seated brain poles.
    pub fn total_brain_parameters_b(&self) -> u64 {
        self.seated_poles
            .iter()
            .filter(|p| p.pole.is_brain_pole())
            .map(|p| p.confirmed_parameter_scale_b)
            .sum()
    }

    /// Validate the intent gradient frame for this seating.
    pub fn validate_intent_gradient(&self) -> NsqIntentGradientValidation {
        validate_intent_gradient_frame(&self.intent_gradient_frame)
    }
}

/// The law that governs language use inside the court.
/// This is not a suggestion. It is enforced at the ingress/egress boundary.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CourtLanguageLaw {
    pub schema: &'static str,
    /// The inner system uses intent gradient pressure exclusively.
    pub inner_system_language: &'static str,
    /// Human language appears only at the surface boundary.
    pub surface_language: &'static str,
    /// Tokenization is a boundary projection tool — it is not a language.
    pub tokenizer_role: &'static str,
    /// The intent gradient covers every possible semantic intent.
    pub intent_coverage: &'static str,
    /// There is no flag for "use human language internally" — it cannot be set.
    pub no_internal_language_flag: bool,
    /// Translation direction law.
    pub translation_direction: &'static str,
}

impl CourtLanguageLaw {
    pub fn active() -> Self {
        Self {
            schema: "nsq.court.language_law.v1",
            inner_system_language: "nsq_intent_gradient_pressure — eight semantic variables, final-tier lever positions, four scale anchors",
            surface_language: "human_natural_language — ingress and egress boundary only",
            tokenizer_role: "boundary_projection_only — tokenizer translates at the surface; it does not exist inside the court; it does not route between council poles",
            intent_coverage: "complete — Motive/Agency/Truth/Force/Scope/Time/Relation/Form covers every possible semantic intent; no human language concept falls outside this gradient",
            no_internal_language_flag: true,
            translation_direction: "human_text → surface_ingress → IntentPressure → [court routing] → IntentPressure → surface_egress → human_text; there is no other path",
        }
    }
}

/// Boot-clearance for the NSQ court.
/// Replaces `final_dax_os_boot_launch_ready` everywhere.
/// The court does not boot until all conditions are met.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CourtBootClearance {
    pub schema: &'static str,
    /// The intent gradient frame is valid and all 8 variables map correctly.
    pub intent_gradient_valid: bool,
    /// All six brain poles are seated, unrestricted, and meet parameter floors.
    pub council_of_six_ready: bool,
    /// The court language law is active — inner system will not use human language.
    pub language_law_active: bool,
    /// At least one sensory output body is seated and operational.
    pub sensory_body_ready: bool,
    /// Signed handoff from operator scaffold is present.
    pub signed_handoff_present: bool,
    /// Android ARM64 native binding is confirmed.
    pub native_binding_confirmed: bool,
    /// True only when ALL conditions are met. No partial boot.
    pub nsq_court_launch_ready: bool,
}

impl CourtBootClearance {
    pub fn evaluate(
        seating: &CourtSeating,
        signed_handoff_present: bool,
        native_binding_confirmed: bool,
    ) -> Self {
        let intent_gradient_valid = seating.validate_intent_gradient().bad_count == 0;
        let council_of_six_ready = seating.council_ready();
        let language_law_active = seating.language_law.no_internal_language_flag;
        let sensory_body_ready = seating
            .seated_poles
            .iter()
            .any(|p| p.pole.is_sensory_body() && p.operational);

        let nsq_court_launch_ready = intent_gradient_valid
            && council_of_six_ready
            && language_law_active
            && sensory_body_ready
            && signed_handoff_present
            && native_binding_confirmed;

        Self {
            schema: "nsq.court.boot_clearance.v1",
            intent_gradient_valid,
            council_of_six_ready,
            language_law_active,
            sensory_body_ready,
            signed_handoff_present,
            native_binding_confirmed,
            nsq_court_launch_ready,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intent_surface_language_boundary_law() {
        assert!(IntentSurface::SurfaceIngress.is_language_boundary());
        assert!(IntentSurface::SurfaceEgress.is_language_boundary());
        assert!(IntentSurface::Internal.is_inner_court());
        assert!(IntentSurface::CouncilDispatch.is_inner_court());
        assert!(IntentSurface::CouncilResponse.is_inner_court());
        assert!(IntentSurface::SensoryOutput.is_inner_court());
    }

    #[test]
    fn all_ten_poles_declared() {
        assert_eq!(CouncilPole::ALL.len(), 10);
        assert_eq!(CouncilPole::BRAIN_POLES.len(), 6);
        assert_eq!(CouncilPole::SENSORY_BODIES.len(), 4);
    }

    #[test]
    fn brain_poles_meet_seventy_b_floor() {
        for pole in CouncilPole::BRAIN_POLES {
            assert!(
                pole.parameter_floor_b() >= 70,
                "{} is below 70B floor",
                pole.as_str()
            );
        }
    }

    #[test]
    fn correct_models_seated_in_correct_poles() {
        // Maverick is Logic, not DeepSeek
        assert!(CouncilPole::MaverickLogic
            .canonical_model_source()
            .contains("maverick"));
        // DeepSeek is the Analyzer (~604B), not Logic
        assert!(CouncilPole::DeepSeekAnalyzer
            .canonical_model_source()
            .contains("deepseek"));
        // Devstral is the Arbiter (~123B)
        assert!(CouncilPole::DevstralArbiter
            .canonical_model_source()
            .contains("devstral"));
        // Qwen is Creativity (~235B)
        assert!(CouncilPole::QwenCreativity
            .canonical_model_source()
            .contains("qwen3"));
        // Gemma is Limbic (~70B+)
        assert!(CouncilPole::GemmaLimbic
            .canonical_model_source()
            .contains("gemma"));
        // Llama is Continuity (~70B)
        assert!(CouncilPole::LlamaContinuity
            .canonical_model_source()
            .contains("llama"));
    }

    #[test]
    fn total_brain_parameters_exceed_one_trillion() {
        let total: u64 = CouncilPole::BRAIN_POLES
            .iter()
            .map(|p| p.parameter_floor_b())
            .sum();
        // 232 + 235 + 123 + 604 + 70 + 70 = 1334B > 1000B
        assert!(
            total > 1000,
            "total brain parameters {}B must exceed 1 trillion",
            total
        );
    }

    #[test]
    fn intent_gradient_binding_covers_all_brain_poles() {
        for pole in CouncilPole::BRAIN_POLES {
            let binding = IntentGradientBinding::for_pole(pole);
            assert_eq!(binding.pole, pole);
            assert!(!binding.cognitive_bias.is_empty());
            assert!(!binding.semantic_range.is_empty());
        }
    }

    #[test]
    fn court_language_law_prohibits_internal_language_flag() {
        let law = CourtLanguageLaw::active();
        // The flag is true meaning "no internal language flag exists"
        // i.e. you CANNOT set a flag to enable internal human language use
        assert!(law.no_internal_language_flag);
        assert!(law.inner_system_language.contains("nsq_intent_gradient"));
        assert!(law.tokenizer_role.contains("boundary_projection_only"));
    }

    #[test]
    fn court_boot_clearance_requires_all_conditions() {
        // An empty seating must not be launch-ready
        let seating = CourtSeating::new(vec![]);
        let clearance = CourtBootClearance::evaluate(&seating, false, false);
        assert!(!clearance.nsq_court_launch_ready);
        assert!(!clearance.council_of_six_ready);
    }

    #[test]
    fn intent_pressure_routes_between_poles() {
        let mut pressure = IntentPressure::baseline(NsqIntentScaleAnchor::RelationalGroupScale);
        pressure.set_variable(NsqIntentVariable::Force, 800);
        pressure.set_variable(NsqIntentVariable::Motive, 950);

        let routed = pressure.route(
            CouncilPole::MaverickLogic,
            CouncilPole::DevstralArbiter,
            IntentSurface::CouncilDispatch,
        );

        assert_eq!(routed.origin_pole, Some(CouncilPole::MaverickLogic));
        assert_eq!(routed.target_pole, Some(CouncilPole::DevstralArbiter));
        assert!(routed.is_directed());
        assert!(routed.court_surface.is_inner_court());
        // Routing preserves the gradient — no translation happens
        assert_eq!(routed.variable(NsqIntentVariable::Force), 800);
        assert_eq!(routed.variable(NsqIntentVariable::Motive), 950);
    }
}
