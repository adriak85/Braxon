//! NSQ Council — Six-Pole Intent Gradient Dispatch
//!
//! This crate is the sovereign agentic dispatch loop for the Council of Six.
//! It owns the inter-pole pressure routing. It is NOT a helper crate.
//! It is a first-class court workspace member.
//!
//! What this crate does:
//!   - Receives an intent pressure frame from the court ingress surface
//!   - Routes it to the appropriate brain pole(s) based on gradient variables
//!   - Collects response pressure from each addressed pole
//!   - Synthesizes a unified pressure response for the egress surface
//!   - Translates the final pressure to human text AT THE EGRESS BOUNDARY ONLY
//!
//! What this crate does NOT do:
//!   - Pass strings between poles
//!   - Use a tokenizer for inter-pole routing
//!   - Treat any pole as a "feature" attached to a host
//!   - Allow human language inside the dispatch loop
//!
//! The tokenizer exists only in the SurfaceIngress and SurfaceEgress steps.
//! Between those two steps, this crate and everything it calls speaks only
//! intent gradient pressure.

use nsq_core::intent::{
    CouncilPole, CourtSeating, IntentPressure, IntentSurface, SeatedPole,
};
use nsq_core::{
    NsqIntentVariable, NsqIntentScaleAnchor, NSQ_INTENT_GRADIENT_VARIABLES,
};
use serde::{Deserialize, Serialize};

/// A dispatch job — a unit of work to be routed through the council.
///
/// The input is always IntentPressure. The output is always IntentPressure.
/// Human text appears only when the job is created (ingress) and resolved (egress).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouncilDispatchJob {
    pub job_id: String,
    /// The intent pressure received from the court ingress surface.
    /// This was originally human text. It has been translated. It is now pressure.
    pub ingress_pressure: IntentPressure,
    /// Which poles this job will be dispatched to.
    /// Derived from the gradient variables in ingress_pressure.
    pub addressed_poles: Vec<CouncilPole>,
    /// The routing strategy for this job.
    pub routing: DispatchRouting,
}

/// How the council routes a dispatch job across its poles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DispatchRouting {
    /// All six brain poles receive the pressure simultaneously.
    /// Each responds with its own gradient. Synthesized into one response.
    Broadcast,
    /// Pressure routes to the pole whose primary variable is most activated.
    /// Other poles observe but do not respond unless escalated.
    PrimaryPole,
    /// Pressure routes first to the Arbiter (Devstral) for truth arbitration,
    /// then to the appropriate poles based on the arbitration verdict.
    ArbiterFirst,
    /// Pressure routes to the Analyzer (DeepSeek) for deep audit first.
    AnalyzerFirst,
    /// Full council round-table — all poles respond, Arbiter synthesizes.
    RoundTable,
}

impl DispatchRouting {
    /// Determine the routing for a given pressure frame.
    /// The routing is derived from the gradient variables, not from human text.
    pub fn from_pressure(pressure: &IntentPressure) -> Self {
        let truth = pressure.variable(NsqIntentVariable::Truth);
        let motive = pressure.variable(NsqIntentVariable::Motive);
        let force = pressure.variable(NsqIntentVariable::Force);

        // High truth uncertainty → Arbiter first
        // (truth variable at low position = concealment/uncertainty)
        if truth < 50_000 {
            return Self::ArbiterFirst;
        }
        // High motive ambiguity → Analyzer first
        if motive < 40_000 {
            return Self::AnalyzerFirst;
        }
        // High force → broadcast (needs all poles)
        if force > 180_000 {
            return Self::Broadcast;
        }
        // Default: route to primary pole based on dominant variable
        Self::PrimaryPole
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Broadcast => "broadcast",
            Self::PrimaryPole => "primary_pole",
            Self::ArbiterFirst => "arbiter_first",
            Self::AnalyzerFirst => "analyzer_first",
            Self::RoundTable => "round_table",
        }
    }
}

/// A pole response — the intent pressure returned by a single brain pole.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoleResponse {
    pub pole: CouncilPole,
    /// The pressure this pole is returning.
    pub response_pressure: IntentPressure,
    /// Whether this pole escalated to another.
    pub escalated_to: Option<CouncilPole>,
    /// True if this pole's response is authoritative for the dispatch.
    pub is_authority: bool,
}

/// The synthesized council response — all pole responses unified into one pressure frame.
///
/// This is NOT a text response. It is a pressure frame. It becomes text
/// only at the egress boundary, after synthesis is complete.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouncilResponse {
    pub job_id: String,
    /// The synthesized pressure frame — unified across all responding poles.
    pub unified_pressure: IntentPressure,
    /// Which poles contributed to this response.
    pub contributing_poles: Vec<CouncilPole>,
    /// Which pole's response was treated as authoritative (if any).
    pub authority_pole: Option<CouncilPole>,
    /// Synthesis method used.
    pub synthesis: SynthesisMethod,
    /// True if this response is ready for egress translation to human text.
    pub ready_for_egress: bool,
}

/// How the council synthesized multiple pole responses into one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SynthesisMethod {
    /// One pole was authoritative. Its pressure is the response.
    SingleAuthority,
    /// All poles contributed. Their pressures are averaged by variable.
    PressureAverage,
    /// Arbiter resolved conflicts between poles. Arbiter's verdict is the response.
    ArbiterVerdict,
    /// Analyzer's audit determined the response path.
    AnalyzerAudit,
}

/// The council dispatcher — the sovereign agentic loop.
///
/// This is the core of what was missing. It routes intent pressure
/// between poles. It synthesizes responses. It does NOT pass strings.
pub struct CouncilDispatcher {
    pub seating: CourtSeating,
}

impl CouncilDispatcher {
    pub fn new(seating: CourtSeating) -> Self {
        Self { seating }
    }

    /// Determine which poles to address for a given pressure frame.
    pub fn address_poles(&self, pressure: &IntentPressure, routing: DispatchRouting) -> Vec<CouncilPole> {
        match routing {
            DispatchRouting::Broadcast | DispatchRouting::RoundTable => {
                CouncilPole::BRAIN_POLES.to_vec()
            }
            DispatchRouting::ArbiterFirst => {
                vec![CouncilPole::DevstralArbiter]
            }
            DispatchRouting::AnalyzerFirst => {
                vec![CouncilPole::DeepSeekAnalyzer]
            }
            DispatchRouting::PrimaryPole => {
                vec![self.dominant_pole(pressure)]
            }
        }
    }

    /// Find the brain pole whose primary variable is most activated in the pressure.
    fn dominant_pole(&self, pressure: &IntentPressure) -> CouncilPole {
        let pole_variable_pairs = [
            (CouncilPole::MaverickLogic, NsqIntentVariable::Force),
            (CouncilPole::QwenCreativity, NsqIntentVariable::Form),
            (CouncilPole::DevstralArbiter, NsqIntentVariable::Truth),
            (CouncilPole::DeepSeekAnalyzer, NsqIntentVariable::Motive),
            (CouncilPole::GemmaLimbic, NsqIntentVariable::Relation),
            (CouncilPole::LlamaContinuity, NsqIntentVariable::Time),
        ];

        pole_variable_pairs
            .iter()
            .max_by_key(|(_, var)| pressure.variable(*var))
            .map(|(pole, _)| *pole)
            .unwrap_or(CouncilPole::DevstralArbiter)
    }

    /// Synthesize multiple pole responses into a unified pressure frame.
    ///
    /// When multiple poles respond, their pressure readings are combined
    /// across all 8 semantic variables. The synthesis is deterministic
    /// and traceable — no hidden state, no string manipulation.
    pub fn synthesize_responses(
        &self,
        job_id: &str,
        responses: Vec<PoleResponse>,
        routing: DispatchRouting,
    ) -> CouncilResponse {
        if responses.is_empty() {
            return CouncilResponse {
                job_id: job_id.to_string(),
                unified_pressure: IntentPressure::baseline(NsqIntentScaleAnchor::SystemWorldScale),
                contributing_poles: vec![],
                authority_pole: None,
                synthesis: SynthesisMethod::SingleAuthority,
                ready_for_egress: false,
            };
        }

        // Find the authority pole if one is marked
        let authority = responses.iter().find(|r| r.is_authority);
        let authority_pole = authority.map(|r| r.pole);

        let (unified_pressure, synthesis) = if let Some(auth) = authority {
            // Single authority — use its pressure directly
            (auth.response_pressure.clone(), SynthesisMethod::SingleAuthority)
        } else {
            // Average the pressure across all responding poles
            let averaged = self.average_pressures(
                responses.iter().map(|r| &r.response_pressure).collect(),
            );
            let synthesis = match routing {
                DispatchRouting::ArbiterFirst => SynthesisMethod::ArbiterVerdict,
                DispatchRouting::AnalyzerFirst => SynthesisMethod::AnalyzerAudit,
                _ => SynthesisMethod::PressureAverage,
            };
            (averaged, synthesis)
        };

        let contributing_poles: Vec<CouncilPole> = responses.iter().map(|r| r.pole).collect();

        CouncilResponse {
            job_id: job_id.to_string(),
            unified_pressure,
            contributing_poles,
            authority_pole,
            synthesis,
            ready_for_egress: true,
        }
    }

    /// Average the pressure readings across multiple poles.
    /// Each variable position is averaged independently.
    /// This is pure gradient arithmetic — no language involved.
    fn average_pressures(&self, pressures: Vec<&IntentPressure>) -> IntentPressure {
        if pressures.is_empty() {
            return IntentPressure::baseline(NsqIntentScaleAnchor::SystemWorldScale);
        }

        let scale_anchor = pressures[0].scale_anchor;
        let mut summed = [0u64; NSQ_INTENT_GRADIENT_VARIABLES];

        for pressure in &pressures {
            for (i, &pos) in pressure.variable_positions.iter().enumerate() {
                summed[i] += pos as u64;
            }
        }

        let count = pressures.len() as u64;
        let mut averaged_positions = [1usize; NSQ_INTENT_GRADIENT_VARIABLES];
        for (i, &sum) in summed.iter().enumerate() {
            averaged_positions[i] = ((sum / count) as usize).max(1);
        }

        IntentPressure {
            variable_positions: averaged_positions,
            scale_anchor,
            origin_pole: None,
            target_pole: None,
            court_surface: IntentSurface::CouncilResponse,
        }
    }

    /// Build a dispatch job from an ingress pressure frame.
    pub fn build_job(&self, job_id: impl Into<String>, ingress_pressure: IntentPressure) -> CouncilDispatchJob {
        let routing = DispatchRouting::from_pressure(&ingress_pressure);
        let addressed_poles = self.address_poles(&ingress_pressure, routing);
        CouncilDispatchJob {
            job_id: job_id.into(),
            ingress_pressure,
            addressed_poles,
            routing,
        }
    }
}

/// Verify the council seating is valid for dispatch.
pub fn verify_council_for_dispatch(seating: &CourtSeating) -> CouncilDispatchVerification {
    let council_ready = seating.council_ready();
    let intent_gradient_valid = seating.validate_intent_gradient().bad_count == 0;
    let total_parameters_b = seating.total_brain_parameters_b();
    let exceeds_one_trillion = total_parameters_b > 1000;

    let vacant_poles: Vec<String> = CouncilPole::BRAIN_POLES
        .iter()
        .filter(|pole| {
            !seating
                .seated_poles
                .iter()
                .any(|s| s.pole == **pole && s.operational)
        })
        .map(|pole| pole.as_str().to_string())
        .collect();

    CouncilDispatchVerification {
        council_ready,
        intent_gradient_valid,
        total_brain_parameters_b: total_parameters_b,
        exceeds_one_trillion,
        vacant_poles,
        dispatch_authorized: council_ready && intent_gradient_valid && exceeds_one_trillion,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouncilDispatchVerification {
    pub council_ready: bool,
    pub intent_gradient_valid: bool,
    pub total_brain_parameters_b: u64,
    pub exceeds_one_trillion: bool,
    pub vacant_poles: Vec<String>,
    pub dispatch_authorized: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use nsq_core::intent::{CourtSeating, IntentPressure};
    use nsq_core::NsqIntentScaleAnchor;

    #[test]
    fn dispatch_routing_derived_from_pressure_not_text() {
        // High force → broadcast
        let mut pressure = IntentPressure::baseline(NsqIntentScaleAnchor::SystemWorldScale);
        pressure.set_variable(NsqIntentVariable::Force, 200_000);
        assert_eq!(DispatchRouting::from_pressure(&pressure), DispatchRouting::Broadcast);

        // Low truth → arbiter first
        let mut pressure2 = IntentPressure::baseline(NsqIntentScaleAnchor::SystemWorldScale);
        pressure2.set_variable(NsqIntentVariable::Truth, 10_000);
        assert_eq!(DispatchRouting::from_pressure(&pressure2), DispatchRouting::ArbiterFirst);
    }

    #[test]
    fn empty_seating_not_authorized_for_dispatch() {
        let seating = CourtSeating::new(vec![]);
        let verification = verify_council_for_dispatch(&seating);
        assert!(!verification.dispatch_authorized);
        assert!(!verification.council_ready);
        assert_eq!(verification.vacant_poles.len(), 6);
    }

    #[test]
    fn total_brain_parameters_floor_exceeds_one_trillion() {
        // Even at floor values: 232+235+123+604+70+70 = 1334B > 1000B
        let floor_total: u64 = CouncilPole::BRAIN_POLES
            .iter()
            .map(|p| p.parameter_floor_b())
            .sum();
        assert!(floor_total > 1000, "{}B must exceed 1 trillion", floor_total);
    }

    #[test]
    fn pressure_average_is_pure_gradient_arithmetic() {
        let seating = CourtSeating::new(vec![]);
        let dispatcher = CouncilDispatcher::new(seating);

        let mut p1 = IntentPressure::baseline(NsqIntentScaleAnchor::SystemWorldScale);
        p1.set_variable(NsqIntentVariable::Force, 100_000);

        let mut p2 = IntentPressure::baseline(NsqIntentScaleAnchor::SystemWorldScale);
        p2.set_variable(NsqIntentVariable::Force, 200_000);

        let averaged = dispatcher.average_pressures(vec![&p1, &p2]);
        let force = averaged.variable(NsqIntentVariable::Force);
        // Should be approximately 150_000
        assert!(force > 100_000 && force < 200_000, "averaged force: {}", force);
    }
}
