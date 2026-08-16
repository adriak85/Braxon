# WOWAS World Reconstruction

This is the consolidated reconstruction workspace for the WOWAS corpus in Braxon.

## Preservation rule
Every file is evidence. Do not skip TSV/CSV/JSON/YAML/TOML/logs/data files, README copies, generated-looking files, retired material, or files whose claims conflict with other documents.

## Required workflow
1. Locate the complete Rust-project `crates/wowas` corpus, including all nested files and historical variants.
2. Read every document and every data artifact.
3. Record provenance and preserve contradictions.
4. Extract instructions embedded in the corpus.
5. Execute required generators only when their inputs and instructions are recovered.
6. Consolidate the resulting world rather than copying source artifacts verbatim.
7. Keep the original source corpus separately referenced.
8. Do not declare a subsystem complete merely because a README says it is complete; validate against later evidence, generated outputs, tests, benchmarks, and contradictions.

## Critical data classes
- prose and narrative
- character definitions
- background population rules
- creature definitions
- world map and map translation
- physics/world rules
- economy
- graphics/rendering data
- generator inputs and outputs
- TSV/CSV/tabular state
- JSON/YAML/TOML configuration
- scripts and build instructions
- tests and benchmarks
- retired/failed variants

## Current state
The repository connector currently exposes the Braxon `crates/` tree but has not yet exposed a resolvable `rust-project/crates/wowas` path. This is recorded as an access/path-resolution issue, not evidence that WOWAS is absent. The reconstruction must not proceed by guessing an alternate corpus or declaring the work complete.
