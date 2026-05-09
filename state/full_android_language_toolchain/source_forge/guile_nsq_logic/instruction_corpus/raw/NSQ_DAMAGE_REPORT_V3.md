# NSQ damage report v3

## patched in this pass

### canonical truth surfaces
- `crates/nsq-core/src/lib.rs`
  - public canonical pair now uses `FullBinaryAnchor` and `MultipositionalLever`
  - hertz-aware lever stabilization is explicit
  - raw host carriers are boundary helpers instead of public canonical meaning

### structural drift surfaces
- `crates/nsq-optimize/src/main.rs`
  - replaced width-class output labels with boundary carrier and projection labels
- `crates/nsq-calibrate/src/main.rs`
  - calibration lock now carries boundary projection labels rather than lane classes
- `crates/nsq-profile/src/main.rs`
  - seeded profile lock updated to the boundary-carrier vocabulary
- `crates/nsq-real-bench/src/main.rs`
  - removed `u32` identity carriers from the internal packed bench shape

### architecture scaffolding
- added boundary architecture spec
- added knowledge graph spec
- added vector imprint spec
- added runtime watcher spec
- added stack-surface registry spec
- added language-master spec
- added matching config seeds under `config/nsq/`

## remaining strongest drift

### direct derived-export surfaces still needing deeper cuts
- `crates/nsq-index/src/lib.rs`
- `crates/nsq-pressure-bench/src/main.rs`
- `crates/nsq-source/src/lib.rs`
- `crates/nsq-lint/src/main.rs`

## continued cut rule

When a surface is canonical or court-facing, remove width-class truth.
When a surface is foreign or transport-facing, isolate the carrier at the boundary and name it as derived export behavior.
