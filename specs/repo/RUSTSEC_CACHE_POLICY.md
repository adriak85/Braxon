# RustSec Cache Policy

`state/braxon/rustsec-advisory-db/` is treated as a local offline security cache, not as canonical runtime substrate.

The Braxon repository may track small Braxon-owned security summaries, manifests, and audit conclusions, but should not track a full mirrored RustSec advisory database unless a later design explicitly promotes a curated subset into Braxon runtime knowledge.

## Tracked

- `state/braxon/security_audit_summary.json`
- dependency audit summaries generated from local tools
- curated Braxon security policy documents

## Local only

- `state/braxon/rustsec-advisory-db/`
- cargo-audit cloned advisory databases
- transient advisory mirror state

## Rationale

Keeping the full advisory DB local preserves offline security review without creating repository noise, mirror drift, or large non-Braxon-owned file churn.
