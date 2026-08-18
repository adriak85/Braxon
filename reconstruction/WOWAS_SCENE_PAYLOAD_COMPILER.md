# WOWAS Scene-Payload Compiler

## Purpose

The WOWAS payload compiler creates a **bounded, deterministic handoff artifact** for one canonical narrative coordinate. It packages repository-backed metadata, NSQ semantic intent, NativeNsqReflexor phase semantics, authored character and dynamics constraints, resonance and tone sources, a bounded rolling-state slice, and cryptographic provenance. It does **not** generate prose and it cannot authorize prose generation.

> `record_id` is the authoritative coordinate identity. `scene_id` is context only because duplicate scene groups exist.

## Contract files

| File | Function |
|---|---|
| `reconstruction/WOWAS_SCENE_PAYLOAD_SCHEMA_v1.json` | Versioned machine-readable contract. |
| `audit/compile_wowas_scene_payload.py` | Deterministic compiler for one `record_id`. |
| `audit/validate_wowas_scene_payload.py` | Fail-closed hash, shape, source, and no-prose validator. |
| `reconstruction/payloads/BOOK_01_FIRST_SCENE.json` | First compiled Book 1 payload; no prose is present. |

The compiler reads the reconciled metadata index and the authored flavor, dynamics, Pip leadership, generated-character, relationship, and 33-book contract layers. It additionally seals the v12 resonance patch and v14 prose/tone guide. Missing required sources, duplicate authoritative identifiers, unresolved book contracts, or a coordinate whose `prose_status` is not `no_generated_prose` block compilation.

## Payload sections

| Section | Contents | Safety rule |
|---|---|---|
| `coordinate` | `record_id`, book, title, `scene_id`, active cast, source classification | `record_id` is authoritative; `scene_id` cannot be used as an output key. |
| `intent` | Eight NSQ variables, four scale anchors, metadata-backed semantic inputs, derivation policy | The compiler uses the NSQ final-tier midpoint contract as an explicit baseline and never fabricates scene-specific lever values. |
| `reflexor` | Ecology, geography, creature, transformation, and world-introduction inputs plus `Publish → Reconcile → DeltaCommit` | Only changed values are committed; same-space override is false; watermark refresh is required. |
| `constraints` | Authored flavor, dynamics, Pip leadership, relationship ledger, book contract, unmapped anchors | Unmapped generated anchors remain quarantined and cannot receive invented identity. |
| `state_slice` | Only active-cast state and book/record bridges | Initial absence is explicit; unrelated characters are excluded rather than silently injected. |
| `resonance` | Source hashes and bounded line references from the resonance patch and tone guide | Private alignment values are not guessed. |
| `watermark` | Active Braxon stamp, input hashes, schema hash, preflight hash, payload hash | Any source or payload mutation blocks validation. |
| `execution_boundary` | Staging, human review, no-prose, and promotion requirements | `prose_generation_permitted` is permanently `false` in this pre-realization stage. |

## Usage

Compile a specific coordinate by its authoritative identifier:

```bash
python3 audit/compile_wowas_scene_payload.py \
  --record-id 'beat_candidate:0033903:WE01_00001' \
  --output reconstruction/payloads/BOOK_01_FIRST_SCENE.json
```

Validate the result:

```bash
python3 audit/validate_wowas_scene_payload.py \
  reconstruction/payloads/BOOK_01_FIRST_SCENE.json
```

An AI connected to the repository can reproduce the same artifact by running those commands against the same commit. The output contains the source paths and SHA-256 values required to verify that it consumed the same repository state. No chat history is required.

## Execution boundary

The payload is a **pre-realization artifact**. A downstream prose engine may read it as a director’s contract, but it must not treat the payload as permission to bypass staging, tone/cadence/style checks, rolling-state updates, or human review. The payload intentionally contains no generated prose field and its validator rejects any attempt to authorize prose or alter the sealed contents.

The native NSQ mapping is descriptive and contract-bound: semantic intent corresponds to the eight-variable NSQ gradient, environmental feedback corresponds to `NativeNsqReflexor::orbit`, and the three reflexor phases preserve the native publish/reconcile/delta-commit cycle. This payload layer does not claim to replace the native runtime; it gives a reproducible AI-facing boundary into the already-defined substrate.

## Validation evidence

The implementation was validated with the following checks:

| Check | Result |
|---|---|
| First Book 1 payload compilation | Passed for `beat_candidate:0033903:WE01_00001`. |
| Payload SHA-256 self-verification | Passed. |
| Watermarked source verification | Passed for 9 inputs, including metadata, authored constraints, resonance, and tone sources. |
| Tamper test | Passed: changing `prose_generation_permitted` was blocked. |
| Same-record reproducibility | Passed: repeated compilation produced byte-identical JSON. |
| Duplicate `scene_id` isolation | Passed for two distinct B25 `record_id` values sharing `B25_S001`; both compiled and validated independently. |
| 33-book smoke matrix | Passed: one independently selected coordinate for each canonical book key `B01`–`B33`. |

No generated prose has been committed by this payload system.
