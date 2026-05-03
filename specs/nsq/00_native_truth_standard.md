# NSQ Native Truth Standard

## Valid proof
A proof is valid only if the measured path is:

NSQ source
-> NSQ parser
-> NSQ compiler / packer
-> NSQ native artifact
-> NSQ native decode / execute surface

## Invalid proof
The following do NOT count as proof:
- placeholder metrics
- bootstrap-filled scoreboards
- JSON-only convenience reporting
- shell-wrapper success counted as language success
- host-language semantic execution standing in for NSQ
- debug, trace, scaffold, or fake-native paths

## Fairness rule
C must be measured as:
C source -> C compiler -> C native artifact -> C execution

NSQ must be measured as:
NSQ source -> NSQ compiler -> NSQ native artifact -> NSQ execution

No lane may inherit the other lane's advantages.

## Immediate objective
Build the smallest real NSQ-native lane that proves:
1. source surface exists
2. parser is deterministic
3. native packed artifact exists
4. decode is loss-bounded and deterministic
5. macros and switches participate in packing
6. membrane/state records participate in packing

## Temporary allowance
Rust may be used as a fabrication tool during bootstrap.
Rust may NOT be treated as the semantic identity of NSQ in final proof.

## Exit criterion
NSQ is ready for serious benchmarking only when:
- source grammar is fixed
- parser is fixed
- native format is fixed
- packed artifact is emitted from NSQ source
- packed artifact is decoded back deterministically
- release build excludes nonessential scaffolding
