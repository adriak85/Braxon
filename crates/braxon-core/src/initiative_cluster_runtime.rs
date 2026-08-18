use nsq_core::{ClusterSnapshot, InitiativeCluster};
use serde::{Deserialize, Serialize};

use crate::{BusValue, HardwareWriteAck, KineticReflexor, ReflexorReport, ValueClass};

pub const INITIATIVE_CLUSTER_RUNTIME_SCHEMA: &str = "braxon.nsq.initiative_cluster_runtime.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InitiativeClusterExecutionReceipt {
    pub schema: String,
    pub cluster_id: String,
    pub generation: u64,
    pub changed_parameters: Vec<String>,
    pub affected_expressions: Vec<String>,
    pub recalculated_count: usize,
    pub unchanged_unmaterialized: Vec<String>,
    pub reflexor_report: ReflexorReport,
    pub released_snapshot: ClusterSnapshot,
}

pub fn execute_through_reflexor(
    cluster: &mut InitiativeCluster,
    changed_parameters: &[String],
) -> Result<InitiativeClusterExecutionReceipt, String> {
    let delta = cluster.evaluate_affected(changed_parameters)?;
    let values = delta.recalculated.iter().map(|result| BusValue {
        key: format!("{}.expression.{}", cluster.cluster_id, result.expression_id),
        class: ValueClass::Parameter,
        value_hash: stable_hash(&format!(
            "{}:{}:{}",
            result.expression_id, result.value, result.generation
        )),
        byte_len: 8,
    });
    let mut reflexor = KineticReflexor::new();
    reflexor.publish(values)?;
    reflexor.reconcile()?;
    let written_keys: Vec<String> = reflexor
        .pending_delta()
        .iter()
        .map(|value| value.key.clone())
        .collect();
    let report = reflexor.commit_hardware(HardwareWriteAck {
        adapter_id: "initiative-cluster-nsq-adapter".into(),
        generation: reflexor.generation(),
        accepted: true,
        written_keys,
    })?;
    let snapshot = cluster.release();
    Ok(InitiativeClusterExecutionReceipt {
        schema: INITIATIVE_CLUSTER_RUNTIME_SCHEMA.into(),
        cluster_id: delta.cluster_id,
        generation: delta.generation,
        changed_parameters: delta.changed_parameters,
        affected_expressions: delta.affected_expressions,
        recalculated_count: delta.recalculated.len(),
        unchanged_unmaterialized: delta.unchanged_unmaterialized,
        reflexor_report: report,
        released_snapshot: snapshot,
    })
}

fn stable_hash(value: &str) -> String {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use nsq_core::Expression;

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
        let mut cluster = InitiativeCluster::new("cluster-runtime").unwrap();
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
        cluster
    }

    #[test]
    fn cluster_executes_through_reflexor_and_releases() {
        let mut cluster = cluster();
        cluster.apply_parameter_delta("p0", 9).unwrap();
        let receipt = execute_through_reflexor(&mut cluster, &["p0".into()]).unwrap();
        assert_eq!(receipt.affected_expressions, vec!["a0"]);
        assert_eq!(receipt.recalculated_count, 1);
        assert_eq!(receipt.unchanged_unmaterialized, vec!["a1", "b0"]);
        assert!(receipt.reflexor_report.hardware_write_acknowledged);
        assert_eq!(receipt.reflexor_report.delta_values, 0);
        assert!(cluster.released);
        assert!(cluster.current_results.is_empty());
    }

    #[test]
    fn released_snapshot_reconstructs_deterministically() {
        let mut first = cluster();
        first.apply_parameter_delta("p2", 8).unwrap();
        let left = execute_through_reflexor(&mut first, &["p2".into()]).unwrap();
        let mut second = InitiativeCluster::reconstruct(left.released_snapshot).unwrap();
        second.apply_parameter_delta("p2", 8).unwrap();
        let right = execute_through_reflexor(&mut second, &["p2".into()]).unwrap();
        assert_eq!(left.affected_expressions, right.affected_expressions);
        assert_eq!(left.recalculated_count, right.recalculated_count);
        assert_eq!(
            left.unchanged_unmaterialized,
            right.unchanged_unmaterialized
        );
    }
}
