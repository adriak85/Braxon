# Runtime Boundaries

Braxon root, NSQ court, host OS, Python ingress, handover, console, and app surfaces must remain semantically distinct.

Python ingress must not claim it constructed a second runtime.

Required concepts:
- native_runtime_constructed: false
- court_roles_duplicated_into_runtime: false
- executed_as_second_runtime: false
- status: ingress_recorded_without_runtime_claim
- authority: NSQ_COURT
- canonical_semantics: base8_switch_topology

Handover must not imply power disconnect unless explicitly requested and validated.

Root handover should preserve:
- release without power disconnect
- all-in check validation
- ten-surface bus validation
- voice/video presence reporting where required by tests
- watermark trigger validation
- semantic address gate validation
- seven suit cycle validation when contract requires it
