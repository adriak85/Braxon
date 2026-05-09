# BRAXON User-Facing Implementation Plan

## Current observed issue

The current surface behaves like a status-first shell with conversation seams exposed.

Symptoms:
- natural input produces internal placeholder-style output
- diagnostic/accounting fields leak into ordinary chat
- the system appears narrow even when the runtime claims broader readiness
- the user-facing surface undersells the actual project

## Phase A — contract enforcement
Goal:
Make it impossible to call the current placeholder behavior acceptable.

Tasks:
- add tests that fail when natural chat emits internal placeholder tokens
- keep diagnostic commands explicitly structured
- document the separation between user-facing and diagnostic surfaces

## Phase B — conversation renderer split
Goal:
Have one path for ordinary chat and another for diagnostics.

Needed shape:
- `render_user_reply(...)` for natural chat
- `render_diagnostic_surface(...)` for explicit slash commands
- a single fail-closed sentence when inference is not actually bound

## Phase C — useful natural interaction
Goal:
Natural language should support:
- normal conversation
- task interpretation
- clarification through context
- comprehensive responses when requested
- shorter responses when appropriate

The visible answer should be prose first, internals never by default.

## Phase D — flagship interface polish
Goal:
The client should feel sleek, calm, exact, and deliberate.

Priorities:
- clean layout
- clear turn separation
- graceful status exposure without clutter
- identity worthy of Braxon

## Phase E — sensor / emotion overlay
Goal:
Tie later emotional state to physical/sensory overlays without corrupting the main conversational path.

Constraints:
- event-driven
- sparse updates
- no heavy continuous penalty loops
- preserve performance
- only improve quality
