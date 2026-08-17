# NSQ Dependency Status

This status is based on executable tests and source inspection. A passing legacy test is not treated as proof that the legacy path uses NSQ as its executable substrate.

| Surface | Current status | Evidence or blocking condition |
|---|---|---|
| `nsq-core` native runtime | **Validated at unit level** | Native NSQ addresses, slots, instructions, generation ownership, fail-closed preflight, direct actuator path, reflexor, Ghost Memory, and Target Field pass 13 tests. |
| Native piston ownership | **Validated at unit level** | `NativeNsqOwnership` rejects same-target overlap and permits reacquisition only after release. |
| Native Ghost Memory | **Validated at unit level** | `NativeNsqGhostWindow` keeps multiple NSQ pages on the wire while bounding active aperture occupancy. |
| Native reflexor | **Validated at unit level** | `NativeNsqReflexor` emits native NSQ deltas through Publish → Reconcile → DeltaCommit and refreshes its watermark. |
| Native Braxon bus | **Validated at unit level** | `NativeNsqBus` arbitrates NSQ intent frames through ten NSQ council addresses and native ownership; its test passes within the 46-test Braxon-core suite. |
| Native Target Field | **Validated at unit level** | `NativeNsqTargetField` emits a native NSQ delta and quiets at the watermark; its test passes within the 13-test nsq-core suite. |
| Legacy `nsq_native` bus | **Untrusted / migration required** | Uses `String` addresses, `String` schema/provenance, and `[f64; 8]` gradients. Its tests prove internal behavior only, not native NSQ execution. |
| Legacy `ghost_memory` | **Untrusted / migration required** | Uses string page and lease identifiers plus byte-length accounting. It cannot yet be treated as the authoritative NSQ wire substrate. |
| Legacy `kinetic_reflexor` | **Untrusted / migration required** | Uses string-keyed `BusValue` state and ordinary value classes. Its three-phase test is useful evidence but does not prove native NSQ state. |
| Legacy `target_field` | **Untrusted / migration required** | Persists through JSON and uses a conventional serialized configuration surface. The native counterpart exists, but callers have not yet been fully migrated. |
| Android native surface | **External acceptance gate** | Host compilation and unit tests do not prove a physical non-root Android 16 Moto G deployment or direct surface actuation. |
| Overhead reduction | **Not yet measured** | Native paths reduce representation-level operations in unit tests, but no controlled baseline benchmark has established CPU, memory, IPC, or wall-clock reduction. |

The canonical rule is therefore: **native paths may be extended; legacy paths may be preserved as evidence but cannot confer operational completion on the whole stack.**
