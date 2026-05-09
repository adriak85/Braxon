# WoWaS Scene Connector, Impact Tracker, and Cast Integration Patch v12

## Purpose
This patch changes the apply strategy from "keep patches around and hope the system runs them later" to "rewrite the scene tree into ordered truth."

The best place to do that is the per-scene book tree itself.

While rewriting the scene files, Braxon should add:
- scene-to-scene connectors
- event / obligation tracking hooks
- impact tracking hooks
- full cast presence with role-in-scene
- generator-character insertion decisions
- image / illustration coverage status

The scene tree then becomes the real authority surface.
Patches become migration instructions and provenance, not a permanent runtime burden.

---

## Core law
Do not make the final system depend on rerunning patch stacks at runtime.

Instead:
1. read the patch lanes
2. rewrite the scene tree into correct order and truth
3. enrich the scene files with the missing connector and tracker metadata
4. regenerate derived indexes from the rewritten tree

---

## Primary rewrite target
The preferred rewrite surface is the scene/book tree carrying repeated beat files and placed scene anchors, especially:
- `scene_index_hub.backup.20260417_043152/books/...`
- other equivalent duplicated per-book scene trees preserving the same beats
- placed beat files already carrying concrete event truth

The goal is to make those files hold the truth directly.

---

## Scene file enrichment requirements
Each scene file should gain a structured metadata block that supports at least the following categories.

### 1. Connector block
Each scene should declare how it connects into nearby scenes.

Required fields:
- `scene_id`
- `book_id`
- `beat_id`
- `connector_in`
- `connector_out`
- `must_follow`
- `must_precede`
- `soft_follow`
- `soft_precede`
- `depends_on_scene_ids`
- `opens_statement_ids`
- `resolves_statement_ids`
- `contradicts_statement_ids`
- `carries_forward`

Rule:
A scene should not float as an isolated beat title if its dramatic function depends on prior cause or later consequence.

### 2. Event / obligation tracker block
Every scene that creates future burden should explicitly declare it.

Required fields:
- `open_statements`
- `resolution_targets`
- `timing_constraints`
- `hard_due_before`
- `hard_due_after`
- `soft_due_before`
- `blocked_by`
- `pays_off`
- `orphan_risk`

Statement classes should include:
- mystery
- promise
- causal requirement
- ordering requirement
- emotional requirement
- world-state requirement

State classes should include:
- open
- soft_due
- hard_due
- resolved
- deferred_with_reason
- invalidated_with_reason
- violated_order
- orphaned
- contradicted

### 3. Impact tracker block
Each scene should also expose impact categories.

Required fields:
- `reader_impact_primary`
- `reader_impact_secondary`
- `public_read_risk`
- `tone_risk`
- `meme_risk`
- `misread_risk`
- `ecology_or_world_impact`
- `relationship_impact`
- `cost_marker`
- `recovery_marker`
- `spectacle_vs_humanity_balance`

Rule:
This tracker exists to catch cases where a concept technically functions in-world but reads badly or weakly to readers.

### 4. Cast presence block
Every scene should declare all meaningful participants and their role in the scene.

Required fields:
- `present_characters`
- `role_primary`
- `role_secondary`
- `role_tertiary`
- `witnesses`
- `offscreen_pressures`
- `background_named_presence`
- `background_generated_presence`
- `absence_that_matters`

Role classes may include:
- focal
- counterforce
- witness
- protector
- pressure source
- explainer
- emotional pivot
- comic relief
- recovery anchor
- world witness
- catalyst

Rule:
A scene should say not only who is there, but what each person is doing structurally.

### 5. Generator-character insertion block
This is the right place to integrate the strongest generated characters into the storyline.

Required fields:
- `candidate_generated_characters`
- `selected_generated_characters`
- `selection_reason`
- `story_function`
- `region_fit`
- `survival_fit`
- `repeat_presence_plan`
- `drop_if_redundant`

Rule:
Generated characters should not be sprayed randomly.
They should be inserted where:
- the place needs social density
- the conflict needs witnesses, helpers, victims, rivals, or texture
- the scene benefits from a distinct person rather than generic crowd language
- later recurrence can pay off

### 6. Image coverage block
Since visual population is also being expanded, each scene should record image status.

Required fields:
- `image_status`
- `image_priority`
- `featured_visual_subjects`
- `missing_visual_references`
- `best_picture_book_targets`

Possible values:
- none
- candidate
- priority
- illustrated
- needs_refresh

Rule:
This allows scene-level picture-book style expansion later without losing canon order.

---

## Rewrite doctrine
When rewriting the tree:

1. keep the strongest concrete placed beats
2. merge patch truth into the actual scene files
3. add connector / tracker / cast / image metadata directly into those files
4. demote or remove placeholder-only entries once their truth has been absorbed into lived scenes
5. regenerate summary indexes from the rewritten scene files

Do not keep truth split between:
- the scene file
n- the patch note
- the summary index
- the conversation memory

The scene file should become the first place truth lives.

---

## Best-candidate integration law
While traversing the scene files, Braxon should also begin placing the best characters from the generator throughout the storyline.

Priority insertion targets include:
- underpopulated but socially consequential transition scenes
- village / city / sanctuary / prison / corridor scenes
- aftermath scenes that need witnesses and texture
- support-cast gaps in Books 14–25
- worldbuilding scenes where a distinctive local person would make the place feel inhabited

Generated-character insertion should obey:
- role clarity
- region fit
- conflict fit
- recurrence potential
- no flattening of core cast

---

## Books with highest immediate payoff
Apply this enrichment first to:
1. Books 24–25
2. Silence / Lucent / Ursula-adjacent lanes
3. Books 14–20 recovery / Rolzen / escape / convergence lanes
4. Books 01–03 school-life / prophecy / diary / Mack / early-bond lanes

---

## Outcome rule
After applying this patch correctly:
- the tree itself holds the truth in the right order
- scene files know what they connect to and what they owe
- unresolved or mistimed beats become visible
- cast presence becomes explicit
- generated characters can be integrated deliberately
- image population can follow real canon instead of guesswork
- the system no longer depends on rerunning patch stacks to know what the story is
