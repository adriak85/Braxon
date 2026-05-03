# nsq-lint

`nsq-lint` checks canonical `.nsq` files for structural errors and range violations. It validates the expected keys and numeric ranges for noise, triple, membrane, and calibration records and emits a structured findings report.

## Responsibilities
- parse canonical NSQ lines
- verify required fields for each record family
- enforce numeric range and parse checks
- count the major record families alongside the findings

## Command
```bash
cargo run -p nsq-lint --release -- <input.nsq>
```

## Inputs and outputs
- Input: canonical `.nsq` text
- Output: JSON with `ok`, findings, and per-family counts

## Workspace links
- Best used before `nsq-optimize`, `nsq-index`, or preservation/packing steps
- This is a canonical surface, not a derived transport helper
