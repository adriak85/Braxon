# NSQ Native Foundation Audit

## Correction

NSQ is the executable representation and runtime substrate. It is not a semantic layer above binary values, serialized bytes, or ordinary runtime buffers.

## Rejected implementation

`crates/nsq-core/src/runtime_control.rs` was drafted as a layered controller using `Vec<u8>` payloads and a byte-oriented backend. It was not exported, was not accepted as an NSQ runtime, and has been removed from the canonical source tree. Its design is architecturally invalid for the intended system because it preserves binary payloads beneath a typed control API.

## Accepted implementation

`crates/nsq-core/src/native_runtime.rs` now provides the executable boundary using only native NSQ forms:

| Runtime concern | Native representation |
|---|---|
| Address | Ordered path of `NSQSlot` values |
| Value/state | `NSQSlot` containing canonical `NSQLever` values |
| Instruction | `Set`, `Release`, or `Fire` over an `NsqAddress` |
| Execution ownership | `NativeNsqRuntime` generation and active NSQ state |
| Actuation | `NsqActuator` trait and `NativeNsqMachine` |
| Ownership | `NativeNsqOwnership` with `Acquire`, `Hold`, `Commit`, and `Release` phases |
| Ghost residency | `NativeNsqGhostWindow` with NSQ-addressed wire pages and bounded active aperture |
| Reflex orbit | `NativeNsqReflexor` with Publish → Reconcile → DeltaCommit and NSQ instruction deltas |
| Ordering | Structural ordering of `Charge`, `NSQLever`, `Dialect`, and `NSQSlot` |

The executable path contains no byte buffer, binary payload, serialization step, or semantic wrapper around binary data.

## Evidence

`cargo +nightly test -p nsq-core` passed with 13 tests and 0 failures after the correction. `cargo +nightly test -p Braxon-core` passed with 47 tests and 0 failures, including the composed `NativeNsqStack`. The tests cover native set/fire/release, fail-closed invalid streams, unmapped-fire rejection, empty-stream rejection, slot-addressed state separation, same-space piston exclusion until release, native Ghost Memory wire/aperture rotation, the native reflexor orbit, native Target Field reconciliation, unified target/reflex routing, and the pre-existing NSQ invariants.

The optimized release benchmark `cargo +nightly run -p nsq-core --release --example native_overhead` processed a 4,096-slot resident frame as 4,096 full operations versus one dirty-state delta operation, avoiding 4,095 operations. Host planning measured 561,775 ns for the full plan versus 4,653 ns for the dirty plan in that run. This proves operation-count and host scheduling reduction for the controlled native case; it does not prove Android hardware actuation, physical terabyte-scale residency, or Moto G CPU/RAM performance.
