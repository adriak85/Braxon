# NSQ Kinetic Semantic Reflex

## Purpose

The **NSQ Kinetic Semantic Reflexor** is the workspace’s on-demand capability-control layer and NSQ ingestion authority. It discovers the complete Cargo workspace, every direct support library, every required runtime/dialect surface, every project-owned implementation source, and the Android/Termux physical boundary. It fully ingests and redefines each discoverable project surface as a verifiable NSQ-addressable capability record.

> The Reflexor does **not** merely recognize or endorse Rust, C, assembly, Guile, Lisp/Scheme, Zig, apropos, or any other external spelling. Each is represented inside NSQ as the semantic authority. A source spelling, compiler binary, assembler, package manager, documentation command, or Android interface is an explicit ingress/egress boundary codec only; it is never a second semantic authority or resident runtime.

## Runtime Boundary

| Layer | Reflex responsibility | Explicitly not claimed |
|---|---|---|
| Workspace crates | Discover every workspace crate through Cargo metadata and map it to an on-demand NSQ capability. | That every crate is already running or has been flattened into a shared process. |
| Direct library dependencies | Record each direct Cargo dependency as a support-library contract. | That the library is dynamically loaded or hardware-backed without a probe. |
| Runtime and dialect declarations | Dynamically ingest and redefine every entry in `config/nsq/nsq_runtime_language_registry.json` as an NSQ-native language contract. | That a foreign runtime is the semantic authority. External tools remain only optional boundary codecs. |
| Native language contracts | Instantiate Rust 1.97, C, and assembly toolchain contracts from `config/nsq/nsq_native_language_contracts.json`, with the requested Rust 1.97 beta resolved to immutable Rust 1.97.0. | That a floating nightly, Android SDK, or external toolchain owns system meaning. |
| Project-owned sources | Recursively enumerate Cargo manifests, Rust, C/C++, assembly, LLVM IR, Guile/Scheme/Lisp, Zig, shell, configuration, documentation, and front-door scripts; SHA-256 each source and create one NSQ source-ingestion capability per file. | That a source file can remain outside the system’s inventory. |
| Display, input, and presentation | Represent acquisition, lifecycle, frames, geometry, raster operations, presentation, input, synchronization, and 3D scene state as NSQ kinetic boundary operations. | An Android GUI, a conventional graphics engine, or a resident rendering loop. |
| Android / Termux | Probe and use Android only as the physical input and display boundary. | A replacement Android runtime. |

## Dialect Mapping

The Reflexor uses the canonical `nsq-core::Dialect` values. C, C++, Rust, Zig, and typed compilation surfaces map to `symbolic`; assembly, architecture assembly, and LLVM IR map to `stamp`; Guile, Scheme, Lisp, and semantic-language surfaces map to `intent`; apropos, man, and Markdown documentation map to `alphabetic`; configuration and control surfaces map to `control`; graphics and presentation map to `graphics`; voice/audio declarations map to `audio`. Every required registry surface is dynamically ingested and redefined inside NSQ, not copied as a static list.

The mapping is explicit and machine-readable in the generated `state/reflex/capability_inventory.json` artifact. Every capability must record `semantic_authority = "nsq"`, `complete_nsq_ingestion = true`, and a foreign surface role limited to boundary ingress/egress. The verifier fails closed if any record violates those conditions, if semantic intent is not instantiated as a valid eight-variable NSQ gradient, or if a discoverable source file is absent from the inventory.

## Complete-Crate Coverage

The Reflexor calls Cargo metadata at execution time. Therefore, the crate inventory is not a copied, stale list. The verifier compares current workspace membership against generated crate contracts, compares every direct Cargo dependency against support-library contracts, and recursively scans the project source tree against source-ingestion contracts. It fails closed if any count differs.

The existing `nsq-hot` crate is a workspace member, so it is included rather than being treated as an orphaned implementation surface.

## Native Samsung Galaxy A17 Front Door

The device profile is located at:

```text
config/device_profiles/samsung_galaxy_a17_termux_aarch64.json
```

The native Termux entry point is:

```bash
bin/braxon-reflex bootstrap --profile samsung_galaxy_a17_termux_aarch64
```

The bootstrap operation verifies the current host instead of trusting the profile name. It checks Termux prefix detection, AArch64 compatibility, the pinned Rust 1.97 toolchain, native C and assembly tools, Guile, apropos, full workspace mapping, direct-library ingestion, exhaustive project-source ingestion, complete dialect coverage, NSQ semantic authority, and physical-boundary isolation. It writes the canonical inventory to `state/reflex/capability_inventory.json` only after discovery and verification execute.

| Command | Result |
|---|---|
| `bin/braxon-reflex bootstrap` | Probe native Termux, validate mapping, and capture the inventory. |
| `bin/braxon-reflex discover` | Emit the full capability model without running a resident runtime. |
| `bin/braxon-reflex verify` | Fail closed if any workspace crate or declared dialect lacks a Reflex record. |
| `bin/braxon-reflex capture` | Persist the capability inventory and integrity envelope. |
| `bin/braxon-reflex operate crate:nsq-cli --execute` | Start one explicit crate operation; no background process persists after completion. |

## Execution Policy

The Reflexor routes non-executable contracts without spawning a process. It permits execution only when the capability is a workspace crate that exposes a binary target and when `--execute` is explicitly supplied. The operation receipt is written under `state/reflex/operations/` and records the route, requested command, and outcome.

This preserves the intended operating loop:

```text
observe → represent → analyze → route → explicitly execute → verify → record
```

The architecture remains lightweight because no phase requires a continuously running renderer, GUI service, language runtime fleet, or model process.
