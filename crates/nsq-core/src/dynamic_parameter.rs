use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{Expression, InitiativeCluster};

pub const DYNAMIC_PARAMETER_SCHEMA: &str = "braxon.nsq.dynamic_parameter.v1";
pub const CANDIDATE_INTENT_SCHEMA: &str = "braxon.nsq.candidate_intent.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CandidateIntent {
    pub schema: String,
    pub semantic_identity: String,
    pub intent: String,
    pub fields: BTreeMap<String, String>,
    pub confidence_bps: u16,
    pub provenance: String,
    pub raw_output_bytes: u64,
}

impl CandidateIntent {
    pub fn extract(model_output: &str, provenance: impl Into<String>) -> Result<Self, String> {
        let mut fields = BTreeMap::new();
        for raw_line in model_output.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let (key, value) = line
                .split_once('=')
                .ok_or_else(|| format!("candidate intent line is not key=value: {line}"))?;
            let key = key.trim();
            let value = value.trim();
            if key.is_empty() || value.is_empty() {
                return Err("candidate intent keys and values must be nonempty".into());
            }
            if fields.insert(key.to_string(), value.to_string()).is_some() {
                return Err(format!("duplicate candidate intent field: {key}"));
            }
        }
        let semantic_identity = fields
            .get("semantic_identity")
            .cloned()
            .ok_or_else(|| "candidate intent requires semantic_identity".to_string())?;
        let intent = fields
            .get("intent")
            .cloned()
            .ok_or_else(|| "candidate intent requires intent".to_string())?;
        let confidence_bps = fields
            .get("confidence_bps")
            .map(|value| {
                value
                    .parse::<u16>()
                    .map_err(|_| "confidence_bps must be an integer")
            })
            .transpose()?
            .unwrap_or(0);
        if confidence_bps > 10_000 {
            return Err("confidence_bps must be in 0..=10000".into());
        }
        let provenance = provenance.into();
        if provenance.trim().is_empty() {
            return Err("candidate intent provenance is required".into());
        }
        Ok(Self {
            schema: CANDIDATE_INTENT_SCHEMA.into(),
            semantic_identity,
            intent,
            fields,
            confidence_bps,
            provenance,
            raw_output_bytes: model_output.len() as u64,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DynamicParameter {
    pub id: String,
    pub value: i64,
    pub type_domain: String,
    pub confidence_bps: u16,
    pub provenance: String,
    pub generation: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DynamicExpression {
    pub id: String,
    pub terms: Vec<(String, i64)>,
    pub bias: i64,
    pub domain: String,
    pub constraints: Vec<String>,
    pub semantic_links: Vec<String>,
    pub nsq_capability: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReconciliationState {
    Candidate,
    Canonical,
    Predicted,
    Observed,
    Corrected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DynamicParameterSet {
    pub schema: String,
    pub semantic_identity: String,
    pub intent: String,
    pub parameters: BTreeMap<String, DynamicParameter>,
    pub dependencies: BTreeMap<String, Vec<String>>,
    pub expressions: BTreeMap<String, DynamicExpression>,
    pub constraints: Vec<String>,
    pub predicted_next_parameters: BTreeMap<String, i64>,
    pub reconciliation_state: ReconciliationState,
    pub generation: u64,
    pub source_provenance: String,
    pub model_output_bytes: u64,
    pub extracted_intent_bytes: u64,
}

impl DynamicParameterSet {
    pub fn canonicalize(candidate: CandidateIntent) -> Result<Self, String> {
        if candidate.schema != CANDIDATE_INTENT_SCHEMA {
            return Err("candidate intent schema mismatch".into());
        }
        let extracted_intent_bytes = serde_json::to_vec(&candidate)
            .map_err(|error| error.to_string())?
            .len() as u64;
        let mut parameters = BTreeMap::new();
        for (key, value) in &candidate.fields {
            let Some(id) = key.strip_prefix("parameter.") else {
                continue;
            };
            if id.trim().is_empty() {
                return Err("parameter id cannot be empty".into());
            }
            let value = value
                .parse::<i64>()
                .map_err(|_| format!("parameter {id} must have an integer value"))?;
            parameters.insert(
                id.to_string(),
                DynamicParameter {
                    id: id.to_string(),
                    value,
                    type_domain: candidate
                        .fields
                        .get(&format!("domain.{id}"))
                        .cloned()
                        .unwrap_or_else(|| "integer".into()),
                    confidence_bps: candidate.confidence_bps,
                    provenance: candidate.provenance.clone(),
                    generation: 0,
                },
            );
        }
        if parameters.is_empty() {
            return Err("candidate intent must provide at least one parameter.* field".into());
        }

        let mut expressions = BTreeMap::new();
        for (key, terms_text) in &candidate.fields {
            let Some(id) = key
                .strip_prefix("expression.")
                .and_then(|rest| rest.strip_suffix(".terms"))
            else {
                continue;
            };
            let mut terms = Vec::new();
            for term in terms_text.split(',') {
                let (parameter_id, coefficient) = term
                    .trim()
                    .split_once(':')
                    .ok_or_else(|| format!("expression {id} has malformed term: {term}"))?;
                if !parameters.contains_key(parameter_id) {
                    return Err(format!(
                        "expression {id} references unknown parameter: {parameter_id}"
                    ));
                }
                terms.push((
                    parameter_id.to_string(),
                    coefficient
                        .parse::<i64>()
                        .map_err(|_| format!("expression {id} coefficient must be an integer"))?,
                ));
            }
            if terms.is_empty() {
                return Err(format!("expression {id} requires at least one term"));
            }
            let bias = candidate
                .fields
                .get(&format!("expression.{id}.bias"))
                .map(|value| {
                    value
                        .parse::<i64>()
                        .map_err(|_| format!("expression {id} bias must be an integer"))
                })
                .transpose()?
                .unwrap_or(0);
            expressions.insert(
                id.to_string(),
                DynamicExpression {
                    id: id.to_string(),
                    terms,
                    bias,
                    domain: candidate
                        .fields
                        .get(&format!("expression.{id}.domain"))
                        .cloned()
                        .unwrap_or_else(|| "integer".into()),
                    constraints: vec!["canonical-nsq".into(), "fail-closed".into()],
                    semantic_links: vec![candidate.semantic_identity.clone()],
                    nsq_capability: format!("intent.{}.evaluate", candidate.intent),
                },
            );
        }
        if expressions.is_empty() {
            let terms = parameters.keys().map(|id| (id.clone(), 1)).collect();
            expressions.insert(
                "intent.result".into(),
                DynamicExpression {
                    id: "intent.result".into(),
                    terms,
                    bias: 0,
                    domain: "integer".into(),
                    constraints: vec!["canonical-nsq".into(), "fail-closed".into()],
                    semantic_links: vec![candidate.semantic_identity.clone()],
                    nsq_capability: format!("intent.{}.evaluate", candidate.intent),
                },
            );
        }
        let mut dependencies = BTreeMap::new();
        for expression in expressions.values() {
            for (parameter_id, _) in &expression.terms {
                dependencies
                    .entry(parameter_id.clone())
                    .or_insert_with(Vec::new)
                    .push(expression.id.clone());
            }
        }
        Ok(Self {
            schema: DYNAMIC_PARAMETER_SCHEMA.into(),
            semantic_identity: candidate.semantic_identity,
            intent: candidate.intent,
            parameters,
            dependencies,
            expressions,
            constraints: vec![
                "model-is-non-authoritative".into(),
                "nsq-owns-canonical-state".into(),
                "logical-space-open-ended".into(),
            ],
            predicted_next_parameters: BTreeMap::new(),
            reconciliation_state: ReconciliationState::Canonical,
            generation: 0,
            source_provenance: candidate.provenance,
            model_output_bytes: candidate.raw_output_bytes,
            extracted_intent_bytes,
        })
    }

    pub fn to_initiative_cluster(
        &self,
        cluster_id: impl Into<String>,
    ) -> Result<InitiativeCluster, String> {
        if self.schema != DYNAMIC_PARAMETER_SCHEMA {
            return Err("dynamic parameter schema mismatch".into());
        }
        let mut cluster = InitiativeCluster::new(cluster_id)?;
        for parameter in self.parameters.values() {
            cluster.add_parameter(parameter.id.clone(), parameter.value)?;
        }
        for expression in self.expressions.values() {
            cluster.add_expression(Expression {
                id: expression.id.clone(),
                initiative_id: self.semantic_identity.clone(),
                terms: expression.terms.clone(),
                bias: expression.bias,
                domain: expression.domain.clone(),
                constraints: expression.constraints.clone(),
                semantic_links: expression.semantic_links.clone(),
                nsq_capability: expression.nsq_capability.clone(),
                revision: 0,
            })?;
        }
        Ok(cluster)
    }

    pub fn predict_next(
        &mut self,
        updates: impl IntoIterator<Item = (String, i64)>,
    ) -> Result<(), String> {
        self.predicted_next_parameters.clear();
        for (id, value) in updates {
            if !self.parameters.contains_key(&id) {
                return Err(format!("prediction references unknown parameter: {id}"));
            }
            self.predicted_next_parameters.insert(id, value);
        }
        self.reconciliation_state = ReconciliationState::Predicted;
        Ok(())
    }

    pub fn apply_observed_delta(
        &mut self,
        updates: impl IntoIterator<Item = (String, i64)>,
    ) -> Result<Vec<String>, String> {
        let mut changed = Vec::new();
        for (id, value) in updates {
            let parameter = self
                .parameters
                .get_mut(&id)
                .ok_or_else(|| format!("observation references unknown parameter: {id}"))?;
            if parameter.value != value {
                parameter.value = value;
                parameter.generation = parameter.generation.saturating_add(1);
                changed.push(id);
            }
        }
        self.generation = self.generation.saturating_add(1);
        self.predicted_next_parameters.clear();
        self.reconciliation_state = if changed.is_empty() {
            ReconciliationState::Observed
        } else {
            ReconciliationState::Corrected
        };
        Ok(changed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_output_is_candidate_only_until_canonicalized() {
        let candidate = CandidateIntent::extract(
            "semantic_identity=source-reconcile\nintent=reconcile\nconfidence_bps=9300\nparameter.source_scope=191233\nparameter.correction_enabled=1\nexpression.reconcile.terms=source_scope:1,correction_enabled:1",
            "model:test",
        )
        .unwrap();
        let set = DynamicParameterSet::canonicalize(candidate).unwrap();
        assert_eq!(set.reconciliation_state, ReconciliationState::Canonical);
        assert_eq!(set.parameters["source_scope"].value, 191233);
        assert_eq!(set.expressions["reconcile"].terms.len(), 2);
        assert!(set
            .constraints
            .iter()
            .any(|constraint| constraint == "model-is-non-authoritative"));
    }

    #[test]
    fn logical_space_has_no_artificial_item_ceiling() {
        let mut candidate = String::from("semantic_identity=unbounded\nintent=expand\n");
        for index in 0..2048 {
            candidate.push_str(&format!("parameter.p{index}=1\n"));
        }
        let set = DynamicParameterSet::canonicalize(
            CandidateIntent::extract(&candidate, "test").unwrap(),
        )
        .unwrap();
        assert_eq!(set.parameters.len(), 2048);
    }

    #[test]
    fn prediction_is_not_authoritative_until_observation() {
        let candidate =
            CandidateIntent::extract("semantic_identity=x\nintent=step\nparameter.p=1", "test")
                .unwrap();
        let mut set = DynamicParameterSet::canonicalize(candidate).unwrap();
        set.predict_next([("p".into(), 2)]).unwrap();
        assert_eq!(set.parameters["p"].value, 1);
        assert_eq!(set.predicted_next_parameters["p"], 2);
        let changed = set.apply_observed_delta([("p".into(), 3)]).unwrap();
        assert_eq!(changed, vec!["p"]);
        assert_eq!(set.parameters["p"].value, 3);
    }
}

pub fn stable_hash(value: &str) -> String {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

pub fn candidate_intent_size(model_output: &str) -> u64 {
    model_output.len() as u64
}

pub fn dynamic_parameter_set_size(set: &DynamicParameterSet) -> u64 {
    serde_json::to_vec(set)
        .map(|bytes| bytes.len() as u64)
        .unwrap_or(0)
}

pub fn expression_bytes(set: &DynamicParameterSet) -> u64 {
    set.expressions.len() as u64 * 64
}
