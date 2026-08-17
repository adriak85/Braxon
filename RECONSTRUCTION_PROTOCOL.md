# Braxon Final Reconstruction Protocol

Status: ACTIVE
Mode: exhaustive semantic reconstruction

This repository is reconstructed as a finalized cohesive system, not as an archive of historical files.

## Context-boundary law

No reconstruction step may depend on the entire repository, a complete file, or a complete historical implementation fitting in one model context window.

Every source unit is therefore processed through a resumable ledger. A source unit may be a repository, directory, file, file segment, implementation family, or cross-file dependency group.

A unit is never considered processed merely because it was opened or summarized.

## Source-unit lifecycle

Each unit advances through these durable states:

DISCOVERED -> SEGMENTED -> DIGESTED -> CLASSIFIED -> COMPARED -> MERGED_OR_RETAINED -> INTEGRATED -> VERIFIED

Failure, context exhaustion, or tool interruption returns the active unit to the last durable state. The next worker/session resumes from that state rather than restarting or guessing.

## Oversized-file rule

If a file cannot fit safely in the available context, it MUST NOT be discarded, truncated into a lossy summary, or treated as unreadable.

It is segmented at stable boundaries (module, item, declaration, function, impl block, test block, or deterministic byte/line ranges when structural boundaries are unavailable). Each segment receives:

- source path
- source revision/blob identity when available
- byte/line range
- segment ordinal
- content hash
- parent unit identity
- discovered symbols/dependencies
- semantic contribution
- unresolved references
- disposition

Segments are individually digested and later recomposed through the ledger. Cross-segment conclusions are made only after all segments required for that conclusion have been processed.

## No-loss rule

A digest is not a substitute for implementation. The final system must contain the actual surviving behavior, rewritten and integrated where necessary. Summaries exist only as reconstruction metadata.

## Duplicate-resolution rule

When multiple implementations serve the same purpose, compare behavior, correctness, completeness, invariants, dependencies, performance characteristics, failure modes, and integration cost. Retain the strongest implementation or construct a deliberate hybrid when the implementations provide complementary capabilities.

Historical names do not determine survival. Code quality and architectural fit do.

## Integration rule

Every retained capability must have exactly one authoritative home in the final architecture. Compatibility adapters are permitted only where they provide a real boundary; duplicate authorities are not.

Every final source file must have a concrete purpose and an owner in the architecture. Empty shells, abandoned experiments, duplicate implementations, obsolete compatibility layers, and historical debris do not enter the finalized stack.

## Verification rule

A subsystem is not complete when it merely compiles. Its contracts, invariants, integration paths, error handling, and representative behavior must be exercised. The final workspace must build as one coherent stack.

## Durable reconstruction ledger

The ledger belongs in the repository and must be updated as work advances. It must record enough information for a new session to resume without relying on conversational memory:

- current reconstruction revision
- source inventory identity
- completed units
- active unit and exact segment
- semantic classifications
- implementation-family relationships
- selected/rejected/merged implementations and rationale
- unresolved dependencies
- verification results
- next executable unit

## Context rollover

When the current context becomes insufficient, stop at the last durable checkpoint, write the checkpoint, and continue with the next unit using the ledger. Never compress unfinished work into an unverified global summary merely to fit the response window.

The reconstruction process is complete only when no source unit containing relevant functionality remains in an unprocessed state and every retained capability is integrated and verified in the final architecture.
