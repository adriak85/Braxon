# Kinetic Semantic Reflexor

The kinetic reflexor is the bus-maintained refresh orbit for parameters, weights, KV state, and fact bindings. It uses one shared Piston generation sequence and three ordered phases.

| Phase | Operation | Required result |
|---|---|---|
| `Publish` | Place the current typed value set on the NSQ bus. | A nonempty, duplicate-free bus snapshot and a `Reconcile` watermark. |
| `Reconcile` | Read the bus snapshot into the system view and compare it with the last acknowledged hardware baseline. | A deterministic delta containing only changed value hashes. |
| `DeltaCommit` | Send exactly that delta to an approved adapter and require an acknowledgement for the same generation and keys. | The acknowledged hardware state becomes the next refresh baseline and the orbit returns to `Publish`. |

Watermarks are operational. Every cycle carries the semantic family, generation, phase, and state hash. A stale acknowledgement, rejected adapter write, duplicate key, empty publication, or mismatched key list blocks the orbit rather than advancing it.

The implementation is deliberately honest about hardware. The NSQ bus and reflexor can prepare and validate a delta, but they cannot claim that a physical phone or storage device was changed without an adapter acknowledgement. The adapter boundary is where platform-specific I/O, permissions, and device acceptance testing belong.

The executable contract is `crates/braxon-core/src/kinetic_reflexor.rs`, exported by `braxon-core`. Its tests cover a complete publish/reconcile/delta-commit/refresh cycle, changed-value-only writes, stale and rejected writes, and duplicate publication rejection.
