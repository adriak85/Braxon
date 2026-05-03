# nsq-archon

`nsq-archon` converts optimizer output into an archon gate report. It chooses an operating mode, classifies intake pressure, and emits notices that downstream linter or picker surfaces can honor.

## Responsibilities
- read an `nsq-optimize` JSON report
- derive `selected_mode`, `intake_pressure`, and `parallel_hint`
- flag membrane-pressure imbalance conditions
- generate guidance messages for linter and picker stages

## Command
```bash
cargo run -p nsq-archon --release -- <optimizer_report.json> <archon_report.json>
```

## Inputs and outputs
- Input: optimizer report JSON
- Output: archon gate report JSON plus a pretty-printed copy on stdout

## Workspace links
- Depends on the shape of `nsq-optimize` output
- Feeds downstream selection and control surfaces that need pressure-aware policy
