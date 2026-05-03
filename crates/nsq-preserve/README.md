# nsq-preserve

`nsq-preserve` converts canonical or ingress-shaped NSQ text into a preserved native artifact. It detects the incoming dialect, parses semantic records, attaches the calibration lock, hashes the source, and writes a structured artifact for later replay or inspection.

## Responsibilities
- detect canonical, S-expression, Lua-shape, or Python-shape ingress
- parse noise, triple, and membrane records into semantic forms
- attach a calibration lock and provenance block
- hash the source input and write a preserved artifact JSON

## Command
```bash
cargo run -p nsq-preserve --release -- <input.nsq> <calibration_lock.json> <output.native.json>
```

## Inputs and outputs
- Input: source text plus a calibration lock
- Output: preserved artifact JSON with semantic records and provenance

## Workspace links
- Consumes `nsq-calibrate` output
- Often followed by packing, inspection, proof, or derived benchmark experiments
