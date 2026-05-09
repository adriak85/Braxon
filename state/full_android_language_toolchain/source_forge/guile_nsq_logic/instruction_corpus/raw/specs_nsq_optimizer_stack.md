# NSQ Optimizer Stack v0

## Goal
Implement deterministic optimization surfaces inside the NSQ lane.

## Components
- nsq-lint: validates record surfaces and range correctness
- nsq-compose: composes multiple NSQ inputs into one working surface
- nsq-optimize: derives optimization suggestions and selected profiles
- nsq-proof: scores proof outputs from artifact + inspect text

## Features
### Auto macro generation
Detect repeated semantic lanes and suggest candidate macros.

### Algorithmic expansion
Detect high-frequency symbol/relation clusters and emit expansion candidates.

### Boundary carrier inference
Infer the smallest safe foreign boundary carriers for derived exports without redefining canonical NSQ.

### Balancing
Measure family distribution:
- noise
- triple
- membrane
- calibrate

### Validating
Optimization must only operate on lint-valid sources.

### Live selection
Choose one active optimization profile:
- dense_small
- balanced
- decode_favoring

## Calibration intent
Calibration is deterministic corpus-driven adjustment, not heuristic drift.
It should tune:
- macro suggestions
- selected optimization profile
- safe boundary projection sizing
- balance recommendations

## Non-goal
No fake benchmark scoring.
No placeholder matrix filling.
No tracer-dependent execution.
