# nsq-query

`nsq-query` runs single or batch queries against an `nsq-index` artifact. It supports symbol, relation, edge, state, anchor-range, and shortest-path queries and is explicit about loading the index once for batch work.

## Responsibilities
- load JSON or binary index artifacts
- run single interactive queries or batch query files
- expose symbol, relation, neighbor, edge, state, anchor, and path operations
- report index load time and aggregate batch timing

## Library surface
- `QueryResult`
- `find_symbol`, `find_rel`, `neighbors`
- `edges_left`, `edges_right`, `edges_rel`
- `states_target`

## Command
```bash
cargo run -p nsq-query --release -- <index_path> <query>
cargo run -p nsq-query --release -- <index_path> --batch <queries.txt>
cargo run -p nsq-query --release -- <index_path> --batch-json <queries.json>
```

## Workspace links
- Depends on `nsq-index`
- Used by `nsq-bench-split` for batch query timing
