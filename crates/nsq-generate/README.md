# nsq-generate

`nsq-generate` creates large synthetic NSQ corpora for benchmarking and pressure testing. It emits coordinated noise, structured triple, membrane, and calibration files from a chosen scale and density.

## Responsibilities
- generate synthetic symbol, relation, and macro traffic
- emit four large corpora under a target output directory
- keep the generated surfaces consistent enough for benchmarking, calibration, and replay experiments

## Command
```bash
cargo run -p nsq-generate --release -- <scale_count> <density> <out_dir>
```

## Generated outputs
- `noise_large.nsq`
- `structured_large.nsq`
- `membrane_large.nsq`
- `calibration_large.nsq`

## Workspace links
- Feeds `nsq-optimize`, `nsq-bench-split`, and the other benchmark surfaces
- Useful for repeatable synthetic pressure scenarios
