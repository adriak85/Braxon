# NSQ Inventory Pass 01 — 2026-08-16

This pass deliberately records artifacts that look redundant or conventional.

## Branches visible

- main — 7804fcdccba12f814941a80cd1e9c0c578d4f004
- braxon-current-preserve-20260507-172011 — ced6af253888bf194375692de86d0678dc70d847
- braxon-dirty-preserve-20260507-172531 — 92257c89b7bf034f2156f202b10ce8856ae0908e
- phone/citadel699-route-gates-20260508_165115 — 06f7c9abe018aa5a14df4619fb4e0f64473773f0
- audit/diversity-transmedia-2026-08-09 — 7804fcdccba12f814941a80cd1e9c0c578d4f004
- triage — b5cbe4f01ae2f48d7467652d20d7a83278d2f379
- codex/seed-citadel-completion — reconstruction branch

## Actual member inventory

`crates/actual_members.txt` records 36 crate names, including crates not present in the current root workspace membership. This file is itself evidence and must not be treated as stale merely because Cargo.toml differs.

Recorded members include:
- braxon-cli
- braxon-core
- braxon-court
- braxon-ingest
- braxon-kingdom-generate
- braxon-showdown
- nsq-archon
- nsq-bench
- nsq-bench-compare
- nsq-bench-split
- nsq-calibrate
- nsq-cli
- nsq-compile
- nsq-compose
- nsq-compress
- nsq-core
- nsq-court
- nsq-debug
- nsq-decode
- nsq-generate
- nsq-index
- nsq-inspect
- nsq-lint
- nsq-native-bench
- nsq-optimize
- nsq-pack
- nsq-preserve
- nsq-pressure
- nsq-pressure-bench
- nsq-prime
- nsq-profile
- nsq-proof
- nsq-query
- nsq-real-bench
- nsq-registry
- nsq-runtime
- nsq-source
- nsq-universal-fetch

## Current root workspace divergence

Current Cargo.toml lists 18 workspace members including nsq-grid and nsq-wake but does not list the full 36-member inventory. Earlier saved Cargo.toml variants contain progressively fewer members and are retained as historical snapshots.

## nsq-core artifact density

Current nsq-core contains:
- active Cargo.toml
- a duplicate-named Cargo.toml.before_dead_edge_repair_20260506_000937 with the same blob SHA as the active manifest
- README.md
- eight_dimensional_gradient_contract.rs
- intent.rs
- lib.rs
- preserve.rs
- seating.rs
- stamp_execution_contract.rs
- watermark_execution_contract.rs
- numerous `lib.rs.before_*` snapshots.

The `lib.rs.before_*` snapshots are not disposable. They represent successive repair states including architecture, derive, runtime comfort-fit, exact model-seat API, precise alignment, brain-seat preservation, council-pole rebuild, duplicate-tail removal, root-foundation repair, seated-pole marker repair, payload-width shaping, and ultimate cleanup.

## nsq-core current semantic evidence

The current lib.rs defines:
- canonical lever maximum 500000
- four levers per canonical bit unit
- alternating full-binary anchors and multipositional levers
- charge polarity (+/-)
- hertz-based lever stabilization and return-to-off/return-to-on averaging
- sound-resonance diagnostics
- spacing sweeps
- intent variables Motive/Agency/Truth/Force/Scope/Time/Relation/Form
- four scale anchors
- positive/negative/neutral sides in one nested historical intent implementation
- ten CouncilPole variants
- IntentSurface variants
- CourtSeating and CourtBootClearance
- collapsed court authority state

The same file contains nested and re-exported intent/court definitions that differ from the separate current `src/intent.rs` architecture. This is a major internal variant and must be preserved for reconstruction.

## Measured-state conflict

The current nsq-core implementation uses 500000 as the active lever range and computes a 62,500,000,000,000,000,000,000 state capacity per four-lever unit.

The preserved global tag uses 2254 zero-inclusive lever states and 1126 as its selected stable upper position.

The architecture watermark later declares >=220000 active floor and 225370 proven effective positions.

These are three historical measurement/topology states. Do not merge their constants without provenance.

## Architecture checkpoint

Commit `8fa6d38ef53be97f30f63cd95387a595b30c14a1` introduced the current Codex preservation-law family and was a merge titled “Enhance NSQ architecture and add NSQASM stamp features.”

## Historical implementation philosophy

Recovered files consistently reject:
- binary/u8/u16/u32/u64 as canonical semantic truth
- passive stamp labels
- metadata-only execution claims
- fake hot-live states
- pointer stubs as model shards
- plugin/wrapper/sidecar runtime substitution
- destructive repository cleanup
- broad rewrites

Recovered files consistently require:
- structure-preserving base-8 representation
- operational stamps
- execution/materialization proof
- fail-closed boundaries
- visible verification
- state continuity and donor/target separation

## Important unresolved question

The repository contains strong historical language saying “NSQ is the machine,” “NSQ is the lowest base language and machine substrate,” and “NSQ Court is the runtime.” The current conversation establishes a stricter architectural correction: NSQ should be treated as a language, while higher-level runtime/cognitive structures remain separate.

This is deliberately recorded as an unresolved historical divergence rather than silently changing old evidence.

## Next inventory targets

Continue into every current crate tree, especially:
- nsq-core historical snapshots
- nsq-court
- nsq-runtime
- nsq-compile / compose / decode / generate / index / lint / optimize / preserve / prime / profile / proof / registry / universal-fetch
- nsqasm-stamp-db
- nsq-grid
- nsq-wake
- nsq-hot
- calibration artifacts
- preserve/proof artifacts
- root and hook files
- all branch-specific trees
- tags and commit history

No deduplication is authorized during this archaeological phase.
