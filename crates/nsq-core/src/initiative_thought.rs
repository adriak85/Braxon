use serde::{Deserialize, Serialize};

use crate::initiative_cluster::{Expression, InitiativeCluster};

pub const INITIATIVE_THOUGHT_SCHEMA: &str = "braxon.nsq.initiative_thought.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThoughtDirection {
    Forward,
    Backward,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThoughtExperiment {
    pub schema: String,
    pub trajectory_id: String,
    pub cluster_id: String,
    pub expression_id: String,
    pub direction: ThoughtDirection,
    pub generation: u64,
    pub parameter_overrides: Vec<(String, i64)>,
    pub target_value: Option<i64>,
    pub semantic_links: Vec<String>,
    pub activation_requests: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThoughtExperimentResult {
    pub schema: String,
    pub trajectory_id: String,
    pub cluster_id: String,
    pub expression_id: String,
    pub direction: ThoughtDirection,
    pub generation: u64,
    pub predicted_value: i64,
    pub sensitivities: Vec<(String, i64)>,
    pub required_parameter_deltas: Vec<(String, i64)>,
    pub semantic_links: Vec<String>,
    pub activation_requests: Vec<String>,
}

impl InitiativeCluster {
    pub fn experiment_forward(
        &self,
        trajectory_id: impl Into<String>,
        expression_id: &str,
        overrides: &[(String, i64)],
        activation_requests: Vec<String>,
    ) -> Result<ThoughtExperimentResult, String> {
        let expression = self.expression(expression_id)?;
        let mut value = expression.bias;
        for (parameter_id, coefficient) in &expression.terms {
            let base = self
                .parameters
                .get(parameter_id)
                .ok_or_else(|| format!("unknown parameter: {parameter_id}"))?
                .value;
            let next = overrides
                .iter()
                .find(|(id, _)| id == parameter_id)
                .map(|(_, value)| *value)
                .unwrap_or(base);
            value = value.saturating_add(next.saturating_mul(*coefficient));
        }
        Ok(ThoughtExperimentResult {
            schema: INITIATIVE_THOUGHT_SCHEMA.into(),
            trajectory_id: trajectory_id.into(),
            cluster_id: self.cluster_id.clone(),
            expression_id: expression.id.clone(),
            direction: ThoughtDirection::Forward,
            generation: self.generation,
            predicted_value: value,
            sensitivities: expression.terms.clone(),
            required_parameter_deltas: Vec::new(),
            semantic_links: expression.semantic_links.clone(),
            activation_requests,
        })
    }

    pub fn experiment_backward(
        &self,
        trajectory_id: impl Into<String>,
        expression_id: &str,
        target_value: i64,
        activation_requests: Vec<String>,
    ) -> Result<ThoughtExperimentResult, String> {
        let expression = self.expression(expression_id)?;
        let predicted_value = self.evaluate_expression(expression)?;
        let required = target_value.saturating_sub(predicted_value);
        let sensitivities = expression.terms.clone();
        let required_parameter_deltas = if required == 0 {
            Vec::new()
        } else {
            let pivot = expression
                .terms
                .iter()
                .find(|(_, coefficient)| coefficient.abs() == 1)
                .or_else(|| {
                    expression
                        .terms
                        .iter()
                        .find(|(_, coefficient)| *coefficient != 0)
                })
                .ok_or_else(|| "backward experiment has no invertible term".to_string())?;
            let (parameter_id, coefficient) = pivot;
            if required % coefficient != 0 {
                return Err(
                    "target cannot be satisfied exactly in the integer parameter domain".into(),
                );
            }
            vec![(parameter_id.clone(), required / coefficient)]
        };
        Ok(ThoughtExperimentResult {
            schema: INITIATIVE_THOUGHT_SCHEMA.into(),
            trajectory_id: trajectory_id.into(),
            cluster_id: self.cluster_id.clone(),
            expression_id: expression.id.clone(),
            direction: ThoughtDirection::Backward,
            generation: self.generation,
            predicted_value,
            sensitivities,
            required_parameter_deltas,
            semantic_links: expression.semantic_links.clone(),
            activation_requests,
        })
    }

    fn expression(&self, expression_id: &str) -> Result<&Expression, String> {
        self.expressions
            .get(expression_id)
            .ok_or_else(|| format!("unknown expression: {expression_id}"))
    }

    fn evaluate_expression(&self, expression: &Expression) -> Result<i64, String> {
        let mut value = expression.bias;
        for (parameter_id, coefficient) in &expression.terms {
            let parameter = self
                .parameters
                .get(parameter_id)
                .ok_or_else(|| format!("unknown parameter: {parameter_id}"))?;
            value = value.saturating_add(parameter.value.saturating_mul(*coefficient));
        }
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::initiative_cluster::Expression;

    fn cluster() -> InitiativeCluster {
        let mut cluster = InitiativeCluster::new("cognition.logic").unwrap();
        cluster.add_parameter("signal", 4).unwrap();
        cluster.add_parameter("gain", 3).unwrap();
        cluster
            .add_expression(Expression {
                id: "project".into(),
                initiative_id: "initiative.cognition".into(),
                terms: vec![("signal".into(), 2), ("gain".into(), 1)],
                bias: 1,
                domain: "algebraic.intent".into(),
                constraints: vec!["integer".into(), "deterministic".into()],
                semantic_links: vec!["jit.activate", "reflexor.observe"]
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
                nsq_capability: "initiative.expression.evaluate".into(),
                revision: 0,
            })
            .unwrap();
        cluster
    }

    #[test]
    fn forward_experiment_preserves_hot_cluster_state() {
        let cluster = cluster();
        let result = cluster
            .experiment_forward(
                "t0",
                "project",
                &[("signal".into(), 5)],
                vec!["kv.window".into()],
            )
            .unwrap();
        assert_eq!(result.predicted_value, 14);
        assert_eq!(cluster.parameters["signal"].value, 4);
        assert_eq!(result.activation_requests, vec!["kv.window"]);
    }

    #[test]
    fn backward_experiment_solves_integer_delta() {
        let cluster = cluster();
        let result = cluster
            .experiment_backward("t1", "project", 13, vec!["piston.window".into()])
            .unwrap();
        assert_eq!(result.predicted_value, 12);
        assert_eq!(result.required_parameter_deltas, vec![("gain".into(), 1)]);
    }
}
