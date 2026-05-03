# nsq-court

`nsq-court` reads an NSQ court seed and produces a report about seat count, authority seats, crash-guarded seats, promotion order, and deadlock escalation. It is a reporting and projection surface, not the finished primitive court path.

## Responsibilities
- load a `court_seed.json` file
- summarize the configured court roles
- preserve authority and continuity notes in the emitted report
- write the report to disk and to stdout

## Command
```bash
cargo run -p nsq-court --release -- <court_seed.json> <court_report.json>
```

## Inputs and outputs
- Input: seed JSON generated from the canonical kingdom/court description
- Output: report JSON with seat and continuity information

## Workspace links
- Works with `Braxon-kingdom-generate` output
- Sits alongside `Braxon-court` for the NSQ-facing view of court structure
