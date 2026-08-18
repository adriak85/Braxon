# WOWAS Authored Character Flavor Audit

This audit read **14 source files**, identified **132 authored/lattice identifiers**, and compared them with **5000 generated identifiers**. It does not generate prose or rewrite original authored sources.

## Integrity boundary

> Character flavor is treated as structured guidance: voice, pressure response, role, relationship function, source stack, dynamic constraints, and continuity invariants. It is not converted into finished prose by this audit.

## Results

| Measure | Value |
|---|---:|
| Source files read | 14 |
| Authored/lattice identifiers | 132 |
| Generated identifiers | 5000 |
| Authored identifiers matched in generated registry | 0 |
| Authored identifiers not matched in generated registry | 132 |
| Dynamics evidence rows | 732 |

## Required realization safeguards

The attached synchronization note correctly identifies three safeguards: use `record_id` rather than raw `scene_id` for duplicate rows; stage prose and require tone/style/token checks before promotion; and carry a rolling state ledger across book boundaries. These are recorded as requirements, not claimed as executed realization behavior.

## Source inventory

| Source | Lines | Bytes | SHA-256 |
|---|---:|---:|---|
| `crates/wowas-final-edition-v10/canon/canonical_story_tree/characters/06_CHARACTER_REGISTRY.json` | 852 | 23197 | `69246bd4cf8299c5b93e6232ba60dcfa92140ecfee8b42b75ead0f9552860e42` |
| `crates/wowas-final-edition-v10/canon/wowas_character_timeline_lattice_v2.tsv` | 321 | 134029 | `a7654facc61d31fd8a7c9bbb16b63f8468dee3574879e4275087336b65fbe4f9` |
| `crates/wowas-final-edition-v10/canon/wowas_character_timeline_lattice_UNIFIED_v14.tsv` | 321 | 134029 | `3af7c2304985cfa25728f0975853f3639d5add779461756dca4a8c6ab425b4ab` |
| `crates/wowas-final-edition-v10/canon/wowas_orbit_file_v2.tsv` | 96 | 25246 | `e8a5d677babe50968769699bdc910a79a3d6a59994d17e560b5f7719633abcc9` |
| `crates/wowas-final-edition-v10/canon/wowas_protected_support_cast_v7.tsv` | 6 | 1914 | `201bf2a03dfffefa436951b5b3e10f0617c7878605574556473c6c2976065d67` |
| `crates/wowas-final-edition-v10/canon/control/prose_and_tone_guide_v14.json` | 106 | 10977 | `9ccf72419edca7aca40c791cb78d30ddf3e43593a67ed4912cedb7edc6ea951a` |
| `crates/wowas-final-edition-v10/canon/patches/PROSE_AND_TONE_GUIDE.json` | 93 | 12346 | `34e83c86426f5fa82536a03c8c13231160a3d224711da99cc1aa11df54b30efb` |
| `crates/wowas-final-edition-v10/canon/patches/v10/wowas_prose_and_tone_patch_v10.md` | 29 | 1047 | `9fa142d5de903583e3d906da5a5301b455d8d4b4e7e228ffda8b00f7960decc6` |
| `crates/wowas-final-edition-v10/canon/patches/v10/wowas_dialogue_and_play_insertion_patch_v10.md` | 192 | 7841 | `9dea730b262f95f7c60638f348d75755a2133abb4bfa37f2f3f19408d82b0dfd` |
| `crates/wowas-final-edition-v10/canon/patches/v12/wowas_quality_romance_calendar_and_resonance_patch_v12.md` | 261 | 6818 | `7d6513045e442a55b3a49b9eaf65b6da9333516e4c6b74477ad19da11af36305` |
| `crates/wowas-final-edition-v10/canon/patches/v12/wowas_scene_connector_cast_tracker_apply_patch_v12.md` | 248 | 6734 | `883a70def3040adc0ca2950e3a1f44a7a1cbbcb013cfe9021f3295498344de08` |
| `crates/wowas-final-edition-v10/canon/canonical_story_tree/characters/01_NAMED_CAST_TOP300.md` | 466 | 33822 | `9e8d1f1f37def9c4be3c40a9dddbab6e287d415243a882adbe421bfba02002a9` |
| `crates/wowas-final-edition-v10/canon/canonical_story_tree/characters/04_SOURCE_HERO_ENGINE.md` | 243 | 16338 | `94b176e0d22cb55a18cd732e12c9f17d863f303d98936a2d2cb23098b4a68d5d` |
| `crates/wowas-final-edition-v10/canon/canonical_story_tree/characters/05_SELF_CORRECTING_CANON_RULES.md` | 136 | 7312 | `c149f4427e3be31465bb05966c5308309af25772f76fd6d5abb8cd68b6de0bec` |

## Interpretation

The audit found **132 authored identifiers not directly matched** by the generated registry. Those should not be silently regenerated or flattened. They need explicit alias/canonical-ID mapping or a source-backed preservation lane. The presence of a generated row is not evidence that its voice, dynamic, or source-specific flavor has been faithfully carried over.
