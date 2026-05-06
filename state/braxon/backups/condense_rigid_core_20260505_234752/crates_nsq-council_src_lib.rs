//! NSQ Council — Six-Pole Intent Gradient Dispatch
//!
//! The sovereign agentic dispatch loop for the Council of Six.
//! This crate owns the inter-pole pressure routing. It is NOT a helper crate.
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
//!   - Treat any pole as a feature attached to a host
//!   - Allow human language inside the dispatch loop

use nsq_core::intent::{
    CouncilPole, CourtSeating, IntentPressure, IntentSurface, SeatedPole,
    NsqIntentVariable, NsqIntentScaleAnchor, NSQ_INTENT_GRADIENT_VARIABLES,
    NsqFinalLeverPosition,
};
use serde::{Deserialize, Serialize};

/// A dispatch job — a unit of work routed through the council.
/// Input is always IntentPressure. Output is always IntentPressure.
/// Human text appears only at ingress (job creation) and egress (resolution).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouncilDispatchJob {
    pub job_id: String,
    /// Intent pressure from the court ingress surface.
    /// Was human text. Has been translated. Is now pressure.
    pub ingress_pressure: IntentPressure,
    /// Which poles this job will be dispatched to (derived from gradient).
    pub addressed_poles: Vec<CouncilPole>,
    pub routing: DispatchRouting,
}

/// How the council routes a dispatch job across its poles.
/// Routing is derived from gradient variables — never from text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DispatchRouting {
    /// All six brain poles receive and respond. Synthesized into one response.
    Broadcast,
    /// Routes to the pole whose primary variable is most activated.
    PrimaryPole,
    /// Routes to Devstral Arbiter for truth arbitration first.
    ArbiterFirst,
    /// Routes to DeepSeek Analyzer for deep audit first.
    AnalyzerFirst,
    /// Full council round-table — all poles respond, Arbiter synthesizes.
    RoundTable,
}

impl DispatchRouting {
    /// Derive routing from the pressure frame — not from human text.
    pub fn from_pressure(pressure: &IntentPressure) -> Self {
        let truth = pressure.variable(NsqIntentVariable::Truth);
        let motive = pressure.variable(NsqIntentVariable::Motive);
        let force = pressure.variable(NsqIntentVariable::Force);

        // Low truth position = concealment/uncertainty → Arbiter first
        if truth < 200 {
            return Self::ArbiterFirst;
        }
        // Low motive position = destructive/exploitative → Analyzer first
        if motive < 150 {
            return Self::AnalyzerFirst;
        }
        // High force = command/overwhelming → broadcast all poles
        if force > 900 {
            return Self::Broadcast;
        }
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
/// No strings. No tokens. Gradient pressure only.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoleResponse {
    pub pole: CouncilPole,
    pub response_pressure: IntentPressure,
    pub escalated_to: Option<CouncilPole>,
    pub is_authority: bool,
}

/// The synthesized council response — all pole responses unified.
/// This is not a text response. It is a pressure frame.
/// It becomes text only at the egress boundary, after synthesis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouncilResponse {
    pub job_id: String,
    pub unified_pressure: IntentPressure,
    pub contributing_poles: Vec<CouncilPole>,
    pub authority_pole: Option<CouncilPole>,
    pub synthesis: SynthesisMethod,
    pub ready_for_egress: bool,
}

/// How the council synthesized multiple pole responses into one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SynthesisMethod {
    SingleAuthority,
    PressureAverage,
    ArbiterVerdict,
    AnalyzerAudit,
}

/// The council dispatcher — the sovereign agentic loop.
/// Routes IntentPressure between poles. Synthesizes responses.
/// Does not pass strings. Does not call a tokenizer.
pub struct CouncilDispatcher {
    pub seating: CourtSeating,
}

impl CouncilDispatcher {
    pub fn new(seating: CourtSeating) -> Self {
        Self { seating }
    }

    /// Determine which poles to address for a given routing strategy.
    pub fn address_poles(
        &self,
        pressure: &IntentPressure,
        routing: DispatchRouting,
    ) -> Vec<CouncilPole> {
        match routing {
            DispatchRouting::Broadcast | DispatchRouting::RoundTable => {
                CouncilPole::BRAIN_POLES.to_vec()
            }
            DispatchRouting::ArbiterFirst => vec![CouncilPole::DevstralArbiter],
            DispatchRouting::AnalyzerFirst => vec![CouncilPole::DeepSeekAnalyzer],
            DispatchRouting::PrimaryPole => vec![self.dominant_pole(pressure)],
        }
    }

    /// Find the brain pole whose primary variable is most activated.
    fn dominant_pole(&self, pressure: &IntentPressure) -> CouncilPole {
        let pairs = [
            (CouncilPole::MaverickLogic, NsqIntentVariable::Force),
            (CouncilPole::QwenCreativity, NsqIntentVariable::Form),
            (CouncilPole::DevstralArbiter, NsqIntentVariable::Truth),
            (CouncilPole::DeepSeekAnalyzer, NsqIntentVariable::Motive),
            (CouncilPole::GemmaLimbic, NsqIntentVariable::Relation),
            (CouncilPole::LlamaContinuity, NsqIntentVariable::Time),
        ];
        pairs
            .iter()
            .max_by_key(|(_, var)| pressure.variable(*var))
            .map(|(pole, _)| *pole)
            .unwrap_or(CouncilPole::DevstralArbiter)
    }

    /// Synthesize multiple pole responses into a unified pressure frame.
    /// Pure gradient arithmetic — no string manipulation, no hidden state.
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

        let authority = responses.iter().find(|r| r.is_authority);
        let authority_pole = authority.map(|r| r.pole);

        let (unified_pressure, synthesis) = if let Some(auth) = authority {
            (auth.response_pressure.clone(), SynthesisMethod::SingleAuthority)
        } else {
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

        let contributing_poles = responses.iter().map(|r| r.pole).collect();

        CouncilResponse {
            job_id: job_id.to_string(),
            unified_pressure,
            contributing_poles,
            authority_pole,
            synthesis,
            ready_for_egress: true,
        }
    }

    /// Average pressure readings across multiple poles.
    /// Each variable position is averaged independently.
    /// Pure gradient arithmetic — no language involved.
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
        let mut averaged = [1u16; NSQ_INTENT_GRADIENT_VARIABLES];
        for (i, &sum) in summed.iter().enumerate() {
            averaged[i] = ((sum / count) as NsqFinalLeverPosition).max(1);
        }
        IntentPressure {
            variable_positions: averaged,
            scale_anchor,
            origin_pole: None,
            target_pole: None,
            court_surface: IntentSurface::CouncilResponse,
        }
    }

    /// Build a dispatch job from an ingress pressure frame.
    pub fn build_job(
        &self,
        job_id: impl Into<String>,
        ingress_pressure: IntentPressure,
    ) -> CouncilDispatchJob {
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
        .map(|p| p.as_str().to_string())
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

    #[test]
    fn dispatch_routing_from_pressure_not_text() {
        let mut pressure = IntentPressure::baseline(NsqIntentScaleAnchor::SystemWorldScale);
        pressure.set_variable(NsqIntentVariable::Force, 1000);
        assert_eq!(DispatchRouting::from_pressure(&pressure), DispatchRouting::Broadcast);

        let mut p2 = IntentPressure::baseline(NsqIntentScaleAnchor::SystemWorldScale);
        p2.set_variable(NsqIntentVariable::Truth, 50);
        assert_eq!(DispatchRouting::from_pressure(&p2), DispatchRouting::ArbiterFirst);
    }

    #[test]
    fn empty_seating_not_authorized_for_dispatch() {
        let seating = CourtSeating::new(vec![]);
        let v = verify_council_for_dispatch(&seating);
        assert!(!v.dispatch_authorized);
        assert!(!v.council_ready);
        assert_eq!(v.vacant_poles.len(), 6);
    }

    #[test]
    fn brain_parameter_floor_exceeds_one_trillion() {
        let total: u64 = CouncilPole::BRAIN_POLES
            .iter()
            .map(|p| p.parameter_floor_b())
            .sum();
        assert!(total > 1000, "{}B must exceed 1 trillion", total);
    }

    #[test]
    fn pressure_average_is_pure_gradient_arithmetic() {
        let seating = CourtSeating::new(vec![]);
        let dispatcher = CouncilDispatcher::new(seating);

        let mut p1 = IntentPressure::baseline(NsqIntentScaleAnchor::SystemWorldScale);
        p1.set_variable(NsqIntentVariable::Force, 200);
        let mut p2 = IntentPressure::baseline(NsqIntentScaleAnchor::SystemWorldScale);
        p2.set_variable(NsqIntentVariable::Force, 600);

        let averaged = dispatcher.average_pressures(vec![&p1, &p2]);
        let force = averaged.variable(NsqIntentVariable::Force);
        assert!(force > 200 && force < 600, "averaged force: {}", force);
    }
}
