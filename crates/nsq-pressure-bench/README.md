# nsq-pressure-bench

`nsq-pressure-bench` measures derived transport pressure paths by writing synthetic packed streams and decoding them back into score summaries. It is explicitly a derived transport benchmark and must not be treated as canonical NSQ truth.

## Responsibilities
- generate pressure-test packed streams for noise or structured modes
- write a custom `NSQPRM01` binary frame
- decode the generated transport and report throughput, compression, and transition metrics
- summarize class counts, bytes per second, and replay hash information

## Command
```bash
cargo run -p nsq-pressure-bench --release -- write-noise <seconds> <native_out>
cargo run -p nsq-pressure-bench --release -- write-structured <seconds> <native_out>
cargo run -p nsq-pressure-bench --release -- decode <native_in> <decoded_txt> <score_json>
```

## Inputs and outputs
- Input: runtime duration or a previously written pressure artifact
- Output: pressure artifact bytes, decoded text, and JSON score reports

## Workspace links
- Derived benchmark lane only
- Useful when comparing stress behavior against canonical-preservation paths
