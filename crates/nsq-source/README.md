# nsq-source

`nsq-source` is the canonical source-ingress and normalization surface. It detects incoming source forms, sanitizes them, converts them into canonical spine lines, and can emit a prime representation with nodes, edges, and states.

## Responsibilities
- detect canonical, S-expression, Lua-shape, Python-shape, and Rust-native ingress forms
- sanitize ingress text while preserving meaningful lines
- translate accepted ingress forms into canonical spine lines
- build a prime representation containing nodes, edges, and states

## Library surface
- `SourceIngressForm`
- `detect_source_ingress_form`
- `sanitize_source_ingress`
- `spine_source`
- `build_prime_representation`

## Command
```bash
cargo run -p nsq-source --release -- spine <input>
cargo run -p nsq-source --release -- sanitize <input>
cargo run -p nsq-source --release -- prime <input>
```

## Workspace links
- Upstream of `nsq-profile`, `nsq-prime`, indexing, preservation, and other canonical pipeline stages
- Base/canonical surface; avoid treating dialect labels as runtime authority
