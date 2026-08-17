# NSQ Contradictions and Variants — 2026-08-16

This is an evidence ledger. Contradictions are not resolved by preference; both states remain preserved until provenance and behavior establish the canonical state.

## 1. NSQ identity boundary

Current reconstruction direction from this session: NSQ is a language. Council Ten, Citadel, and higher cognitive/runtime organizations are not NSQ primitives.

Historical repository state is broader and sometimes calls NSQ a “lowest base language and machine substrate,” “semantic routing substrate,” or “runtime execution system.” Preserve those statements as historical architecture rather than silently rewriting them. The repository's own files distinguish gradient, brain, spine, court/compositor, metadata, and NSQ substrate in some places, while other architecture files combine substrate/runtime authority more tightly.

Resolution status: OPEN. Canonical language definition must be recovered before higher-level runtime design is finalized.

## 2. Intent gradient boundary

`ARCHITECTURE.md` states: “The intent gradient IS the language of the inner system,” with eight variables: Motive, Agency, Truth, Force, Scope, Time, Relation, Form. It also says tokenization exists only at the surface boundary and that the dispatch loop does not speak tokens/strings.

`.codex/architecture/gradient.md` separately says the eight-dimensional gradient is not NSQ itself and is distinct from the court, brain, spine, watermark, metadata, and systems.

These are materially different architectural claims. Preserve both.

Resolution status: OPEN. Do not collapse the gradient into NSQ until the historical chain establishes that relationship.

## 3. Lever chronology

Older global-tag state records zero-inclusive lever states = 2254, four levers per bit unit, and 25,811,642,826,256 states per bit unit, with a tested/stable upper position of 1126 and a 0.001777778 hertz spacing.

Newer architecture state records an active lever floor >= 220000 and 225370 proven effective positions, while explicitly declaring 1126 legacy-only.

These are not to be overwritten. The 2254/1126 state is historical evidence of an earlier calibration/topology; the 220000/225370 state is a later active target/proof boundary.

Resolution status: chronology established; exact physical relationship still requires underlying calibration artifacts.

## 4. Global tag vs architecture watermark

`BRAXON_GLOBAL_TAG.json` uses `BRAXON_GLOBAL_NSQ_CHARACTER_STAMP_SCORE_V1` and records the 2254/1126-era measurement model.

Current architecture files require `BRAXON_NSQ_BASE8_LEVER_SCALE_GE220000_PROVEN225370_V1`.

Both tags must remain preserved. The newer watermark must not cause deletion of the older global tag.

## 5. Runtime boundary

Some repository law describes NSQ Court as the runtime authority and says there is no separate runtime layer above court. Other material distinguishes NSQ substrate from Braxon runtime, host OS, ingress, handover, console, and app surfaces.

Resolution status: preserve the boundary documents and do not infer a new runtime hierarchy from names alone.

## 6. Court / Council

Historical `ARCHITECTURE.md` defines ten poles: six brain poles and four sensory-generation bodies, and separately describes a 25-role operational court. Current conversation explicitly rejects treating Council Ten as part of NSQ.

Therefore Council/Court structures remain higher-level historical/runtime architecture unless the language specification proves otherwise.

## 7. Rust implementation

Rust appears extensively as implementation/build infrastructure, including workspace crates, NSQ core/court/runtime packages, NSQASM stamp DB, NSQ grid/wake additions, and later Citadel work.

Rust artifacts are evidence of attempted implementations. They are not automatically authoritative over NSQ semantics.

## 8. Workspace duplicate preservation

`Cargo.toml.before_architecture_now_20260506_014334` and `Cargo.toml.before_dead_edge_repair_20260506_000937` have the same blob SHA `38d4bad7...` but distinct paths and historical names. `Cargo.toml.before_add_nsq_wake_workspace` has blob SHA `32c6cc...` and adds `nsqasm-stamp-db` relative to the earlier state.

The duplicate files are meaningful provenance even when byte-identical. Do not deduplicate them away.

## 9. Current workspace expansion

Current `Cargo.toml` adds both `crates/nsqasm-stamp-db`, `crates/nsq-grid`, and `crates/nsq-wake` beyond the earlier saved workspace state. The earlier saved workspace already had `nsqasm-stamp-db` in one intermediate stage.

This establishes a concrete evolution chain rather than one static workspace design.

## 10. `_°`

Historical `_°` notation remains unresolved. It must not be replaced with `∅`, `NULL`, or another conventional null token until historical semantics and parser behavior are recovered.

Required investigations:
- semantic meaning
- lexical/token meaning
- parser acceptance
- shell/CLI transport behavior
- whether the degree sign was literal or notation
- relationship to any underscore/null/unbound primitive

## 11. Preservation rule

The project explicitly says not to broad-rewrite, not to simplify NSQ into conventional CS language when meaning changes, and to make the fewest effective edits. The reconstruction must therefore prefer additive archaeological ledgers over destructive normalization.

## 12. Branch chronology currently visible

Visible branches include:
- main
- braxon-current-preserve-20260507-172011
- braxon-dirty-preserve-20260507-172531
- phone/citadel699-route-gates-20260508_165115
- audit/diversity-transmedia-2026-08-09
- triage
- codex/seed-citadel-completion

These refs are evidence and must be inventoried before declaring the reconstruction complete.

## 13. Commit-level nugget

Commit `8fa6d38ef53be97f30f63cd95387a595b30c14a1` is a merge titled “Enhance NSQ architecture and add NSQASM stamp features.” Its diff introduced the Codex operating override, NSQ architecture file, gradient/materialization/runtime-boundary/stamp/watermark documents, and associated contracts. This is an important historical checkpoint because it establishes when the current preservation-law family entered the repository.

## Status

This ledger is intentionally incomplete. It records recovered contradictions so subsequent passes do not accidentally erase them. The next passes must continue into the complete tree, all historical branches/tags/commits, source crates, NSQASM/stamp artifacts, calibration data, and files that merely resemble upstream/common infrastructure.
