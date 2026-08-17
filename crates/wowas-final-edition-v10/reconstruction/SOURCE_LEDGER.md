# WOWAS Source Ledger

This ledger records source surfaces recovered from `adriak85/Braxon` `main` without collapsing historical variants into one undifferentiated document.

| Surface | Role | Authority state | Notes |
|---|---|---|---|
| `canon/WOWAS_CANON_AUTHORITY_v14.md` | first-load canon authority | active | Explicit v14 locks and loading law |
| `canon/control/WOWAS_SOURCE_OF_TRUTH_REGISTRY_v14.md` | control registry | active | Defines load order, cross-links, expansion targets |
| `canon/wowas_canon_v1.md` | cohesive canon | active | Manuscript/scene law and core canon locks |
| `canon/active/book_spine_33.tsv` | book navigation | active/index | 33-row capacity spine; B26-B33 are inserted-capacity rows |
| `canon/active/character_timeline_lattice_v14_33.tsv` | character lattice | active/index | Large generated placement surface; must be consumed in segments |
| `canon/active/scene_index_33.tsv` | scene navigation | active/index | Large scene index; source explicitly says not to load whole index into context |
| `canon/wowas_clean_scene_index_v2.tsv` | clean scene authority | active | v14 authority calls this the 14,739-scene surface |
| `canon/wowas_character_timeline_lattice_UNIFIED_v14.tsv` | character authority lattice | active | Supersedes older character lattice variants |
| `canon/wowas_monster_species_registry_v8.tsv` | creature registry | active | 319 current species; expansion target 5,000 |
| `canon/canonical_story_tree/` | canonical asset tree | active | Books, characters, world and scene-heading index |
| `canon/CURRENT_APPLY_ORDER_v10.md` | historical apply order | superseded | Preserved because source says all historical material remains auditable |
| `canon/CURRENT_APPLY_ORDER_v11.md` | historical apply order | superseded | Same rule |
| backup scene-index directories | historical/deleted | quarantine | Authority explicitly forbids reactivation |

## Recovered structural facts

- Package name: `wowas-final-edition-v10`.
- Package manifest declares the canon/control surfaces above.
- `src/lib.rs` exposes the same surfaces as runtime constants and provides an explicit generation source order.
- The runtime source order ends with the canonical story tree, characters, world, and the v13 authority router.
- The active v14 authority describes a 25-book canonical core while the active 33-book spine provides inserted capacity. The source-of-truth registry explicitly permits the 33-book expansion when cast density requires it. Both must therefore remain represented in the graph.

## Explicit construction debt recovered from the authority

The v14 authority itself identifies unfinished cross-link work:

1. Event tracker linking scenes to character appearances per book.
2. Monster-to-scene linker.
3. Creature count per book.
4. Books 1–12 character-lattice scene cross-reference.
5. Zaz full-book coverage.

These are construction tasks, not evidence that the source files should be ignored.
