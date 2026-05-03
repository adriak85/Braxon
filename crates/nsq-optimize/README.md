# nsq-optimize

`nsq-optimize` analyzes canonical `.nsq` corpora and emits an optimizer report that guides later selection, calibration, and archon control decisions. It counts record families, infers boundary-carrier projections, clusters relation and target families, and proposes macro or expansion candidates.

## Responsibilities
- scan canonical NSQ corpora for noise, triple, membrane, and calibration records
- count symbols, macros, relations, and target families
- infer boundary-carrier and projection hints from observed ranges
- generate macro-promotion and expansion suggestions
- select a live profile for later runtime or control stages

## Command
```bash
cargo run -p nsq-optimize --release -- <input.nsq> <report.json>
```

## Inputs and outputs
- Input: canonical `.nsq` corpus
- Output: optimizer report JSON plus the same report on stdout

## Workspace links
- Upstream of `nsq-calibrate` and `nsq-archon`
- Use this after linting and before calibration or control decisions
