# nsq-real-bench

`nsq-real-bench` builds and queries a readable derived transport artifact from text or files. It is a convenience benchmark and report surface for packed host-carrier layouts and is explicitly not canonical NSQ truth.

## Responsibilities
- build a derived packed JSON artifact from inline text or a file
- reconstruct metrics from an existing artifact in `query` mode
- report symbol, transition, class, artifact-size, and binary-size summaries

## Command
```bash
cargo run -p nsq-real-bench --release -- build-text <artifact> <text...>
cargo run -p nsq-real-bench --release -- build-file <artifact> <path>
cargo run -p nsq-real-bench --release -- query <artifact>
```

## Inputs and outputs
- Input: inline text, a source file, or a previously built artifact
- Output: derived packed JSON artifact plus JSON metric reports

## Workspace links
- Complements `nsq-decode`
- Derived artifact only; regenerate from canonical sources when semantics shift
