# Blake Null: NSQ Semantic Digest

## Canon rule

BLAKE3 remains the raw byte verifier.

SHA-256 remains the compatibility verifier.

Blake Null is the NSQ-native semantic digest over canonical parsed NSQ structure.

Blake Null does not replace BLAKE3.

Blake Null sits beside BLAKE3.

BLAKE3 proves what bytes were stored.

Blake Null proves what NSQ means after parsing and canonicalization.

The final trust rule is:

raw byte identity plus parsed NSQ identity equals trusted NSQ identity.

## Verification stack

The full proof stack is:

1. byte count
2. SHA-256
3. BLAKE3 when available
4. NSQ parse
5. NSQ lint
6. NSQ canonical form
7. Blake Null semantic digest
8. court-route validity
9. model/runtime binding validity

A file passes only when raw identity and semantic identity both pass.

## Why not modify BLAKE3

If BLAKE3 is changed, it is no longer BLAKE3.

That would weaken outside verification.

The correct design is not altered BLAKE3.

The correct design is BLAKE3 plus Blake Null.

## Canonicalization

Blake Null digests canonical NSQ form.

Canonical NSQ form must:

- preserve NSQ native meaning
- remove transport-only layout noise
- preserve anchors
- preserve levers
- preserve slot assignments
- preserve runtime surface identities
- preserve court route
- preserve prime-path multiplication objects
- preserve introduced vectors
- preserve model stamp references
- preserve language surface identity
- preserve platform binding

Foreign spellings are projection only.

Canonical meaning is sovereign.

## Current implementation lane

The first local implementation may be Python for clarity.

The production implementation should move into Rust/C.

Hot loops can move into ASM after the canonical form stabilizes.

ASM should accelerate the stable kernel, not become the first unreadable truth layer.
