# Expanded Braxon Reconstruction Audit

**Date:** 2026-08-17  
**Target branch:** `reconstruction`  
**Repository:** `https://github.com/adriak85/Braxon`

## Executive result

The complete branch and repository review was extended to every cloned repository and every fetched remote branch. Candidate extraction was performed from all branch heads without executing branch-provided scripts. Only implementation-bearing changes that could be integrated coherently and validated against the current Braxon API were transferred into `reconstruction`.

The consolidated Braxon workspace now passes the Rust 1.85 workspace compile check and the full workspace test suite. The final branch intentionally does **not** claim that the host handover is fully released: the runtime reports the missing ten-surface and watermark artifacts and keeps power disconnect disabled. This is the correct fail-closed behavior.

## Repository and branch coverage

The audit covered Braxon plus these related repositories: `0`, `DAX-FULL`, `Dax`, `Dax-Autonomous-System`, `PAPI`, `f1ux-service`, `fastapi-llm-bot`, and `termux-packages`. All fetched branches were inventoried in `audit/expanded/braxon_all_branches.tsv`, and all branch candidates were classified in `branch_candidate_metrics.tsv`, `candidate_details.txt`, and `overlap_classification.txt`.

Branch labels were not treated as evidence of completeness. Historical snapshots, generated output, documentation-only branches, and branches whose direct tree comparison would delete the complete Braxon baseline were excluded from direct wholesale merging. The transfer rule was additive-only unless a change was independently validated against the current Reconstruction tree.

## Improvements transferred

The following validated additions were transferred from the strongest candidate branch family:

| Area | Transfer | Validation status |
|---|---|---|
| Braxon seed materialization | `crates/braxon-core/src/seed_citadel.rs`, `wowas_seeded.rs`, and exports | Compiled and unit-tested |
| NSQ Citadel | `crates/nsq-citadel/` implementation, manifest, binary, seed, wire, capital, coaching, and bit modules | Compiled and unit-tested; 3 seed tests passed |
| NSQ system | Source tree, intent extraction, and rebuild planner | Repaired against the authoritative current `nsq-core` API; workspace check passed |
| No-hidden-files traversal | Only `.git` internals are excluded; dotfiles, generated files, backups, uncommon extensions, and build outputs remain represented | Static inspection passed |
| Sweet-spot benchmark | Corrected the selector and replaced stale magic frontier assertions with measured invariants | 4 `nsq-core` tests passed |
| Runtime integration tests | Added deterministic binary-path fallback and changed handover test to validate truthful fail-closed behavior | All Braxon surface tests passed |

The seed implementation was hardened to use a stable FNV-1a digest rather than `DefaultHasher`, avoiding nondeterministic cross-process hashing.

## Function-level validation

`function_inventory.tsv` records source-file, function-declaration, test-marker, function-bearing-file, and test-bearing-file counts for the Reconstruction head and all related repository heads. The inventory is static coverage evidence; a historical branch cannot be proven executable merely because it contains a function declaration.

The Reconstruction head contains Rust, Python, JavaScript, TypeScript, and Go source surfaces. The Rust workspace was executable under Rust/Cargo 1.85.1. The final workspace test run reported **zero failing targets**; all unit, integration, binary, and doc-test targets completed successfully.

The following Braxon executable surfaces were explicitly exercised: application listing, application inspection, application verification, natural conversation transcript, Python runtime ingress, OS-power handover reporting, speech surface, NSQ core benchmarks, NSQ Citadel seed behavior, NSQ stamp verification, NSQ compose, NSQ compress, NSQ doctor, WOWAS authority contracts, and the workspace’s remaining crate tests.

## Related repository validation

Native validation was attempted on every related repository head using the available language toolchains. Failures were retained as evidence rather than hidden:

| Repository | Result | Concrete blocker |
|---|---|---|
| `0` | Rust blocked; Python compile passed | Rust packages require Rust 1.96 while the available validation toolchain is 1.85 |
| `DAX-FULL` | Rust blocked; Python compile failed | Invalid pinned `wgpu` revision; `dax_final_core.py` begins with invalid `kimport` syntax |
| `Dax` | Rust blocked; Python compile failed | Invalid TOML inline table; multiple Python syntax errors |
| `Dax-Autonomous-System` | Rust blocked; Python compile failed | Multiple workspace roots; indentation and generated-Python syntax errors |
| `PAPI` | No applicable Cargo/Python/Go test surface detected | Static inventory only |
| `f1ux-service` | Python compile passed | No Cargo or Go manifest |
| `fastapi-llm-bot` | Python compile passed | No Cargo or Go manifest |
| `termux-packages` | Python compile passed | No Cargo or Go manifest |

These failures are not silently imported into Braxon. They remain candidate-repository defects and are documented in `related_heads_validation.log`.

## Truth and release-gate findings

The `handover os-power-release` function now reports:

- `full_release_complete: false`
- `all_in_check_validated: true`
- `semantic_address_gate_completely_validated: true`
- `ten_surface_bus_validated: false`
- `watermark_trigger_set_completely_validated: false`
- `power_disconnect_requested: false`

The missing watermark inputs are `state/nsq/citadel699/current/request_capsule.json`, `target_models.json`, and `materialization.json`. The implementation reports these missing prerequisites instead of emitting a false “release complete” claim.

The Target Field requirement remains explicitly tracked in `reconstruction/TARGET_FIELD.md`; no actual Target Field implementation was found across the reviewed authoritative source paths.

## Final quality gates

| Gate | Result |
|---|---|
| `cargo check --workspace` with Rust 1.85.1 | PASS |
| `cargo test --workspace --no-fail-fast` with Rust 1.85.1 | PASS |
| Direct `rustfmt --check` on changed Rust and integration files | PASS |
| Braxon application surface tests | PASS |
| Braxon conversation surface tests | PASS |
| Braxon runtime surface tests | PASS, including truthful blocked-release behavior |
| NSQ core benchmark and invariant tests | PASS |
| Related repository native validation | Mixed; blockers documented, not transferred |
| Hidden/uncommon file traversal review | PASS for the selected NSQ source scanner; only `.git` excluded |

The working tree changes, audit evidence, branch metrics, candidate comparisons, function inventory, and native validation logs are retained under `audit/` and `audit/expanded/` for reproducibility.
