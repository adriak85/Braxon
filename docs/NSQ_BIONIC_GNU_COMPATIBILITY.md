# NSQ Bionic-to-GNU/libc Compatibility Contract

## Scope and truth boundary

Braxon targets **AArch64 Android with Bionic**. The project does **not** replace Bionic with glibc, does **not** claim the complete GNU C library is embedded, and does **not** reclassify public operating-system interfaces as privately owned merely because an independently authored bridge implements them. The compatibility lane is an opt-in, staged overlay for specific missing interfaces required by local source builds and tools. It preserves Android/Bionic as the platform libc and requires compile, link, exported-symbol, and target-run evidence before an interface may be reported as available.[1] [2]

> **Compatibility is an executable contract, not a label.** A surface is only available when its header declaration, ABI-compatible implementation or bridge, library artifact, target link, and target probe have all been demonstrated.

| Boundary | Authoritative rule |
|---|---|
| Platform libc | Android **Bionic** remains authoritative. |
| GNU/libc interface | A named public interface may be declared and implemented by a first-party bridge without importing glibc source. |
| Ownership | The Braxon implementation source can be assessed for independent authorship; the interface name, ABI, and upstream documentation remain subject to their original provenance. |
| Installation | Headers and libraries are staged, then symlinked into an overlay. The scripts forbid direct system and Termux-prefix overwrites. |
| Runtime | The overlay is materialized on demand. It does not create a resident runtime. |
| Failure | Missing artifacts, unsupported kernel behavior, unavailable symbols, or missing target probes must report the exact connection needed. |

## Machine and ABI contract

The current target contract is `aarch64-linux-android` with an Android API floor of 24. The overlay source uses explicitly selected **AArch64 syscall numbers** only where the interface is implemented through a Linux syscall, and it otherwise bridges existing Bionic primitives such as `sem_timedwait`, `pthread`, `prctl`, `read`, or `write`. AArch64 barriers are retained in operations where the existing source explicitly requires ordering.[1]

| Interface | Header | Contract class | AArch64 bridge | Current source state |
|---|---|---|---|---|
| `sem_clockwait` | `semaphore.h` | Timed semaphore wait | Bionic `sem_timedwait` plus monotonic/realtime conversion and barrier | First-party overlay source retained |
| `pthread_getname_np` | `pthread.h` | Thread-name query | Bionic pthread plus `PR_GET_NAME` bridge | First-party overlay source retained |
| `close_range` | `unistd.h` | Descriptor-range close | syscall `436`; bounded fallback only without flags | First-party overlay source retained |
| `statx` | `sys/stat.h` | Extended file status | syscall `291` | First-party overlay source retained |
| `copy_file_range` | `unistd.h` | Kernel file copy | syscall `285` | First-party overlay source retained |
| `getrandom` | `sys/random.h` | Kernel random bytes | syscall `278` | First-party overlay source retained |
| `memfd_create` | `sys/mman.h` | Anonymous descriptor | syscall `279` | First-party overlay source retained |
| `eventfd` | `sys/eventfd.h` | Event counter creation | syscall `19` | First-party overlay source retained |
| `eventfd_read` | `sys/eventfd.h` | Event counter read | Bionic `read` bridge | First-party overlay source retained |
| `eventfd_write` | `sys/eventfd.h` | Event counter write | Bionic `write` bridge | First-party overlay source retained |
| `pipe2` | `unistd.h` | Atomic pipe creation | syscall `59` | First-party overlay source retained |
| `dup3` | `unistd.h` | Atomic descriptor duplicate | syscall `24` | First-party overlay source retained |
| `accept4` | `sys/socket.h` | Flagged socket accept | syscall `242` | First-party overlay source retained |

The complete, machine-readable interface record—including function class, header, bridge, source ownership boundary, target proof state, and universal lexical spelling—is maintained in the [Bionic compatibility matrix][3]. The matrix is the authoritative list; this table is a readable projection.

## Universal tokenizer and NSQ dialect ingestion

Each interface has a stable lexical identity such as `bionic.gnu.sem_clockwait`. The active Braxon tokenizer operates at character level, preserving the native token identifier, deterministic universal identifier, reverse mapping, and shared NSQ address for every character. This means the compatibility surface is not a separate vocabulary: it is encoded through the same `TokenizerBridge` used by normal NSQ operations.[4]

| NSQ dialect | Compatibility meaning |
|---|---|
| **Alphabetic** | Canonical lexical sequence, for example `bionic.gnu.getrandom`. |
| **Numeric** | AArch64 syscall number or an explicitly documented numeric ABI value where applicable. |
| **Intent** | Request for a bounded compatibility capability with ABI and precondition checks. |
| **Symbolic** | Header, interface name, function class, calling boundary, and bridge classification. |
| **Stamp** | Source hash, source path, build flags, exported-symbol inspection, target probe, and upstream-interface provenance. |
| **Control** | Stage, build, archive, shared-link, overlay, compile, target-run, release, and cleanup transitions. |
| **Graphics** | Explicitly **not applicable**; no graphics semantics are fabricated for libc calls. |
| **Audio** | Explicitly **not applicable**; no audio semantics are fabricated for libc calls. |

The `BionicCompatibility` verifier loads the matrix, encodes every universal lexical identity through `TokenizerBridge`, and fails closed if the native vocabulary cannot map a required character, the source is absent, a source record is malformed, or a proof state is overstated.[3] [4]

## Proof and release discipline

The checked-in overlay source exists, but a current clone may not contain the generated Android library or target proof receipt. Therefore the only truthful current state is **source-present / target-materialization-required**. A valid target proof must demonstrate all of the following in order:

1. The staged headers compile for the declared Android/AArch64 target.
2. The first-party source compiles into an object, archive, and shared library under the staged overlay path.
3. `llvm-nm` and `readelf -Ws` show the declared exported symbols.
4. A target binary links using only the declared overlay and Bionic boundary.
5. The target probe runs and validates each required success or error-path contract.
6. The resulting evidence is stamped with source identity, target ABI, build flags, library hashes, and probe result.

This is intentionally stricter than merely finding a header or an old output file. The executable source lane is [the unified Android libc overlay script][1]; it stages headers, compiles the independent implementation, builds static and shared libraries, uses symbol inspection, and runs a probe. The compact NSQ matrix and verifier provide the reusable semantic route without forcing the full compiler/runtime tree to remain resident.[1] [3]

## Ownership and licensing

The compatibility matrix records the overlay source as **first-party implementation of public interfaces**. That status is limited: it must not be used to claim ownership of Bionic, glibc, Linux ABI definitions, public function names, compiler behavior, or upstream documentation. Any incorporated upstream source or headers retain their licenses, notices, and redistribution obligations. The private-eligibility decision is therefore source-level and evidence-based, never the consequence of translation, renaming, wrapping, or an NSQ representation.[3]

## References

[1]: ../scripts/toolchains/unified_android_libc_contract_overlay.sh "Unified Android libc contract overlay implementation"
[2]: ../scripts/toolchains/resolve_cpython_android_native_contracts_full.sh "CPython Android native contract resolver"
[3]: ../config/nsq/bionic_gnu_compatibility_matrix.json "Bionic-to-GNU/libc compatibility matrix"
[4]: ../crates/braxon-core/src/tokenizer_bridge.rs "Tokenizer bridge and universal NSQ addressing"
