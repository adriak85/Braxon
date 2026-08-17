# DAX-FULL → NSQ Intent Extraction Ledger

## Rule

DAX-FULL is a source corpus, not a codebase to cherry-pick blindly. Every source language and artifact can contain semantic intent that must be extracted and re-expressed in native NSQ. Preserve the original artifact and provenance; never discard it merely because an equivalent appears elsewhere.

## Corpus discovery

Repository: `adriak85/DAX-FULL`
Default branch: `main`
Repository is private and owned by the project account.
Reported repository size: approximately 772,610 KiB.
GitHub language accounting currently reports:

- Makefile: 5,077,209 bytes
- Rust: 52,530 bytes
- Python: 35,394 bytes
- DTrace: 12,604 bytes
- Shell: 294 bytes
- C: 56 bytes
- C++: 56 bytes

These language totals are inventory evidence only. They are not a complete semantic inventory and must not be interpreted as “only these files matter.”

## Extraction targets

### Python
Extract:
- intent definitions and labels
- state machines
- recursive/control-flow assumptions
- token/grammar behavior
- data transformations
- invariants encoded as assertions
- test cases as behavioral specifications
- comments/docstrings that explain design intent
- constants and lookup tables
- error/failure semantics

### Rust
Extract:
- type relationships
- ownership/state-transition semantics
- trait boundaries
- compile-time invariants
- unsafe assumptions
- serialization/representation decisions
- tests and fixtures
- comments and TODOs that reveal intended behavior

Rust is implementation evidence, not authority over NSQ semantics.

### Assembly / low-level artifacts
DAX-FULL currently reports DTrace in its language inventory. Do not assume DTrace is ordinary assembly; inspect actual file contents. Also scan filenames/content for ASM, S, S-file, inline assembly, compiler output, instruction tables, linker scripts, ABI descriptions, and machine-level constants.

Extract:
- register/state assumptions
- calling/entry conventions
- bit-level operations
- timing/order constraints
- memory-layout intent
- hardware-facing invariants
- instruction-selection intent
- constants that carry semantic meaning

### Makefiles / Shell / build files
These are not “just infrastructure.” Extract:
- build ordering
- generated artifacts
- bootstrap dependencies
- environment assumptions
- feature gates
- target relationships
- toolchain versions
- hidden sequencing constraints
- commands that reveal the intended architecture

### C/C++
Even tiny language totals must be inspected. Small files can carry critical ABI or hardware semantics.

## Preservation rule

For every discovered source artifact create a provenance record containing:

`source_repo | ref | path | blob/commit | language | semantic_region | extracted_intent | NSQ_candidate | confidence | contradictions | dependencies`

Do not replace source artifacts with the extracted NSQ representation. The source remains evidence.

## Intent extraction rule

“Intent” means more than functions named `intent`. Extract semantic purpose from behavior:

`artifact → behavior → invariant → state transition → intended capability → NSQ expression`

A function, constant, test, build rule, parser branch, error path, or low-level instruction can contain intent even when no word resembling “intent” appears.

## Cross-language reconciliation

When Python, Rust, low-level code, tests, and build scripts describe the same operation:

1. Preserve each source representation.
2. Compare their actual behavior and stated intent.
3. Record differences instead of choosing immediately.
4. Identify which behavior is historical, which is later, and which is contradictory.
5. Derive the semantic invariant.
6. Express that invariant in NSQ.
7. Keep the original implementations as provenance.

## NSQ boundary

The extracted result is not a Python subsystem, Rust subsystem, or ASM subsystem inside NSQ. NSQ receives the recovered semantic intent and provides the language-level representation. Higher-level runtime systems may consume that NSQ representation without becoming part of the NSQ language.

## Current status

Initial repository-level discovery is complete. Detailed per-file extraction is still required. The presence of substantial Makefile content means build scripts must be treated as first-class historical evidence rather than skipped as “upstream/common.”

This ledger intentionally does not claim completion until the full DAX-FULL tree, historical refs, and relevant artifacts have been enumerated and reconciled.
