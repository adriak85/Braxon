# WoWaS Anti-Placeholder Expansion Patch v10

## Purpose
This patch prevents scaffold language from surviving into final manuscript prose.

It specifically demotes repeated abstract markers such as:
- "tone and structure lock"
- "the book finds its law"
- "the book finds its death law"
- "the book finds its rebirth law"
- "reconstructed continuity scene"
- "filled to target scene count using uploaded source anchors"

These phrases may remain useful as generator signals, but they are not acceptable final scene content.

Use with:
- `patches/PROSE_AND_TONE_GUIDE.json`
- `patches/v10/wowas_dialogue_and_play_insertion_patch_v10.md`
- `patches/v10/wowas_book_dialogue_seed_registry_v10.md`

## Core law
Scaffold text is an instruction to expand, not text to preserve.

If a scene title, beat label, reconstruction label, or source-derived filler contains abstract manuscriptless language, the generator must convert it into lived scene content before prose output.

## Demoted placeholder classes

### Class A — direct scaffold phrases
These must never survive verbatim as final manuscript scene titles or internal prose:
- tone and structure lock
- the book finds its law
- the book finds its death law
- the book finds its rebirth law
- the book finds its resonance law

### Class B — reconstruction filler
These must never be treated as meaningful scene realization by themselves:
- reconstructed continuity scene ####
- filled to target scene count using uploaded source anchors
- source-derived reconstruction
- continuity filler language that has no lived action, dialogue, or sensory consequence

### Class C — abstract equation titles
These may be useful as design shorthand, but final manuscript must convert them into natural language scene content:
- Farewell = Instrument
- Waking = Proof
- Rebirth = Obligation
- Ghost = Obligation
- Presence ≠ Solved
- Nearness = Terminus

They are prompts, not finished prose.

## Expansion trigger rule
When the generator encounters a placeholder class, it must immediately do all of the following in order:

1. identify the real canon event, relation, or transformation underneath the placeholder
2. consult `patches/v10/wowas_book_dialogue_seed_registry_v10.md`
3. select the strongest matching dialogue/play/domestic/moral-pressure target
4. ground the scene in Pip-limited felt perception
5. expand through action, speech, sensation, and consequence
6. discard the placeholder phrase from final prose unless needed as invisible generation metadata

## Replacement rules by placeholder type

### "The book finds its death law"
Interpret as:
- the world or cast arriving at the irreversible emotional and moral truth of death in that section
- a scene of accountability, wound, grief, sacrifice, final naming, or impossible cost

Preferred replacement forms:
- spoken reckoning
- mortal-wound reaction scene
- grief dialogue
- quiet ruin scene with specific human speech
- Pip feeling death as lived consequence, not abstract concept

### "The book finds its rebirth law"
Interpret as:
- the cast discovering what return actually costs and obligates
- bodily waking, changedness, fragile joy, relief, confusion, or responsibility after survival

Preferred replacement forms:
- waking scene
- touch/food/breath/voice proof-of-life scene
- changed-people conversation
- fragile joy scene
- rebuilt-world lived scene

### "The book finds its resonance law"
Interpret as:
- the world-scale alignment of song, self, sacrifice, and return becoming emotionally legible

Preferred replacement forms:
- a scene where impossible scale is made intimate through a person
- a spoken confirmation of love, arrival, grief, or belonging
- Mack/Xethrolund/Pip emotional specificity under world-scale conditions

### abstract equation scene titles
Convert them into scene-function prose before expansion.

Examples:
- "Farewell = Instrument" becomes a real farewell scene where music carries loss.
- "Waking = Proof" becomes a bodily waking scene proving survival.
- "Rebirth = Obligation" becomes a dialogue scene about what the returned owe life.
- "Ghost = Obligation" becomes a scene where memory or loss still demands action.

Do not preserve the equation syntax in final prose unless a character explicitly and naturally says something equivalent.

## Title realization rule
If a source title is abstract placeholder shorthand, the final manuscript chapter title should be rewritten into natural story-facing language.

Allowed:
- a specific sensory or emotional title
- a relational title
- a concrete action title
- a poetic but lived title

Disallowed:
- generic reconstruction phrasing
- internal editorial shorthand
- equation syntax left uninterpreted
- "tone and structure lock" style titles

## Scene sufficiency rule
A scene is not considered expanded unless it includes at least three of the following:
- concrete bodily sensation
- natural spoken dialogue
- a visible action or choice
- relational movement
- setting texture
- emotional consequence
- aftermath carryover

If fewer than three are present, the scene is still scaffold.

## Book 24 and 25 emergency rule
For `Born To Die` and `Death Is Rebirth`, repeated placeholder scenes must be treated as highest-priority manuscript debt.

When Books 24 and 25 contain repeated:
- death law
- rebirth law
- tone and structure lock
- reconstruction filler

the generator must prefer:
1. accountability and mortal-wound speech
2. grief and aftermath dialogue
3. waking / proof-of-life embodiment
4. changed-returned conversations
5. fragile joy and lived-world rebuilding

before generating any further connective summary.

## Anti-drift rule
The generator may not satisfy expansion merely by paraphrasing the placeholder.

Bad:
- "The book found its rebirth law as the world learned to live again."

Still bad:
- "This was the moment rebirth became real."

Acceptable:
- a scene where someone wakes, cannot speak at first, touches the ground, realizes it is holding, hears another survivor laugh or cry, and the changed condition is spoken aloud by people who know what they almost lost

## Anti-repetition rule
If the same placeholder reappears in multiple reconstructed scenes, each realization must resolve into a different lived angle:
- one may become grief dialogue
- one may become waking embodiment
- one may become domestic aftermath
- one may become moral obligation conversation
- one may become fragile humor or relief

The generator must not emit near-duplicate prose for repeated placeholder triggers.

## Outcome rule
After applying this patch, placeholder phrases remain only as hidden generation cues.

They must not survive as reader-facing manuscript content except in rare quoted/meta cases that are clearly intentional.
