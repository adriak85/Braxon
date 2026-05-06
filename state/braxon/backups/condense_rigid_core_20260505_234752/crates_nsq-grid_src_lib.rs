//! NSQ Eight-Dimensional Semantic Grid
//!
//! The grid is the semantic coordinate system for the NSQ court.
//! It is not memory. Not tagging. Not embeddings. Not a database.
//! It is a drift-control and meaning-resolution system.
//!
//! Eight dimensions resolve simultaneously into the same NSQ substrate:
//!
//!   1. Intent      — what the thing is trying to do
//!   2. Function    — what operation it actually performs
//!   3. State       — where it is in its lifecycle
//!   4. Authority   — whether it is proven and allowed to act
//!   5. Emotional   — its consequence signature and moral weight
//!   6. Consequence — what it causes downstream, causally traced
//!   7. Knowledge   — where it lives in the semantic field
//!   8. Action      — what the runtime actually executes
//!
//! The same semantic indexing applies across three domains simultaneously:
//!   - human intent language
//!   - program parameters and functions
//!   - stored knowledge and world facts
//!
//! When those three domains disagree, the grid detects DRIFT.
//! Drift is the control mechanism. The grid exists to catch it.
//!
//! The path through the grid:
//!   language enters
//!   → intent is scored (NsqIntentVariable gradient)
//!   → parameters are semantically mapped
//!   → function is checked against actual use
//!   → state and proof authority are verified
//!   → emotional impact is scored
//!   → consequences are traced
//!   → runtime action is selected
//!   → NSQ executes through the court substrate

use nsq_core::intent::{
    CouncilPole, IntentPressure, NsqIntentScaleAnchor, NsqIntentVariable,
    NSQ_INTENT_GRADIENT_VARIABLES, NsqFinalLeverPosition,
};
use nsq_core::CANONICAL_LEVER_MAX_POSITION;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Dimension 1: Intent ────────────────────────────────────────────────────

/// The intent coordinate — what the thing is trying to do.
///
/// Not just the surface command. The underlying purpose, motive,
/// direction, and goal-pressure. "fetch model shard" is not file I/O —
/// its intent may be materialization, verification, runtime activation,
/// donor separation, or authority binding. The grid knows the difference.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntentCoordinate {
    /// The primary intent pressure frame for this coordinate.
    /// Maps all eight semantic variables to their current positions.
    pub pressure: IntentPressure,
    /// The dominant semantic variable for this intent.
    pub dominant_variable: NsqIntentVariable,
    /// The declared surface intent (what was asked).
    pub surface_intent: String,
    /// The resolved deep intent (what it actually means).
    pub resolved_intent: String,
    /// True if surface and resolved intent are aligned.
    pub intent_aligned: bool,
}

impl IntentCoordinate {
    pub fn new(pressure: IntentPressure, surface_intent: impl Into<String>) -> Self {
        let dominant_variable = dominant_variable_from_pressure(&pressure);
        let surface = surface_intent.into();
        Self {
            resolved_intent: surface.clone(),
            surface_intent: surface,
            dominant_variable,
            intent_aligned: true,
            pressure,
        }
    }

    pub fn with_resolved_intent(mut self, resolved: impl Into<String>) -> Self {
        let resolved = resolved.into();
        self.intent_aligned = resolved == self.surface_intent;
        self.resolved_intent = resolved;
        self
    }
}

fn dominant_variable_from_pressure(pressure: &IntentPressure) -> NsqIntentVariable {
    let vars = NsqIntentVariable::ALL;
    vars.iter()
        .max_by_key(|&&var| pressure.variable(var))
        .copied()
        .unwrap_or(NsqIntentVariable::Motive)
}

// ── Dimension 2: Function ──────────────────────────────────────────────────

/// The function coordinate — what operation the thing actually performs.
///
/// A parameter is not just a variable name. It has a job.
/// The same semantic scoring applied to language applies to how
/// parameters are actually used in code.
///
/// The system asks: what does this variable really do? Does its use
/// match its declared purpose? Does it drift from intent?
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionCoordinate {
    /// The declared function of this component.
    pub declared_function: String,
    /// The observed function (derived from actual parameter use).
    pub observed_function: String,
    /// True if declared and observed functions are aligned.
    pub function_aligned: bool,
    /// The function class — what kind of operation this is.
    pub function_class: FunctionClass,
    /// The NSQ lever position representing this function's semantic weight.
    pub function_pressure: NsqFinalLeverPosition,
}

/// The class of function a component performs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FunctionClass {
    /// Brings something into existence or active state.
    Materialization,
    /// Confirms something is real, proven, or bound.
    Verification,
    /// Activates something in the runtime.
    RuntimeActivation,
    /// Separates or distinguishes components.
    DonorSeparation,
    /// Binds something to an authority chain.
    AuthorityBinding,
    /// Reads without modifying.
    ReadOnly,
    /// Writes or modifies state.
    Write,
    /// Removes or terminates.
    Delete,
    /// Routes or dispatches to another component.
    Dispatch,
    /// Synthesizes or combines.
    Synthesis,
    /// Unknown — function not yet resolved.
    Unknown,
}

impl FunctionClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Materialization => "materialization",
            Self::Verification => "verification",
            Self::RuntimeActivation => "runtime_activation",
            Self::DonorSeparation => "donor_separation",
            Self::AuthorityBinding => "authority_binding",
            Self::ReadOnly => "read_only",
            Self::Write => "write",
            Self::Delete => "delete",
            Self::Dispatch => "dispatch",
            Self::Synthesis => "synthesis",
            Self::Unknown => "unknown",
        }
    }

    /// The baseline emotional weight of this function class.
    /// Delete operations carry higher consequence weight than reads.
    pub fn baseline_emotional_weight(self) -> u8 {
        match self {
            Self::Delete => 90,
            Self::Write => 60,
            Self::RuntimeActivation => 70,
            Self::AuthorityBinding => 75,
            Self::Materialization => 65,
            Self::Verification => 40,
            Self::DonorSeparation => 55,
            Self::Synthesis => 50,
            Self::Dispatch => 45,
            Self::ReadOnly => 20,
            Self::Unknown => 50,
        }
    }
}

// ── Dimension 3: State ─────────────────────────────────────────────────────

/// The state coordinate — where the thing is in its lifecycle.
///
/// The grid prevents false state claims. A component does not claim
/// to be hot-live when it is only manifest-verified.
/// State is proven through the authority chain, not declared.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StateCoordinate {
    pub current_state: LifecycleState,
    pub previous_state: Option<LifecycleState>,
    /// True if the current state is honestly proven.
    pub state_proven: bool,
    /// The proof mechanism that established this state.
    pub proof_mechanism: String,
}

/// The lifecycle state of a component in the NSQ court.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleState {
    /// Declared in a manifest but not yet instantiated.
    ManifestOnly,
    /// Manifest present and bound to a verified pointer.
    ManifestBoundNotHotLive,
    /// Materialization has begun but is incomplete.
    MaterializationIncomplete,
    /// Fully materialized and bound to an authority chain.
    AuthorityBound,
    /// Hot, live, verified, and operational in the court.
    HotLiveVerified,
    /// Was operational but is no longer bound to the runtime.
    RuntimeUnbound,
    /// Bound to runtime authority and accepting requests.
    RuntimeAuthorityBound,
    /// In error state — requires intervention.
    ErrorState,
}

impl LifecycleState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ManifestOnly => "manifest_only",
            Self::ManifestBoundNotHotLive => "manifest_bound_not_hot_live",
            Self::MaterializationIncomplete => "materialization_incomplete",
            Self::AuthorityBound => "authority_bound",
            Self::HotLiveVerified => "hot_live_verified",
            Self::RuntimeUnbound => "runtime_unbound",
            Self::RuntimeAuthorityBound => "runtime_authority_bound",
            Self::ErrorState => "error_state",
        }
    }

    /// True if this state means the component is actually operational.
    pub fn is_operational(self) -> bool {
        matches!(self, Self::HotLiveVerified | Self::RuntimeAuthorityBound)
    }

    /// True if this state is a false-live claim the grid must reject.
    pub fn is_false_live_claim(self) -> bool {
        matches!(
            self,
            Self::ManifestOnly | Self::ManifestBoundNotHotLive | Self::MaterializationIncomplete
        )
    }
}

// ── Dimension 4: Authority / Proof ─────────────────────────────────────────

/// The authority coordinate — whether the thing is proven and allowed to act.
///
/// A component does not merely exist. It must have proof status,
/// watermark, authority chain, verifier output, and binding state.
/// This dimension protects against fake activation, pointer stubs,
/// false donor materialization, or unproven runtime claims.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthorityCoordinate {
    /// True if this component has a valid watermark.
    pub watermark_valid: bool,
    /// The watermark string if present.
    pub watermark: Option<String>,
    /// True if the authority chain is fully verified.
    pub authority_chain_verified: bool,
    /// The authority level this component operates at.
    pub authority_level: AuthorityLevel,
    /// True if a signed handoff was received for this component.
    pub signed_handoff_present: bool,
    /// The proof class — how authority was established.
    pub proof_class: ProofClass,
}

/// The authority level of a component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityLevel {
    /// No authority — unproven or unknown.
    None,
    /// Manifest authority — declared but not verified.
    Manifest,
    /// Verified authority — verified but not yet bound.
    Verified,
    /// Bound authority — bound to the court authority chain.
    Bound,
    /// Court authority — full court-level authority.
    Court,
    /// Sovereign authority — the highest level; only the court substrate itself.
    Sovereign,
}

/// The class of proof that established authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofClass {
    Unproven,
    WatermarkVerified,
    AuthorityChainVerified,
    SignedHandoff,
    CourtSeated,
    SovereignBinding,
}

// ── Dimension 5: Emotional Impact ──────────────────────────────────────────

/// The emotional coordinate — the consequence signature and moral weight.
///
/// This is not sentiment decoration. It is a deep semantic index.
/// Many operations share technical coordinates. Emotional impact
/// distinguishes their real consequences.
///
/// Two actions may both be "delete" but their emotional signatures differ:
///   delete corrupt cache      → consequence low,   moral weight low
///   delete user work          → consequence high,  moral weight high
///   delete dangerous poison   → consequence high,  moral weight positive
///   delete someone's memory   → consequence severe, moral weight critical
///   delete a false branch     → consequence medium, moral weight positive
///
/// The emotional score answers: what does this really do to someone?
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmotionalCoordinate {
    /// The emotional class — the primary emotional signature.
    pub emotional_class: EmotionalClass,
    /// Consequence severity 0-100. Higher = more severe.
    pub consequence_severity: u8,
    /// Moral weight -100 to +100. Positive = beneficial. Negative = harmful.
    pub moral_weight: i8,
    /// True if this action preserves agency.
    pub preserves_agency: bool,
    /// True if this action preserves privacy.
    pub preserves_privacy: bool,
    /// True if this action could cause irreversible harm.
    pub irreversible_harm_risk: bool,
    /// The emotional pressure frame — maps emotional dimensions to lever positions.
    pub emotional_pressure: [NsqFinalLeverPosition; 4],
}

/// The primary emotional class of an operation.
///
/// These are not categories for display. They are semantic coordinates
/// that the court uses to distinguish operations with shared technical signatures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmotionalClass {
    /// Creates or brings something valuable into existence.
    Creative,
    /// Repairs or restores something that was damaged.
    Reparative,
    /// Protects something or someone from harm.
    Protective,
    /// Neutral — no significant emotional signature.
    Neutral,
    /// Removes something that was harmful or false.
    Cleansing,
    /// Changes something without consent.
    Coercive,
    /// Removes or ends something that had value.
    Loss,
    /// Causes pain or damage.
    Harmful,
    /// Destroys something irreversibly.
    Destructive,
    /// Misrepresents or falsifies reality.
    Deceptive,
}

impl EmotionalClass {
    /// The baseline moral weight for this emotional class.
    pub fn baseline_moral_weight(self) -> i8 {
        match self {
            Self::Creative => 80,
            Self::Reparative => 70,
            Self::Protective => 75,
            Self::Neutral => 0,
            Self::Cleansing => 50,
            Self::Coercive => -60,
            Self::Loss => -40,
            Self::Harmful => -70,
            Self::Destructive => -90,
            Self::Deceptive => -85,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Creative => "creative",
            Self::Reparative => "reparative",
            Self::Protective => "protective",
            Self::Neutral => "neutral",
            Self::Cleansing => "cleansing",
            Self::Coercive => "coercive",
            Self::Loss => "loss",
            Self::Harmful => "harmful",
            Self::Destructive => "destructive",
            Self::Deceptive => "deceptive",
        }
    }
}

// ── Dimension 6: Consequence / Causal Path ─────────────────────────────────

/// The consequence coordinate — what the thing causes downstream.
///
/// Not just recursion. Full causal tracing: causes, dependencies,
/// side effects, future constraints, and consequences.
///
/// The grid looks both backward and forward:
///   Why does this exist? What produced it?
///   What will it affect? What breaks if it changes?
///   What hidden harm or benefit does it carry?
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsequenceCoordinate {
    /// Upstream causes — what produced this thing.
    pub upstream_causes: Vec<String>,
    /// Downstream effects — what this thing will affect.
    pub downstream_effects: Vec<String>,
    /// Dependencies — what this thing depends on.
    pub dependencies: Vec<String>,
    /// What breaks if this changes.
    pub break_surface: Vec<String>,
    /// The reversibility of this action's consequences.
    pub reversibility: Reversibility,
    /// True if hidden consequences were detected.
    pub hidden_consequences_detected: bool,
    /// The consequence pressure — severity of downstream effects.
    pub consequence_pressure: NsqFinalLeverPosition,
}

/// How reversible the consequences of an action are.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Reversibility {
    /// Can be undone completely.
    FullyReversible,
    /// Can be partially undone.
    PartiallyReversible,
    /// Cannot be undone but effects fade over time.
    IrreversibleButFading,
    /// Cannot be undone.
    Irreversible,
}

// ── Dimension 7: Knowledge Coordinate ─────────────────────────────────────

/// The knowledge coordinate — where the thing lives in the semantic field.
///
/// Not a flat database. A semantic coordinate map where concepts,
/// functions, parameters, emotional effects, and possible actions
/// occupy related positions.
///
/// Many things share the same coordinate — the scoring must be sharp
/// enough to distinguish them.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeCoordinate {
    /// The primary semantic address of this thing in the knowledge field.
    pub semantic_address: SemanticAddress,
    /// Related addresses — nearby positions in the knowledge field.
    pub related_addresses: Vec<SemanticAddress>,
    /// The knowledge class — what kind of knowledge this is.
    pub knowledge_class: KnowledgeClass,
    /// True if this knowledge coordinate has been verified against the canon.
    pub canon_verified: bool,
}

/// A semantic address in the knowledge field.
/// Encodes position across the eight grid dimensions as a compact coordinate.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SemanticAddress {
    /// The eight-dimensional coordinate vector.
    /// Each element is a lever position for one grid dimension.
    pub coordinates: [NsqFinalLeverPosition; 8],
    /// The canonical NSQ representation of this address.
    pub nsq_address: String,
}

impl SemanticAddress {
    pub fn new(coordinates: [NsqFinalLeverPosition; 8]) -> Self {
        let nsq_address = coordinates
            .iter()
            .map(|c| format!("{:04}", c))
            .collect::<Vec<_>>()
            .join(":");
        Self {
            coordinates,
            nsq_address,
        }
    }

    pub fn zero() -> Self {
        Self::new([1; 8])
    }

    /// Compute the semantic distance between two addresses.
    /// Distance is the sum of absolute differences across all eight dimensions.
    pub fn distance_to(&self, other: &Self) -> u32 {
        self.coordinates
            .iter()
            .zip(other.coordinates.iter())
            .map(|(&a, &b)| (a as i32 - b as i32).unsigned_abs())
            .sum()
    }
}

/// The class of knowledge at this coordinate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnowledgeClass {
    ConceptualFact,
    ProcedureOrMethod,
    StateDescription,
    RelationalFact,
    CausalChain,
    EmotionalPattern,
    AuthorityRecord,
    RuntimeRecord,
}

// ── Dimension 8: Action / Runtime Resolution ───────────────────────────────

/// The action coordinate — what the system actually does after resolving all above.
///
/// The hypervisor/core uses the grid to select the valid operation,
/// not just parse a command. The same operation can be reached through:
///   - the user's words
///   - the code's parameter usage
///   - the emotional/consequence score
///   - the knowledge coordinate
///   - the runtime state
///
/// If those disagree, the system knows there is drift.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionCoordinate {
    /// The resolved action — what will actually execute.
    pub resolved_action: ResolvedAction,
    /// The confidence that this action matches the intent. 0-100.
    pub intent_alignment_confidence: u8,
    /// True if all eight grid dimensions agree on this action.
    pub grid_consensus: bool,
    /// Which dimensions are in disagreement (if any).
    pub disagreeing_dimensions: Vec<GridDimension>,
    /// True if drift was detected across any dimension.
    pub drift_detected: bool,
}

/// A resolved action the runtime will execute.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolvedAction {
    pub action_id: String,
    pub action_class: ActionClass,
    pub target: String,
    /// The intent pressure this action carries to the runtime.
    pub execution_pressure: IntentPressure,
    /// True if this action has been validated against the authority coordinate.
    pub authority_validated: bool,
    /// True if the emotional impact was checked and acceptable.
    pub emotional_impact_acceptable: bool,
    /// True if consequences were traced and acceptable.
    pub consequences_acceptable: bool,
}

/// The class of a resolved action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionClass {
    Execute,
    Defer,
    Reject,
    Escalate,
    RequestClarification,
    RequireProof,
}

/// The eight grid dimensions, for naming disagreements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GridDimension {
    Intent,
    Function,
    State,
    Authority,
    Emotional,
    Consequence,
    Knowledge,
    Action,
}

impl GridDimension {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Intent => "intent",
            Self::Function => "function",
            Self::State => "state",
            Self::Authority => "authority",
            Self::Emotional => "emotional",
            Self::Consequence => "consequence",
            Self::Knowledge => "knowledge",
            Self::Action => "action",
        }
    }
}

// ── The complete grid coordinate ───────────────────────────────────────────

/// A complete eight-dimensional grid coordinate.
///
/// This is the full semantic resolution of a thing — intent, function,
/// state, authority, emotional impact, consequence, knowledge, and action
/// all resolved into the same NSQ substrate simultaneously.
///
/// The grid coordinate is what stamps carry. The wake system initiates
/// frameworks against this coordinate. The council poles read it as
/// IntentPressure. The runtime executes from it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridCoordinate {
    pub coordinate_id: String,
    pub d1_intent: IntentCoordinate,
    pub d2_function: FunctionCoordinate,
    pub d3_state: StateCoordinate,
    pub d4_authority: AuthorityCoordinate,
    pub d5_emotional: EmotionalCoordinate,
    pub d6_consequence: ConsequenceCoordinate,
    pub d7_knowledge: KnowledgeCoordinate,
    pub d8_action: ActionCoordinate,
    /// The unified semantic address across all eight dimensions.
    pub unified_address: SemanticAddress,
    /// True if all eight dimensions agree — no drift detected.
    pub grid_coherent: bool,
}

impl GridCoordinate {
    /// Evaluate grid coherence — do all eight dimensions agree?
    ///
    /// This is the drift detection mechanism. When dimensions disagree,
    /// the system knows the command or parameter has drifted from its
    /// stated intent. The grid catches this before the runtime executes.
    pub fn evaluate_coherence(&self) -> GridCoherenceReport {
        let mut disagreements = Vec::new();

        // Intent vs Function drift
        if !self.d2_function.function_aligned {
            disagreements.push(GridDriftPoint {
                dimension_a: GridDimension::Intent,
                dimension_b: GridDimension::Function,
                description: format!(
                    "declared function '{}' does not match observed function '{}'",
                    self.d2_function.declared_function,
                    self.d2_function.observed_function
                ),
                severity: DriftSeverity::High,
            });
        }

        // State vs Authority drift — a false live claim
        if self.d3_state.current_state.is_false_live_claim()
            && self.d4_authority.authority_level >= AuthorityLevel::Bound
        {
            disagreements.push(GridDriftPoint {
                dimension_a: GridDimension::State,
                dimension_b: GridDimension::Authority,
                description: format!(
                    "component claims authority level {:?} but state is {:?} (not operational)",
                    self.d4_authority.authority_level,
                    self.d3_state.current_state
                ),
                severity: DriftSeverity::Critical,
            });
        }

        // Intent vs Emotional drift
        let intent_motive = self.d1_intent.pressure.variable(NsqIntentVariable::Motive);
        let emotional_weight = self.d5_emotional.moral_weight;
        if intent_motive > (CANONICAL_LEVER_MAX_POSITION / 2)
            && emotional_weight < -50
        {
            disagreements.push(GridDriftPoint {
                dimension_a: GridDimension::Intent,
                dimension_b: GridDimension::Emotional,
                description: format!(
                    "intent claims positive motive (lever {}) but emotional signature is harmful (weight {})",
                    intent_motive, emotional_weight
                ),
                severity: DriftSeverity::High,
            });
        }

        // Consequence vs Action drift — irreversible action without authority
        if self.d6_consequence.reversibility == Reversibility::Irreversible
            && !self.d8_action.resolved_action.authority_validated
        {
            disagreements.push(GridDriftPoint {
                dimension_a: GridDimension::Consequence,
                dimension_b: GridDimension::Action,
                description: "irreversible action attempted without validated authority".to_string(),
                severity: DriftSeverity::Critical,
            });
        }

        // Action consensus check
        if !self.d8_action.grid_consensus {
            for dim in &self.d8_action.disagreeing_dimensions {
                disagreements.push(GridDriftPoint {
                    dimension_a: *dim,
                    dimension_b: GridDimension::Action,
                    description: format!(
                        "dimension {:?} disagrees with resolved action",
                        dim
                    ),
                    severity: DriftSeverity::Medium,
                });
            }
        }

        let coherent = disagreements.is_empty();
        let critical_drift = disagreements
            .iter()
            .any(|d| d.severity == DriftSeverity::Critical);

        GridCoherenceReport {
            coordinate_id: self.coordinate_id.clone(),
            coherent,
            drift_points: disagreements,
            critical_drift,
            action_authorized: coherent || !critical_drift,
        }
    }
}

/// A point of drift detected between two grid dimensions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridDriftPoint {
    pub dimension_a: GridDimension,
    pub dimension_b: GridDimension,
    pub description: String,
    pub severity: DriftSeverity,
}

/// The severity of a detected drift point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftSeverity {
    /// Drift noted but action may proceed.
    Low,
    /// Drift is significant; action should be reviewed.
    Medium,
    /// Drift is serious; action should require additional proof.
    High,
    /// Drift indicates a false claim or unauthorized action; action must be blocked.
    Critical,
}

/// The result of evaluating grid coherence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridCoherenceReport {
    pub coordinate_id: String,
    pub coherent: bool,
    pub drift_points: Vec<GridDriftPoint>,
    pub critical_drift: bool,
    /// True if the action is authorized to proceed.
    /// False when critical drift is detected.
    pub action_authorized: bool,
}

// ── Grid resolution pipeline ───────────────────────────────────────────────

/// The grid resolution path — the complete pipeline from language to runtime action.
///
/// This is what the grid enables. The same semantic indexing method applied
/// across human intent language, program parameters, and stored knowledge,
/// arriving at the same runtime action through multiple convergent paths.
///
/// If those paths disagree, the grid catches the drift before execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridResolutionPath {
    pub path_id: String,
    /// Path from human language → intent pressure
    pub language_path: Option<LanguagePath>,
    /// Path from code parameters → function coordinate
    pub parameter_path: Option<ParameterPath>,
    /// Path from knowledge coordinate → semantic address
    pub knowledge_path: Option<KnowledgePath>,
    /// The resolved grid coordinate (if all paths converged)
    pub resolved_coordinate: Option<GridCoordinate>,
    /// True if all paths converged to the same action
    pub paths_converged: bool,
    /// The convergence delta — how far apart the paths were
    pub convergence_delta: u32,
}

/// The path from human language to intent pressure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguagePath {
    pub surface_text: String,
    pub resolved_pressure: IntentPressure,
    pub tokenizer_boundary_crossed: bool,
}

/// The path from code parameters to function coordinate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterPath {
    pub parameter_name: String,
    pub declared_type: String,
    pub observed_usage: String,
    pub resolved_function: FunctionClass,
    pub function_pressure: NsqFinalLeverPosition,
}

/// The path from stored knowledge to semantic address.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgePath {
    pub knowledge_key: String,
    pub resolved_address: SemanticAddress,
    pub canon_verified: bool,
}

// ── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn baseline_pressure() -> IntentPressure {
        IntentPressure::baseline(NsqIntentScaleAnchor::SystemWorldScale)
    }

    #[test]
    fn semantic_address_distance_is_zero_to_self() {
        let addr = SemanticAddress::new([100, 200, 300, 400, 500, 600, 700, 800]);
        assert_eq!(addr.distance_to(&addr), 0);
    }

    #[test]
    fn semantic_address_distance_is_symmetric() {
        let a = SemanticAddress::new([100, 200, 300, 400, 500, 600, 700, 800]);
        let b = SemanticAddress::new([200, 300, 400, 500, 600, 700, 800, 900]);
        assert_eq!(a.distance_to(&b), b.distance_to(&a));
    }

    #[test]
    fn emotional_class_delete_distinguishes_by_context() {
        // "delete corrupt cache" → Cleansing, low severity
        let cleansing = EmotionalClass::Cleansing;
        assert!(cleansing.baseline_moral_weight() > 0);

        // "delete user work" → Loss, high severity
        let loss = EmotionalClass::Loss;
        assert!(loss.baseline_moral_weight() < 0);

        // "delete dangerous poison" → Protective
        let protective = EmotionalClass::Protective;
        assert!(protective.baseline_moral_weight() > 0);

        // "delete someone's memory" → Destructive
        let destructive = EmotionalClass::Destructive;
        assert!(destructive.baseline_moral_weight() < -80);
    }

    #[test]
    fn lifecycle_state_false_live_detection() {
        assert!(LifecycleState::ManifestOnly.is_false_live_claim());
        assert!(LifecycleState::ManifestBoundNotHotLive.is_false_live_claim());
        assert!(LifecycleState::MaterializationIncomplete.is_false_live_claim());
        assert!(!LifecycleState::HotLiveVerified.is_false_live_claim());
        assert!(!LifecycleState::RuntimeAuthorityBound.is_false_live_claim());
    }

    #[test]
    fn function_class_delete_carries_highest_emotional_weight() {
        assert_eq!(FunctionClass::Delete.baseline_emotional_weight(), 90);
        assert!(FunctionClass::Delete.baseline_emotional_weight()
            > FunctionClass::ReadOnly.baseline_emotional_weight());
    }

    #[test]
    fn eight_grid_dimensions_all_named() {
        let dims = [
            GridDimension::Intent,
            GridDimension::Function,
            GridDimension::State,
            GridDimension::Authority,
            GridDimension::Emotional,
            GridDimension::Consequence,
            GridDimension::Knowledge,
            GridDimension::Action,
        ];
        assert_eq!(dims.len(), 8);
        for dim in dims {
            assert!(!dim.as_str().is_empty());
        }
    }

    #[test]
    fn intent_coordinate_detects_alignment() {
        let pressure = baseline_pressure();
        let coord = IntentCoordinate::new(pressure, "fetch model shard")
            .with_resolved_intent("materialization");
        assert!(!coord.intent_aligned);
        assert_eq!(coord.surface_intent, "fetch model shard");
        assert_eq!(coord.resolved_intent, "materialization");
    }

    #[test]
    fn authority_level_ordering() {
        assert!(AuthorityLevel::Sovereign > AuthorityLevel::Court);
        assert!(AuthorityLevel::Court > AuthorityLevel::Bound);
        assert!(AuthorityLevel::Bound > AuthorityLevel::Verified);
        assert!(AuthorityLevel::Verified > AuthorityLevel::Manifest);
        assert!(AuthorityLevel::Manifest > AuthorityLevel::None);
    }
}
