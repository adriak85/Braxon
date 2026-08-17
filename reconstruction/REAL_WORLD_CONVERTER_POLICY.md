# WOWAS Real-World Converter Policy

## Source hierarchy

The converter uses stable source identifiers rather than copying uncited prose. Wikidata provides cross-source entity identity for cities; GeoNames provides gazetteer normalization, alternate names, and geographic lookup; UNESCO World Heritage records provide cited heritage-list identity for selected landmarks. A source record is not treated as a complete cultural or historical interpretation merely because it has a stable identifier.

## Five-pass conversion

| Pass | Purpose | Output rule |
|---|---|---|
| 1. Identity | Normalize the city or landmark name and stable source identifier | `FACT`, source URL, source ID |
| 2. Source claim | Record the narrow claim supported by the source | No uncited expansion |
| 3. Cross-source normalization | Connect city, landmark, alternate names, and geography | Preserve all source references |
| 4. WOWAS alignment | Attach the source record to an existing book, scene, event beat, and world anchor | Create a separate transformation seed |
| 5. Reader-load control | Keep real-world detail off-page until relevant | Promote only through scene relevance or user preference |

## Fact-fiction boundary

`FACT_SOURCE_ONLY` records contain only source-supported identity or listing membership. `WOWAS_TRANSFORM_SEED_REVIEW_REQUIRED` records are creative alignment options, not facts. They may suggest arrival context, observation, history questions, quest hooks, or world-system echoes, but they must not be rendered as factual claims until editorial review attaches appropriate sources.

## Provenance

Every source record carries a bottom-footnote serial, source URL, source ID, retrieval or evidence origin, pass status, citation-required flag, and SERIEL linkage. Every alignment record carries its source serial, scene ID, event-beat ID, book number, beat kind, transformation options, and reader-projection rule.

## Coverage boundary

The current executable registry includes 30 major cities and 20 UNESCO landmark records. This is a verified seed set, not a claim to exhaustive worldwide coverage. Additional cities and landmarks can be added through the same schema and five-pass process without changing the canonical WOWAS runtime or exceeding reader-facing scene capacity.
