# nsq-proof

`nsq-proof` scores an artifact against its inspected readable output. It measures decoded record density, symbol diversity, structural edge count, decoded-bytes ratio, and a replay SHA-256 for the original artifact.

## Responsibilities
- read a packed artifact and its decoded inspection text
- count decoded records and structural triple edges
- collect unique symbols from the inspection surface
- write a JSON proof score

## Command
```bash
cargo run -p nsq-proof --release -- <artifact.nsqb> <inspect.txt> <score.json>
```

## Inputs and outputs
- Input: packed artifact plus readable inspect text
- Output: JSON proof score written to disk and printed to stdout

## Workspace links
- Pairs with `nsq-pack` and `nsq-inspect`
- Good for replay and density checks after preservation or packing
