# WOWAS Authored Character Flavor and Dynamics Review

## Scope

This review covers the original non-generator character registry, the unified character timeline lattice, orbit and support-cast records, the prose-and-tone guides, dialogue/play guidance, romance/calendar/resonance guidance, and the attached synchronization note. The review creates structured constraint artifacts only. It does **not** generate prose, rewrite original authored material, or treat procedural character rows as authored personalities.

## Findings

The original sources contain 50 named character records and 94 authored dynamics records. The current generated registry contains 5,000 procedural records, but its distribution shows significant flattening that must be corrected before any realization pass:

| Finding | Measurement | Interpretation |
|---|---:|---|
| Original named character records | 50 | Source-backed identity/flavor baseline |
| Authored dynamics rows | 94 | Relationship and pressure functions to preserve |
| Generated character rows | 5,000 | Procedural metadata, not authored personalities |
| Generated role labels | 20 | Each repeated exactly 250 times; role repetition is not character diversity |
| Generated tiers | 5 | Each repeated exactly 1,000 times |
| Generated story-background law | 1 | One shared law across all 5,000 rows; insufficient as a personality differentiator |
| Generated source anchors | 12 | Anchor buckets are not equivalent to 50 authored identities |
| Generated visual-status variants | 60 | Variation exists, but must not substitute for voice or dynamic variation |

The generated registry is therefore **not yet flavor-diverse enough** to receive the original characters’ dynamics automatically. The new flavor-constraint lattice propagates source-backed traits, source stacks, voice/pressure fields, role functions, and authored relationship functions to generated candidates as metadata constraints. It keeps generated IDs distinct and does not overwrite the authored character registry.

## Preserved authored invariants

The original character sources remain the primary characterization layer. Canonical identifiers, aliases, pronouns, ages, species, faction roles, locked traits, shadow relationships, magic domains, source stacks, first/last-book windows, and authored relationship functions are preserved in `authored_character_flavor_lattice.tsv`. Orbit and relationship rows are kept separately in `authored_dynamics_lattice.tsv`, preventing relationship propagation from mutating character identity.

The flavor expansion layer is `generated_character_flavor_constraints.tsv`. It contains 5,000 rows, one for each generated candidate, and records the source anchor, matched authored identity when available, inherited locked traits, source stack, voice constraint, pressure response, dynamic relation, counterpart, and polarity. The file explicitly marks every row `prose_status=no_generated_prose`.

## Unmapped anchors

Seven generator-only anchor buckets—`Kael`, `Pael`, `Soth`, `Thessa`, `Corrath`, `Kyreal`, and `Vellin`—account for 2,915 generated rows that do not map directly to the 50-character authored registry. They are retained as **unmapped source anchors**, not falsely aliased to a nearby authored character. They require an explicit source-backed alias or new authored character record before they can inherit a specific original flavor. No silent nearest-name mapping was performed.

## Dynamic realization safeguards

The attached synchronization note identifies three valid pre-realization safeguards. First, output identity must use `record_id`, never only `scene_id`, because the reconciled metadata intentionally contains duplicate scene-ID groups. Second, prose must be written to staging and promoted only after tone, style, cadence, and token-length validation. Third, the book-to-book state ledger must carry world, ecological, emotional, relationship, and character-state changes forward so later volumes do not reset continuity.

These safeguards are recorded as requirements, not falsely reported as full manuscript realization. The present artifacts remain metadata-only.

## Conclusion

The original characters’ flavor has been preserved and expanded into a traceable constraint layer, but the audit found that the procedural generator’s equal-count role/tier buckets would flatten voices if used without these constraints. The system is therefore **not cleared for unbounded prose realization yet**. It is ready for a constrained realization stage only after the seven unmapped anchors are explicitly dispositioned and the staged tone/state gates are wired to the generated-character constraints.
