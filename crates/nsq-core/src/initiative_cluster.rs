use std::collections::{BTreeMap, BTreeSet, VecDeque};

use serde::{Deserialize, Serialize};

pub const INITIATIVE_CLUSTER_SCHEMA: &str = "braxon.nsq.initiative_cluster.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Parameter {
    pub id: String,
    pub value: i64,
    pub revision: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Expression {
    pub id: String,
    pub initiative_id: String,
    pub terms: Vec<(String, i64)>,
    pub bias: i64,
    pub domain: String,
    pub constraints: Vec<String>,
    pub semantic_links: Vec<String>,
    pub nsq_capability: String,
    pub revision: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClusterSnapshot {
    pub schema: String,
    pub cluster_id: String,
    pub parameters: BTreeMap<String, Parameter>,
    pub expressions: BTreeMap<String, Expression>,
    pub linked_clusters: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpressionResult {
    pub expression_id: String,
    pub value: i64,
    pub revision: u64,
    pub generation: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClusterDelta {
    pub schema: String,
    pub cluster_id: String,
    pub generation: u64,
    pub changed_parameters: Vec<String>,
    pub affected_expressions: Vec<String>,
    pub recalculated: Vec<ExpressionResult>,
    pub unchanged_unmaterialized: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InitiativeCluster {
    pub schema: String,
    pub cluster_id: String,
    pub parameters: BTreeMap<String, Parameter>,
    pub expressions: BTreeMap<String, Expression>,
    pub linked_clusters: Vec<String>,
    pub current_results: BTreeMap<String, ExpressionResult>,
    pub generation: u64,
    pub released: bool,
}

impl InitiativeCluster {
    pub fn new(cluster_id: impl Into<String>) -> Result<Self, String> {
        let cluster_id = cluster_id.into();
        if cluster_id.trim().is_empty() {
            return Err("initiative cluster id cannot be empty".into());
        }
        Ok(Self {
            schema: INITIATIVE_CLUSTER_SCHEMA.into(),
            cluster_id,
            parameters: BTreeMap::new(),
            expressions: BTreeMap::new(),
            linked_clusters: Vec::new(),
            current_results: BTreeMap::new(),
            generation: 0,
            released: false,
        })
    }

    pub fn add_parameter(&mut self, id: impl Into<String>, value: i64) -> Result<(), String> {
        let id = id.into();
        if id.trim().is_empty() || self.parameters.contains_key(&id) {
            return Err("parameter id must be nonempty and unique".into());
        }
        self.parameters.insert(
            id.clone(),
            Parameter {
                id,
                value,
                revision: 0,
            },
        );
        Ok(())
    }

    pub fn add_expression(&mut self, expression: Expression) -> Result<(), String> {
        if expression.id.trim().is_empty() || self.expressions.contains_key(&expression.id) {
            return Err("expression id must be nonempty and unique".into());
        }
        if expression.terms.is_empty() || expression.nsq_capability.trim().is_empty() {
            return Err("expression requires terms and an NSQ capability".into());
        }
        for (parameter_id, _) in &expression.terms {
            if !self.parameters.contains_key(parameter_id) {
                return Err(format!(
                    "expression references unknown parameter: {parameter_id}"
                ));
            }
        }
        self.expressions.insert(expression.id.clone(), expression);
        Ok(())
    }

    pub fn link_cluster(&mut self, cluster_id: impl Into<String>) -> Result<(), String> {
        let cluster_id = cluster_id.into();
        if cluster_id.trim().is_empty() || self.linked_clusters.contains(&cluster_id) {
            return Err("linked cluster id must be nonempty and unique".into());
        }
        self.linked_clusters.push(cluster_id);
        self.linked_clusters.sort();
        Ok(())
    }

    pub fn apply_parameter_delta(&mut self, id: &str, next_value: i64) -> Result<(), String> {
        let parameter = self
            .parameters
            .get_mut(id)
            .ok_or_else(|| format!("unknown parameter: {id}"))?;
        parameter.value = next_value;
        parameter.revision = parameter.revision.saturating_add(1);
        self.released = false;
        Ok(())
    }

    pub fn evaluate_affected(&mut self, changed: &[String]) -> Result<ClusterDelta, String> {
        if self.released {
            return Err("released cluster must be reconstructed before evaluation".into());
        }
        let changed_set: BTreeSet<String> = changed.iter().cloned().collect();
        let mut reverse: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for expression in self.expressions.values() {
            for (parameter_id, _) in &expression.terms {
                reverse
                    .entry(parameter_id.clone())
                    .or_default()
                    .push(expression.id.clone());
            }
        }
        let mut affected = BTreeSet::new();
        let mut queue: VecDeque<String> = changed_set.iter().cloned().collect();
        while let Some(parameter_id) = queue.pop_front() {
            for expression_id in reverse.get(&parameter_id).into_iter().flatten() {
                if affected.insert(expression_id.clone()) {
                    queue.push_back(expression_id.clone());
                }
            }
        }
        self.generation = self.generation.saturating_add(1);
        let mut recalculated = Vec::new();
        for expression_id in &affected {
            let expression = self
                .expressions
                .get(expression_id)
                .ok_or_else(|| format!("unknown expression: {expression_id}"))?;
            let mut value = expression.bias;
            for (parameter_id, coefficient) in &expression.terms {
                let parameter = self
                    .parameters
                    .get(parameter_id)
                    .ok_or_else(|| format!("unknown parameter: {parameter_id}"))?;
                value = value.saturating_add(parameter.value.saturating_mul(*coefficient));
            }
            let result = ExpressionResult {
                expression_id: expression_id.clone(),
                value,
                revision: expression.revision,
                generation: self.generation,
            };
            self.current_results
                .insert(expression_id.clone(), result.clone());
            recalculated.push(result);
        }
        let unchanged_unmaterialized = self
            .expressions
            .keys()
            .filter(|id| !affected.contains(*id))
            .cloned()
            .collect();
        Ok(ClusterDelta {
            schema: INITIATIVE_CLUSTER_SCHEMA.into(),
            cluster_id: self.cluster_id.clone(),
            generation: self.generation,
            changed_parameters: changed_set.into_iter().collect(),
            affected_expressions: affected.into_iter().collect(),
            recalculated,
            unchanged_unmaterialized,
        })
    }

    pub fn release(&mut self) -> ClusterSnapshot {
        let snapshot = self.snapshot();
        self.current_results.clear();
        self.released = true;
        snapshot
    }

    pub fn snapshot(&self) -> ClusterSnapshot {
        ClusterSnapshot {
            schema: INITIATIVE_CLUSTER_SCHEMA.into(),
            cluster_id: self.cluster_id.clone(),
            parameters: self.parameters.clone(),
            expressions: self.expressions.clone(),
            linked_clusters: self.linked_clusters.clone(),
        }
    }

    pub fn reconstruct(snapshot: ClusterSnapshot) -> Result<Self, String> {
        if snapshot.schema != INITIATIVE_CLUSTER_SCHEMA {
            return Err("initiative cluster snapshot schema mismatch".into());
        }
        let mut cluster = Self::new(snapshot.cluster_id)?;
        cluster.parameters = snapshot.parameters;
        cluster.expressions = snapshot.expressions;
        cluster.linked_clusters = snapshot.linked_clusters;
        for expression in cluster.expressions.values() {
            for (parameter_id, _) in &expression.terms {
                if !cluster.parameters.contains_key(parameter_id) {
                    return Err(format!(
                        "snapshot expression references unknown parameter: {parameter_id}"
                    ));
                }
            }
        }
        Ok(cluster)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expression(id: &str, terms: Vec<(&str, i64)>) -> Expression {
        Expression {
            id: id.into(),
            initiative_id: "initiative-1".into(),
            terms: terms
                .into_iter()
                .map(|(key, coefficient)| (key.into(), coefficient))
                .collect(),
            bias: 0,
            domain: "test".into(),
            constraints: vec!["deterministic".into()],
            semantic_links: vec!["reflexor.delta".into()],
            nsq_capability: "initiative.evaluate".into(),
            revision: 0,
        }
    }

    fn cluster() -> InitiativeCluster {
        let mut cluster = InitiativeCluster::new("cluster-a").unwrap();
        for (id, value) in [("p0", 2), ("p1", 3), ("p2", 5), ("p3", 7), ("p4", 11)] {
            cluster.add_parameter(id, value).unwrap();
        }
        cluster
            .add_expression(expression("a0", vec![("p0", 1), ("p1", 2)]))
            .unwrap();
        cluster
            .add_expression(expression("a1", vec![("p2", 1), ("p3", 1)]))
            .unwrap();
        cluster
            .add_expression(expression("b0", vec![("p4", 3)]))
            .unwrap();
        cluster.link_cluster("cluster-b").unwrap();
        cluster
    }

    #[test]
    fn recalculates_only_affected_expressions() {
        let mut cluster = cluster();
        cluster.apply_parameter_delta("p0", 9).unwrap();
        let delta = cluster.evaluate_affected(&["p0".into()]).unwrap();
        assert_eq!(delta.affected_expressions, vec!["a0"]);
        assert_eq!(delta.recalculated[0].value, 15);
        assert_eq!(delta.unchanged_unmaterialized, vec!["a1", "b0"]);
    }

    #[test]
    fn release_and_reconstruct_are_deterministic() {
        let mut cluster = cluster();
        let snapshot = cluster.release();
        assert!(cluster.current_results.is_empty());
        assert!(cluster.released);
        let mut rebuilt = InitiativeCluster::reconstruct(snapshot).unwrap();
        let delta = rebuilt.evaluate_affected(&["p2".into()]).unwrap();
        assert_eq!(delta.recalculated[0].value, 12);
        assert_eq!(delta.affected_expressions, vec!["a1"]);
    }

    #[test]
    fn linked_clusters_preserve_separate_state() {
        let cluster = cluster();
        assert_eq!(cluster.linked_clusters, vec!["cluster-b"]);
        assert!(!cluster.parameters.contains_key("cluster-b.p0"));
        assert!(!cluster.expressions.contains_key("cluster-b.a0"));
    }

    #[test]
    fn unknown_dependency_is_rejected() {
        let mut cluster = InitiativeCluster::new("cluster").unwrap();
        cluster.add_parameter("p0", 1).unwrap();
        assert!(cluster
            .add_expression(expression("bad", vec![("missing", 1)]))
            .is_err());
    }
}
