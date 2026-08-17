# WOWAS Original 15,000-Row Recursive Metadata Alignment

## Authority starting point

This release starts from the **origin/main active scene index** and retains the exact historical 15,000-row artifact as a separately labeled candidate layer. The exact artifact is preserved because the requested 15,000-row source is part of reachable Git history, but it is not silently treated as newer or more authoritative than `main`. The alignment output therefore retains both source lineages and records their hashes.

| Layer | Rows | Role |
|---|---:|---|
| `origin/main` active scene index | 13,889 | Main-branch starting surface |
| Exact recovered 15,000-row candidate | 15,000 | Historical candidate, preserved without promotion by label |
| v6 scene patch additions | 13 | Explicit patch rows |
| Generated character candidates | 5,000 | Structured metadata only |
| Character encounter beat candidates | 821 | Structured event/beat metadata only |
| Wildlife encounter candidates | 100 | Structured creature/event metadata only |
| Desert population candidates | 150 | Structured world/population metadata only |
| **Total aligned metadata rows** | **34,973** | Includes all source layers and candidate lanes |

The full output is `crates/wowas-final-edition-v10/canon/active/reconciled_15000/scene_index_reconciled_metadata.tsv`. The manifest and validator are adjacent to it.

## Sequential patching

The reconciler applies explicit scene-table changes in version order: v6 scene additions, v10 scene rules, and v11 scene/dialogue laws. Every patch file is hashed and recorded in the manifest. Patch rules that match a book band or target receive patch identifiers on the affected metadata rows. Unmatched rules remain visible in the patch ledger for later resolution rather than being silently discarded.

The broader patch/update inventory is recursively enumerated from canon patch and control surfaces. Duplicate patch copies are retained by path and content hash; identical copies are not counted as independent instructions. Later prose, tone, dialogue, romance, calendar, resonance, and recursive-tree documents govern future realization but do not cause prose to be generated during this metadata alignment.

## Character and beat expansion

The alignment includes structured candidates from the main-branch character generator and event generators. These records carry explicit `record_kind` values such as `character_candidate`, `beat_candidate`, `encounter_candidate`, and `world_population_candidate`. They are linked to book, character, creature, world, route, or event fields where the source provides them. They are **not authored prose**, are not counted as completed manuscript scenes, and remain eligible for later source review and prose realization.

## Duplicate handling

There are **365 duplicate scene-ID groups** in the combined metadata surface. No duplicate was removed merely because another row appeared more canonical by filename or label. Every row has a distinct `record_id`, source path, record kind, and provenance lineage. Duplicate groups are written to `duplicate_scene_id_ledger.json` with the policy `preserve_all_rows; do_not_collapse_without_source_resolution`.

## Prose boundary

No generated prose was imported or created by this release. Every aligned record is marked `prose_status=no_generated_prose`. A structured candidate can describe a possible beat, relationship, encounter, character use, or world introduction, but it cannot be promoted to finished prose or manuscript completion without a separate realization and quality gate.

## Current truth

This release corrects the earlier 408-row collapse. The 408-row set was an authority-eligibility subset under the anti-scaffold rule, not an acceptable complete metadata index. The present aligned surface preserves the exact 15,000-row candidate, the main starting surface, explicit patch additions, and structured character/beat/world candidates while keeping status and provenance visible.
