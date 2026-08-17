# SERIEL / Lattice Graph Contract

The graph is an overlay on the existing WOWAS indexes. It does not replace the source indexes.

## Node identity

Every node carries:

- `node_id`
- `node_type`
- `source_path`
- `source_record_key` (when the source provides one)
- `source_line_start`
- `source_line_end`
- `authority_state`
- `variant`
- `quality_state`

## Edge identity

Every edge carries:

- `from`
- `relation`
- `to`
- `source_path`
- `source_line_start`
- `source_line_end`
- `authority_state`

## Required lattices

`narrative`, `timeline`, `character`, `relationship`, `atlas`, `faction`, `magic`, `world-state`, `creature`, `consequence`, `prose`, and `provenance`.

A node may occupy multiple lattices. Cross-lattice edges are first-class and are not flattened into prose summaries.

## SERIEL rule

Where an existing source record already has a SERIEL/index identity, that identity remains the navigation key. The reconstruction adds graph edges around it instead of generating a competing identifier.

## Oversized sources

Large TSV/JSON/text sources are segmented deterministically for processing. Segment metadata preserves the original path, ordering, and line offsets. No segment is treated as an independent source object.

## Authority resolution

When two source nodes conflict:

1. current v14 authority controls active canon;
2. current v1 cohesive canon controls manuscript law;
3. source-of-truth registry controls ingestion/routing;
4. explicit later valid patches supersede older patches;
5. historical/backup variants remain provenance but are not silently promoted.

## Runtime retrieval

The source explicitly requires scene-level retrieval rather than loading the entire saga into context. Graph traversal therefore resolves by book/scene/node identity and retrieves only the required source segments.
