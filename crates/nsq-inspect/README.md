# nsq-inspect

`nsq-inspect` validates packed NSQ artifacts at the marker and payload-carrier level. It is the smallest inspection surface for `.nsqb`-style packed files that start with the `NSQPACK01` marker.

## Responsibilities
- read an artifact from disk
- verify the native marker prefix
- report total artifact and payload carrier-unit counts

## Library surface
- `inspect_file(path)`

## Command
```bash
cargo run -p nsq-inspect --release -- <artifact.nsqb>
```

## Inputs and outputs
- Input: packed artifact path
- Output: JSON with marker validity and carrier-unit sizes

## Workspace links
- Pairs with `nsq-pack` and `nsq-proof`
- Used by `nsq-prime` as a presence/health prerequisite
