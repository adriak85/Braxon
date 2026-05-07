# Testing Rules

Visible commands are preferred.

Expected core verification examples:
- cargo fmt
- cargo test -p nsq-core -- --nocapture
- cargo test -p nsq-runtime -- --nocapture
- cargo test -p braxon-core -- --nocapture
- cargo test -p braxon-ingest -- --nocapture
- cargo nextest run --workspace --bins --lib --all-targets --all-features --release --no-fail-fast

Do not use quiet flags when the user needs proof.

Tests must prove behavior where behavior matters.

For stamp work, tests must show that stamp use causes an observable wake/result.

For materialization work, tests must distinguish:
- manifest-bound
- external/materialization boundary
- hot-live verified
- fake/pointer-stub rejection
