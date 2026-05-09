# NSQ Proof Plan

## Objective
Prove NSQ as a native information language rather than a dressed-up C/Rust benchmark.

## What must be true before proof is valid

### 1) NSQ-native source exists
We need real `.nsq` source surfaces, not only Rust stand-ins.

Minimum source classes:
- raw noise lane
- structured lane
- macro/hot-swap lane
- membrane/state lane
- decode/export lane

### 2) NSQ-native compile path exists
We need a real path:

`NSQ source -> NSQ IR -> NSQ packed artifact -> decode/export`

Not:
`idea of NSQ -> Rust benchmark`

### 3) Canonical artifact spec exists
Artifact must define:
- symbol table
- macro table
- switch/lever encoding
- membrane/state encoding
- delta/reference encoding
- checksum/hash field
- deterministic decode rules

### 4) Proof metrics are information metrics
Primary score:
- decoded human-readable records
- decoded structured relations
- recoverable symbols
- recoverable transitions
- native artifact bytes
- decoded bytes
- information per native byte
- information per second

Secondary score:
- build time
- query time
- artifact size
- corruption handling
- deterministic replay

### 5) C comparison must be fair
C must use:
- standard C model
- its own normal binary conventions
- no NSQ levers/macros/membrane semantics

NSQ must use:
- its own native source form
- its own packed representation
- its own switch/lever semantics
- its own macro bank
- its own decode path

## Deliverables

### A. Lock spec
Create:
- `specs/nsq_native_artifact.md`
- `specs/nsq_source_surface.md`
- `specs/nsq_proof_scoring.md`

### B. Create crates
Create these crates if missing:
- `crates/nsq-source`
- `crates/nsq-ir`
- `crates/nsq-pack`
- `crates/nsq-decode`
- `crates/nsq-proof`
- `crates/nsq-sound` (optional after proof text works)

### C. First proof targets
1. noise.nsq
2. structured.nsq
3. membrane.nsq

### D. First valid proof run
For each lane:
1. compile native artifact
2. decode to text
3. score recovered information
4. compare native size
5. compare decode integrity
6. compare replay determinism

### E. App matrix comes after proof lane
Only after A-D are real:
- c_native
- rust_native
- python_native
- kotlin_native
- nsq_native
- nsq_to_c
- nsq_to_rust
- nsq_to_kotlin

## Immediate implementation order

1. Write locked specs
2. Create NSQ source files
3. Implement pack/decode pair
4. Implement proof scoring
5. Run proof lane
6. Then build app matrix

## Failure conditions
If any of these are true, proof is invalid:
- NSQ lane is actually Rust pretending to be NSQ
- C lane is forced into NSQ semantics
- score measures bytes only instead of information
- app matrix uses placeholders
- no deterministic decode exists

## Success condition
NSQ is proven only if:
- native NSQ source compiles
- native packed artifact stores meaning densely
- decode restores meaningful structure deterministically
- score shows higher recoverable information density or materially better native storage behavior for the claimed lanes
