# NSQ Substrate Lock

NSQ is the native operating substrate for BRAXON.

## Canonical invariants

1. Canonical NSQ meaning is native, not machine-lowered.
2. Canonical NSQ uses NSQ-native classes (`nu*`), not standard unsigned machine classes (`u*`).
3. Canonical NSQ is built from alternating switch/lever semantics.
4. The current lever design target is **1126**.
5. The first semantic cell in a word/range declares language or operating surface.
6. Following cells in-range may declare symbols, macros, or algorithmic expressions.
7. Canonical NSQ may be compiled into derived machine artifacts, but derived artifacts may never redefine canonical meaning.
8. Any `u8/u16/u32/u64/u128` storage used in implementation is derived-only unless explicitly proven to be a transparent transport detail.
9. Upper layers may consume NSQ substrate meaning, but may not morph or reinterpret the substrate into legacy assumptions.
10. Substrate violations are architecture bugs.

## Derived-only patterns

The following are derived-only unless explicitly isolated in compatibility/export layers:

- symbol interning tables as identity
- macro interning tables as identity
- `symbol_to_id`
- `macro_to_id`
- `u8/u16/u32/u64/u128` as canonical meaning classes
- flattened parser outputs that erase switch/lever semantics
- benchmark transport frames
- adjacency-packed identity maps

## Required boundary

The substrate boundary is:

source surface -> canonical NSQ-native -> optional derived/export artifacts

The boundary is one-way.
