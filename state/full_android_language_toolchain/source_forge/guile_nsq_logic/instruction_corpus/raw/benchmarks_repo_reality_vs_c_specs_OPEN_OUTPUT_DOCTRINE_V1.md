# Open Output Benchmark Doctrine V1

## Core principle
The contest measures how much real operator-usable value each base can produce
from the same intake under the same clock.

The systems are not forced into the same internal method during generation.

## Fairness
Fairness means:
- same intake
- same start condition
- same timed window
- same stop condition
- same post-run evaluation discipline
- native strengths fully allowed
- no sabotage
- no artificial parity inflation
- no compensatory handicapping

Fairness does NOT mean:
- forcing NSQ to act like C
- forcing C to act like NSQ
- charging one system a translation tax during the timed phase
- flattening stronger substrate advantages
- ignoring artifact reuse and replay stability

## Timed phase
During the timed phase, each system may emit in its strongest native form.

Examples:
- native text
- native binary
- native packed artifact
- native graph
- native table
- native structured stream
- native semantic substrate

No human-readable normalization is required during the timed phase.

## Post-run phase
Only after the timed phase closes may outputs be:
- decoded
- inspected
- projected
- normalized
- rendered into human-readable form

Post-run normalization is evaluation work, not generation work.

## Intake rule
Both systems receive the same source intake.

The intake may be:
- repo tree
- code corpus
- mixed structured corpus
- damaged corpus
- corruption slices
- cross-file references

## Open output rule
This is an open-output contest.

A system is allowed to produce:
- more symbols
- more structure
- more relations
- denser artifacts
- richer operator briefings
- more reusable native state
- more queryable substrate

A system is not capped to the other system's output shape.

## Primary score
- information_per_second

Information is measured after the run by projecting each native result into
operator-readable evidence.

## Supporting scores
- readable_lines_per_second
- structural_nodes_per_second
- relation_edges_per_second
- artifact_compactness
- decoded_bytes_per_artifact_byte
- deterministic_repeat_match
- corruption_survival_rate
- null_output_rate
- operator_reuse_value

## Required reporting
Each run should preserve:
- elapsed_ms
- native artifact path or native output payload
- replay hash
- post-run normalized readable output
- post-run structure counts
- post-run relation counts
- failure mode if any

## Explicit NSQ rule
If NSQ can emit a denser, more reusable, more queryable native substrate within
the same time window, that is a fair advantage and must count.

## Explicit C rule
If C can emit more directly or more quickly within the same time window, that is
a fair advantage and must count.

## Prohibited mistake
Do not normalize both systems into the same narrow intermediate during the timed
run. That would hide native strength and produce a false result.

## Guiding question
Given the same intake and same clock, which base produces the most recoverable,
operator-usable value, and which preserves that value in the strongest native
form for later world-building work?
