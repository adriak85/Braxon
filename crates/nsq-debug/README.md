# nsq-debug

`nsq-debug` is a trace-oriented inspection tool for canonical `.nsq` input. It shows normalization effects, stripped comments, duplicate removal, and the summary statistics produced by the index builder.

## Responsibilities
- read canonical NSQ text
- run `normalize_canonical_text`
- build an in-memory index for summary statistics
- emit a JSON trace with preview lines and phase counters

## Command
```bash
cargo run -p nsq-debug --release -- <input.nsq>
```

## Inputs and outputs
- Input: a canonical `.nsq` file
- Output: JSON trace including normalized preview lines and index stats

## Workspace links
- Depends on `nsq-index`
- Useful for debugging corpora before deeper query or benchmark work
