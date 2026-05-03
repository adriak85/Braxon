# NSQ Prime Open Output Doctrine

## Core rule
The contest is open-output.
It measures how much real value each base can produce in the allowed time.

## Fairness
Fairness means:
- same time budget
- same source problem
- same start condition
- same stop condition
- native strengths fully allowed
- no sabotage
- no artificial flattening
- no forced internal-method parity
- no extra weights on the stronger base

Fairness does NOT mean:
- forcing NSQ to behave like C
- forcing artifact-first systems to emit human text during the timed phase
- forcing compact substrates to expand early
- charging decode cost against timed generation if decode is post-competition normalization
- reducing viable output to fit a weaker comparison shape

## Timed phase
During the timed phase, each base may produce its strongest native output.
That output may be:
- human-readable text
- compact artifact
- structured graph
- packed substrate
- replayable state
- other native product

The timed phase ends at the stop signal.
Only output produced before stop counts as native timed output.

## Post-timed normalization
Normalization happens only after the timed phase is over.

Normalization may include:
- decode
- inspect
- projection to human-readable form
- structural extraction
- relation extraction
- replay verification
- artifact size measurement

Normalization must not reduce or cap what the base was allowed to produce natively.

## Primary measures
- timed_native_output_bytes
- timed_native_output_artifacts
- timed_native_output_records
- post_normalized_human_readable_bytes
- post_normalized_human_readable_lines
- post_normalized_structural_nodes
- post_normalized_relation_edges
- replay_stability
- corruption_survival
- null_output_rate

## NSQ rule
NSQ is allowed to use:
- compact substrate output
- reusable artifacts
- delayed decode
- replayable packed forms
- structure-first production
- semantic density advantages

These are fair advantages.

## C rule
C is allowed to use:
- direct low-level execution
- immediate text emission
- native parsing speed
- direct procedural extraction

These are fair advantages.

## Forbidden unfairness
The benchmark must not:
- require live human-readable emission during the timed phase
- require NSQ to use Rust wrappers if that is not the prime path
- inject decode cost into timed generation unless decode is part of the true native task
- discard artifact compactness as a form of output
- cap NSQ output just because C would emit less
- replace open-output scoring with forced equal-shape scoring

## Verdict
The benchmark reports separate winners for:
- fastest native production
- most total viable native output
- most normalized human-readable output
- most structural output
- most relational output
- best replay stability
- best corruption survival
- best substrate reuse
- best overall operator leverage

## Guiding question
Which base produces the most usable reality in the time allowed,
without handicapping the stronger substrate?
