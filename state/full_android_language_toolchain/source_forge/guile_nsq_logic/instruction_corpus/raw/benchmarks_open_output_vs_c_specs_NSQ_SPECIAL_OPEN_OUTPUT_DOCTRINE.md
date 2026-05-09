# NSQ Special Open Output Doctrine

## Purpose
This benchmark must measure what each base is best at natively.

It must not reduce the contest to:
- immediate human-readable text only
- same-shape output only
- same internal method only
- same substrate only

## NSQ-native strengths that must be visible
The benchmark must explicitly preserve and score:

- compact native output
- structure-first output
- relation-first output
- replayable artifacts
- delayed decode
- post-run inspectability
- post-run queryability
- transformability into further work
- corruption tolerance
- operator leverage after the run
- reduced future glue burden

## Timed phase rule
During the timed phase, a system may emit its strongest native output.

For NSQ this may include:
- packed substrate
- compact artifact
- dense symbolic form
- replayable record stream
- structure and relation artifacts without live expansion to prose

For C this may include:
- direct textual extraction
- direct procedural parsing
- direct low-level output
- immediate readable reporting

The timed phase ends before normalization.

## Post-run rule
Only after the timed phase may the evaluator:
- decode
- inspect
- project to human-readable form
- count structural nodes
- count relation edges
- measure compactness
- verify replay stability
- evaluate operator reuse value

## What must NOT happen
The benchmark must not:
- require live human-readable expansion during timed generation
- discard compact artifacts as "not real output"
- charge NSQ a decode tax during the timed phase
- cap NSQ output to the shape C would emit
- force same-size or same-form artifacts
- reward verbosity over density
- confuse adapter overhead with base capability
- replace open-output with equal-shape parity

## Primary measures
- timed_native_output_bytes
- timed_native_output_records
- timed_native_output_artifacts
- post_normalized_readable_bytes
- post_normalized_readable_lines
- post_normalized_structural_nodes
- post_normalized_relation_edges
- replay_stability
- corruption_survival_rate
- artifact_compactness
- decoded_bytes_per_artifact_byte
- operator_reuse_value

## NSQ-special emphasis
A benchmark is correctly constructed for NSQ when it can show:

1. how much native substrate NSQ can emit under clock
2. how much structure and relation density that substrate preserves
3. how much readable evidence can be recovered after stop
4. how reusable the emitted artifact is for later work
5. how stable replay is across repeats
6. how much viable output survives corruption
7. how much future operator effort is saved by having a reusable base artifact

## Verdict requirement
The final report must show separate winners for:

- fastest timed production
- most timed native output
- most compact viable output
- most post-normalized readable output
- most structural output
- most relational output
- best replay stability
- best corruption survival
- best operator reuse value
- best overall leverage

The final report must not collapse everything into one text-speed number.
