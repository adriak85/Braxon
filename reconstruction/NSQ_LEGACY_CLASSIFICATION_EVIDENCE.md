# Legacy Layer Classification Evidence

The legacy surfaces were inspected for representation types that cannot be authoritative when NSQ replaces binary.

| Legacy surface | Observed non-native representation | Classification |
|---|---|---|
| `crates/braxon-core/src/nsq_native.rs` | `String` schemas, IDs, provenance, roles, address prefixes, and `[f64; 8]` gradients | Preserve as migration evidence; do not treat as native runtime. |
| `crates/braxon-core/src/ghost_memory.rs` | String page/lease identifiers and `wire_bytes`, `active_cpu_bytes`, and `byte_len` accounting | Preserve as prior aperture behavior evidence; native NSQ Ghost Window supersedes execution authority. |
| `crates/braxon-core/src/kinetic_reflexor.rs` | String-keyed `BusValue`, string hashes, `byte_len`, and serialized watermark text | Preserve as prior synchronization evidence; native NSQ reflexor supersedes execution authority. |
| `crates/braxon-core/src/target_field.rs` | `[f64; 8]` coordinates, string authority/status, JSON load/persist, and JSON actuation configuration | Preserve as external/configuration evidence; native NSQ Target Field supersedes execution authority. |

The classification is not a claim that these legacy modules are useless. Their tests and domain behavior remain useful during migration. It is a claim about authority: none of these representations may be used to assert that the system is running natively on NSQ.

The accepted replacements are `NativeNsqBus`, `NativeNsqGhostWindow`, `NativeNsqReflexor`, `NativeNsqTargetField`, and the composed `NativeNsqStack`. Their focused tests execute over `NsqAddress`, `NSQSlot`, `NSQLever`, and `NsqInstruction` forms.
