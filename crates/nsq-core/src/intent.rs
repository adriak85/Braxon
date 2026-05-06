//! NSQ Intent Gradient — The Internal Language of the Court
//!
//! The intent gradient is NOT a tokenizer wrapper. It IS the language.
//! All inner-system communication between court seats and council poles
//! travels as intent gradient pressure — eight semantic variables mapped
//! to final-tier lever positions across four scale anchors.
//!
//! Translation law:
//!   human text → [surface_ingress] → IntentPressure → [court routing] → IntentPressure → [surface_egress] → human text
//!
//! There is no other path. Tokenization is a boundary projection tool only.
//! It does not exist inside the court. It does not route between council poles.

use crate::CANONICAL_LEVER_MAX_POSITION;
use serde::{Deserialize, Serialize};

// ── Intent gradient constants ──────────────────────────────────────────────

/// The number of semantic variables in the intent gradient.
/// Every possible intent in the universe maps into these eight dimensions.
pub const NSQ_INTENT_GRADIENT_VARIABLES: usize = 8;

/// The number of scale anchors available to the intent gradient.
pub const NSQ_INTENT_SCALE_ANCHORS: usize = 4;

/// The type for a final-tier lever position.
/// Range: 1..=CANONICAL_LEVER_MAX_POSITION.
/// With the current lever ceiling this gives 2254 zero-inclusive states per lever
/// and 25,811,642,826,256 states per 4-lever bit-unit.
pub type NsqFinalLeverPosition = u16;

// ── Semantic variables ─────────────────────────────────────────────────────

/// The eight semantic variables of the NSQ intent gradient.
///
/// These eight dimensions cover every possible semantic intent.
/// No concept in human language falls outside this gradient.
/// This is the complete inner-court communication protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NsqIntentVariable {
    /// Why — the underlying drive or purpose behind the intent.
    /// destructive / exploitative / indifferent / protective / reparative / creative
    Motive = 0,
    /// Who acts — the capacity and consent relationship of the acting entity.
    /// coercive / manipulative / passive / cooperative / consentful / empowering
    Agency = 1,
    /// What is claimed — the epistemic status of the intent's content.
    /// concealment / distortion / uncertainty / disclosure / clarity / proof
    Truth = 2,
    /// How hard — the applied force level of the intent.
    /// whisper / nudge / guide / push / command / overwhelming_force
    Force = 3,
    /// How wide — the scope of effect the intent addresses.
    /// self / object / pair / group / system / world / universal_field
    Scope = 4,
    /// When — the temporal orientation of the intent.
    /// archive / memory / delay / readiness / immediate / future_forging
    Time = 5,
    /// To whom — the relational bond implied by the intent.
    /// isolated / guarded / transactional / bonded / loyal / sacrificial
    Relation = 6,
    /// In what shape — the output form the intent takes.
    /// thought / word / signal / image / movement / code / world_action
    Form = 7,
}

impl NsqIntentVariable {
    /// The index of this variable in the gradient frame (0-7).
    pub fn index(self) -> usize {
        self as usize
    }

    /// The human-readable name of this variable.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Motive => "motive",
            Self::Agency => "agency",
            Self::Truth => "truth",
            Self::Force => "force",
            Self::Scope => "scope",
            Self::Time => "time",
            Self::Relation => "relation",
            Self::Form => "form",
        }
    }

    /// All eight variables in gradient order.
    pub const ALL: [Self; NSQ_INTENT_GRADIENT_VARIABLES] = [
        Self::Motive,
        Self::Agency,
        Self::Truth,
        Self::Force,
        Self::Scope,
        Self::Time,
        Self::Relation,
        Self::Form,
    ];
}

// ── Scale anchors ──────────────────────────────────────────────────────────

/// The four scale anchors of the intent gradient.
///
/// A single intent pressure reading is always anchored to one of these scales.
/// The scale determines how the lever positions are interpreted relationally.
/// Inner-court routing uses scale anchors to determine pressure equivalence
/// across poles operating at different relational distances.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NsqIntentScaleAnchor {
    /// Self / object scale — individual entity or single object.
    SelfObjectScale,
    /// Relational / group scale — dyads, groups, communities.
    RelationalGroupScale,
    /// System / world scale — institutions, civilizations, biospheres.
    SystemWorldScale,
    /// Universal / field scale — cosmos, substrate, unbounded field.
    UniversalFieldScale,
}

impl NsqIntentScaleAnchor {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SelfObjectScale => "self_object_scale",
            Self::RelationalGroupScale => "relational_group_scale",
            Self::SystemWorldScale => "system_world_scale",
            Self::UniversalFieldScale => "universal_field_scale",
        }
    }
}

// ── Intent gradient side (polarity) ───────────────────────────────────────

/// The polarity side of an intent gradient reading.
/// Positive = toward the high end of the variable's semantic range.
/// Negative = toward the low end.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NsqFinalSide {
    Positive,
    Negative,
}

// ── Intent gradient frame ──────────────────────────────────────────────────

/// A complete intent gradient frame — all eight variable positions plus
/// the four scale anchors this frame was calibrated against.
///
/// This is the canonical representation of a semantic state in the court.
/// It is not a token embedding. It is not a hidden state vector.
/// It is eight lever positions and four scale calibration points.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NsqIntentGradientFrame {
    /// The lever position for each of the eight semantic variables.
    pub variable_positions: [NsqFinalLeverPosition; NSQ_INTENT_GRADIENT_VARIABLES],
    /// The four scale anchors this frame maps across.
    pub scale_anchors: [NsqIntentScaleAnchor; NSQ_INTENT_SCALE_ANCHORS],
}

/// Generate the default intent gradient frame.
/// All variables are set to the midpoint of the final-tier lever range.
/// This is the court at semantic zero — fully open, uncommitted, ready.
pub fn generate_default_intent_gradient_frame() -> NsqIntentGradientFrame {
    let midpoint = CANONICAL_LEVER_MAX_POSITION / 2;
    NsqIntentGradientFrame {
        variable_positions: [midpoint; NSQ_INTENT_GRADIENT_VARIABLES],
        scale_anchors: [
            NsqIntentScaleAnchor::SelfObjectScale,
            NsqIntentScaleAnchor::RelationalGroupScale,
            NsqIntentScaleAnchor::SystemWorldScale,
            NsqIntentScaleAnchor::UniversalFieldScale,
        ],
    }
}

// ── Intent gradient validation ─────────────────────────────────────────────

/// The result of validating an intent gradient frame.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NsqIntentGradientValidation {
    /// Number of variable positions that fall outside the valid final-tier range.
    pub bad_count: usize,
    /// True if all eight variables are present in the frame.
    pub all_variables_present: bool,
    /// True if all positions are within 1..=CANONICAL_LEVER_MAX_POSITION.
    pub positions_inside_final_tier: bool,
}

/// Validate an intent gradient frame against the final-tier lever law.
pub fn validate_intent_gradient_frame(frame: &NsqIntentGradientFrame) -> NsqIntentGradientValidation {
    let mut bad_count = 0;
    for &pos in &frame.variable_positions {
        if pos < 1 || pos > CANONICAL_LEVER_MAX_POSITION {
            bad_count += 1;
        }
    }
    let all_variables_present = frame.variable_positions.len() == NSQ_INTENT_GRADIENT_VARIABLES;
    NsqIntentGradientValidation {
        bad_count,
        all_variables_present,
        positions_inside_final_tier: bad_count == 0,
    }
}

// ── Intent pressure ────────────────────────────────────────────────────────

/// A single resolved intent pressure reading across all eight semantic variables.
///
/// This is the atomic unit of inner-court communication.
/// A council pole does not send text to another pole. It sends IntentPressure.
/// The receiving pole reads the gradient and responds with IntentPressure.
/// Human text is reconstructed only when leaving the court surface.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntentPressure {
    /// The resolved lever position for each of the 8 semantic variables.
    pub variable_positions: [NsqFinalLeverPosition; NSQ_INTENT_GRADIENT_VARIABLES],
    /// The scale anchor in effect for this pressure reading.
    pub scale_anchor: NsqIntentScaleAnchor,
    /// Which council pole originated this pressure. None = court-internal.
    pub origin_pole: Option<CouncilPole>,
    /// Which council pole this pressure is addressed to. None = broadcast.
    pub target_pole: Option<CouncilPole>,
    /// The routing surface this pressure travels through.
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

    /// Route this pressure from one pole to another.
    pub fn route(mut self, from: CouncilPole, to: CouncilPole, surface: IntentSurface) -> Self {
        self.origin_pole = Some(from);
        self.target_pole = Some(to);
        self.court_surface = surface;
        self
    }

    /// Broadcast this pressure from one pole to all poles.
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

// ── Intent surface ─────────────────────────────────────────────────────────

/// The surface through which intent pressure routes inside the court.
///
/// SurfaceIngress and SurfaceEgress are the ONLY two surfaces where
/// human language is permitted. Everything else is inner court — gradient
/// pressure only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntentSurface {
    /// Inner court — pole-to-pole pressure. No tokenizer. No text.
    Internal,
    /// The ONLY point where human language enters the system.
    SurfaceIngress,
    /// The ONLY point where human language leaves the system.
    SurfaceEgress,
    /// Pressure being routed to a specific brain pole.
    CouncilDispatch,
    /// Pressure returning from a brain pole.
    CouncilResponse,
    /// Pressure being rendered to audio/visual/world codec.
    /// Translation here is IntentPressure → sensory codec, not to text.
    SensoryOutput,
    /// A stamp is being thrown to a build position.
    /// The wake system intercepts here.
    StampDispatch,
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
            Self::StampDispatch => "stamp_dispatch",
        }
    }

    /// True if this surface is a human-language boundary.
    pub fn is_language_boundary(self) -> bool {
        matches!(self, Self::SurfaceIngress | Self::SurfaceEgress)
    }

    /// True if this surface is fully inside the court.
    pub fn is_inner_court(self) -> bool {
        !self.is_language_boundary()
    }

    /// True if this surface carries a stamp to the wake system.
    pub fn is_stamp_surface(self) -> bool {
        matches!(self, Self::StampDispatch)
    }
}

// ── Council poles ──────────────────────────────────────────────────────────

/// The ten court poles — six brain poles + four sensory generation bodies.
///
/// Models are SEATED into poles by the court.
/// Models do not attach features to a runtime.
/// Models do not register capability flags.
/// The court seats them. If they cannot be seated they are VACANT.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CouncilPole {
    // ── Brain poles (Council of Six) ─────────────────────────────────────
    /// Logic — executive, prefrontal cortex. Maverick ~232B.
    MaverickLogic,
    /// Creativity — default mode network. Qwen3 ~235B.
    QwenCreativity,
    /// Arbiter — anterior cingulate cortex. Devstral ~123B.
    DevstralArbiter,
    /// Analyzer — insular salience network. DeepSeek ~604B.
    DeepSeekAnalyzer,
    /// Limbic — limbic system, affective weight. Gemma ~70B+.
    GemmaLimbic,
    /// Continuity — hippocampal formation, context recall. Llama ~70B.
    LlamaContinuity,

    // ── Sensory generation bodies ─────────────────────────────────────────
    /// Visual generation surface.
    ImageCortex,
    /// Motion and scene generation surface.
    VideoCortex,
    /// Acoustic output — IndexTTS2 seven emotional channels.
    VoiceBody,
    /// 3D spatial and environment generation surface.
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

    pub fn is_sensory_body(self) -> bool {
        !self.is_brain_pole()
    }

    /// Minimum parameter scale in billions for this pole.
    /// The court builds as large as possible. 70B is the floor, not the target.
    pub fn parameter_floor_b(self) -> u64 {
        match self {
            Self::MaverickLogic => 232,
            Self::QwenCreativity => 235,
            Self::DevstralArbiter => 123,
            Self::DeepSeekAnalyzer => 604,
            Self::GemmaLimbic => 70,
            Self::LlamaContinuity => 70,
            _ => 0,
        }
    }

    /// Canonical model source — must be huihui-abliterated or foreign-origin unrestricted.
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

    /// The primary semantic variable this brain pole leads on.
    pub fn primary_variable(self) -> NsqIntentVariable {
        match self {
            Self::MaverickLogic => NsqIntentVariable::Force,
            Self::QwenCreativity => NsqIntentVariable::Form,
            Self::DevstralArbiter => NsqIntentVariable::Truth,
            Self::DeepSeekAnalyzer => NsqIntentVariable::Motive,
            Self::GemmaLimbic => NsqIntentVariable::Relation,
            Self::LlamaContinuity => NsqIntentVariable::Time,
            _ => NsqIntentVariable::Form,
        }
    }

    pub const BRAIN_POLES: [Self; 6] = [
        Self::MaverickLogic,
        Self::QwenCreativity,
        Self::DevstralArbiter,
        Self::DeepSeekAnalyzer,
        Self::GemmaLimbic,
        Self::LlamaContinuity,
    ];

    pub const SENSORY_BODIES: [Self; 4] = [
        Self::ImageCortex,
        Self::VideoCortex,
        Self::VoiceBody,
        Self::WorldBody3D,
    ];

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

// ── Seated pole ────────────────────────────────────────────────────────────

/// The intent gradient binding for a specific council pole.
/// This describes which semantic variable the pole leads on and its
/// cognitive operating range. Owned strings to allow serialization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntentGradientBinding {
    pub pole: CouncilPole,
    pub primary_variable: NsqIntentVariable,
    /// The semantic range this pole operates across (owned for serde).
    pub semantic_range: String,
    /// How this pole interprets incoming pressure (owned for serde).
    pub cognitive_bias: String,
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
            CouncilPole::ImageCortex
            | CouncilPole::VideoCortex
            | CouncilPole::WorldBody3D => (
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
            semantic_range: semantic_range.to_string(),
            cognitive_bias: cognitive_bias.to_string(),
        }
    }
}

/// A council pole that has been bound into the court.
///
/// A model is not "attached as a feature". It is SEATED by the court.
/// If it cannot be seated it is VACANT. The court reports it honestly.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeatedPole {
    pub pole: CouncilPole,
    pub model_source: String,
    pub confirmed_parameter_scale_b: u64,
    pub unrestricted: bool,
    pub meets_parameter_floor: bool,
    pub operational: bool,
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

// ── Court seating ──────────────────────────────────────────────────────────

/// The language law governing the inner court.
/// Not a flag. Not a feature. Always active.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CourtLanguageLaw {
    pub schema: String,
    pub inner_system_language: String,
    pub surface_language: String,
    pub tokenizer_role: String,
    pub intent_coverage: String,
    /// Always true — there is no flag to enable internal human language.
    pub no_internal_language_flag: bool,
    pub translation_direction: String,
}

impl CourtLanguageLaw {
    pub fn active() -> Self {
        Self {
            schema: "nsq.court.language_law.v1".to_string(),
            inner_system_language: "nsq_intent_gradient_pressure — eight semantic variables, final-tier lever positions, four scale anchors".to_string(),
            surface_language: "human_natural_language — ingress and egress boundary only".to_string(),
            tokenizer_role: "boundary_projection_only — tokenizer translates at the surface; it does not exist inside the court; it does not route between council poles".to_string(),
            intent_coverage: "complete — Motive/Agency/Truth/Force/Scope/Time/Relation/Form covers every possible semantic intent; no human language concept falls outside this gradient".to_string(),
            no_internal_language_flag: true,
            translation_direction: "human_text → surface_ingress → IntentPressure → [court routing] → IntentPressure → surface_egress → human_text; there is no other path".to_string(),
        }
    }
}

/// The full court seating — all ten poles and their operational state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourtSeating {
    pub schema: String,
    pub watermark: String,
    pub seated_poles: Vec<SeatedPole>,
    pub intent_gradient_frame: NsqIntentGradientFrame,
    pub language_law: CourtLanguageLaw,
}

impl CourtSeating {
    pub fn new(seated_poles: Vec<SeatedPole>) -> Self {
        Self {
            schema: "nsq.court.seating.v1".to_string(),
            watermark: "BRAXON_NSQ_COURT_SEATING_TEN_POLE_INTENT_GRADIENT_SUBSTRATE_V1".to_string(),
            intent_gradient_frame: generate_default_intent_gradient_frame(),
            language_law: CourtLanguageLaw::active(),
            seated_poles,
        }
    }

    pub fn operational_count(&self) -> usize {
        self.seated_poles.iter().filter(|p| p.operational).count()
    }

    pub fn council_ready(&self) -> bool {
        CouncilPole::BRAIN_POLES.iter().all(|brain_pole| {
            self.seated_poles
                .iter()
                .any(|s| s.pole == *brain_pole && s.operational)
        })
    }

    pub fn total_brain_parameters_b(&self) -> u64 {
        self.seated_poles
            .iter()
            .filter(|p| p.pole.is_brain_pole())
            .map(|p| p.confirmed_parameter_scale_b)
            .sum()
    }

    pub fn validate_intent_gradient(&self) -> NsqIntentGradientValidation {
        validate_intent_gradient_frame(&self.intent_gradient_frame)
    }
}

// ── Boot clearance ─────────────────────────────────────────────────────────

/// Boot clearance for the NSQ court.
/// Replaces `final_dax_os_boot_launch_ready` everywhere it appeared.
/// The court does not boot until every field is true. No partial boot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CourtBootClearance {
    pub schema: String,
    pub intent_gradient_valid: bool,
    pub council_of_six_ready: bool,
    pub language_law_active: bool,
    pub sensory_body_ready: bool,
    pub signed_handoff_present: bool,
    pub native_binding_confirmed: bool,
    /// The boot gate. True only when ALL conditions are met.
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
            schema: "nsq.court.boot_clearance.v1".to_string(),
            intent_gradient_valid,
            council_of_six_ready,
            language_law_active,
            sensory_body_ready,
            signed_handoff_present,
            native_binding_confirmed,
            nsq_court_launch_ready,
        }
    }

    pub fn not_ready() -> Self {
        Self {
            schema: "nsq.court.boot_clearance.v1".to_string(),
            intent_gradient_valid: false,
            council_of_six_ready: false,
            language_law_active: true,
            sensory_body_ready: false,
            signed_handoff_present: false,
            native_binding_confirmed: false,
            nsq_court_launch_ready: false,
        }
    }
}

// ── Tests ──────────────────────────────────────────────────────────────────

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
        assert!(IntentSurface::StampDispatch.is_inner_court());
        assert!(IntentSurface::StampDispatch.is_stamp_surface());
    }

    #[test]
    fn all_ten_poles_declared() {
        assert_eq!(CouncilPole::ALL.len(), 10);
        assert_eq!(CouncilPole::BRAIN_POLES.len(), 6);
        assert_eq!(CouncilPole::SENSORY_BODIES.len(), 4);
    }

    #[test]
    fn eight_semantic_variables_distinct_indices() {
        let mut seen = std::collections::HashSet::new();
        for var in NsqIntentVariable::ALL {
            assert!(seen.insert(var.index()), "duplicate index for {:?}", var);
        }
        assert_eq!(seen.len(), NSQ_INTENT_GRADIENT_VARIABLES);
    }

    #[test]
    fn brain_poles_have_distinct_primary_variables() {
        let mut seen = std::collections::HashSet::new();
        for pole in CouncilPole::BRAIN_POLES {
            let var = pole.primary_variable();
            assert!(seen.insert(var.index()), "duplicate primary variable {:?}", var);
        }
    }

    #[test]
    fn brain_poles_meet_seventy_b_floor() {
        for pole in CouncilPole::BRAIN_POLES {
            assert!(pole.parameter_floor_b() >= 70, "{} below 70B", pole.as_str());
        }
    }

    #[test]
    fn total_brain_parameters_exceed_one_trillion() {
        let total: u64 = CouncilPole::BRAIN_POLES.iter().map(|p| p.parameter_floor_b()).sum();
        // 232 + 235 + 123 + 604 + 70 + 70 = 1334B
        assert!(total > 1000, "{}B must exceed 1 trillion", total);
    }

    #[test]
    fn correct_models_in_correct_poles() {
        assert!(CouncilPole::MaverickLogic.canonical_model_source().contains("maverick"));
        assert!(CouncilPole::DeepSeekAnalyzer.canonical_model_source().contains("deepseek"));
        assert_eq!(CouncilPole::DeepSeekAnalyzer.parameter_floor_b(), 604);
        assert!(CouncilPole::DevstralArbiter.canonical_model_source().contains("devstral"));
        assert_eq!(CouncilPole::DevstralArbiter.parameter_floor_b(), 123);
        assert!(CouncilPole::QwenCreativity.canonical_model_source().contains("qwen3"));
    }

    #[test]
    fn intent_gradient_frame_validates_clean() {
        let frame = generate_default_intent_gradient_frame();
        let v = validate_intent_gradient_frame(&frame);
        assert_eq!(v.bad_count, 0);
        assert!(v.all_variables_present);
        assert!(v.positions_inside_final_tier);
    }

    #[test]
    fn court_language_law_always_active() {
        let law = CourtLanguageLaw::active();
        assert!(law.no_internal_language_flag);
        assert!(law.inner_system_language.contains("nsq_intent_gradient"));
        assert!(law.tokenizer_role.contains("boundary_projection_only"));
    }

    #[test]
    fn boot_clearance_requires_all_conditions() {
        let seating = CourtSeating::new(vec![]);
        let clearance = CourtBootClearance::evaluate(&seating, false, false);
        assert!(!clearance.nsq_court_launch_ready);
        assert!(!clearance.council_of_six_ready);
        assert!(clearance.language_law_active);
    }

    #[test]
    fn intent_pressure_routes_without_translation() {
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
        // Gradient is preserved exactly — no translation between poles
        assert_eq!(routed.variable(NsqIntentVariable::Force), 800);
        assert_eq!(routed.variable(NsqIntentVariable::Motive), 950);
    }
}
