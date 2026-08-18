use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{execute_dynamic_parameter_pipeline, DynamicPipelineReceipt};

pub const RIEMANN_REFLEXOR_SCHEMA: &str = "braxon.riemann.predictive_reflexor.v1";
pub const MAX_JIT_ACTIVATION_BYTES: u64 = 15 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivationRequest {
    pub record_id: String,
    pub required_parameters: Vec<String>,
    pub required_capabilities: Vec<String>,
    pub provenance: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivationReceipt {
    pub request: ActivationRequest,
    pub activated_parameters: Vec<String>,
    pub activated_capabilities: Vec<String>,
    pub bounded_window_bytes: u64,
    pub retry_permitted: bool,
    pub authorized: bool,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProofState {
    Unresolved,
    Probable,
    Certified,
    IndependentlyReproduced,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerminalState {
    Running,
    Won,
    LoopDetected,
    ResourceBound,
    AuthorityBlocked,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProofAttempt {
    pub record_id: String,
    pub state: ProofState,
    pub primary_source: String,
    pub independent_source: String,
    pub primary_certificate: bool,
    pub independent_certificate: bool,
    pub residual_nano: u64,
    pub activation: ActivationReceipt,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningRecord {
    pub signature: String,
    pub record_id: String,
    pub proof_state: ProofState,
    pub residual_nano: u64,
    pub priority_delta: i64,
    pub trusted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZeroRegionHypothesis {
    pub record_id: String,
    pub sigma_milli: i64,
    pub t_start_milli: i64,
    pub t_end_milli: i64,
    pub priority: u64,
    pub generation: u64,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZeroObservation {
    pub record_id: String,
    pub minimum_residual_nano: u64,
    pub certified: bool,
    pub observed_sigma_milli: i64,
    pub observed_t_milli: i64,
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReflexorSearchStep {
    pub hypothesis: ZeroRegionHypothesis,
    pub execution: DynamicPipelineReceipt,
    pub observation: Option<ZeroObservation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RiemannSemanticReflexor {
    pub schema: String,
    pub generation: u64,
    pub hypotheses: BTreeMap<String, ZeroRegionHypothesis>,
    pub observations: BTreeMap<String, ZeroObservation>,
    pub active_parameters: BTreeMap<String, u64>,
    pub active_capabilities: BTreeMap<String, u64>,
    pub activation_receipts: Vec<ActivationReceipt>,
    pub proof_attempts: Vec<ProofAttempt>,
    pub learning_records: Vec<LearningRecord>,
    pub visited_signatures: BTreeMap<String, u64>,
    pub terminal_state: TerminalState,
    pub attempt_budget: u64,
    pub attempts_used: u64,
    pub steps: Vec<ReflexorSearchStep>,
}

impl RiemannSemanticReflexor {
    pub fn seed(start_t_milli: i64, count: usize) -> Result<Self, String> {
        if count == 0 || start_t_milli <= 0 {
            return Err("Riemann Reflexor seed requires positive start and count".into());
        }
        let mut hypotheses = BTreeMap::new();
        for index in 0..count {
            let center = start_t_milli.saturating_add(index as i64 * 100);
            let record_id = format!("zeta-region-{index:04}");
            hypotheses.insert(
                record_id.clone(),
                ZeroRegionHypothesis {
                    record_id,
                    sigma_milli: 500,
                    t_start_milli: center.saturating_sub(50),
                    t_end_milli: center.saturating_add(50),
                    priority: 1_000_000_u64.saturating_sub(index as u64),
                    generation: 0,
                    status: "predicted".into(),
                },
            );
        }
        Ok(Self {
            schema: RIEMANN_REFLEXOR_SCHEMA.into(),
            generation: 0,
            hypotheses,
            observations: BTreeMap::new(),
            active_parameters: BTreeMap::new(),
            active_capabilities: BTreeMap::new(),
            activation_receipts: Vec::new(),
            proof_attempts: Vec::new(),
            learning_records: Vec::new(),
            visited_signatures: BTreeMap::new(),
            terminal_state: TerminalState::Running,
            attempt_budget: u64::MAX,
            attempts_used: 0,
            steps: Vec::new(),
        })
    }

    pub fn predict_next(&self) -> Option<&ZeroRegionHypothesis> {
        self.hypotheses
            .values()
            .filter(|hypothesis| !self.observations.contains_key(&hypothesis.record_id))
            .max_by_key(|hypothesis| {
                (
                    hypothesis.priority,
                    std::cmp::Reverse(hypothesis.record_id.clone()),
                )
            })
    }

    pub fn activate_just_in_time(
        &mut self,
        request: ActivationRequest,
    ) -> Result<ActivationReceipt, String> {
        if request.record_id.trim().is_empty() || request.provenance.trim().is_empty() {
            return Err("JIT activation requires record_id and provenance".into());
        }
        if request.required_parameters.is_empty() || request.required_capabilities.is_empty() {
            return Err("JIT activation requires parameters and capabilities".into());
        }
        if request
            .required_parameters
            .iter()
            .any(|parameter| parameter != "height" && parameter != "sigma")
        {
            return Err("JIT activation rejected an unapproved parameter".into());
        }
        if request
            .required_capabilities
            .iter()
            .any(|capability| !capability.starts_with("zeta."))
        {
            return Err("JIT activation rejected a capability outside the zeta authority".into());
        }
        let bounded_window_bytes =
            (request.required_parameters.len() + request.required_capabilities.len()) as u64 * 8;
        if bounded_window_bytes > MAX_JIT_ACTIVATION_BYTES {
            return Err("JIT activation exceeds the bounded firing window".into());
        }
        let mut activated_parameters = Vec::new();
        for parameter in &request.required_parameters {
            if !self.active_parameters.contains_key(parameter) {
                self.active_parameters
                    .insert(parameter.clone(), self.generation);
                activated_parameters.push(parameter.clone());
            }
        }
        let mut activated_capabilities = Vec::new();
        for capability in &request.required_capabilities {
            if !self.active_capabilities.contains_key(capability) {
                self.active_capabilities
                    .insert(capability.clone(), self.generation);
                activated_capabilities.push(capability.clone());
            }
        }
        let receipt = ActivationReceipt {
            request,
            activated_parameters,
            activated_capabilities,
            bounded_window_bytes,
            retry_permitted: true,
            authorized: true,
            reason: "required semantic parameters and capabilities activated for one bounded retry"
                .into(),
        };
        self.activation_receipts.push(receipt.clone());
        Ok(receipt)
    }

    pub fn execute_prediction(
        &mut self,
        record_id: &str,
    ) -> Result<DynamicPipelineReceipt, String> {
        let hypothesis = self
            .hypotheses
            .get(record_id)
            .ok_or_else(|| format!("unknown Riemann hypothesis region: {record_id}"))?
            .clone();
        if !self.active_parameters.contains_key("height")
            || !self.active_parameters.contains_key("sigma")
            || !self.active_capabilities.contains_key("zeta.evaluate")
        {
            self.activate_just_in_time(ActivationRequest {
                record_id: record_id.into(),
                required_parameters: vec!["height".into(), "sigma".into()],
                required_capabilities: vec!["zeta.evaluate".into()],
                provenance: "predictive-semantic-reflexor".into(),
            })?;
        }
        let output = format!(
            "semantic_identity=riemann-zero-region\nintent=probe\nconfidence_bps=1000\nparameter.height={}\nparameter.sigma={}\nexpression.zeta_residual.terms=height:1,sigma:1",
            hypothesis.t_start_milli,
            hypothesis.sigma_milli
        );
        let receipt = execute_dynamic_parameter_pipeline(
            &output,
            "predictive-semantic-reflexor",
            format!("riemann-{record_id}"),
            [("height".into(), hypothesis.t_start_milli)],
            [("height".into(), hypothesis.t_start_milli.saturating_add(1))],
        )?;
        self.generation = self.generation.saturating_add(1);
        Ok(receipt)
    }

    pub fn begin_run(&mut self, attempt_budget: u64) -> Result<(), String> {
        if attempt_budget == 0 {
            return Err("self-directed run requires a positive attempt budget".into());
        }
        self.attempt_budget = attempt_budget;
        self.attempts_used = 0;
        self.terminal_state = TerminalState::Running;
        Ok(())
    }

    pub fn terminal_state(&self) -> &TerminalState {
        &self.terminal_state
    }

    pub fn self_prove(
        &mut self,
        record_id: &str,
        residual_nano: u64,
        primary_source: impl Into<String>,
        independent_source: impl Into<String>,
        primary_certificate: bool,
        independent_certificate: bool,
    ) -> Result<ProofAttempt, String> {
        let primary_source = primary_source.into();
        let independent_source = independent_source.into();
        if primary_source.trim().is_empty() || independent_source.trim().is_empty() {
            return Err("self-proof requires primary and independent sources".into());
        }
        let activation = self.activate_just_in_time(ActivationRequest {
            record_id: record_id.into(),
            required_parameters: vec!["height".into(), "sigma".into()],
            required_capabilities: vec![
                "zeta.lucas_lehmer".into(),
                "zeta.independent_verify".into(),
            ],
            provenance: "self-proof-initiative".into(),
        })?;
        let state = if primary_certificate
            && independent_certificate
            && primary_source != independent_source
        {
            ProofState::IndependentlyReproduced
        } else if residual_nano <= 10_000 {
            ProofState::Probable
        } else {
            ProofState::Unresolved
        };
        let reason = match state {
            ProofState::IndependentlyReproduced => {
                "two distinct proof-grade sources supplied certificates".into()
            }
            ProofState::Probable => {
                "small residual observed but complete independent proof evidence is absent".into()
            }
            _ => "self-proof attempt did not establish a certificate".into(),
        };
        let attempt = ProofAttempt {
            record_id: record_id.into(),
            state,
            primary_source,
            independent_source,
            primary_certificate,
            independent_certificate,
            residual_nano,
            activation,
            reason,
        };
        self.proof_attempts.push(attempt.clone());
        Ok(attempt)
    }

    pub fn learn_from_proof_attempt(
        &mut self,
        attempt: &ProofAttempt,
    ) -> Result<TerminalState, String> {
        if !self.hypotheses.contains_key(&attempt.record_id) {
            return Err(format!(
                "learning references unknown region: {}",
                attempt.record_id
            ));
        }
        if !matches!(self.terminal_state, TerminalState::Running) {
            return Ok(self.terminal_state.clone());
        }
        let signature = format!(
            "{}:{:?}:{}",
            attempt.record_id, attempt.state, attempt.residual_nano
        );
        if self.visited_signatures.contains_key(&signature) {
            self.terminal_state = TerminalState::LoopDetected;
            return Ok(self.terminal_state.clone());
        }
        self.attempts_used = self.attempts_used.saturating_add(1);
        self.visited_signatures
            .insert(signature.clone(), self.attempts_used);
        let (priority_delta, trusted) = match attempt.state {
            ProofState::IndependentlyReproduced => (i64::MIN, true),
            ProofState::Certified => (-1_000, true),
            ProofState::Probable => (250, false),
            ProofState::Blocked => (0, false),
            ProofState::Unresolved => (500, false),
        };
        if let Some(hypothesis) = self.hypotheses.get_mut(&attempt.record_id) {
            if priority_delta == i64::MIN {
                hypothesis.priority = 0;
            } else if priority_delta >= 0 {
                hypothesis.priority = hypothesis.priority.saturating_add(priority_delta as u64);
            } else {
                hypothesis.priority = hypothesis
                    .priority
                    .saturating_sub(priority_delta.unsigned_abs());
            }
        }
        self.learning_records.push(LearningRecord {
            signature,
            record_id: attempt.record_id.clone(),
            proof_state: attempt.state.clone(),
            residual_nano: attempt.residual_nano,
            priority_delta,
            trusted,
        });
        self.terminal_state = match attempt.state {
            ProofState::IndependentlyReproduced => TerminalState::Won,
            ProofState::Blocked => TerminalState::AuthorityBlocked,
            _ if self.attempts_used >= self.attempt_budget => TerminalState::ResourceBound,
            _ => TerminalState::Running,
        };
        Ok(self.terminal_state.clone())
    }

    pub fn reconcile_observation(
        &mut self,
        record_id: &str,
        observation: ZeroObservation,
        execution: DynamicPipelineReceipt,
    ) -> Result<(), String> {
        if observation.record_id != record_id {
            return Err("observation record_id does not match predicted region".into());
        }
        let hypothesis = self
            .hypotheses
            .get_mut(record_id)
            .ok_or_else(|| format!("unknown Riemann hypothesis region: {record_id}"))?;
        hypothesis.generation = self.generation;
        hypothesis.status = if observation.certified {
            "certified-observation".into()
        } else {
            "observed-unresolved".into()
        };
        hypothesis.priority = if observation.certified {
            0
        } else {
            hypothesis.priority.saturating_add(1)
        };
        self.observations
            .insert(record_id.into(), observation.clone());
        self.steps.push(ReflexorSearchStep {
            hypothesis: hypothesis.clone(),
            execution,
            observation: Some(observation),
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn predictive_reflexor_routes_hypothesis_then_observation() {
        let mut reflexor = RiemannSemanticReflexor::seed(14_134, 3).unwrap();
        let next = reflexor.predict_next().unwrap().record_id.clone();
        let execution = reflexor.execute_prediction(&next).unwrap();
        let observation = ZeroObservation {
            record_id: next.clone(),
            minimum_residual_nano: 7,
            certified: false,
            observed_sigma_milli: 500,
            observed_t_milli: 14_134,
            source: "bounded-mpmath-probe".into(),
        };
        reflexor
            .reconcile_observation(&next, observation, execution)
            .unwrap();
        assert_eq!(reflexor.observations.len(), 1);
        assert_eq!(reflexor.steps[0].execution.peak_resident_bytes, 0);
        assert_eq!(reflexor.hypotheses[&next].status, "observed-unresolved");
    }

    #[test]
    fn inactive_requirements_are_activated_before_retry() {
        let mut reflexor = RiemannSemanticReflexor::seed(14_134, 1).unwrap();
        let id = reflexor.predict_next().unwrap().record_id.clone();
        let receipt = reflexor.execute_prediction(&id).unwrap();
        assert_eq!(receipt.peak_resident_bytes, 0);
        assert_eq!(reflexor.activation_receipts.len(), 1);
        assert!(reflexor.active_parameters.contains_key("height"));
        assert!(reflexor.active_capabilities.contains_key("zeta.evaluate"));
        assert!(reflexor.activation_receipts[0].retry_permitted);
    }

    #[test]
    fn unauthorized_capability_fails_closed() {
        let mut reflexor = RiemannSemanticReflexor::seed(14_134, 1).unwrap();
        let result = reflexor.activate_just_in_time(ActivationRequest {
            record_id: "zeta-region-0000".into(),
            required_parameters: vec!["height".into()],
            required_capabilities: vec!["shell.execute".into()],
            provenance: "test".into(),
        });
        assert!(result.is_err());
        assert!(reflexor.activation_receipts.is_empty());
    }

    #[test]
    fn self_directed_learning_stops_on_repeat_or_win() {
        let mut reflexor = RiemannSemanticReflexor::seed(14_134, 1).unwrap();
        reflexor.begin_run(3).unwrap();
        let id = reflexor.predict_next().unwrap().record_id.clone();
        let probable = reflexor
            .self_prove(&id, 7, "primary", "independent", false, false)
            .unwrap();
        assert_eq!(
            reflexor.learn_from_proof_attempt(&probable).unwrap(),
            TerminalState::Running
        );
        assert_eq!(
            reflexor.learn_from_proof_attempt(&probable).unwrap(),
            TerminalState::LoopDetected
        );
        assert_eq!(reflexor.learning_records.len(), 1);

        let mut winner = RiemannSemanticReflexor::seed(14_134, 1).unwrap();
        winner.begin_run(3).unwrap();
        let id = winner.predict_next().unwrap().record_id.clone();
        let proof = winner
            .self_prove(&id, 0, "primary", "independent", true, true)
            .unwrap();
        assert_eq!(
            winner.learn_from_proof_attempt(&proof).unwrap(),
            TerminalState::Won
        );
        assert_eq!(*winner.terminal_state(), TerminalState::Won);
    }

    #[test]
    fn prediction_cannot_be_promoted_to_certified_proof() {
        let mut reflexor = RiemannSemanticReflexor::seed(14_134, 1).unwrap();
        let id = reflexor.predict_next().unwrap().record_id.clone();
        let execution = reflexor.execute_prediction(&id).unwrap();
        reflexor
            .reconcile_observation(
                &id,
                ZeroObservation {
                    record_id: id.clone(),
                    minimum_residual_nano: 0,
                    certified: false,
                    observed_sigma_milli: 500,
                    observed_t_milli: 14_134,
                    source: "un-certified-prediction".into(),
                },
                execution,
            )
            .unwrap();
        assert_ne!(reflexor.hypotheses[&id].status, "certified-observation");
    }
}
