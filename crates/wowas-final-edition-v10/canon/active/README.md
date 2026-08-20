# Active WoWaS Canon

This directory is the authoritative active narrative surface for the reconstruction branch.

Authority order:

1. `book_spine_33.tsv` — canonical serial order and book identities.
2. `wowas_clean_scene_index_v2.tsv` at the parent `canon/` path — canonical scene inventory consumed by the current story-seed implementation.
3. `character_timeline_lattice_v14_33.tsv` — canonical character continuity.
4. `canon_laws.tsv` and `canon_blocklist.tsv` — inclusion/exclusion rules.
5. `provenance_registry.tsv` — absorbed-source provenance only; it does not override canon.
6. `final_canonical_projection.md` — final-state projection, not an independent story authority.

Legacy, deprecated, rescue, patch, review, and superseded material must not be promoted into this active surface unless an explicit canonical decision restores it. Unresolved absorption material belongs under `canon/audit/`, not active canon.

Derived indexes, lattices, character controls, watermark/reflex surfaces, creature registries, and transmedia projections must agree with the authority order above. A stale derived surface is repaired from active authority; it is never allowed to become a competing authority.

The active story runtime must consume canonical scene records rather than synthetic demonstration content.
