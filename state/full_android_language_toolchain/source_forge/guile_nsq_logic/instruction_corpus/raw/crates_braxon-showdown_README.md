# Braxon-showdown

`Braxon-showdown` is a readiness gate for the kingdom and court asset set. It checks whether the expected generated configs, specs, and ledgers exist and fails fast if the current Braxon surface is incomplete.

## Responsibilities
- resolve the workspace root from `BRAXON_HOME` or `HOME`
- check the presence of canonical court configs, court specs, and runtime ledgers
- emit a structured JSON readiness report
- exit nonzero when any required artifact is missing

## Command
```bash
cargo run -p Braxon-showdown --release --
```

## Inputs and outputs
- Input: the on-disk workspace artifact set
- Output: JSON with `ready` plus per-artifact checks and paths

## Workspace links
- Validates outputs from `Braxon-kingdom-generate`
- Useful as a quick kingdom/court smoke test before deeper runtime work
