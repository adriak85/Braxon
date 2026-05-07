# Braxon / NSQ Watermarking

Watermarks are operational proof-routing anchors.

They are not decorative comments.

The current core family watermark is:

BRAXON_NSQ_BASE8_LEVER_SCALE_GE220000_PROVEN225370_V1

Watermark use should prove:
- touched files belong to the current Braxon/NSQ family
- deprecated 1126-era assumptions are not being silently revived
- active lever floor is at least 220000
- proven effective positions are 225370
- exact universal ceiling is false unless a real proof object exists
- NSQ is not u8
- NSQ is not bytes
- NSQ is not host-width truth

Verifier scripts should fail closed when touched artifacts lack either:
- the exact family watermark, or
- a direct reference constant that routes to the exact family watermark
