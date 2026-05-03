# nsq-profile

`nsq-profile` profiles the `nsq-source` spine and prime phases for a single input corpus. It runs the compiled `nsq-source` binary, stores the produced spine and prime artifacts in a work directory, and reports phase timings in milliseconds.

## Responsibilities
- locate the release `nsq-source` binary
- run `nsq-source spine` and `nsq-source prime`
- capture their outputs into a work directory
- emit per-phase timing and status data

## Command
```bash
cargo run -p nsq-profile --release -- <input.nsq> <workdir>
```

## Inputs and outputs
- Input: canonical source file and a work directory
- Output: `profile.spine.nsq`, `profile.prime.json`, and a JSON phase report

## Workspace links
- Depends on `nsq-source`
- Useful for comparing source-ingress overhead before later indexing or preservation steps
