# nsq-bench

`nsq-bench` is the lightweight text/file benchmark surface for parity and prime modes. It reports byte, line, token, hash, and token-class summaries without introducing a full index or derived binary artifact.

## Responsibilities
- benchmark plain parity views from text, files, or stdin
- benchmark prime views with unique-token, concept-edge, and token-class counts
- emit compact JSON suitable for quick scripting or regression checks

## Command
```bash
cargo run -p nsq-bench --release -- parity-text <text...>
cargo run -p nsq-bench --release -- parity-file <path>
cargo run -p nsq-bench --release -- prime-text <text...>
cargo run -p nsq-bench --release -- prime-file <path>
cargo run -p nsq-bench --release -- stdin
```

## Inputs and outputs
- Input: inline text, a file path, or stdin
- Output: JSON timing-free structural summary for the chosen mode

## Workspace links
- Independent quick-measurement surface
- Useful before escalating to `nsq-bench-split`, `nsq-native-bench`, or `nsq-real-bench`
