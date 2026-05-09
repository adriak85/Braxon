# WoWaS Recursive Tree Rewrite Manifest v12

## Purpose
This manifest converts the current WoWaS apply strategy from:
- keep old scene trees
- keep later patch stacks
- hope the runtime keeps reapplying them correctly

into:
- rewrite the scene tree directly
- preserve one ordered truth inside the book files
- regenerate derived indexes from the rewritten tree
- keep patches only as provenance and migration law

The system should not need to keep rerunning patch debt forever if the tree can be rewritten into correct authority now.

---

## Canon root
`crates/wowas-final-edition-v10/canon`

---

## Rewrite priority
Rewrite the tree itself first.

Primary target surfaces:
- `scene_index_hub.backup.20260417_043152/books/...`
- equivalent repeated book / beat trees carrying the same scene families
- concrete `placed/*.md` beat files
- per-book `00_book.md`, `01_contained_beats.md`, `03_time_map.md`, `04_scene_spine.md`

Derived indexes and summary surfaces should be regenerated after the tree is rewritten.

---

## Recursive authority order

### Layer 1 — live tree truth
Treat the existing live tree as the starting body, not as disposable scaffolding.

Preserve especially:
- later-book placed beats already carrying concrete event truth
- Silence / Lucent / Ursula ordering where already present
- Blood Cello / rebirth lane files already carrying real event load
- concrete school / diary / Mack / early-bond scenes in Books 01–03
- Rolzen / escape / recovery / convergence lanes in Books 14–20

### Layer 2 — control bundles and addenda
Apply in this order:
1. `wowas_final_canon_control_bundle_v7.md`
2. `wowas_final_canon_control_bundle_v8_addendum.md`
3. `wowas_final_canon_control_bundle_v10_addendum.md`
4. `wowas_final_canon_control_bundle_v11_addendum.md`

Later addenda override earlier bundle language where conflict exists.

### Layer 3 — v9 polish lane
Apply as migration truth:
- `patches/v9/wowas_v8_polish/wowas_morph_description_patch_v9.tsv`

### Layer 4 — v10 patch lane
Treat these as first-class migration authority:
- `patches/v10/wowas_anti_placeholder_expansion_patch_v10.md`
- `patches/v10/wowas_book_build_selection_order_patch_v10.md`
- `patches/v10/wowas_books_24_25_priority_override_patch_v10.md`
- `patches/v10/wowas_character_timeline_lattice_patch_v10.tsv`
- `patches/v10/wowas_dialogue_and_play_insertion_patch_v10.md`
- `patches/v10/wowas_magic_system_patch_v10.md`
- `patches/v10/wowas_orbit_patch_v10.tsv`
- `patches/v10/wowas_prose_and_tone_patch_v10.md`
- `patches/v10/wowas_scene_authority_cleanup_patch_v10.md`
- `patches/v10/wowas_scene_patch_v10.tsv`

### Layer 5 — flat patch surfaces still carrying truth
Reconcile and absorb:
- `wowas_scene_patch_v6.tsv`
- `wowas_scene_patch_v10.tsv`
- `wowas_scene_patch_v11.tsv`
- `wowas_orbit_file_v2.tsv`
- `wowas_orbit_patch_v6.tsv`
- `wowas_orbit_patch_v10.tsv`
- `wowas_character_timeline_lattice_v2.tsv`
- `wowas_character_timeline_lattice_patch_v6.tsv`
- `wowas_character_timeline_lattice_patch_v10.tsv`
- `timeline_lattice.tsv`
- `wowas_corridor_encounter_pressure_patch_v8.tsv`
- `wowas_county_creature_patch_v6.tsv`
- `wowas_ecology_pressure_rules_v2.md`
- `wowas_county_corridor_pressure_map_v1.tsv`
- `wowas_county_corridor_pressure_map_v2.tsv`
- `wowas_american_morphed_life_lattice_v2.tsv`
- `wowas_monster_species_registry_v8.tsv`
- `wowas_monster_count_alignment_v8.md`
- `wowas_endgame_judgment_matrix_v10.tsv`
- `wowas_conflict_ledger_v6.tsv`
- `wowas_arc_insertion_registry_v7.tsv`
- `wowas_protected_support_cast_v7.tsv`
- `wowas_clean_scene_index_v2.tsv`

Rule: use the latest lane when two patch files disagree unless a later addendum overrides it explicitly.

---

## Hard conversation truth to preserve

### Character / relationship law
- Rolzen and Rylos are distinct characters and must never be conflated.
- Main cast is adult-coded now, roughly 20–29, not the older younger branch.
- Rylos darkness should not telegraph early; betrayal lands near the Diamond Break, not long beforehand.
- Pip spends 400 years in portal war / stasis under Xethrolund.
- Rolzen is found feral and mutilated; Pip heals him in captivity and their bond forms through healing, escape, and years of recovery.
- Rolzen is force / boom, crystal-ball / gravity-juggle based.
- Pip practices sonomancy constantly in secret and hides the scale of it.

### World / sequence law
- Silence -> Lucent -> Ursula ordering must remain correct.
- Later-book placed beats already carrying that lane should be preserved, not flattened away.
- Books 24–25 must cash out through wound, accountability, waking, changed return, fragile joy, and lived obligation.

### Manuscript law
- scaffold density does not equal completion
- reconstruction filler is debt until realized into lived prose
- direct scene anchors outrank count-padding rows

---

## Scene file enrichment requirement
While rewriting the scene tree, enrich each meaningful scene file with metadata blocks supporting at least:

### Connector block
- `scene_id`
- `beat_id`
- `connector_in`
- `connector_out`
- `must_follow`
- `must_precede`
- `depends_on_scene_ids`
- `opens_statement_ids`
- `resolves_statement_ids`
- `carries_forward`

### Event / obligation block
- `open_statements`
- `resolution_targets`
- `timing_constraints`
- `hard_due_before`
- `hard_due_after`
- `blocked_by`
- `pays_off`
- `orphan_risk`

### Cast presence block
- `present_characters`
- `role_primary`
- `role_secondary`
- `witnesses`
- `offscreen_pressures`
- `background_named_presence`
- `background_generated_presence`
- `absence_that_matters`

### Image block
- `image_status`
- `image_priority`
- `featured_visual_subjects`
- `missing_visual_references`
- `best_picture_book_targets`

Rule: the scene file should become the first place truth lives.

---

## Generator-character integration rule
While traversing the tree, insert the best generated characters deliberately rather than randomly.

Priority insertion targets:
- underpopulated corridor and settlement scenes
- sanctuary / city / prison / village scenes needing social density
- aftermath and recovery scenes needing witnesses and texture
- support-cast gaps in Books 14–25
- worldbuilding scenes where a distinctive local person would make the place inhabited

Selection fields to preserve inside scene files:
- `candidate_generated_characters`
- `selected_generated_characters`
- `selection_reason`
- `story_function`
- `region_fit`
- `survival_fit`
- `repeat_presence_plan`

---

## Rewrite order by book cluster
1. Books 24–25
2. Silence / Lucent / Ursula lanes
3. Books 14–20 recovery / Rolzen / escape / convergence lanes
4. Books 01–03 school / prophecy / diary / Mack / early-bond lanes
5. remaining books outward from strongest anchors

---

## Done condition
This pass is ready to call complete only when:
- scene tree truth no longer depends on runtime patch replay
- placeholder book anchors are rewritten into real ordered authority
- scene connectors and obligation hooks are present
- cast presence is explicit
- generated-character insertion logic is embedded in the tree
- derived indexes can be regenerated from the rewritten tree without losing truth
