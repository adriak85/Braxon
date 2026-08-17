# WOWAS Full Construction — Execution Index

Source of truth: `adriak85/Braxon` `main`.
Target corpus: `crates/wowas-final-edition-v10`.
Construction branch: `wowas-full-construction-20260816`.

## Authority chain recovered from the crate

1. `canon/WOWAS_CANON_AUTHORITY_v14.md`
2. `canon/control/WOWAS_SOURCE_OF_TRUTH_REGISTRY_v14.md`
3. `canon/control/prose_and_tone_guide_v14.json`
4. `canon/control/PATCH_INGESTION_LEDGER_v14.md`
5. `canon/control/magic_system_control_v14.*`
6. `canon/control/character_placement_control_v14.*`
7. `canon/wowas_canon_v1.md`
8. `canon/wowas_character_timeline_lattice_UNIFIED_v14.tsv`
9. `canon/wowas_clean_scene_index_v2.tsv`
10. `canon/wowas_monster_species_registry_v8.tsv`
11. `canon/canonical_story_tree/`
12. `canon/wowas_final_authority_system_v13.md` for routing/audit
13. `canon/wowas_final_authority_manifest_v13.json` for routing/audit

## Existing SERIEL/navigation surfaces

The recovered source already contains multiple explicit navigation/index surfaces. They are preserved rather than replaced:

- `canon/active/book_spine_33.tsv`
- `canon/active/character_timeline_lattice_v14_33.tsv`
- `canon/active/scene_index_33.tsv`
- `canon/canonical_story_tree/_scene_heading_index.tsv`
- `canon/wowas_clean_scene_index_v2.tsv`
- `canon/wowas_character_timeline_lattice_UNIFIED_v14.tsv`
- `canon/wowas_monster_species_registry_v8.tsv`

These are treated as navigational graph inputs. No new indexing convention is allowed to erase their identity.

## Graph model

The reconstruction graph is keyed by stable source identity and connects:

`SERIEL -> book -> scene -> microevent -> character -> relationship/orbit -> location/atlas -> faction -> magic state -> world state -> creature -> consequence -> prose rule -> source provenance`

A node may participate in multiple lattices simultaneously. Source lineage remains attached to every derived node.

## Canon authority conflict handling

The v14 authority surface states that the active canonical saga is 25 books, 14,739 scenes, 319 monsters, and 51+ tracked characters. The active 33-book spine also exists and explicitly marks books 26–33 as inserted-capacity material. This is retained as a first-class conflict/variant rather than silently discarded. The v14 source-of-truth registry explicitly permits expansion to 33 books if cast/creature density requires it.

The construction therefore preserves both surfaces and resolves them through authority order plus capacity rules; it does not delete the 33-book index.

## Character and creature scale

The source explicitly defines:

- 5,000 story characters as the expansion target.
- 2,000,000 ancillary people primarily for atlas/game/CYOA/world population.
- 5,000 creatures as the reviewed expansion target.
- The currently authoritative monster registry reports 319 species.

Generated candidates are not automatically promoted. Weak, derivative, tasteless, or compromised concepts are routed to rehash/review.

## Scene-generation law

Scene density does not equal manuscript completion. Concrete placed scenes and direct source extracts outrank shorthand, reconstruction rows, and count-padding. Placeholder language must be realized into lived scenes before reader-facing prose.

The current authority explicitly requires scene-level retrieval rather than loading the complete scene index into context. This reconstruction therefore keeps source objects segmentable and addressable rather than flattening the entire corpus into one prompt-sized artifact.

## Historical material

Superseded and backup material is retained as provenance/audit evidence. It is not promoted into active canon merely because it exists. The authority specifically identifies deleted Aortic Labyrinth material and chrono-decay material as historical-only.
