# nsq-bench-compare

`nsq-bench-compare` runs side-by-side benchmark comparisons between the NSQ index/query path and a simple baseline parser. It is useful for honest performance deltas rather than single-system timing in isolation.

## Responsibilities
- read a JSON task spec containing corpus paths and query sets
- build an `nsq-index` artifact for each task
- measure index-build and query timings for NSQ and for the local baseline
- report symbol and edge counts for both approaches

## Command
```bash
cargo run -p nsq-bench-compare --release -- <task_spec.json>
```

## Inputs and outputs
- Input: a JSON array of named benchmark tasks
- Output: JSON rows with NSQ versus baseline timing and cardinality data

## Workspace links
- Depends on `nsq-index`
- Complements `nsq-bench-split` when you need comparison rather than isolated mode timing
