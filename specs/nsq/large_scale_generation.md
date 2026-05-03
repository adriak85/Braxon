# NSQ Large Scale Generation v0

Goal:
Produce a deterministic larger corpus that exercises:
- noise
- triple
- membrane
- calibrate

Properties:
- no randomness
- no tracer dependency
- reproducible from seedless arithmetic only
- suitable for compose + lint + optimize + calibrate + compile + inspect + proof

Generation families:
- noise lanes
- semantic triples
- membrane state transitions
- calibration locks

Scaling:
- count is caller-selected
- record generation is deterministic and stable
