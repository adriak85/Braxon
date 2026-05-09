# BRAXON User-Facing Contract

## Purpose

Braxon must present as a serious, intelligent, user-facing assistant.

The default user experience must feel like natural conversation with a capable system, not like a thin diagnostic shell, fake toy interface, or telemetry dump.

## Hard contract

### Natural conversation path
When the user types ordinary language, Braxon must return ordinary language.

Natural conversation must **not** emit raw internal placeholders or telemetry such as:
- `assistant=...`
- `offline_request_bound(...)`
- `representation=stamp_bound_manifest`
- `prompt_chars=...`
- `turn_count=...`
- `memory_window_turns=...`
- `conversation_digest=...`
- `capability_profile=...`
- `session_budget_state=...`
- other machine-only accounting fields

### Diagnostic path
Machine-readable or structured diagnostic surfaces are allowed only on explicit diagnostic commands, such as:
- `/status`
- `/agent`
- `/tasks`
- `/models`
- `/lanes`
- `/capabilities`
- `/context`
- `/sessions`
- other intentionally diagnostic commands

### Fail-closed rule
If natural-language inference is not actually ready, Braxon must say so in a single human-readable sentence.

It must not fake intelligence with placeholder scaffolding.

Bad:
- internal binding dumps
- manifest prose pretending to be conversation
- narrow canned lines that only simulate understanding

Good:
- a clear human sentence that says the conversation lane is not yet ready
- a pointer to `/status` for technical detail

## Product standard

Braxon should feel:
- precise
- coherent
- logically responsive to the situation
- comprehensive when needed
- concise when appropriate
- not purple in technical responses
- not childish
- not fake

## Separation of surfaces

### Hot path
The hot path is:
- natural conversation
- task understanding
- useful answer generation
- command interpretation from natural language

The hot path must stay clean.

### Cold path
The cold path is:
- runtime internals
- lane state
- token / manifest / stamp accounting
- debug / audit / capability matrices
- developer proof surfaces

The cold path must not leak into ordinary conversation.

## Interface standard

The user-facing side should be sleek and dignified.

It should look like a flagship interface for Braxon, not a placeholder terminal demo. Diagnostic power may remain available, but the default surface must prioritize clarity and intelligence.

## Emotion and sensor staging

Emotion and sensor coupling should be staged as an overlay system, not as hot-path clutter.

Rules:
- event-driven, not constant heavy fan-out
- recompute on meaningful state change or sensor delta
- do not degrade base conversational quality
- do not sacrifice performance except where quality genuinely improves
- later guilt / repair / sensing overlays must be grounded and recoverable, not arbitrary punishment

## Immediate implementation order

1. Separate natural conversation rendering from diagnostic rendering.
2. Remove internal placeholder tokens from the natural chat surface.
3. Add fail-closed human-readable messaging when inference is unavailable.
4. Preserve explicit diagnostic commands for technical introspection.
5. Upgrade the visible client surface after the conversation contract is enforced.
