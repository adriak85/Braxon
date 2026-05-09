# Braxon-kingdom-generate

`Braxon-kingdom-generate` turns the canonical kingdom court description into the Braxon and NSQ court configuration surfaces used elsewhere in the workspace. It also emits base-8-aware kingdom metrics and generated constitutional/spec text.

## Responsibilities
- read the canonical kingdom source from `config/kingdom/court_canonical.json`
- derive Braxon office config and NSQ court seed/config projections
- generate constitutional documentation under `specs/`
- preserve base-8 counts in emitted kingdom metrics instead of flattening into host-width meaning

## Generated outputs
- `specs/court/COURT_CONSTITUTION.md`
- `specs/nsq/court_of_archons.md`
- `config/braxon_court.json`
- `config/nsq/court_seed.json`
- `config/nsq_court.json`

## Command
```bash
cargo run -p Braxon-kingdom-generate --release --
```

## Workspace links
- Upstream input for `Braxon-court`, `nsq-court`, and `Braxon-showdown`
- Keeps the court seed path explicit and reproducible
