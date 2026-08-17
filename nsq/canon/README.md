# NSQ Canonical Reconstruction

This directory is the canonical landing zone for intent extracted from the repository at commit `ced6af253888bf194375692de86d0678dc70d847`.

The repository is treated as a source corpus, not as a clean architecture. The reconstruction rule is:

1. Inspect every source surface and preserve intent, semantics, invariants, measurements, and proof evidence.
2. Do not preserve historical duplication merely because it exists in multiple paths.
3. Do not promote generated output, backups, stale snapshots, or wrappers into architectural authority.
4. Keep benchmark evidence attached to the capability it measures.
5. NSQ is the canonical representation and runtime substrate; Rust, shell, C, Python, generated files, and historical layouts are source evidence or implementation substrates, not competing architectural authorities.
6. A capability becomes canonical only when its intent can be stated, its source evidence can be named, and its invariant/proof status is explicit.

Canonical layers:

- `intent_map.nsq` — extracted semantic map.
- `benchmark_truth.json` — benchmark evidence promoted from the repository's result surfaces.
- `migration_rules.md` — rules for deciding what is canonical, supporting, historical, or disposable.
- `inventory.md` — repository-wide directory inventory and intent classification.
- `rebuild.nsq` — ordered reconstruction graph.

The original corpus remains untouched on this branch. This is deliberate: reconstruction happens by extraction, not blind copying.
