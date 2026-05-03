# nsq-native-bench

`nsq-native-bench` produces quick corpus summaries for text, files, directories, or stdin. It reports bytes, lines, tokens, token classes, transitions, and hashes for a native-readable corpus without building a derived query index.

## Responsibilities
- read inline text, a single file, a directory tree, or stdin
- filter directory walks to relevant text/code assets
- summarize token and class counts for the captured corpus
- emit a consistent JSON report for scripting and comparisons

## Command
```bash
cargo run -p nsq-native-bench --release -- text <text...>
cargo run -p nsq-native-bench --release -- file <path>
cargo run -p nsq-native-bench --release -- dir <path>
cargo run -p nsq-native-bench --release -- stdin
```

## Inputs and outputs
- Input: direct text, a file path, a directory path, or stdin
- Output: JSON report with token, transition, and hash metrics

## Workspace links
- Complements `nsq-bench` and `nsq-real-bench`
- Good for broad corpus inspection before packing or indexing
