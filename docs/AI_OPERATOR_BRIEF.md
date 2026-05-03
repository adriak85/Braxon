# BRAXON / NSQ Development Capsule Operator Brief

## Purpose
This capsule is a development-facing source unit for AI-assisted work on BRAXON and NSQ.

## Non-negotiable rules
- Work from the canonical local source tree.
- Do not replace NSQ-native surfaces with foreign-runtime dependency assumptions.
- S-expression, lua-shape, and python-shape are internal lowering forms, not external runtime requirements.
- Native runtime lanes outrank legacy hook compatibility surfaces.
- Prefer consolidation into the solid core over parallel layers.
- Do not invent substitute architectures when canonical structures already exist.

## Canonical surfaces
- `source/crates/nsq-compile`
- `source/crates/nsq-runtime`
- `source/crates/nsq-native-bench`
- `source/config/kingdom/court_canonical.json`
- `hooks/hook_matrix.json`
- `hooks/dialects.json`

## AI task expectations
When generating code:
- use existing crates first
- extend native runtime lanes before touching legacy hook compatibility surfaces
- preserve direct repo intake
- preserve proof and bench reproducibility
- emit exact commands
- avoid speculative rewrites

## Dialect policy
Allowed source dialects:
- canonical
- sexpr
- lua_shape
- python_shape

All lower internally to canonical NSQ.
No external Lua or Python runtime is required for this lowering path.
Python 3 runtime authority belongs to the native `nsq-runtime` lane, not the lowering forms.

## Success test
The system is successful when:
- an AI can inspect this capsule
- understand the legacy hook/dialect compatibility surfaces and the authoritative native runtime lanes
- generate code in-system
- run proof and bench commands
- preserve determinism and canonical ownership
