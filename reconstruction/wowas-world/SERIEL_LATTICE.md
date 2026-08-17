# WOWAS SERIEL Reconstruction Lattice

The SERIEL is the traversal spine for the rebuilt world. It is not a replacement for canon; it is the addressable relation layer connecting reconstructed entities while preserving source provenance.

## Node classes

- `book`
- `scene`
- `character`
- `creature`
- `species`
- `zone`
- `route`
- `faction`
- `law`
- `magic_system`
- `world_rule`
- `economy_state`
- `generator`
- `artifact`
- `source_variant`
- `transmedia_surface`

## Edge classes

- `contains`
- `precedes`
- `follows`
- `appears_in`
- `located_in`
- `travels_to`
- `causes`
- `constrains`
- `derives_from`
- `supersedes`
- `conflicts_with`
- `supports`
- `generated_by`
- `validated_by`
- `materializes_as`
- `shares_identity_with`

## Provenance invariant

Every reconstructed node and edge carries the originating repository path, blob identity when available, stage, and authority status. Historical/superseded artifacts remain addressable evidence. `superseded` means “not active authority,” never “delete from the corpus.”

## Sequential invariant

A later reconstruction state may consume an earlier reconstructed state, but it must never rewrite the meaning of an earlier state without recording the transformation. Authority changes are represented as explicit edges.

## Loading invariant

The active authority explicitly requires scene-level resolution rather than loading the complete 14,739-scene index into context. The graph therefore uses stable addresses and lazy retrieval as the intended operating model.

## Initial verified anchors

- 25-book authority roster in `WOWAS_CANON_AUTHORITY_v14.md`.
- 14,739-scene authority claim.
- 319-species monster registry claim.
- 51+ tracked-character floor.
- 33-book projection spine in the final canonical projection.
- `_scene_heading_index.tsv` already contains book/marker/title relationships, including Book 01 through Book 06 entries in the inspected segment.
- Active character, creature, scene, and world surfaces are explicitly named by the v14 authority.
