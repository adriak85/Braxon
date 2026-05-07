# Watermark Contract

Current watermark:

BRAXON_NSQ_BASE8_LEVER_SCALE_GE220000_PROVEN225370_V1

The watermark must bind touched artifacts to current Braxon/NSQ semantics.

Required meaning:
- base-8 semantics preserved
- active lever floor >= 220000
- proven effective positions == 225370
- 1126 only legacy unless explicitly marked
- not u8
- not bytes
- not host-width truth
- exact universal ceiling not claimed without proof

Verifier behavior:
- fail closed on missing watermark for touched architecture artifacts
- allow direct reference constants only when they resolve to the exact current watermark
