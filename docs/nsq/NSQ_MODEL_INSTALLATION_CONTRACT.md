# NSQ Model Installation Contract

## Canon rule

Braxon model installation is local, offline-first, manifest-bound, and verification-bound.

A model is not installed because a pointer or donor name exists.

A model is installable when the local registry, binding file, source target map, reconstruction manifest, and stamp bundle are present.

A model is installed when required files exist and pass exact byte count plus full-file cryptographic hashes.

A model is hot-live only when the runtime can actually route through it.

## Required model states

catalog_only:
The model has pointer metadata but no local proof of usable bytes.

manifest_bound:
The model has a local manifest and binding state.

source_targets_present:
Small source/config/tokenizer targets exist locally.

reconstruction_ready:
The system has enough local data, stamps, deltas, exceptions, or deterministic generation rules to reconstruct the artifact.

hash_verified:
The reconstructed artifact matches exact byte count plus full SHA-256 plus full BLAKE3 when available.

nsq_semantic_verified:
The artifact also parses and validates through NSQ semantic digest rules.

hot_live:
The runtime can route local inference or local semantic execution through the model lane.

## Hash rule

Exact byte count plus full SHA-256 plus full BLAKE3 can prove byte identity after bytes exist.

One-out-of-six-hundred-ninety-nine sampling is not full identity proof by itself.

One-out-of-six-hundred-ninety-nine sampling is allowed as:

- route witness
- drift detector
- sparse precheck
- reconstruction guidance
- pressure test

It becomes reconstructive only when paired with deterministic generation, complete source map, erasure/parity data, or a full manifest of recoverable pieces.

## Model install target

The current Braxon target is a local NSQ/stamp-bound model lane.

Expected local control files include:

- models/braxon/manifest.json
- state/braxon/offline_model_registry.json
- state/braxon/braxon_binding.json
- state/braxon/braxon_nsq_pipeline.status
- assets/braxon_core/source_ingest/braxon_transport
- assets/braxon_core/weights/nsq
- state/nsq/model_reconstruction_manifest.json

The runtime should report the difference between:

- model metadata exists
- source targets exist
- compressed NSQ artifact exists
- manifest claims hash identity
- hash identity actually verifies
- runtime can use it
