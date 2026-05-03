# WoWaS Source-of-Truth Registry v14

Status: installed control surface.
Authority: `canon/WOWAS_CANON_AUTHORITY_v14.md`.

This file is the v14 registry for final-home controls. It is not a patch file.

## Load order

1. `canon/WOWAS_CANON_AUTHORITY_v14.md`
2. `canon/control/WOWAS_SOURCE_OF_TRUTH_REGISTRY_v14.md`
3. `canon/control/prose_and_tone_guide_v14.json`
4. `canon/control/PATCH_INGESTION_LEDGER_v14.md`
5. `canon/control/magic_system_control_v14.md`
6. `canon/control/character_placement_control_v14.md`
7. `canon/wowas_canon_v1.md`
8. `canon/wowas_character_timeline_lattice_UNIFIED_v14.tsv`
9. `canon/wowas_clean_scene_index_v2.tsv`
10. `canon/wowas_monster_species_registry_v8.tsv`
11. `canon/canonical_story_tree/`
12. `canon/wowas_final_authority_system_v13.md` for routing/audit support only
13. `canon/wowas_final_authority_manifest_v13.json` for routing/audit support only

## First-load authority

`WOWAS_CANON_AUTHORITY_v14.md` is the first-load authority for current canon.
Older v10/v11/v13 apply-order or patch-absorption files are retained as history/audit unless their instructions have been installed here or in a v14 final control file.

## Canon identity locks

- Pip is male.
- Pip uses he/him.
- Pip is gay; the people Pip is romantically/sexually interested in are male.
- Pip is not a robot.
- Pip is a neurodivergent chipmunk in the first arc.
- Pip becomes human through the portal / Diamond Break transition path.
- Rylos Vayne Johnson is the active canonical replacement for the deprecated Boojay name/branch.
- Boojay is deprecated/history only unless a file is explicitly describing the old branch.
- Chrono decay is not active canon.
- Aortic labyrinth is not active canon.
- Hidden old first-book prose must not be promoted to completed canon.

## Control classes

| Class | Final source | Notes |
|---|---|---|
| Canon authority | `canon/WOWAS_CANON_AUTHORITY_v14.md` | First load. |
| Prose/tone | `canon/control/prose_and_tone_guide_v14.json` | Final guide for diction, POV, Pip cognition, scene expansion, and failed-mode filters. |
| Patch ingestion | `canon/control/PATCH_INGESTION_LEDGER_v14.md` | Patch folders are law while being ingested, then history only. |
| Magic systems | `canon/control/magic_system_control_v14.md` | Final-home magic/control surface. |
| Character placement | `canon/control/character_placement_control_v14.md` | Final-home scene/character placement and scale surface. |
| Scene index | `canon/wowas_clean_scene_index_v2.tsv` and `canon/canonical_story_tree/_scene_heading_index.tsv` | Must cross-link to characters, atlas, orbit, magic, world state, arcs, and microevents. |
| Character lattice | `canon/wowas_character_timeline_lattice_UNIFIED_v14.tsv` | Active lattice for placements and timeline logic. |
| Creature registry | `canon/wowas_monster_species_registry_v8.tsv` plus future v14 creature control | Must expand toward ~5,000 reviewed creatures. |
| Canon tree | `canon/canonical_story_tree/` | Asset-folder style active story tree. |

## Patch law

Patch folders are not useless. They are instruction law until ingested.

After ingestion:

- Their content must live in final control files or canonical assets.
- Patch files must not remain the active source of truth.
- Patch folders may be archived/quarantined only after the ledger records where each patch was applied.
- Most recent valid instruction wins when two sources conflict.

## Expansion targets

- 25-book v14 structure remains active unless cast density forces inserted books/volumes.
- 33-book expansion is allowed for capacity if 5,000 story characters and 5,000 creatures cannot breathe in 25 books.
- Scene expansion target is generally 3,000–5,000 words where needed.
- 14.5k–14.7k scene surface implies roughly 43.5M–73.7M words at 3k–5k average.
- Grok/Brock estimate of ~94M words remains a high-end planning estimate to reconcile against final scene count and average scene length.
- 5,000 story characters need dossier/orbit/lattice/impact/microarc placement.
- 2,000,000 ancillary people are primarily atlas/game/CYOA/world population, not all prose-frontline cast.
- 5,000 creatures need originality/taste review and rehash queue for weak concepts.

## Required cross-links per scene beat

Every scene beat should resolve, when relevant, to:

- book/volume
- absolute timeline position
- scene ID
- microevent ID
- arc and microarc IDs
- POV lock: Pip unless an explicit authority file grants exception
- characters in order of appearance
- character codes
- dossiers
- orbit/relationship lattice
- atlas/location
- faction/region
- magic system state
- world state
- creature/monster entries
- consequence and impact score
- prose/tone guide rule set

## Rehash rules

Rehash anything that is weak, derivative, tasteless, or compromised.
The specific failure class called out by the author is the old `rape elk`-type failure: creature horror must not collapse into crude exploitative shock naming or lazy violation concepts.

## Install posture

This registry is installed so crate code and future tools can route through v14 controls instead of loose patch folders or deprecated apply-order files.
