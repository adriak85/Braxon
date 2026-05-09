# Braxon-cli

`Braxon-cli` is the small crate-local CLI for exposing Braxon identity from `Braxon-core`. It is intentionally narrow and complements the richer root `Braxon` binary in the workspace root.

## Responsibilities
- parse the `Braxon` command with Clap
- default to `status`
- print the current `BRAXONIdentity`

## Command
```bash
cargo run -p Braxon-cli --release -- status
```

## Inputs and outputs
- Input: an optional `status` subcommand
- Output: a single identity line such as `<name> <version>`

## Workspace links
- Depends on `Braxon-core`
- Useful as the smallest identity check when validating that the Braxon surface is wired
