# WOWAS Completion and Post-Commit Repair Report

## Authority decision

The authoritative narrative index is now `canon/active/scene_index_reasonable_window.tsv` with **2,019 scenes**. The former `scene_index_15000.tsv` remains a deprecated compatibility and comparison artifact; it is not the current reader-ingest target. The completion generator defaults to `WOWAS_SCENE_TARGET=2019` and refuses targets outside the validated source window.

The scene index is generated from the 33-book spine and assigns every canonical scene a deterministic event-beat serial, domain flags, a quest hook, a world-introduction anchor, and a provenance trace. The current index contains 2,019 unique scene IDs, 2,019 unique descriptions after event-beat reconciliation, and coverage across characters, creatures, world introductions, and quests.

## Validated surfaces

| Surface | Result |
|---|---:|
| Book spine | 33 books |
| Canonical scene window | 2,019 rows |
| Unique scene IDs | 2,019 |
| Unique scene descriptions | 2,019 |
| Event beats | 2,019 |
| Generated characters | 5,000 |
| Timeline schedule rows | 20,000 |
| Relationship ledger rows | 45,000 |
| Creature seeds | 5,000 |
| Background population | 2,000,000 |
| Real-world sources | 50: 30 cities and 20 landmarks |
| Real-world domain seeds | 400 |
| SERIEL records | 153,631 |
| SERIEL unlinked records | 0 |

All generated background rows now reference an actual scene and event beat in the canonical window. The background validator reports zero malformed rows, 2,000,000 unique population serials, 5,000 creature seeds, 2,019 scene anchors, and 2,019 event beats. Background records remain marked `deterministic_seeded_requires_batch_review`; this is an honest editorial-quality boundary, not a failed structural gate.

## Cross-link and provenance repair

The relationship generator no longer emits fabricated `Bxx_REL_xxxxx` scene identifiers. It deterministically assigns each relationship row to a real scene in the character’s book, with fallback to an available canonical scene only when a book has no local scene. The timeline scheduler assigns all four phases of each character’s arc to existing scenes. The reasonable-window validator reports zero missing timeline references, zero missing relationship references, and zero missing background references.

SERIEL is regenerated from the repaired canonical scene input and current generated maps. Every record has a bottom-footnote serial and a bounded bidirectional link set; the current crosswalk reports 153,631 records and zero unlinked records.

## Content and representation gates

The identity integration validator reports 5,000 characters with the requested LGBTQ+ representation distribution and a heavier gay-male weighting. Role preference metadata is kept non-graphic and age-gated, with `top_role_preference` represented as a character-profile field rather than explicit prose. The adversarial validator passes provenance-loss, fact-fiction leakage, unreviewed-canon-creation, duplicate-transform, and reader-overflow checks.

The real-world converter validator passes 50 sources, 400 domain seeds, citation-bearing alignments, reader projection bounds, and SERIEL linkage. External source endpoints returned HTTP 403 during validation; the stored source identifiers, URLs, and fact-fiction boundary fields remain present, but endpoint reachability must not be represented as independently re-fetched evidence.

The 33-book structural spine is complete. The current source tree contains validated prose directories for 25 books; the remaining eight are represented by spine and operational contracts rather than falsely claimed full prose. The manifest therefore distinguishes structural coverage from prose completion.

## Runtime evidence

A disposable package-local Rust harness was used because the installed compiler is Rust 1.75 while the repository declares Rust 1.78 and pins nightly, which is unavailable in the sandbox. With the rust-version guard ignored, the focused native suite passed **47 Braxon-core tests and 15 NSQ-core tests**, including native NSQ execution, direct Blaixe dispatch, ghost-window rotation, piston ownership, KV-cache bounds, kinetic reflexor phase changes, Target Field reconciliation, and WOWAS seeded-world behavior. One initial failure was a missing fixture in the disposable harness; after copying the repository’s context-manifest and chain-root fixtures, the suite passed cleanly. No Android-target build was performed.

## Bulk contract profile

The repaired gates were profiled over the actual generated corpus. Schema validation completed in approximately 5.44 seconds, background validation in approximately 13.17 seconds, reasonable-window cross-link validation in approximately 11.17 seconds, and adversarial validation in approximately 0.08 seconds on the sandbox host. These are host measurements, not Android performance claims.

## Release disposition

The TSV schema failure is repaired. The schema gate now performs explicit required-column and row-width assertions, emits SHA-256 cache manifests, and fails closed on missing or malformed inputs. The reasonable-window migration is integrated into scene, relationship, timeline, background, and SERIEL generation. The remaining non-release claims are editorial prose expansion for eight spine-only books, human-quality review of deterministic background records, and physical Android 16 no-root acceptance testing on the Moto G target.
