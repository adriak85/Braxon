use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{execute_dynamic_parameter_pipeline, DynamicPipelineReceipt};

pub const RIEMANN_REFLEXOR_SCHEMA: &str = "braxon.riemann.predictive_reflexor.v1";

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

    pub fn execute_prediction(
        &mut self,
        record_id: &str,
    ) -> Result<DynamicPipelineReceipt, String> {
        let hypothesis = self
            .hypotheses
            .get(record_id)
            .ok_or_else(|| format!("unknown Riemann hypothesis region: {record_id}"))?
            .clone();
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
