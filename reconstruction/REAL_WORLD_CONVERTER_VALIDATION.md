# Real-World Converter Validation

The converter currently contains 30 city records and 20 UNESCO landmark records. Each source record has a stable identifier, source URL, source claim, five completed conversion passes, a bottom-footnote serial, and a `FACT_SOURCE_ONLY` boundary. Each source is aligned to a bounded WOWAS scene, event beat, and book anchor.

The domain alignment layer contains 400 records: 50 each for location, artifact, faction, culture, hazard, route, quest, and character-world-role. Every alignment has a citation URL, scene ID, event-beat ID, book number, reader promotion policy, and `source_fact_not_rewritten` status. Fictional transformation remains `WOWAS_TRANSFORM_SEED_REVIEW_REQUIRED`.

The compact scene window remains 2,019 scenes. Real-world details are constrained to one source detail per scene or one domain echo per scene and are promoted only when relevant to the user, active quest, scene, or character world role. The digital-variance policy may vary presentation, but source facts and canonical WOWAS continuity remain immutable.

SERIEL validation reports 167,177 linked records and zero unlinked records.

The sandbox-side HTTP check returned status 403 for the Wikidata and UNESCO URLs. This is recorded as an access restriction, not treated as source invalidity. The URLs and stable IDs remain attached to every record, and the official UNESCO, GeoNames, and Wikidata pages were read through the browser before normalization. No claim of live endpoint availability is made.

The current registry is a verified seed set, not an exhaustive world census. More cities and landmarks can be added through the same converter without changing the schema or inflating reader-facing scenes.
