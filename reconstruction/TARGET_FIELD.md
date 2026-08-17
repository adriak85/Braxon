# Target Field Reconstruction Contract

## Status

**Implemented and integrated.** The executable implementation is `crates/braxon-core/src/target_field.rs`, exported through `Braxon-core` and initialized by the root `handover os-power-release` command.

The Target Field is a deterministic eight-dimensional intent-gradient coordinate state. It is not treated as an authority by label alone: its schema, authority, model-count reconciliation, finite-coordinate invariant, persistence, and actuation behavior are validated in code and unit tests.

## Contract

The persisted artifact is `state/braxon/target_field.json` with schema `braxon.target_field.v1`. The field is derived from `config/nsq/braxon_council_ten_stack.json` when the persisted artifact is absent. Its required model count is reconciled as six brain models plus four sensory bodies, for a total of ten.

The coordinate space is `eight_dimensional_intent_gradient` under the canonical `base8_switch_topology` semantics. Coordinates are finite `f64` values. The actuation surface reports resource pressure, information pressure, load-shed fraction, cache-flush request, state-reconstruction request, and the evaluated coordinate.

## Failure behavior

Malformed persisted state, an authority or schema mismatch, non-finite coordinates, or unreconciled model counts cause the loader to fail closed. Missing persisted state is recoverable: the implementation creates a deterministic artifact from the validated council-ten manifest. The root handover includes the Target Field and actuation result in its JSON response.

## Integration and acceptance evidence

The implementation is integrated into the root handover and is covered by `target_field::tests::target_field_is_reconciled_and_deterministic` and `target_field::tests::target_field_persists_and_reloads`. The runtime surface test confirms that the handover exposes the Target Field while retaining the genuine ten-surface release gate. Rust 1.96 workspace checks and the targeted tests pass.

The Target Field does not claim that the host release is complete. The ten-surface gate remains fail-closed until its own required proof and bus conditions validate.
