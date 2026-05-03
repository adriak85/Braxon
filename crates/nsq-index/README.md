# nsq-index

`nsq-index` builds derived query indexes from canonical `.nsq` corpora. Both the JSON and binary index formats are disposable transport artifacts that must be regenerated from canonical sources when semantics change.

## Responsibilities
- normalize canonical text
- parse triple and membrane lines into edge and state tables
- build adjacency maps and anchor indexes
- write compact JSON and binary-frame index artifacts
- expose helper queries such as shortest-path and anchor-range lookups

## Library surface
- `build_index_from_text`
- `write_index_json` and `write_index_binary`
- `read_index_json` and `read_index_binary`
- `shortest_path` and `anchors_in_range`

## Command
```bash
cargo run -p nsq-index --release -- <corpus.nsq> <out.idx.json> [out.idx.bin]
```

## Workspace links
- Used by `nsq-query`, `nsq-debug`, `nsq-bench-split`, and `nsq-bench-compare`
- Derived artifact only; never canonical semantic truth
