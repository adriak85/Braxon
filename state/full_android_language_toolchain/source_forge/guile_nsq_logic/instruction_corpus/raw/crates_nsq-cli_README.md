# nsq-cli

`nsq-cli` is the lightweight NSQ operator shell. It provides a small command surface for status, parse, eval, select, ingest, fetch, wake, and doctor flows, with an interactive REPL as the default experience.

## Responsibilities
- parse NSQ operator commands with Clap
- provide a simple REPL front door
- inspect local paths for ingest and basic environment health for doctor mode
- expose stubbed parse, eval, select, and fetch surfaces while the deeper runtime is still being recoded

## Command
```bash
cargo run -p nsq-cli --release -- status
cargo run -p nsq-cli --release -- repl
cargo run -p nsq-cli --release -- doctor
```

## Inputs and outputs
- Input: command-line subcommands or REPL text commands
- Output: human-readable status lines, environment checks, or stub responses

## Workspace links
- Front-door operator tool, but not canonical runtime authority
- Keeps local shell checks close to the NSQ surface while deeper runtime work continues
