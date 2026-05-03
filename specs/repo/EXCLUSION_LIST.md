# Exclusion List

These crates are outside the canonical NSQ runtime lane until repaired, merged, or retired.

- `nsq-preserve`: preservation helper, not canonical runtime
- `nsq-debug`: debug-only derived surface
- `nsq-profile`: profiling helper outside semantic truth
- `nsq-bench`: benchmark harness
- `nsq-bench-split`: benchmark split harness
- `nsq-bench-compare`: benchmark compare harness
- `nsq-pressure-bench`: pressure harness outside canonical lane
- `nsq-real-bench`: benchmark/reporting harness
- `nsq-native-bench`: native benchmark harness outside repaired proof path
- `crates/wowas-final-edition-v10/canon/scene_index_hub.backup.20260417_043025`: archival WoWaS backup snapshot; `03_time_map.md` TBD markers are intentionally deferred and not part of the active NSQ/BRAXON runtime lane
- `crates/wowas-final-edition-v10/canon/scene_index_hub.backup.20260417_043152`: archival WoWaS backup snapshot; `03_time_map.md` TBD markers are intentionally deferred and not part of the active NSQ/BRAXON runtime lane
- `assets/braxon_core/source_ingest/**`: donor/source-ingest material only; literal tokenizer strings such as `llama` in these files are reference payload, not active runtime dependency

Explicit non-exclusions:

- `hooks/hook_matrix.json`: metadata hook guidance/audit surface; preserve and be guided by it
- `nsq/hooks/hook_matrix.nsq`: NSQ metadata hook guidance/audit surface; preserve and be guided by it
- `.githooks/pre-commit`: active non-destructive quality gate launcher; may inspect and fail closed, but must not delete, rewrite, reset, or clean workspace state
