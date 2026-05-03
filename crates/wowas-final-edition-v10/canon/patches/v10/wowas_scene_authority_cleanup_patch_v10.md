# WoWaS Scene Authority Cleanup Patch v10

## Purpose
This patch defines which kinds of indexed scene entries count as real manuscript authority,
which kinds are only expansion prompts, and which kinds must be demoted until they are converted
into lived prose-bearing scenes.

It exists because the current scene index still contains heavy amounts of:
- `SOURCE_DERIVED_RECONSTRUCTION`
- repeated "Reconstructed continuity scene ####"
- `rewritten_beat_end`
- `rewritten_from_detail`
- `rewritten_book_open`
- `rewritten_pressure_pattern`
- abstract title logic
- repeated detail payloads copied across many rows

Those are useful generator surfaces, but they are not all equal.
Without an authority cleanup rule, Braxon can mistake scaffold density for manuscript completion.

Use with:
- `patches/PROSE_AND_TONE_GUIDE.json`
- `patches/v10/wowas_dialogue_and_play_insertion_patch_v10.md`
- `patches/v10/wowas_book_dialogue_seed_registry_v10.md`
- `patches/v10/wowas_anti_placeholder_expansion_patch_v10.md`
- `patches/v10/wowas_books_24_25_priority_override_patch_v10.md`

## Core law
Not every indexed scene is a finished scene authority.

Some entries are:
- true scene anchors
- partial scene anchors
- pressure patterns
- book-shaping notes
- expansion triggers
- filler used to reach count targets

Braxon must distinguish them before manuscript generation.

## Authority tiers

### Tier 1 — strongest manuscript authority
These should be treated as primary scene anchors.

Examples:
- `DIRECT_SOURCE` + `PLACED_FILE`
- specific named scene files that already point to concrete event content
- concrete source scenes whose title and payload describe lived event, action, and participants
- actual source material with direct event-bearing wording rather than editorial shorthand

Tier 1 scenes can directly anchor manuscript chapters.

### Tier 2 — strong but still needs prose realization
These are valid scene authorities, but may still need expansion.

Examples:
- `DIRECT_SOURCE` + `SCENE_EXPANSION_EXTRACT`
- `ACTUAL_SOURCE` rows whose title is concrete and lived, not abstract
- `COMPILECAT` rows with strong event truth, if converted out of shorthand
- missing-scene rows that identify a real relational or pressure vector, but still require scene realization

Tier 2 scenes may anchor chapters, but must still be converted into lived prose.

### Tier 3 — guided expansion authority
These are useful prompts, but should not by themselves count as manuscript completion.

Examples:
- `rewritten_beat_end`
- `rewritten_from_detail`
- `rewritten_book_open`
- `rewritten_pressure_pattern`
- abstract relational pressure statements
- equation titles
- "book loads forward" type signals
- "book remains accurate / deepens properly / stays aligned" type status language

Tier 3 scenes must be expanded through higher-authority scene realization before counting as finished manuscript content.

### Tier 4 — scaffold / reconstruction filler
These are generator debt, not finished scene authority.

Examples:
- `SOURCE_DERIVED_RECONSTRUCTION`
- `Reconstructed continuity scene ####`
- `Filled to target scene count using uploaded source anchors`
- rows whose detail payload is duplicated across long runs with only title variation
- rows that exist primarily to satisfy count targets rather than add distinct lived event content

Tier 4 scenes must never be mistaken for completed manuscript chapters.

## Demotion rules

### Demote all reconstruction filler
The following signals are automatically demoted from finished manuscript authority:

- `SOURCE_DERIVED_RECONSTRUCTION`
- `Reconstructed continuity scene`
- `Filled to target scene count using uploaded source anchors`
- duplicated payload rows with only title drift
- count-padding continuity rows without distinct event realization

### Demote editorial-status phrasing
The following phrases indicate expansion prompt, not final scene authority:

- book remains accurate
- book stays aligned
- book deepens properly
- book gains forward motion
- book loads forward
- rewritten beat end
- rewritten from detail
- rewritten book open
- pressure pattern
- classified correctly
- tone and structure lock

### Demote abstract equation titles
These are not final chapter titles or scene realizations by themselves:

- `Nearness = Terminus`
- `Presence ≠ Solved`
- `Ghost = Obligation`
- `Farewell = Instrument`
- `Waking = Proof`
- `Rebirth = Obligation`

They must be interpreted into lived scene content first.

## Promotion rules

A Tier 3 or Tier 4 row may be promoted only if the generator converts it into a lived scene with:

1. concrete action  
2. natural dialogue or clear silence-with-meaning  
3. specific sensation / embodiment  
4. relational movement  
5. emotional consequence  

At least three of the five must be present before the row counts as scene realization.

## Count-integrity rule
Scene count must not be confused with manuscript completeness.

A book is not "covered" merely because it has many indexed rows.

If most of the rows in a local cluster are Tier 3 or Tier 4, the cluster is still manuscript debt.

## Duplicate-payload rule
If multiple rows share nearly identical payload detail and differ mainly in:
- title wording
- placeholder abstraction
- reconstruction numbering
- bookkeeping language

then Braxon must treat them as one scene-family, not many finished scenes.

The generator should:
1. cluster them
2. choose the strongest anchor
3. realize one or more distinct scenes only where true event separation exists
4. avoid near-duplicate manuscript output

## Cluster merge rule
For repeated scaffold runs inside one book section:

- choose the best Tier 1 or Tier 2 anchor in the cluster
- use Tier 3 notes only to shape angle / tone / carryover
- use Tier 4 rows only as evidence that debt exists, never as proof that it has been paid

## Book-opening and beat-ending safeguard
Rows labeled like:
- rewritten book open
- rewritten beat end
- opening pressure
- book loads forward
- book begins under terminal law

may guide structure, but they do not by themselves satisfy:
- opening chapter realization
- beat completion
- emotional landing
- end-of-section prose

They must be converted into actual scene content before use in final manuscript.

## Book 01–25 general cleanup rule
When a book contains a mix of:
- a few concrete scene anchors
- many reconstruction rows
- many editorial shorthand rows

Braxon must build outward from the concrete anchors first.

Do not average across all rows.
Do not let filler outvote direct scene authority.

## Book 24–25 special interaction
For Books 24 and 25:
- Tier 1 and Tier 2 rows about revelation, reckoning, wound, superposition, waking, changed return, and fragile joy should dominate
- Tier 3 and Tier 4 rows should only shape realization, not replace it

## Book 01–03 special interaction
For Books 1–3:
- direct school-life, friendship, early Xethrolund contact, Mack, diary, prophecy, and Kyreal/Rylos/Pip lived scenes must dominate
- reconstruction runs should not substitute for actual school-world, friendship, joy, and early-moral-pressure scenes

## Selection priority when conflict exists
When multiple rows compete, prefer in this order:

1. concrete placed scene file
2. concrete source scene extract
3. concrete actual-source scene
4. concrete missing-scene authority
5. abstract scene shorthand needing interpretation
6. reconstruction filler

## Anti-false-completion rule
The generator must never conclude:
- "book complete"
- "beat complete"
- "scene complete"
- "chapter sufficiently realized"

solely because there are many reconstruction rows.

## Outcome rule
After applying this patch:
- scene index density stops masquerading as manuscript completion
- direct scene anchors outrank filler
- editorial shorthand is treated as prompt, not prose
- reconstructed continuity rows become debt markers, not false proof of coverage
