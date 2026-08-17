# Braxon Reconstruction Audit Report

**Audit date:** 2026-08-17

**Canonical branch:** [`reconstruction`](https://github.com/adriak85/Braxon/tree/reconstruction)

**Final commit:** `d99a081bc84316e8e7bf65a8f5079ea011db2351`

## Executive result

The Braxon repository was cloned and audited as a complete Git tree, including dotfiles, unusual extensions, generated-looking artifacts, historical files, binary files, and repository metadata visible through Git. The new `reconstruction` branch was created from the complete `origin/main` tree and pushed successfully. It contains **180,397 tracked files**, has **zero paths missing** relative to `origin/main`, and introduces **32 additional paths**. There are **zero deleted paths** versus `origin/main`; the remaining changed paths are deliberate implementation or provenance changes.

The branch is therefore a complete-tree consolidation, not a checkout of one of the older partial reconstruction snapshots. The final remote commit matches the local commit exactly.

## Branch review and benchmarking

The audit reviewed all Braxon remote branches and computed branch file counts, commit distance from main, direct tree differences, added/deleted paths, and exact tree-equivalence groups. The key result is that several branches named as reconstruction or rebuild branches are materially truncated historical snapshots and must not replace the current main tree.

| Branch family | Observed condition | Decision |
|---|---|---|
| `main`, `Maine`, `braxon-final-cohesive-stack-20260816` | Exact tree-equivalent complete tree at 180,365 files before consolidation | Used as the base |
| `reconstruction-final-20260816` | Complete main tree plus six changed paths, including the reconstruction protocol, NSQ ingestion changes, gradient-contract changes, and validation metadata | Selected implementation delta |
| `reconstruction-final-20260816-worker-test` and `worker-execution*` | Same 180,367-file snapshot and same tree across worker aliases; less complete than the newer reconstruction candidate | Treated as duplicate worker snapshots |
| `nsq-final-reconstruction-20260816` | 11,779-file partial tree; missing 168,597 paths relative to main | Extracted only its unique `nsq/canon` artifacts |
| `nsq-cohesive-rebuild-20260816` | 11,777-file partial tree; missing 168,597 paths relative to main | Extracted only `crates/nsq-system` and workspace registration |
| `wowas-full-construction-20260816` | Near-complete tree with 15 direct differences, including 12 unique WOWAS construction paths and three deletions/changes relative to main | Extracted only unique non-destructive construction artifacts; did not replace main |
| `wowas-full-construction` and `wowas/full-construction-20260816` | Exact duplicate tree-equivalent partial snapshots | Retained as historical evidence only |
| `triage` | Divergent snapshot with hundreds of additions and deletions and a different tree identity | Not used as a canonical base |

The related repositories selected for review were `0`, `DAX-FULL`, `Dax`, `Dax-Autonomous-System`, `PAPI`, `f1ux-service`, `fastapi-llm-bot`, and `termux-packages`. Their branch inventories and overlapping implementation surfaces were recorded locally during the audit. The related repositories did not provide a validated Target Field implementation or a safer complete-tree base for Braxon.

## Consolidated contents

The `reconstruction` branch includes the complete main tree plus the validated unique surfaces below.

| Consolidated surface | Source | Rationale |
|---|---|---|
| Reconstruction protocol, NSQ ingestion and gradient-contract changes, validation workflow, and check metadata | `origin/reconstruction-final-20260816` | Newest complete-tree reconstruction delta |
| `crates/nsq-system` and workspace registration | `origin/nsq-cohesive-rebuild-20260816` | Unique cohesive source scanner and intent/rebuild planner, extracted without its truncated historical tree |
| `nsq/canon` artifacts | `origin/nsq-final-reconstruction-20260816` | Unique canonical NSQ artifacts, extracted without its truncated historical tree |
| WOWAS reconstruction state, ledgers, lattice, corpus index, stage state, TSV tool, and workflow | `origin/wowas-full-construction-20260816` | Unique construction artifacts; the stage state itself records incomplete downstream materialization and validation |
| `reconstruction/TARGET_FIELD.md` | New audit ledger | Makes the unresolved Target Field requirement explicit rather than silently treating a label or claim as implementation |
| `audit/consolidation_source_map.tsv` | New audit provenance | Records why each source branch was selected and prevents accidental reintroduction of truncated snapshots |

## No-hidden-files correction

The selected `nsq-system` scanner originally skipped both `.git` and `target` directories. Because the requirement was to avoid hiding any kind of file, the consolidated branch changes this behavior to skip only `.git` internals, which are Git’s object database rather than project source. Dotfiles, build output, backups, generated files, uncommon extensions, and other file classes remain visible to the scanner. The source classification still labels historical and generated artifacts; classification is not deletion or exclusion.

## Target Field finding

No exact executable or authoritative documented implementation matching `Target Field`, `Target_Field`, or `target-field` was found in Braxon’s authoritative source/documentation paths or the reviewed related repository heads. The final branch therefore does **not** make an unsupported claim that Target Field is complete. Instead, [`reconstruction/TARGET_FIELD.md`](https://github.com/adriak85/Braxon/blob/reconstruction/reconstruction/TARGET_FIELD.md) records the unresolved requirement, evidence reviewed, required contract inputs, and acceptance criteria.

This is the only exact match in the final branch because the ledger itself names the requirement. A real implementation still requires a definition of its purpose, owner subsystem, input/output contract, persistence or serialization format, invariants, failure behavior, and representative tests or benchmarks.

## Validation status

Structural validation passed. The final branch has no missing paths relative to `origin/main`, no deletions relative to `origin/main`, a clean diff check for the committed changes, valid JSON for the consolidated NSQ and WOWAS metadata, and a syntactically valid WOWAS TSV reconstruction script. The remote branch points to the same final commit as the local checkout.

Focused Rust compilation and tests could not be executed because the sandbox does not contain the `cargo` executable. This is recorded as an environment limitation, not a passing result. The branch contains the repository’s existing CI workflow for NSQ validation, so Rust checks remain available in CI or any environment with the declared Rust toolchain.

The WOWAS stage state is intentionally not represented as complete: it records stage 3 materialization as next, generator replay as pending execution environment, world materialization as pending, and validation as pending. This was preserved rather than upgraded based on a status label or benchmark claim.

## References

[1]: https://github.com/adriak85/Braxon "Braxon repository"
[2]: https://github.com/adriak85/Braxon/tree/reconstruction "Braxon Reconstruction branch"
[3]: https://github.com/adriak85/0 "adriak85/0"
[4]: https://github.com/adriak85/DAX-FULL "adriak85/DAX-FULL"
[5]: https://github.com/adriak85/Dax "adriak85/Dax"
[6]: https://github.com/adriak85/Dax-Autonomous-System "adriak85/Dax-Autonomous-System"
[7]: https://github.com/adriak85/PAPI "adriak85/PAPI"
[8]: https://github.com/adriak85/f1ux-service "adriak85/f1ux-service"
[9]: https://github.com/adriak85/fastapi-llm-bot "adriak85/fastapi-llm-bot"
[10]: https://github.com/adriak85/termux-packages "adriak85/termux-packages"
