# nsq-calibrate

`nsq-calibrate` converts an optimizer report into a calibration lock. The lock freezes selected profile information, promoted macros, hot targets, and representation-boundary hints for downstream preservation and replay steps.

## Responsibilities
- read `nsq-optimize` output
- extract promoted macros and expansion targets
- retain threshold settings and representation-boundary projections
- propose rebalance actions when noise, triple, membrane, or calibration counts drift

## Command
```bash
cargo run -p nsq-calibrate --release -- <optimizer_report.json> <calibration_lock.json>
```

## Inputs and outputs
- Input: optimizer report JSON
- Output: calibration lock JSON plus a pretty-printed copy on stdout

## Workspace links
- Consumes `nsq-optimize` output
- Feeds `nsq-preserve` and any lane that needs a frozen representation lock
