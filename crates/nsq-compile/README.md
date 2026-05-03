# nsq-compile

`nsq-compile` is currently a disabled legacy compile path. The binary exists to make the state explicit: compilation into derived machine forms stays off until canonical NSQ semantics are repaired on the native `nu` substrate.

## Responsibilities
- refuse the legacy compile route instead of silently drifting back into it
- communicate why the old path is blocked
- preserve the place in the pipeline where a native compile surface will return later

## Command
```bash
cargo run -p nsq-compile --release --
```

## Inputs and outputs
- Input: none
- Output: an explanatory error message and a nonzero exit status

## Workspace links
- Referenced by `nsq-prime` as a required surface presence check
- Must be rebuilt natively before this crate becomes active again
