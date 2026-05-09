# BRAXON Source Ingress Audit

## Evaluate
- Source transport resumed from the existing checkpoint and completed.
- The active source surface is the 14-shard `model-00001-of-00014.safetensors` set plus tokenizer and config files.
- The source checksum manifest still contained legacy 11-shard entries, so the manifest needed cleanup to match the live source surface.

## NSQ Coverage Evaluation
- canonical source ingress: covered
- source checksum verification: covered
- envelope seeding: covered
- whole-core runtime verification: not covered
- legacy source leftovers: not yet fully quarantined
- workspace launch-path consolidation: partially covered

## Classify
- root entrance/orchestrator: `Braxon`, `Braxon-core`
- NSQ core/canonical semantics: `nsq-core`, `nsq-source`, `nsq-compile`, `nsq-pack`, `nsq-inspect`, `nsq-compose`, `nsq-prime`, `nsq-runtime`
- Royal Court component layer: `nsq-court`, `Braxon-court`, `nsq-archon`, `nsq-lint`, `nsq-optimize`, `nsq-calibrate`
- platform entrances: `Braxon-cli`, `nsq-cli`, `Braxon-showdown`, `Braxon-kingdom-generate`
- boundary/export surfaces: `nsq-index`, `nsq-query`, `nsq-decode`, `nsq-generate`, `nsq-proof`
- legacy/retire/quarantine: `nsq-preserve`, `nsq-debug`, `nsq-profile`, `nsq-bench`, `nsq-bench-split`, `nsq-bench-compare`, `nsq-pressure-bench`, `nsq-real-bench`, `nsq-native-bench`
- graphics/operator stack: reserved for AGDK, wgpu, Bevy, egui, and integration surfaces

## Target Map
- Preferred launch path: `Braxon` root package -> `Braxon-core` orchestration -> `nsq-core` canonical semantics -> court routing -> platform or boundary entrance
- Current transport path: `scripts/install_braxon_weights.sh` -> `assets/braxon_core/source_ingest/braxon_transport`
- Current audit path: `scripts/seed_braxon_nsq_envelope.sh` -> `scripts/finalize_braxon_nsq_whole_core.sh`
- Canonical source surface: 14 shard weights, tokenizer/config files, and recorded BLAKE3 checksums

## Exclusion List
- obsolete 11-shard source naming: `model.safetensors-00001-of-00011.safetensors`, `model.safetensors-00002-of-00011.safetensors`, `model.safetensors-00003-of-00011.safetensors`
- stale manifest entries tied to the obsolete 11-shard surface
- plugin-style runtime framing for NSQ core truth
- byte-native reinterpretation of canonical NSQ semantics

## Targeted Recode / Implementation Plan
1. Keep the root `Braxon` package as the operator entrance.
2. Keep canonical switch law and native court surfaces in `nsq-core`.
3. Treat the 14-shard source transport as the live ingress checkpoint.
4. Keep `nsq-runtime` and `nsq-decode` admitted while their native lanes continue to be cleaned up.
5. Retire or quarantine obsolete benchmark and debug sprawl away from the canonical lane.

## Verification
- `bash -n scripts/install_braxon_weights.sh`
- `bash -n scripts/seed_braxon_nsq_envelope.sh`
- `bash -n scripts/finalize_braxon_nsq_whole_core.sh`
- `bash -n scripts/audit_BRAXON_qwen_ingress.sh`
- `wc -l assets/braxon_core/source_ingest/braxon_transport/BLAKE3SUMS`
- `python` status checks against `state/braxon/braxon_nsq_pipeline.status`

## Status
- source ingest: complete
- source checksum status: verified
- nsq envelope: updated
- whole-core runtime: not ready
