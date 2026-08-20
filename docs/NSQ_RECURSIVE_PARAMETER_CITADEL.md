# NSQ Recursive Parameter–Citadel Law

## Purpose

This document records an **implemented and tested** architectural law in Braxon. It does not introduce a biological simulator or a resident neural runtime. Instead, it requires the same executable state-transition grammar at a designated local parameter operation and at the Citadel scale.

> **Shared law:** `identity → local materialized state → multi-input pressure → routed response → integration → generation transition → persistent reconstruction`.

The local role is named `designated_local_parameter_integration` in the source. The Purkinje reference is structural: it denotes a local integration point that receives distinct inputs, resolves local pressure, emits a routed result, and remains part of a larger state. It is not a claim that the software simulates a Purkinje cell.

## Implemented components

| Scale | Active implementation | Executable state law |
|---|---|---|
| NSQ substrate | `crates/nsq-core/src/native_runtime.rs` | Addressed identity, ownership phases, atomic instruction execution, generation guards, and active/resident windows. |
| Local parameter operation | `crates/nsq-core/src/initiative_cluster.rs` | Parameter identity, dependency graph, selective affected-expression evaluation, generation advance, release, snapshot, and deterministic reconstruction. |
| Routed organism integration | `crates/braxon-core/src/initiative_cluster_runtime.rs` | Affected results publish through the kinetic reflexor, reconcile, receive an acknowledged write, and release a snapshot. |
| Designated recursive bridge | `crates/braxon-core/src/parameter_citadel.rs` | One on-demand operation executes the parameter cycle and a Citadel materialization together, then proves their shared invariants. |
| Citadel scale | `crates/nsq-citadel/src/materialization.rs` | Seed identity, ten-body materialization, capital/pole dispatch, addressed actuation, generation, delta integrity, inventory reconciliation, and on-demand rematerialization. |
| Persistent state | `ClusterSnapshot`, `CitadelInventory`, `CitadelMaterialization` | Released cluster snapshots preserve generation; Citadel inventory and materialization preserve source hashes, addresses, and generation records. |

## Required invariants

The designated bridge returns `ParameterCitadelInvariants`. A transaction is rejected unless every field is true.

| Invariant | Concrete requirement |
|---|---|
| `identity_preserved` | The released cluster identity remains stable and the Citadel seed identity derives from it. |
| `local_state_materialized` | The parameter snapshot and Citadel bodies both exist. |
| `multi_input_pressure_resolved` | At least one affected expression is selectively recalculated from the changed local parameter state. |
| `routed_response_integrated` | The parameter reflexor receives an acknowledged write and the Citadel fires all materialized bodies. |
| `generation_preserved` | Cluster snapshot, reconstructed cluster, and Citadel materialization all carry the same transition generation. |
| `persistent_state_reconstructible` | Reconstructed cluster state equals the released snapshot for identity, parameters, expressions, and links. |
| `no_resident_runtime` | The operation is on-demand; it starts no permanent model, graphical runtime, or background service. |

## On-demand rematerialization correction

Citadel generation was tightened so a generation greater than one can rematerialize in a **fresh on-demand runtime window**. A release is now attempted only when that owner lease exists in the current window. This preserves the monotonic generation record without incorrectly requiring the old resident window to remain alive.

## Tests

The implementation is covered by:

- `parameter_citadel::tests::parameter_cycle_and_citadel_materialization_share_the_recursive_law`
- `parameter_citadel::tests::released_parameter_state_reconstructs_at_the_same_generation_for_the_next_cycle`
- `parameter_citadel::tests::operation_fails_closed_on_empty_duplicate_or_unknown_local_pressure`
- `materialization::tests::fresh_runtime_can_rematerialize_a_later_generation_without_prior_residency`
- existing parameter-cluster and Citadel materialization tests

The intended audit question is therefore executable: a parameter cluster is not accepted as passive data, and a Citadel is not accepted as merely ten named poles. Both must pass the same state-transition and reconstruction law.

## Historical comparison boundary

The comparison includes `origin/phone/citadel699-route-gates-20260508_165115`, `origin/codex/seed-citadel-completion`, and the reconstruction lineage. The historical phone/Citadel branch predates the current `nsq-citadel` and `dynamic_parameter` implementations. Reconstruction therefore does not need to copy old source text; it must preserve and test the older architectural requirement in the current executable substrate.
