//! Designated recursive parameter-to-Citadel operation.
//!
//! This is an executable structural-law bridge, not a neural simulation. The
//! `purkinje` designation names the required local integration role: each
//! parameter cluster receives a localized change, resolves only the pressure it
//! affects, routes that result, integrates it with a Citadel materialization,
//! advances one generation, and releases a state that can be reconstructed.

use nsq_citadel::{CitadelMaterialization, CitadelNativeRuntime, CoachingMode, IntentSeed};
use nsq_core::{ClusterSnapshot, InitiativeCluster};
use serde::{Deserialize, Serialize};

use crate::{execute_through_reflexor, InitiativeClusterExecutionReceipt};

pub const PARAMETER_CITADEL_OPERATION_SCHEMA: &str = "braxon.nsq.parameter_citadel_operation.v1";
pub const PURKINJE_PARAMETER_OPERATION_ROLE: &str = "designated_local_parameter_integration";

/// An auditable proof that a local parameter transition and its Citadel
/// materialization completed the same recursive state law.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParameterCitadelInvariants {
    pub identity_preserved: bool,
    pub local_state_materialized: bool,
    pub multi_input_pressure_resolved: bool,
    pub routed_response_integrated: bool,
    pub generation_preserved: bool,
    pub persistent_state_reconstructible: bool,
    pub no_resident_runtime: bool,
}

impl ParameterCitadelInvariants {
    pub fn all_pass(&self) -> bool {
        self.identity_preserved
            && self.local_state_materialized
            && self.multi_input_pressure_resolved
            && self.routed_response_integrated
            && self.generation_preserved
            && self.persistent_state_reconstructible
            && self.no_resident_runtime
    }
}

/// Receipt for one designated parameter-Citadel cycle.
#[derive(Debug, Clone)]
pub struct ParameterCitadelOperation {
    pub schema: String,
    pub role: String,
    pub identity: String,
    pub generation: u64,
    pub changed_parameters: Vec<String>,
    pub parameter_execution: InitiativeClusterExecutionReceipt,
    pub citadel_materialization: CitadelMaterialization,
    pub persistent_snapshot: ClusterSnapshot,
    pub invariants: ParameterCitadelInvariants,
}

/// Execute the shared recursive law:
///
/// identity → local delta → dependency pressure → routed response → Citadel
/// integration → generation transition → released, reconstructible state.
///
/// The function is on-demand. It creates no resident model, GUI, or background
/// service; the Citadel runtime is local to this transaction and its durable
/// representation is the released parameter snapshot plus the materialization
/// receipt.
pub fn execute_parameter_citadel_operation(
    cluster: &mut InitiativeCluster,
    updates: impl IntoIterator<Item = (String, i64)>,
    coaching: CoachingMode,
) -> Result<ParameterCitadelOperation, String> {
    if cluster.released {
        return Err(
            "released parameter cluster must be reconstructed before a new Citadel cycle".into(),
        );
    }

    let mut updates = updates.into_iter().collect::<Vec<_>>();
    if updates.is_empty() {
        return Err("parameter-Citadel operation requires at least one local update".into());
    }
    updates.sort_by(|left, right| left.0.cmp(&right.0));
    for pair in updates.windows(2) {
        if pair[0].0 == pair[1].0 {
            return Err(format!("duplicate parameter update: {}", pair[0].0));
        }
    }

    for (id, value) in &updates {
        cluster.apply_parameter_delta(id, *value)?;
    }
    let changed_parameters = updates.iter().map(|(id, _)| id.clone()).collect::<Vec<_>>();
    let local_generation_before_execution = cluster.generation;
    let parameter_execution = execute_through_reflexor(cluster, &changed_parameters)?;

    if parameter_execution.recalculated_count == 0 {
        return Err("parameter-Citadel operation requires routed affected expressions".into());
    }
    if !parameter_execution
        .reflexor_report
        .hardware_write_acknowledged
    {
        return Err("parameter-Citadel operation requires an acknowledged routed response".into());
    }
    if parameter_execution.generation != local_generation_before_execution.saturating_add(1) {
        return Err("parameter-Citadel generation did not advance exactly once".into());
    }

    let identity = format!("parameter-citadel::{}", parameter_execution.cluster_id);
    let intent = canonical_operation_intent(
        &identity,
        parameter_execution.generation,
        &updates,
        &parameter_execution,
    );
    let seed = IntentSeed::new(&identity, &intent);
    let mut citadel_runtime = CitadelNativeRuntime::new(coaching);
    let citadel_materialization = citadel_runtime
        .materialize_seed(&seed, parameter_execution.generation)
        .map_err(|error| error.to_string())?;

    let persistent_snapshot = parameter_execution.released_snapshot.clone();
    let reconstructed = InitiativeCluster::reconstruct(persistent_snapshot.clone())?;
    let invariants = ParameterCitadelInvariants {
        identity_preserved: persistent_snapshot.cluster_id == parameter_execution.cluster_id
            && identity.ends_with(&parameter_execution.cluster_id),
        local_state_materialized: !persistent_snapshot.parameters.is_empty()
            && !citadel_materialization.bodies.is_empty(),
        multi_input_pressure_resolved: parameter_execution.recalculated_count > 0
            && !parameter_execution.affected_expressions.is_empty(),
        routed_response_integrated: parameter_execution
            .reflexor_report
            .hardware_write_acknowledged
            && citadel_materialization.receipt.fired == citadel_materialization.bodies.len(),
        generation_preserved: persistent_snapshot.generation == parameter_execution.generation
            && reconstructed.generation == parameter_execution.generation
            && citadel_materialization.generation == parameter_execution.generation,
        persistent_state_reconstructible: reconstructed.cluster_id
            == persistent_snapshot.cluster_id
            && reconstructed.parameters == persistent_snapshot.parameters
            && reconstructed.expressions == persistent_snapshot.expressions
            && reconstructed.linked_clusters == persistent_snapshot.linked_clusters,
        no_resident_runtime: true,
    };
    if !invariants.all_pass() {
        return Err("parameter-Citadel structural invariants did not all pass".into());
    }

    Ok(ParameterCitadelOperation {
        schema: PARAMETER_CITADEL_OPERATION_SCHEMA.into(),
        role: PURKINJE_PARAMETER_OPERATION_ROLE.into(),
        identity,
        generation: parameter_execution.generation,
        changed_parameters,
        parameter_execution,
        citadel_materialization,
        persistent_snapshot,
        invariants,
    })
}

fn canonical_operation_intent(
    identity: &str,
    generation: u64,
    updates: &[(String, i64)],
    execution: &InitiativeClusterExecutionReceipt,
) -> String {
    let updates = updates
        .iter()
        .map(|(id, value)| format!("{id}={value}"))
        .collect::<Vec<_>>()
        .join(";");
    let affected = execution.affected_expressions.join(",");
    format!(
        "identity={identity} generation={generation} updates={updates} affected={affected} role={PURKINJE_PARAMETER_OPERATION_ROLE}"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use nsq_core::Expression;

    fn cluster() -> InitiativeCluster {
        let mut cluster = InitiativeCluster::new("organ-band").unwrap();
        cluster.add_parameter("signal", 2).unwrap();
        cluster.add_parameter("context", 3).unwrap();
        cluster
            .add_expression(Expression {
                id: "integrate".into(),
                initiative_id: "organ-band".into(),
                terms: vec![("signal".into(), 2), ("context".into(), 1)],
                bias: 1,
                domain: "integer".into(),
                constraints: vec!["canonical-nsq".into()],
                semantic_links: vec!["organ-band".into()],
                nsq_capability: "organ-band.integrate".into(),
                revision: 0,
            })
            .unwrap();
        cluster
    }

    #[test]
    fn parameter_cycle_and_citadel_materialization_share_the_recursive_law() {
        let mut cluster = cluster();
        let operation = execute_parameter_citadel_operation(
            &mut cluster,
            [("signal".into(), 8), ("context".into(), 5)],
            CoachingMode::Balanced,
        )
        .unwrap();

        assert_eq!(operation.role, PURKINJE_PARAMETER_OPERATION_ROLE);
        assert_eq!(operation.identity, "parameter-citadel::organ-band");
        assert_eq!(operation.generation, 1);
        assert_eq!(operation.citadel_materialization.bodies.len(), 10);
        assert_eq!(operation.citadel_materialization.receipt.fired, 10);
        assert!(operation.invariants.all_pass());
        assert!(cluster.released);
    }

    #[test]
    fn released_parameter_state_reconstructs_at_the_same_generation_for_the_next_cycle() {
        let mut first = cluster();
        let first_operation = execute_parameter_citadel_operation(
            &mut first,
            [("signal".into(), 8)],
            CoachingMode::Balanced,
        )
        .unwrap();
        let mut rebuilt =
            InitiativeCluster::reconstruct(first_operation.persistent_snapshot).unwrap();
        assert_eq!(rebuilt.generation, 1);
        let second_operation = execute_parameter_citadel_operation(
            &mut rebuilt,
            [("context".into(), 9)],
            CoachingMode::Balanced,
        )
        .unwrap();
        assert_eq!(second_operation.generation, 2);
        assert_eq!(second_operation.citadel_materialization.generation, 2);
        assert!(second_operation.invariants.all_pass());
    }

    #[test]
    fn operation_fails_closed_on_empty_duplicate_or_unknown_local_pressure() {
        let mut empty = cluster();
        assert!(execute_parameter_citadel_operation(
            &mut empty,
            Vec::<(String, i64)>::new(),
            CoachingMode::Balanced,
        )
        .is_err());

        let mut duplicate = cluster();
        assert!(execute_parameter_citadel_operation(
            &mut duplicate,
            [("signal".into(), 8), ("signal".into(), 9)],
            CoachingMode::Balanced,
        )
        .is_err());

        let mut unknown = cluster();
        assert!(execute_parameter_citadel_operation(
            &mut unknown,
            [("unknown".into(), 8)],
            CoachingMode::Balanced,
        )
        .is_err());
    }
}
