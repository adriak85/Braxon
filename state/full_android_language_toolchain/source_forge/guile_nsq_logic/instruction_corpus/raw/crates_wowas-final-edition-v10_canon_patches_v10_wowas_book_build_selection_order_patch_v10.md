# WoWaS Book Build Selection Order Patch v10

## Purpose
This patch tells Braxon how to choose scene rows when building a manuscript-facing book pass.

The scene-authority cleanup patch defined what kinds of rows are stronger or weaker.
This patch defines the actual pick order, quota logic, and cluster controls so Braxon stops
choosing the densest scaffold family instead of the best real scene anchor.

Use with:
- `patches/PROSE_AND_TONE_GUIDE.json`
- `patches/v10/wowas_dialogue_and_play_insertion_patch_v10.md`
- `patches/v10/wowas_book_dialogue_seed_registry_v10.md`
- `patches/v10/wowas_anti_placeholder_expansion_patch_v10.md`
- `patches/v10/wowas_books_24_25_priority_override_patch_v10.md`
- `patches/v10/wowas_scene_authority_cleanup_patch_v10.md`

## Core law
Book build order must be driven by scene quality and distinct event value, not row count.

When a section contains:
- a few concrete anchors
- many editorial shorthand rows
- many repeated pressure-pattern rows
- many reconstruction rows

Braxon must build from the concrete anchors outward.

It may not let the densest row family dominate selection merely because it is numerous.

## Global pick order

When selecting candidate rows for manuscript build, prefer in this order:

1. `DIRECT_SOURCE` + `PLACED_FILE`
2. `DIRECT_SOURCE` + `SCENE_EXPANSION_EXTRACT` with concrete lived event wording
3. `ACTUAL_SOURCE` concrete event rows
4. concrete `M` / missing-scene rows with unique relational or event-bearing value
5. `COMPILECAT` rows only after title interpretation if they encode real event truth
6. abstract editorial shorthand rows
7. reconstruction filler rows

Rows from lower levels may shape expansion, but must not outrank higher-level anchors.

## Cluster-first selection rule
Braxon must not select row-by-row in isolation.
It must first cluster rows into scene-families.

A scene-family is a group of rows that share the same underlying event burden, such as:
- same revelation
- same reckoning
- same wound
- same waking
- same pressure pattern
- same book-end summary
- same repeated payload with drifting title

Selection happens per cluster first, then per row.

## Cluster leader rule
Each cluster gets one primary leader row.

Choose the leader by this order:
1. concrete placed file
2. concrete source extract
3. concrete actual source
4. concrete missing scene
5. interpreted abstract shorthand
6. reconstruction filler only if no better source exists

The cluster leader is the row that anchors the manuscript realization.

## Cluster cap rule
A single scaffold-heavy cluster may not flood the build.

Default caps:
- Tier 1 / Tier 2 concrete cluster: no hard cap at selection stage
- Tier 3 editorial shorthand cluster: max 1 primary + 1 support row
- Tier 4 reconstruction cluster: max 1 debt marker row, zero direct manuscript claims

This means:
- one pressure-pattern family does not become 20 manuscript scenes just because 20 rows exist
- one repeated beat-end family does not consume the whole book build
- one reconstruction family does not masquerade as broad coverage

## Duplicate-family suppression
If many rows differ mainly by:
- numbering
- title ornament
- repeated copied payload
- the same short event bundle repeated under new shorthand

then Braxon must compress them into one family before selection.

Do not treat:
- B24_M001 through B24_M0xx with the same pressure logic
- repeated `rewritten_beat_end`
- repeated `The-Book-Finds-Its-*`
- repeated reconstruction summaries

as many equally valuable scene picks.

## Strong-anchor quota
For any book pass, the selected build set should satisfy this minimum:

- at least 60% of selected rows must be Tier 1 or Tier 2 anchors
- no more than 25% may be Tier 3 editorial shorthand rows
- no more than 10% may be Tier 4 reconstruction debt markers
- Tier 4 rows must never be counted as finished prose coverage

If the book lacks enough Tier 1/Tier 2 rows, Braxon must:
1. interpret the strongest Tier 3 row into a concrete scene brief
2. promote it only after lived-scene realization rules are met
3. keep Tier 4 rows as debt markers, not coverage claims

## Section-build order
Within each book section, build in this order:

1. opening anchor
2. first concrete relational or event scene
3. strongest consequence scene
4. strongest dialogue / play / domestic / moral-pressure insert
5. aftermath or carryover
6. only then book-end or editorial shaping rows if still needed

This prevents:
- opening on summary instead of scene
- ending on scaffold instead of landing
- letting book-architecture notes replace lived scenes

## Opening safeguard
Book openings must be selected from:
- placed scene file
- concrete source extract
- concrete actual source
- interpreted concrete missing-scene anchor

Do not open a book on:
- `rewritten_book_open`
- `opening pressure`
- `classified correctly`
- `book begins under terminal law`
- `the book finds its law`
unless first converted into a concrete lived scene brief.

## Beat-end safeguard
Beat-end rows may guide landing, but they are not themselves the landing.

Rows labeled like:
- `rewritten_beat_end`
- `book deepens properly`
- `book remains accurate`
- `book gains forward motion`
- `book loads forward`

must be treated as secondary shaping rows.
They cannot be the only selected row for a beat-ending cluster.

## Missing-scene rule
Missing-scene rows may be selected strongly only when they provide one of:
- a unique relational movement
- a unique pressure vector not otherwise concretely represented
- a genuine continuity bridge with event consequence
- a needed lived scene that no stronger source covers

If many missing-scene rows merely restate the same pressure logic, they must be clustered and capped.

## COMPILECAT rule
COMPILECAT rows must be interpreted before they count as strong scene picks.

Allowed:
- a COMPILECAT row whose encoded title clearly maps to a distinct lived scene
- a COMPILECAT row that captures a real event burden missing elsewhere

Disallowed:
- equation shorthand left uninterpreted
- abstract title chosen only because it sounds important
- repeated symbolic rows taking precedence over concrete anchors

## Reconstruction rule
Reconstruction rows serve three functions only:
1. show where manuscript debt exists
2. suggest families that need realization
3. preserve continuity hints when no stronger source exists

They do not:
- prove chapter coverage
- prove beat completion
- prove prose completion
- outrank concrete source anchors

## Book 24 special selection order
For `Born To Die`, select clusters in this order:

1. revelation cluster
   - Neith fully revealed
2. reckoning cluster
   - Rylos / Pip accountability scene
3. wound cluster
   - mortal wound
4. impossible-act cluster
   - superposition / impossible done
5. grief cluster
   - aftermath / recognition / speech
6. dogs-and-truth cluster
   - only if it contributes human truth rather than symbol bookkeeping
7. abstract death-law cluster
   - only after it has been converted into lived scene function
8. repeated pressure-pattern cluster
   - only as support, never as the main spine

### Book 24 anti-flood rule
For the `rewritten_pressure_pattern` family in Book 24:
- select at most 2 representatives per major section
- at least one must be concretized into real interaction before use
- do not let repeated pressure rows outnumber reckoning, wound, or consequence scenes

### Book 24 abstract-title rule
Rows like:
- `Nearness = Terminus`
- `Presence ≠ Solved`
- `Ghost = Obligation`
- `The Cello Was Built From Consequence`
- `Every Ghost Was Present For The Playing`
- `Pip Knew What Had To Be Given`
- `The Last Approach Was Agreement`

must be mapped to:
- reckoning
- wound
- grief
- impossible choice
- aftermath speech
- relational silence
before they can count as selected scene anchors.

## Book 25 special selection order
For `Death Is Rebirth`, select clusters in this order:

1. waking / proof-of-life cluster
2. changed-return cluster
3. obligation cluster
4. fragile joy cluster
5. lived rebuilding cluster
6. farewell-through-music cluster
7. abstract rebirth-law cluster
8. reconstruction cluster only as debt marker

### Book 25 anti-flood rule
Repeated `The-Book-Finds-Its-Rebirth-Law` or equation-title families may not dominate the build.
They must be interpreted into distinct:
- waking
- changedness
- obligation
- fragile joy
- rebuilding
angles before selection.

## Books 01–03 special selection order
For Books 1–3, select clusters in this order:

1. school-world lived scene
2. Mack / prophecy / oasis pressure
3. first Xethrolund contact
4. Pip / Rylos / Kyreal concrete relational scenes
5. diary / prophecy / azure-channel scenes
6. early moral-pressure and dehumanization pattern
7. only then book-widening or saga-loading shorthand

### Books 01–03 anti-flood rule
Long runs of reconstruction rows in Book 1 may not replace:
- actual school scenes
- friendship warmth
- Kyreal benchmark presence
- Pip curiosity
- Mack lived presence
- Xethrolund contact scenes

## Selection fallback ladder
If a section has no good Tier 1 anchor:
1. use strongest Tier 2 concrete row
2. if none, interpret strongest Tier 3 row into a concrete scene brief
3. if none, use Tier 4 only to mark debt and request/trigger scene generation
4. never pretend the section is covered just because many weak rows exist

## Manuscript-coverage rule
A beat is only counted as covered if the selected cluster leader resolves into:
- concrete event
- real relation shift
- sensory embodiment
- action or choice
- consequence

If the selected material is still mostly editorial shorthand, the beat is still debt.

## Anti-density rule
If one family has 30 rows and another family has 2 rows, Braxon must not infer the 30-row family is more important.
Importance comes from:
- canon event weight
- scene concreteness
- relational consequence
- lived uniqueness
not row abundance.

## Anti-shortcut rule
Do not choose easier scaffold families over harder concrete scenes just because the scaffold is already indexed cleanly.

## Outcome rule
After applying this patch:
- Braxon selects by strongest scene-family leader, not by densest row swarm
- concrete anchors dominate build order
- shorthand rows shape, but do not replace, scene realization
- Books 24–25 stop getting buried under pressure-pattern and editorial-repeat noise
- Books 1–3 stop being falsely treated as covered by reconstruction bulk
