# nsq-prime

`nsq-prime` is the small workspace presence check for the core source, compile, pack, and inspect surfaces. It answers a narrow question: are the expected NSQ pipeline surfaces present in this checkout?

## Responsibilities
- verify the presence of `nsq-source`
- verify the presence of `nsq-compile`
- verify the presence of `nsq-pack`
- verify the presence of `nsq-inspect`
- exit nonzero when the expected surfaces are incomplete

## Library surface
- `PrimeReport`
- `prime_report(root)`

## Command
```bash
cargo run -p nsq-prime --release --
```

## Workspace links
- Useful as a minimal pipeline readiness check
- Often paired with broader `Braxon verify` or crate-level smoke tests
