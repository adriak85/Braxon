# nsq-compose

`nsq-compose` writes a repo-surface `.nsq` file from a set of canonical lines. The current binary ships a small sample composition, while the library exposes the reusable file-writing helper.

## Responsibilities
- create parent directories for composed output
- write ordered NSQ lines as a newline-terminated file
- provide a small binary example for repo-surface composition

## Library surface
- `compose_repo_surface(lines, out_path)`

## Command
```bash
cargo run -p nsq-compose --release -- <out.nsq>
```

## Inputs and outputs
- Input: a target output path
- Output: a composed `.nsq` file containing the sample repo-surface triples used by the binary

## Workspace links
- Used as a simple composition primitive
- Pairs naturally with `nsq-source`, `nsq-pack`, and `nsq-index`
