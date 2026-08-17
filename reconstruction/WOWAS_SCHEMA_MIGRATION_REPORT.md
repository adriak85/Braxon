# WOWAS Post-Commit Schema Migration Report

The post-commit review identified TSV schema drift in two generated surfaces. The relationship ledger used `serial`, `character_id`, `other_id`, `event_id`, `scene_link`, and `status`; the timeline schedule used `schedule_serial`, `character_id`, `assigned_scene_id`, and `event_beat_id`. The initial gate incorrectly expected alternate names and failed closed.

The schema registry was corrected to the canonical names rather than weakening validation. The repaired gate validates the current 2,019-scene index plus nine generated TSV surfaces. It checks required columns and row widths, computes SHA-256 hashes and byte sizes, and writes `reconstruction/WOWAS_SCHEMA_CACHE_MANIFEST.json`.

| Measure | Result |
|---|---:|
| Validated TSV surfaces | 10 |
| Schema gate status | pass |
| Schema failures | 0 |
| Total validated rows | 2,082,519 |
| Canonical scene rows | 2,019 |
| Background population rows | 2,000,000 |
| Relationship rows | 45,000 |
| Timeline schedule rows | 20,000 |

The cache manifest is deterministic and fail-closed: a missing file, missing required column, or malformed row produces a failure and prevents the pipeline from being treated as complete. The manifest records each file’s schema version, header, required columns, row count, byte count, and SHA-256 digest.

The legacy `scene_index_reasonable_window.tsv` remains available for comparison and migration evidence. It is not included as an authoritative required surface, because the adopted reader-ingest target is the 2,019-scene reasonable window. Its pre-existing terminal whitespace warnings are therefore not silently treated as clean source; they remain visible in full-tree diff diagnostics and are excluded only from the authoritative source/documentation diff check.

The 2,000,000 background records pass structural validation and all point to the current scene and event-beat set. They remain marked `deterministic_seeded_requires_batch_review`. This status does not claim that every record has independently authored prose quality; editorial review remains a separate gate.
