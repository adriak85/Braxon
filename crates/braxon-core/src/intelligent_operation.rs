use crate::{
    bootstrap_live_bus, BraxonBus, BraxonBusReport, CouncilSurface, IntentOutcome,
    LiveBusBootstrapReport, NsqIntent, NsqNativeBus, OutputClassification, PistonPhase,
    UnifiedSelfState, NSQ_NATIVE_INTENT_SCHEMA,
};
use nsq_core::{
    Charge, Dialect, NSQLever, NSQSlot, NativeNsqMachine, NativeNsqRuntime, NsqAddress,
    NsqInstruction,
};
use serde::{Deserialize, Serialize};

pub const INTELLIGENT_OPERATION_SCHEMA: &str = "braxon.nsq.intelligent_operation.v1";
pub const OPERATOR_INTELLIGENCE_CAPABILITY: &str = "feature:operator.intelligence";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelligentAction {
    pub capability: String,
    pub selected_intent: String,
    pub state_transition: String,
    pub completed: bool,
    pub classification: OutputClassification,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelligentOperation {
    pub schema: String,
    pub capability: String,
    pub input: String,
    pub semantic_interpretation: String,
    pub selected_intent: String,
    pub action: IntelligentAction,
    pub answer: String,
    pub answer_classification: OutputClassification,
    pub collective_self_state: UnifiedSelfState,
    pub native_transaction_generation: u64,
    pub native_instruction_count: u64,
    pub native_fired_count: u64,
    pub lease_released: bool,
    pub live_bus_bootstrap: LiveBusBootstrapReport,
    pub audit_bus: BraxonBusReport,
}

/// Execute a complete on-demand operator-intelligence turn. The answer is derived
/// from the selected semantic intent and the transaction that was actually applied;
/// the embedded audit data is for verification and is not the normal user-facing
/// output surface.
pub fn execute_operator_intelligence(
    input: impl AsRef<str>,
) -> Result<IntelligentOperation, String> {
    let input = input.as_ref().trim();
    let audit_bus = BraxonBus::speak(input);
    if !audit_bus.processing.input_accepted {
        return Err(exact_bus_blocker(&audit_bus));
    }
    let root = std::env::current_dir().unwrap_or_else(|_| ".".into());
    let live_bus_bootstrap = bootstrap_live_bus(&root, input)
        .map_err(|error| format!("operator intelligence live-bus bootstrap failed: {error}"))?;
    let selected = audit_bus
        .pressure_candidates
        .iter()
        .find(|candidate| candidate.selected)
        .ok_or("operator interpretation produced no selected intent")?;
    let collective_self_state = audit_bus
        .collective_self_state
        .clone()
        .ok_or("operator interpretation produced no collective self-state")?;
    collective_self_state.validate()?;

    let mut lease_bus = NsqNativeBus::new(council_surfaces())?;
    let intent = NsqIntent {
        schema: NSQ_NATIVE_INTENT_SCHEMA.into(),
        intent_id: format!("operator-{}", short_hash(input)),
        source_surface: "operator_intelligence".into(),
        capability: OPERATOR_INTELLIGENCE_CAPABILITY.into(),
        gradient: intent_gradient(selected.priority_score),
        target_addresses: vec!["council/0/operator".into()],
        provenance: "operator_input_tokenizer_verified".into(),
        narrative: false,
    };
    let decision = lease_bus.decide(&intent);
    if decision.outcome != IntentOutcome::Accepted {
        return Err(format!(
            "operator intent could not acquire the NSQ address: {}",
            decision.reason
        ));
    }

    let position = u64::from(selected.nsq_lever_position.max(1));
    let address = NsqAddress::root(slot(position)?);
    let value = slot(position.saturating_add(1).min(500_000))?;
    let mut runtime = NativeNsqRuntime::new(NativeNsqMachine::default());
    let applied = runtime.execute(&[
        NsqInstruction::Set {
            address: address.clone(),
            value,
        },
        NsqInstruction::Fire {
            address: address.clone(),
        },
    ])?;
    lease_bus.advance_piston(&intent.intent_id, PistonPhase::Commit)?;
    let released = runtime.execute(&[NsqInstruction::Release {
        address: address.clone(),
    }])?;
    lease_bus.advance_piston(&intent.intent_id, PistonPhase::Release)?;
    if !lease_bus.active_addresses().is_empty() {
        return Err("operator address lease was not released after execution".into());
    }
    if released.released != 1 {
        return Err("operator native state was not released after execution".into());
    }

    let disagreement = collective_self_state.disagreement_present;
    let answer = format!(
        "I interpreted your request as `{}` and executed the NSQ action `{}` through the operator capability. The transaction completed at generation {} and the address was released. Individual perspectives were retained{}.",
        selected.interpretation,
        selected.intent,
        applied.generation,
        if disagreement { "; disagreement remains visible in the collective state" } else { "" }
    );
    Ok(IntelligentOperation {
        schema: INTELLIGENT_OPERATION_SCHEMA.into(),
        capability: OPERATOR_INTELLIGENCE_CAPABILITY.into(),
        input: input.into(),
        semantic_interpretation: selected.interpretation.clone(),
        selected_intent: selected.intent.clone(),
        action: IntelligentAction {
            capability: OPERATOR_INTELLIGENCE_CAPABILITY.into(),
            selected_intent: selected.intent.clone(),
            state_transition: "live_bus_virtual_address_bootstrap→tokenized_input→semantic_priority→address_lease→native_set_fire→collective_state→release".into(),
            completed: true,
            classification: OutputClassification::DerivedState,
        },
        answer,
        answer_classification: OutputClassification::UserPresentation,
        collective_self_state,
        native_transaction_generation: applied.generation,
        native_instruction_count: u64::try_from(applied.executed.saturating_add(released.executed))
            .map_err(|_| "native instruction count exceeds u64")?,
        native_fired_count: u64::try_from(applied.fired)
            .map_err(|_| "native fired count exceeds u64")?,
        lease_released: true,
        live_bus_bootstrap,
        audit_bus,
    })
}

fn council_surfaces() -> Vec<CouncilSurface> {
    (0..10)
        .map(|index| CouncilSurface {
            surface_id: format!("surface-{index}"),
            role: if index < 6 { "brain" } else { "sensory" }.into(),
            address_prefix: format!("council/{index}/"),
            active: index == 0,
        })
        .collect()
}

fn slot(position: u64) -> Result<NSQSlot, String> {
    let lever = NSQLever::new(Charge::Positive, position)?;
    Ok(NSQSlot::new(Dialect::Intent, vec![lever]))
}

fn intent_gradient(priority_score: f32) -> [f64; 8] {
    let score = f64::from(priority_score.clamp(-1.0, 1.0));
    [score, 1.0, 1.0, score.abs(), 1.0, 0.0, 1.0, 1.0]
}

fn exact_bus_blocker(report: &BraxonBusReport) -> String {
    match report.status.as_str() {
        "input_rejected_empty_operator_request" => "operator intelligence requires nonempty input".into(),
        "tokenizer_bridge_unavailable" => {
            "operator intelligence is blocked because the active tokenizer bridge is unavailable; connect assets/braxon_core/tokenizer/braxon_unified_tokenizer.json".into()
        }
        "tokenizer_bridge_unresolved_or_invalid" => {
            let unresolved = report
                .tokenizer_bridge
                .as_ref()
                .map(|receipt| receipt.unresolved_tokens.join(","))
                .unwrap_or_else(|| "unknown".into());
            format!("operator intelligence is blocked by unresolved tokenizer input: {unresolved}; extend the active tokenizer mapping before retry")
        }
        other => format!("operator intelligence is blocked before semantic execution: {other}"),
    }
}

fn short_hash(input: &str) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in input.bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operator_intelligence_executes_a_tokenized_semantic_transaction_then_releases_the_address() {
        let operation =
            execute_operator_intelligence("verify terminal launch path through the operator bus")
                .unwrap();
        assert_eq!(operation.capability, OPERATOR_INTELLIGENCE_CAPABILITY);
        assert!(operation.action.completed);
        assert_eq!(operation.native_fired_count, 1);
        assert_eq!(operation.native_instruction_count, 3);
        assert!(operation.lease_released);
        assert_eq!(
            operation.answer_classification,
            OutputClassification::UserPresentation
        );
        assert!(operation.answer.contains("executed the NSQ action"));
        assert!(operation.collective_self_state.validate().is_ok());
    }

    #[test]
    fn operator_intelligence_reports_exact_missing_token_connection_without_a_fake_answer() {
        let error = execute_operator_intelligence("truth🙂").unwrap_err();
        assert!(error.contains("unresolved tokenizer input"));
        assert!(!error.contains("completed"));
    }
}
