> NOTE: any u16/u32 or similar width-class notation in this file is derived boundary-carrier description only, never canonical NSQ truth.

# NSQ Native Artifact

## Status
Canonical source-of-truth representation for NSQ.

This artifact preserves:
- source dialect
- source surface
- semantic identity
- calibration context
- provenance

This artifact does **not** define canonical NSQ in terms of:
- u16 symbol IDs
- u16 macro IDs
- u32 anchor lanes
- ordered string intern tables
- adjacency packing
- benchmark transport frames

Those may exist only as **derived artifacts**.

---

## Canonical principles

1. Native NSQ is preservation-first.
2. Language surfaces are implemented as real source surfaces, not collapsed into one reduced transport line format.
3. Semantic records remain directly named and traceable.
4. Derived acceleration formats are disposable and regenerable.
5. Canonical truth must survive without index tables, binary packing, or integer-lane interning.

---

## Canonical artifact structure

A native artifact must preserve at least:
- artifact_version
- source_path
- source_dialect
- source_hash
- calibration_lock
- records[]
- provenance

---

## Canonical record families

### noise
Preserves:
- symbol
- macro_name
- a
- b
- pos
- amp
- original source form

### triple
Preserves:
- subject
- relation
- object
- layer
- plane
- anchor
- weight
- flags
- original source form

### membrane
Preserves:
- cell
- state
- flux
- gate
- phase
- original source form

---

## Source surface preservation

Every canonical record must preserve:
- original line or form
- dialect it came from
- parsed semantic content

This is required so that:
- sexpr remains sexpr-traceable
- lua-shape remains lua-shape-traceable
- python-shape remains python-shape-traceable
- canonical form remains canonical-traceable

---

## Derived artifacts

The following are allowed only as derived artifacts:
- packed binary transport
- integer-ID symbol tables
- adjacency indexes
- range-optimized storage lanes
- benchmark wire formats
- pressure or real-bench artifacts

Derived artifacts must never be treated as canonical truth.

---

## Invalid canonical patterns

The following are invalid as definitions of NSQ-native canonical state:
- legacy integer symbol lanes are forbidden in canonical artifacts
- legacy integer macro lanes are forbidden in canonical artifacts
- legacy integer anchor lanes are forbidden in canonical artifacts
- string interning as semantic identity
- lowering all dialects into one reduced canonical transport line
- classifying identity through derived lane classes for canonical storage is forbidden

---

## Required boundary

If a compiler path reduces direct symbolic identity into integer-packed representation
before canonical preservation is written, that compiler path is not a native compiler.
It is a derived transport compiler and must be named and treated accordingly.
