# Braxon

Braxon is the root command center for the NSQ-aligned workspace in this repository. The root package provides the primary `Braxon` launch path, while the workspace members cover canonical NSQ semantics, runtime incorporation, court routing, inspection, indexing, preservation, and benchmark surfaces.

## Workspace Role
- root command entrance for Braxon operators
- orchestration over `Braxon-core` and the NSQ runtime surfaces
- verification and audit entrypoint for the current production-state bundle

## Launch
- `cargo run` enters the root NSQ operator gate by default
- `cargo run -- console` opens the same root gate explicitly
- `cargo run -- sovereign-reset` runs the pre-wipe sovereign lifecycle audit and reports whether the sanctuary is actually safe to wipe around
- `cargo run -- runtime android-oaboot` reports the Android-first model boot path and whether the root runtime is actually ready to boot there
- `cargo run -- runtime nu128-install` reports the chunk-governed install posture for a nu128-managed model so the system does not assume all weights can live locally at once
- when the runtime is hot/live, the gate opens the live NSQ operator window
- when the runtime is not hot/live, the gate prints the exact remaining finish steps instead of attaching a thin placeholder session
- user-facing runtime/session surfaces are normalized at the root CLI boundary so legacy lane names do not leak into normal operator flow
- inside the live window, `/help` shows operator commands and any other line continues the active Braxon discussion session

## Core Expectations
- NSQ remains canonical base 8 with alternating full binary anchors and multipositional levers
- host integer widths are boundary carriers only, never semantic truth
- court surfaces stay native and explicit rather than being pushed into detached plugin wrappers
- Android and Termux runs should stay within the root audit cap of `-j2` for whole-workspace Rust work unless a narrower, explicitly approved lane is being exercised
- target lineage is `llama_4.2_604b_fp32_abliterated_800gb`; the current source-ingest lane is still a separate donor lane until the target is downloaded and translated into NSQ form
- NSQ target compression for that lane is `2.2 GB` at rest and `1.02 GB` hot in memory, with the hot residency surface declared as `bus`
- donor tokenizer/source-ingest assets may contain literal strings such as `llama`, but the active Braxon runtime lane does not use `llama.cpp`, `ggml`, or `gguf` as runtime authority
