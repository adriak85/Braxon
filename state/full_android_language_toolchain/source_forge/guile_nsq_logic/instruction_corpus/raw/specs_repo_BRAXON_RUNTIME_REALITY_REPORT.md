# Braxon Runtime Reality Report

## Concise Reality Report

- The active donor ingest root is `assets/braxon_core/source_ingest/braxon_transport`.
- That donor root is real and complete enough to verify against `BLAKE3SUMS`.
- The donor anchor present there is Huihui `Qwen3.5-27B` abliterated lineage, not a restrictive fallback base.
- `state/braxon/braxon_binding.json` was previously bound to stale side-config surfaces under `assets/braxon_core/model_config`.
- `Braxon runtime infer` does not open safetensors or load live parameters. It emits a stamp/report shape from `crates/nsq-runtime/src/lib.rs`.
- `assets/braxon_core/tokenizer/braxon_unified_tokenizer.json` is consumed today only as semantic-feed context in `crates/nsq-runtime/src/semantic_context.rs`.
- `assets/braxon_core/tokenizer/braxon_supermodel_tokenizer.json` is not consumed by runtime code today.
- `volumes/chunks/delta_cluster_*.Braxon` is not wired into live codepaths today.
- `volumes/chunks` is absent in the current workspace snapshot, so the large delta-bank path is unproven at both file-presence and codepath levels.
- `assets/braxon_core/bootstrap/braxon_slice/*.delta` is present as bridge material but is not expanded into live parameter reality today.
- `assets/voice` is absent in the current workspace snapshot, so no additional voice-delta live input is presently available to audit.
- `assets/braxon_core/weights/nsq/Braxon-32B_extended.nsqb` is now explicitly marked as a manifest-bound candidate, not hot/live whole-core proof.
- ZLM session surfaces are configured nominally and consistently as session metadata, but not backed by hot/live parameter embodiment.

## Truth-Layer Map

- Canonical substrate truth: NSQ base-8 anchor/lever machine law. See [NSQ_CANONICAL_IDENTITY_NOTE.md](/data/data/com.termux/files/home/Braxon/specs/repo/NSQ_CANONICAL_IDENTITY_NOTE.md).
- Donor ingress truth: `assets/braxon_core/source_ingest/braxon_transport`.
- Bridge truth: `assets/braxon_core/bootstrap/braxon_slice/*.delta`, `assets/braxon_core/tokenizer/braxon_unified_tokenizer.json`, `assets/braxon_core/tokenizer/braxon_supermodel_tokenizer.json`.
- Binding truth: `state/braxon/braxon_binding.json` now points at donor transport config and marks tokenizer/parameter state as not runtime-unified and not hot/live.
- Manifest/status truth: `state/braxon/braxon_nsq_pipeline.status`, `state/braxon/offline_model_registry.json`, `models/braxon/manifest.json`, `state/braxon/production_state_bundle.json`.
- Overclaim candidate surface: `assets/braxon_core/weights/nsq/Braxon-32B_extended.nsqb` remains a manifest bundle candidate, not whole-core runtime proof.

## Delta-Expansion Map

- `volumes/chunks/delta_cluster_01.Braxon` through `delta_cluster_42.Braxon`: unproven in current repo state. No live code references were found.
- `assets/voice/*`: unproven in current repo state because no voice subtree is present in this workspace snapshot.
- `assets/braxon_core/bootstrap/braxon_slice/qwen_qwen3_8b_tokenizer_json.delta`: partial bridge only.
- `assets/braxon_core/bootstrap/braxon_slice/qwen_qwen3_8b_vocab_json.delta`: partial bridge only.
- `assets/braxon_core/bootstrap/braxon_slice/qwen_qwen3_8b_merges_txt.delta`: partial bridge only.
- `assets/braxon_core/bootstrap/braxon_slice/qwen_qwen3_8b_model_safetensors_index_json.delta`: partial bridge only.
- `assets/braxon_core/bootstrap/braxon_slice/qwen_qwen3_8b_tokenizer_config_json.delta`: partial bridge only.
- `assets/braxon_core/bootstrap/braxon_slice/qwen_qwen3_8b_config_json.delta`: partial bridge only.
- `assets/braxon_core/bootstrap/braxon_slice/qwen3_8b_tok_cfg.delta`: partial bridge only.
- `assets/braxon_core/bootstrap/braxon_slice/qwen3_8b_config.delta`: partial bridge only.
- Current stop point: these bridge deltas are cataloged and named, but not expanded into live donor parameter state or runtime-applied tensors.

## Tokenizer-Unification Map

- `assets/braxon_core/source_ingest/braxon_transport/tokenizer.json`: live donor tokenizer source.
- `assets/braxon_core/source_ingest/braxon_transport/tokenizer_config.json`: live donor tokenizer config source.
- `assets/braxon_core/tokenizer/braxon_unified_tokenizer.json`: partial bridge. Proven consumer is semantic-feed loading, not runtime tokenization.
- `assets/braxon_core/tokenizer/braxon_supermodel_tokenizer.json`: partial bridge or stale abandoned product. No live runtime consumer was found.
- Current truthful binding state: `semantic_feed_bound_not_runtime_unified`.
- Current truthful answer: tokenizer unification is not real as a runtime-bound tokenizer today.

## Hot/Live Loading Map

- Live donor weight location: `assets/braxon_core/source_ingest/braxon_transport/model-00001-of-00014.safetensors` through `model-00014-of-00014.safetensors`.
- Live donor index location: `assets/braxon_core/source_ingest/braxon_transport/model.safetensors.index.json`.
- Current codepaths that touch live donor truth: checksum/inventory scripts and binding/config reconciliation.
- Current codepaths that do not touch live tensors: `crates/nsq-runtime/src/lib.rs` offline inference lane, CLI status/report surfaces, `.nsqb` manifest candidate generation.
- Current truthful parameter binding state: `direct_source_materialization_required`.
- Current truthful whole-core state: `manifest_verified_not_hot_live`.
- Current truthful answer: hot/live parameter loading is not real today.

## Retirement / Reform Enforcement

- Retire `verified` or `whole_core` wording when proof is only manifest bookkeeping.
- Retire stale `assets/braxon_core/model_config/*` authority for donor truth.
- Retire `qwen_core_binding.json` assumptions in generation scripts.
- Retire old tokenizer-bound and parameter-bound strings that implied runtime completion.
- Retire stale donor filename drift that prevented manifest verification.
- Retire workspace false-green from the missing `crates/nsq-ir` member.
- Retire any wording that treats NSQ as IR, wrapper, or transport envelope.

## Pool-Ready Implementation List

### Donor Ingest

- Repair artifact naming away from `Braxon-32B` surfaces that still imply the wrong donor class.
- Add explicit donor provenance fields to binding, manifest, and registry surfaces.
- Normalize `qwen_transport.*` roots under explicit legacy/quarantine labels.

### Delta Expansion

- Add delta-cluster discovery over `volumes/chunks/delta_cluster_*.Braxon`.
- Define delta metadata schema for parameter families, tokenizer interactions, semantic domains, and conflict risk.
- Implement donor-index-to-delta application planning over the safetensors index.
- Implement live delta expansion into a parameter overlay structure.

### Tokenizer Unification

- Build a real tokenizer merge pipeline rooted in donor tokenizer truth plus NSQ docs, WoWaS, codebase references excluding tokenizer trees, and factual corpus inputs.
- Add runtime-bound tokenizer proof that the generated tokenizer is what live loading consumes.
- Demote `braxon_supermodel_tokenizer.json` unless a runtime consumer is added.

### Hot/Live Loading

- Add a real safetensors loader path with memory mapping or equivalent bounded loading.
- Add donor-index-driven tensor lookup and activation.
- Add hot/live load proof surfaces distinct from manifest proof.
- Add runtime tests that fail unless actual tensor pages are opened and addressed.

### Registry / Status / CLI

- Split manifest proof, donor ingest proof, delta expansion proof, tokenizer runtime proof, and hot/live proof into separate states.
- Rename `BRAXON_core_ready`-style outputs that currently read as stronger than they are.
- Add explicit negative proof fields such as `hot_live_parameter_embodiment=false`.

### NSQ Rewrite

- Add stamp planning over repeated tensor families, tokenizer metadata, and semantic bundles.
- Add translation planning from donor tensor/index truth into NSQ-native stampable families.
- Add proof surfaces showing where rewrite is candidate-only versus embodied.

## First Repair Batch Applied

- Removed the missing `crates/nsq-ir` workspace member so the root workspace resolves again.
- Rebound `state/braxon/braxon_binding.json` to donor transport config and generation config instead of stale side-config files.
- Downgraded tokenizer and parameter binding states to truthful non-runtime-complete values.
- Fixed offline registry repair logic so the primary `Braxon` asset is treated as the real Braxon core asset.
- Restored the donor-root `README.md` expected by the checksum manifest.
- Patched `.nsqb` generation to mark hot/live embodiment as false, delta expansion as not implemented, and tokenizer runtime unification as not proven.
- Patched finalize/verify/envelope scripts so manifest verification no longer produces fake whole-core green.
