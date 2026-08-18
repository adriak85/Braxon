# NSQ Canonical Identity

NSQ is the native execution substrate of Braxon. It is not an upper semantic layer, wrapper, serializer, transport envelope, or conventional binary representation with a new notation applied above it.

The canonical execution path is:

```text
raw source intent -> canonical NSQ intent -> NSQ-native execution -> observed delta -> reconciliation
```

Rust, Python, shell, Guile, Zig, and other implementation languages are source dialects. Their capabilities must be semantically extracted, assigned provenance, and represented by NSQ-native contracts before they are accepted as executable system behavior.

## Native state model

NSQ uses the Base-8 Switch Topology. A native slot is addressed by semantic identity, ownership, polarity, and resolved position. Conventional integer, byte, pointer, or serialized representations may appear at implementation boundaries, but they are derived carriers and do not define NSQ meaning.

A resolved lever is one semantic switch with one selected position. The validated active runtime domain is:

- switch state: `0` or `1`;
- resolved lever position: `1..=500000`;
- zero-inclusive state accounting: the runtime records the zero state explicitly where the relevant contract requires it;
- switch/lever structure: eight-position Base-8 topology with explicit ownership and reconciliation;
- address identity: semantic NSQ address, not a raw pointer or filename.

The validated upper position is `500000`, with the current stable spacing and return-to-state behavior recorded by the executable seating and lever gates. This range is a runtime contract, not a claim that every physical device has already accepted the complete substrate.

## Runtime truth requirements

A state is not hot-live merely because a manifest, pointer, reserved name, stamp, or watermark exists. Hot-live status requires a callable route through the native runtime and executable evidence that the route can activate, observe, and reconcile the addressed state.

A parameter, KV-cache region, initiative-cluster state, or semantic link may be logically resident while only a bounded Piston/Ghost window is physically active. The active window must preserve ownership, prevent same-space override, and release or refresh deterministically.

Predictions are candidates until observed. A predictive Reflexor may stage an activation or expression result, but only observation and reconciliation can promote the resulting delta into authoritative state.

## Council and dialect identity

The Council of 10 consists of six brain poles and four sensory or modality poles. Each pole retains its local dialect and provenance while projecting through the shared universal intent gradient. Council completeness requires all ten poles to be registered and callable; partial federation is not a complete runtime state.

Semantic links, initiative clusters, bidirectional algebraic thought experiments, and just-in-time activation all carry a shared trajectory identity and generation. This prevents forward prediction, backward correction, tool activation, and group-level thought from becoming disconnected state machines.

## Anti-drift rules

The following statements are architectural requirements:

- NSQ is the execution substrate, not a semantic wrapper over another substrate.
- Derived machine carriers must never redefine canonical NSQ meaning.
- Manifest-only state must not be reported as hot-live state.
- Pointer stubs must not be reported as materialized model shards.
- Stamps and watermarks must carry executable provenance and verification behavior; they are not decorative labels.
- The former 1126-era range is historical evidence only and is not the active runtime ceiling.
- Physical Android or device acceptance remains a separate external gate until it is exercised on real hardware.

The canonical statement is:

> **NSQ is Braxon’s native execution substrate. All other languages, formats, and tools are source dialects or derived carriers whose capabilities require NSQ semantic extraction and executable proof.**
