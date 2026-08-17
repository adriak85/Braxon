# WOWAS World Reconstruction — Sequential Build State

Source of truth: `adriak85/Braxon` `main`.

Primary authored corpus: `crates/wowas-final-edition-v10/`.
NSQ stamped corpus: `state/nsq/stamps/libraries/crates__wowas-final-edition-v10/`.

## Reconstruction contract

This is a semantic rebuild, not a file copy. Every source artifact is evidence. Nothing is filtered because it is old, duplicated, generated-looking, metadata, TSV/CSV, a backup, or apparently upstream.

Each stage consumes the previous reconstructed state and records provenance. Conflicts are preserved as competing evidence until reconciled by later evidence; source artifacts are never silently discarded.

## Stage order

1. Inventory every tree/blob and preserve path + blob SHA + size.
2. Parse every artifact, using bounded line segments for oversized text (`sed`-compatible segmentation in the execution environment).
3. Extract explicit and implicit instructions and intents from prose, code, tables, configuration, tests, generators, and generated material.
4. Reconstruct canonical entities and relations without copying source files into the output model.
5. Apply canonical transformations in the corpus-specified order, preserving every intermediate state.
6. Build the SERIEL-backed graph/lattice and attach provenance to every node and edge.
7. Execute recovered generators only at their prescribed stage; feed their results back through parsing rather than treating generated output as automatically authoritative.
8. Materialize the cohesive world model: narrative, characters, creatures, geography, physics, economy, graphics/rendering, and interactive/game structures.
9. Validate against tests, benchmarks, later variants, and contradictions.
10. Emit the final reconstructed state plus a complete provenance/index manifest.

## Corpus anchors verified on main

The v10 crate contains `canon`, `src`, `tests`, `command_list.txt`, `README.md`, installation material, and the `.non_wowas_source_inspiration_lattice.file` artifact. The NSQ stamp library contains `asm` and `metadata`, including multiple apply-order generations and canonical story-tree indexes.

The NSQ metadata explicitly records native local reconstruction paths and source SHA-256 identities for canonical inputs. Those identities are provenance, not replacement content.

## No-completion shortcut

A README, manifest, benchmark, generated stamp, or prior claim of completeness never closes a stage by itself. Completion requires the corresponding reconstructed state and validation evidence.
