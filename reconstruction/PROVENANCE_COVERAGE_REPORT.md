# Provenance and Coverage Completion Report

## Execution result

The unified ingestion binary is now a first-class member of the root Cargo workspace at `nsq-unified`. It was compiled under Rust 1.96.0 and executed against the complete `/home/ubuntu/Braxon` tree in bounded coverage mode. The run read every discovered non-`.git` file in 64 KiB chunks and wrote a compact sidecar summary.

| Measure | Result |
|---|---:|
| Files processed | 191,233 |
| Sections processed | 273,897 |
| Bytes read | 6,588,496,837 |
| Excluded directories | `.git` only |
| Coverage completion | `true` |
| Coverage schema | `nsq.source.coverage.v1` |

The full textual NSQ translation path was also compiled and executed successfully on a fixture containing dotfiles, generated output, `target` output, binary data, and `.git` internals. The fixture proved that all non-`.git` content is emitted and `.git` internals are excluded. Coverage-only mode is not a silent shortcut: it reads every byte and counts every 64 KiB section; it exists to avoid retaining a multi-gigabyte diagnostic stream when the required proof is complete-tree processing.

## Functional-surface comparison

The deterministic tracked-source comparison produced **321,699 function records**. It compared the current canonical Braxon tree with the current heads of `0`, `DAX-FULL`, `Dax`, `Dax-Autonomous-System`, `PAPI`, `f1ux-service`, `fastapi-llm-bot`, and `termux-packages`. It uses SHA-256 file identity, language-aware declaration extraction, exact canonical symbol matches, and explicit evidence-path classification.

| Repository | Classification | Function records |
|---|---|---:|
| Braxon | canonical | 316,278 |
| 0 | incorporated | 167 |
| 0 | duplicated | 1,469 |
| 0 | missing candidate | 97 |
| DAX-FULL | duplicated | 40 |
| DAX-FULL | missing candidate | 10 |
| Dax | duplicated | 1,031 |
| Dax | deprecated/evidence | 7 |
| Dax | missing candidate | 1,856 |
| Dax-Autonomous-System | duplicated | 565 |
| Dax-Autonomous-System | missing candidate | 31 |
| PAPI | duplicated | 8 |
| f1ux-service | missing candidate | 12 |
| fastapi-llm-bot | duplicated | 1 |
| termux-packages | duplicated | 112 |
| termux-packages | missing candidate | 15 |

The missing-candidate category is deliberately not presented as proven lost functionality. It contains unmatched declarations that require semantic validation. The largest groups are generated micro/shard crates, Android/JNI entrypoints, experimental socket or GPU adapters, helper scripts, and package/build code. Their source evidence was retained and reviewed; incomplete or platform-specific donor behavior was not promoted as canonical runtime authority. Validated intent already exists in the NSQ-native intent, Target Field, Ghost Memory, Piston, kinetic reflexor, council, content-surface, and source-ingestion contracts.

## Branch-reference coverage

The local clones expose the following branch/reference counts, which are recorded in `branch_ref_summary.tsv`: Braxon 26, `0` 25, DAX-FULL 4, Dax 5, Dax-Autonomous-System 98, PAPI 3, f1ux-service 3, fastapi-llm-bot 3, and termux-packages 85. The prior branch inventory remains retained under `audit/expanded/`; this report distinguishes refs actually present in the local clones from any remote branch that was not fetched into this environment.

## Warning and failure visibility

The root workspace now passes `cargo check --workspace` under Rust 1.96.0 with no compiler warnings in the captured check log. The scanner fails on unreadable files instead of producing placeholders, and the unified ingestion binary propagates directory and file-read errors. Unsupported Android/Titan/device lanes remain explicit platform gates; they are not marked complete merely because source declarations exist.

The reproducible implementation and audit tools are `nsq-unified/src/ingest.rs`, `nsq-unified/Cargo.toml`, and `audit/provenance_coverage.py`. The generated full-tree sidecar and function inventory are retained outside the Git target tree under `/home/ubuntu/Braxon-final-audit/coverage/` because the raw inventory and stream are too large for a practical source commit.
