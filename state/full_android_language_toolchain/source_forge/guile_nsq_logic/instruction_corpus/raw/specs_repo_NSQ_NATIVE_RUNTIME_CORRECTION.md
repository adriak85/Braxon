# NSQ Native Runtime Correction

## Wrong current surfaces

- `hooks/hook_matrix.json`
  Declared hook surfaces as authoritative for internal work. That is wrong because hook matrices may describe compatibility lowering, but they are not the canonical runtime execution path.
- `hooks/dialects.json`
  Kept `python_shape` and `python3_shape` framed as if Python authority lived in source lowering. That is wrong because source ingress is not runtime incorporation.
- `prompts/SYSTEM_PROMPT_BRAXON_DEV.md`
  Instructed operators to treat hook and dialect files as authoritative. That sends work back into the wrong architecture.
- `docs/AI_OPERATOR_BRIEF.md`
  Told operators to extend hook surfaces. That reinforces hook-first implementation instead of native runtime incorporation.
- `crates/nsq-source/src/lib.rs`
  Still models `PythonShape` and `RustHook` as dialect families for lowering. That is acceptable only as legacy source-ingress compatibility, not as runtime truth.
- `crates/nsq-runtime/src/lib.rs`
  Previously computed lever positions from `text.as_bytes()`. That is wrong because host byte carriers must not become canonical meaning.

## Why they are wrong

- Hook, plugin, wrapper, and sidecar framing keeps foreign language surfaces detached from NSQ runtime.
- Python 3 and the other required base languages must have explicit native ingress, canonical NSQ representation, and court-routed execution lanes.
- Byte-derived lever positions flatten canonical switch meaning into host transport details.

## Native Python 3 runtime lane

- Surface: `python3_native_runtime_lane`
- Ingress: native call ingress such as `score(task='alpha', retries=3)`
- Canonical representation: `NuWord` cells whose first cell declares the language surface and whose later cells preserve symbol, macro, and algorithm footprint in alternating anchor/lever pairs
- Lever resolution: semantic hertz samples derived from Unicode scalar values and stabilized through float-aware averaging into the `1..1126` multipositional range
- Court route: `policer -> lexer -> parser -> router -> inspector`
- Root launch path: `Braxon runtime python3 "<call>"`

## First real native runtime slice

The first runtime slice is implemented in `crates/nsq-runtime` and exposed through the root workspace entrance.

- `Python3RuntimeLane::execute_slice` now performs native ingress, switch-faithful encoding, and court-route reporting.
- `native_runtime_registry()` defines the corrected architectural model for adjacent required language and interface lanes so they do not drift back into hook/plugin/wrapper framing.
- The root command `Braxon runtime python3` executes the lane directly instead of reporting coverage-only metadata.
- The root command is wired through the root package dependency on `nsq-runtime`, preserving the command-center launch path while keeping canonical runtime meaning inside NSQ base-8 switch topology.
