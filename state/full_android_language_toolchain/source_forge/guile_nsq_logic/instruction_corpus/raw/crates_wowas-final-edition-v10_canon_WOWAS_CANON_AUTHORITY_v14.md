# WOWAS CANON AUTHORITY — v14
# Whispers of Willow and Stone
# Single authoritative surface. All prior READMEs, apply orders, and
# patch absorption registries are SUPERSEDED by this file.
# Load THIS file first. Do not recursively read older control bundles.

## WHAT THIS IS

25-book fantasy saga. 14,739 scenes. 319 monsters. 51+ tracked characters.
Full placement lattice covering all 25 books.

## CANON LOCKS (DO NOT OVERRIDE)

- Mack = half-mile willow tree with booming human speech. NOT a walking human.
- Aortic Labyrinth = DELETED. Only exists in backup dirs. Do not reactivate.
- Mack chrono decay = DELETED. Not canon. Do not reintroduce.
- Portal time after cursed Mack = exactly 400 years.
- Blood Cello = midpoint of Book 25, not the ending.
- Blood Cello final sentence = subject-verb-object only, no adjectives.
- Book 1 spine = Prophecy → Void-Fold → Boojay abuse arc → Dark Triad → Diamond Prison shatters → Blood Cello.
- Glass Forge / Glass Palace arc = Books 2–3 only. NOT Book 1.
- Pip (Indalwin Willowjayce) = he/him pronouns throughout. Pip is gay.
- Boojay = jackrabbit with broken poison-lit hind leg.
- Daisy May and Majiskii = two separate Great Danes (mother and daughter).
- Xethrolund's first form = blank azure-light book (his interface/diary).
- Pip's book = Endless Codex.
- Rolzen's magic = force/boom NOT fire. Never conflate Rolzen with Rylos.
- Rylos darkness must not telegraph early. Betrayal lands near Diamond Break.
- Xethrolund = glyph-compiled deterministic NSQ execution. Quantum is his line.
- Flux is Rolzen's line. Never swap.

## BOOK ROSTER (25 BOOKS)

| # | Title | Era Band | Key Beat |
|---|-------|----------|----------|
| 1 | Choices Make World | Era I | Prophecy → Blood Cello spine |
| 2 | The Diary and the Blue Light | Era I | Xethrolund introduced |
| 3 | The First Thread | Era I | Glass Forge begins |
| 4 | Crocodile Smile | Era I | Glass Palace arc |
| 5 | The Portal Opens | Era II | Portal war begins |
| 6 | The Refusal and the Wound | Era II | |
| 7 | The Rift Widens | Era II | |
| 8 | Stasis | Era II | Portal stasis |
| 9 | Runes and Wound | Era II | |
| 10 | The Trap Tightens | Era II | |
| 11 | Mack Falls | Era III | Mack becomes willow |
| 12 | The Diamond Breaks | Era III | Diamond Prison shatters |
| 13 | The Afterwake | Era IV — Post-Diamond | Pip emerges. 400 years visible. |
| 14 | The Long Reorientation | Era IV | Thorn Ash'Koral returns |
| 15 | Suffocating in Silence | Era IV | Pip finds Rolzen feral. Silence descends. |
| 16 | The Deepening World | Era IV | Quantum field first use. Grief settles. |
| 17 | Black Heart Scar | Era IV | Rolzen barely human. Flux field taught. |
| 18 | Strange Gravity | Era IV | Rylos/Xeth search. Fight. Devastating. |
| 19 | The Cracking Core | Era IV | Missouri→Oklahoma. Mack approaching recognizable. |
| 20 | The Beforemath | Era IV | Rolzen fully restored. |
| 21 | The Final Descent | Era IV | Rippers' last offensive. Pip refuses. |
| 22 | The Last Open Gate | Era IV | Corveth's final reckoning. Sorra's protocol. |
| 23 | The Final Resonance | Era IV | 42Hz. Xethrolund: he ran, he came, he arrived. |
| 24 | Born To Die | Era V | Neith fully revealed. Rylos mortal wound. |
| 25 | Death Is Rebirth | Era V | Blood Cello midpoint. Terminus. |

## AUTHORITATIVE FILES (LOAD THESE, IN THIS ORDER)

### 1. Canon Law
- `wowas_canon_v1.md` — Single canonical surface. All earlier control bundles absorbed.

### 2. Character Placement (all 25 books)
- `wowas_character_timeline_lattice_UNIFIED_v14.tsv` — Full lattice. Supersedes v2, patches v6/v10.

### 3. Scene Index
- `wowas_clean_scene_index_v2.tsv` — 14,739 scenes across 25 books. Full coverage.

### 4. Monster Registry
- `wowas_monster_species_registry_v8.tsv` — 319 species. Linked to corridors and zones.

### 5. Character Registry
- `canonical_story_tree/characters/06_CHARACTER_REGISTRY.json`

### 6. World
- `canonical_story_tree/world/wowas_world_zone_map.json`

### 7. Prose and Tone
- `patches/PROSE_AND_TONE_GUIDE.json`

### 8. Magic System
- `wowas_magic_system_patch_v10.md`

### 9. Final Authority Router
- `wowas_final_authority_system_v13.md` — 13-layer authority router. Use when in doubt.

## SUPERSEDED FILES (DO NOT LOAD)

These files are HISTORICAL RECORD only. Do not apply, load, or treat as current:

- `CURRENT_APPLY_ORDER_v10.md` — superseded by this file
- `CURRENT_APPLY_ORDER_v11.md` — superseded by this file
- `README_v8.md`, `README_v10.md` — superseded
- `BRAXON_ready_manifest_v7.md`, `BRAXON_ready_manifest_v8_addendum.md` — superseded
- `wowas_final_canon_control_bundle_v7.md` — absorbed
- `wowas_final_canon_control_bundle_v8_addendum.md` — absorbed
- `wowas_final_canon_control_bundle_v10_addendum.md` — absorbed
- `wowas_final_canon_control_bundle_v11_addendum.md` — absorbed
- `wowas_patch_absorption_registry_v13.md` — absorbed
- `wowas_character_timeline_lattice_v2.tsv` — superseded by UNIFIED_v14
- `wowas_character_timeline_lattice_patch_v6.tsv` — absorbed into UNIFIED_v14
- `wowas_character_timeline_lattice_patch_v10.tsv` — absorbed into UNIFIED_v14
- `scene_index_hub.backup.20260417_043025/` — BACKUP ONLY. Contains DELETED content.
- `scene_index_hub.backup.20260417_043152/` — BACKUP ONLY. Contains DELETED content.

## BACKUP DIRECTORIES — CONTAIN DELETED/WRONG CANON

The following directories exist for historical record ONLY.
They contain DELETED scenes including The Aortic Labyrinth.
DO NOT reactivate anything from these directories:

- `scene_index_hub.backup.20260417_043025/`
- `scene_index_hub.backup.20260417_043152/`

## LOADING RULE FOR BRAXON (CRITICAL)

BRAXON loads WoWaS one scene at a time via `BRAXON_core::resolve_address()`.
Do NOT load the full scene index into context at once.
Do NOT load the full character lattice into context at once.
Query by book+scene_id. The address resolver handles retrieval.
Loading the whole saga at once causes context overflow and model degradation.
That is not a WoWaS problem. That is a loading problem. Load by scene.

## WHAT STILL NEEDS TO BE DONE

- [ ] Event tracker: link scenes to character appearances per book
- [ ] Monster-to-scene linker: which monsters appear in which scenes
- [ ] Creature count per book (should match character count per book)
- [ ] Books 1-12 character lattice entries need scene index cross-reference
- [ ] Zaz full book coverage (currently B16-B19 only — appears earlier)

## VERSION

Authority: v14
Patches absorbed through: v13
Last lattice update: B13-B25 full cast placement
Scene index: 14,739 entries
Monsters: 319 species
Characters tracked: 51+
