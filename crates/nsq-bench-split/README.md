# nsq-bench-split

`nsq-bench-split` exposes three benchmark modes so the repo can measure core in-memory cost, cold-start end-to-end latency, and warm binary-index query latency separately. The code is explicit about what each mode does and does not include.

## Responsibilities
- run `core` benchmarks for in-process build plus query timing
- run `cold` benchmarks for corpus read, index build, JSON write, JSON read, and query timing
- run `warm` benchmarks for binary-index load plus query timing
- reuse `nsq-query` semantics when running batch queries against the loaded index

## Command
```bash
cargo run -p nsq-bench-split --release -- core <corpus.nsq> <queries.json> [iters]
cargo run -p nsq-bench-split --release -- cold <corpus.nsq> <index.idx.json> <queries.json> [iters]
cargo run -p nsq-bench-split --release -- warm <index.idx.bin> <queries.json> [iters]
```

## Inputs and outputs
- Input: canonical corpora, query files, and optional iteration counts
- Output: JSON benchmark report with mode-specific timing fields and query/index sizes

## Workspace links
- Depends on `nsq-index` and `nsq-query`
- Use this when you need honest timing splits instead of a single aggregated number
