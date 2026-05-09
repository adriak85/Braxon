# Lattice Overlap — Reconciliation Note

## The issue

`wowas_american_morphed_life_lattice_v2.tsv` (122 rows) and `wowas_monster_species_registry_v8.tsv` (319 rows) partially overlap. Both contain ecological pressure entries for some of the same corridor counties. A BRAXON ingest that loads both without disambiguation could pull two conflicting descriptions for the same ecological pressure zone.

## Resolution rule

Treat the files as **separate layers with distinct purposes**:

- `wowas_monster_species_registry_v8.tsv` = **encounter-ready species** — fully specified for fight integration, encounter templates, and scene insertion. This is the operative runtime layer.

- `wowas_american_morphed_life_lattice_v2.tsv` = **world-texture seed layer** — atmospheric ecological background that makes counties feel inhabited. These entries are NOT encounter-ready and should NOT be pulled into fight generation logic.

## BRAXON ingest instruction

When loading both files, apply this priority rule:
- If `monster_code` exists in both files → use the `v8_monster_registry` entry for all encounter logic
- Use `life_lattice` entries ONLY for:
  - background ecological description in travel prose
  - county-color generation (what does this place smell like, what grows here, what calls at night)
  - weather and terrain texture
- NEVER use `life_lattice` entries as encounter triggers

## Overlap entries that need explicit disambiguation

The following species concepts appear in both files under different names. The v8 registry entry is authoritative for encounter use; the lattice entry is atmospheric only:

| County | Life Lattice Concept | v8 Registry Equivalent |
|--------|---------------------|------------------------|
| Canadian County, OK | Silo-Breath Bluestem | wowas::canadian_county_flora_01 (Bone-dust Cottonwood zone) |
| Larimer County, CO | Resin-Breath Lodgepole Rot | wowas::larimer_county_bird_01 (Front-Range Hawk zone, fungal supplement) |
| Crawford County, MO | Various cave fungi | wowas::crawford_county_stone_01 and wowas::crawford_county_amphibian_01 |
