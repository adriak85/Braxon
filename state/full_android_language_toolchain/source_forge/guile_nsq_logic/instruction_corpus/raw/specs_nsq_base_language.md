# NSQ Base Language

## Status

NSQ is a standalone base language and native runtime substrate.

It is not:
- a thin transport encoding
- a packed-ID compiler target
- a foreign-language import wrapper
- a benchmark frame format
- a host-runtime delegation layer

## Core rule

Lowered languages must be rebuilt into NSQ-native execution forms.

They must not define runtime truth by:
- live host-language import
- direct host interpreter dependence
- integer-lane semantic identity
- flatten-first transport compilation

## Native responsibilities

NSQ itself must define:
- native lexical primitives
- native value representation
- native record and state forms
- native execution rules
- native storage model
- native runtime control flow
- native module/main/library/config relationship

## Value model

Host integer types are not NSQ truth.

Rust or machine unsigned carriers such as:
- single-octet carriers
- dual-octet carriers
- quad-octet carriers
- extended host-width carriers

may appear in implementation details, but they do not define NSQ.

If NSQ is base-8 oriented, then NSQ must explicitly define:
- base-8 literal grammar
- base-8 storage grammar
- base-8 execution semantics
- conversion rules to/from lowered languages
- whether any non-base-8 representation is forbidden, tolerated, or derived-only

## Lowered language rule

Python, Lua, Rust-surface, shell-surface, and other language inputs are not to remain primary runtimes.

They must map into:
- NSQ-native structures
- NSQ-native execution units
- NSQ-native callable forms
- NSQ-native module bindings

## Runtime rule

NSQ runtime is the primary runtime for the NSQ-managed stack.

Derived artifacts may exist for:
- indexing
- query acceleration
- benchmarking
- compatibility export
- temporary transport

But those must be regenerated from NSQ-native truth.

## Immediate repo correction target

A path is invalid as base NSQ if it does any of the following before NSQ-native truth exists:
- assigns semantic identity through packed integer IDs
- collapses all source forms into a reduced transport line
- depends on host-language runtime execution as the real substrate
- treats derived indexes as primary truth
