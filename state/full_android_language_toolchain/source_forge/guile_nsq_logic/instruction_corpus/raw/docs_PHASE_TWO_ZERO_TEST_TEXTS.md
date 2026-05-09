# Phase Two Zero-Test Texts

Current goal:
Expand phase-one smoke coverage into real contract coverage for every zero-direct-test binary crate.

## Corrected rule carried forward
- Do not rewrite public identity output to satisfy a bad test.
- Fix tests to match the declared surface contract unless the product contract itself changed on purpose.

## Zero-direct-test crates from the current audit
- Braxon-court
- Braxon-showdown
- nsq-archon
- nsq-bench
- nsq-bench-compare
- nsq-bench-split
- nsq-calibrate
- nsq-cli
- nsq-compile
- nsq-compose
- nsq-court
- nsq-debug
- nsq-decode
- nsq-generate
- nsq-inspect
- nsq-ir
- nsq-lint
- nsq-native-bench
- nsq-optimize
- nsq-pack
- nsq-preserve
- nsq-pressure-bench
- nsq-prime
- nsq-profile
- nsq-proof
- nsq-query
- nsq-real-bench
- wowas-final-edition-v10

## Batch A: no-arg / identity / presence contract tests
Target crates:
- nsq-prime
- nsq-ir
- nsq-inspect
- nsq-pack
- nsq-compose
- nsq-query
- nsq-cli
- Braxon-court
- nsq-court

Text to encode in tests:
- command runs
- exit status is successful when invoked on the documented minimal surface
- stdout is a single stable report or JSON payload
- required key strings are present
- output is not empty
- output shape matches the README contract, not ad hoc wording

## Batch B: fail-closed missing-input tests
Target crates:
- nsq-decode
- nsq-lint
- nsq-optimize
- nsq-calibrate
- nsq-archon
- nsq-proof
- nsq-preserve
- nsq-profile
- nsq-debug
- nsq-compile

Text to encode in tests:
- missing input path exits nonzero
- stderr or stdout contains a stable reason
- no output artifact is written on failure
- invalid path does not silently pass
- reserved / disabled surfaces remain explicitly reserved instead of pretending to work

## Batch C: fixture-backed happy-path tests
Target crates:
- nsq-bench
- nsq-bench-compare
- nsq-bench-split
- nsq-native-bench
- nsq-pressure-bench
- nsq-real-bench
- Braxon-showdown
- wowas-final-edition-v10

Text to encode in tests:
- tiny generated fixture input is accepted
- declared output artifact is created
- key report fields are present
- output size is nonzero
- repeated runs are stable on shape even if values vary

## Naming rule
Use one integration file per crate:
- crates/<crate>/tests/phase_two.rs

## Assertion rule
Prefer:
- exact keys
- exact required files
- exact success / failure status
- exact output shape

Avoid:
- fragile prose matching
- broad substring checks unless the surface is intentionally human-readable
