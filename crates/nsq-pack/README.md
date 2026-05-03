# nsq-pack

`nsq-pack` bundles one or more files into a packed artifact with the `NSQPACK01` marker and emits a manifest describing the carrier-unit sizes. It is a packaging surface, not canonical semantic authority.

## Responsibilities
- prepend the native pack marker
- append file headers and payload bytes for each input artifact
- create parent directories for the output path
- emit a manifest with source, payload, and artifact carrier-unit totals

## Library surface
- `PackManifest`
- `pack_files(inputs, out_path)`

## Command
```bash
cargo run -p nsq-pack --release -- <out.pack> <input1> [input2 ...]
```

## Workspace links
- Pairs with `nsq-inspect` and `nsq-proof`
- Presence is checked by `nsq-prime`
