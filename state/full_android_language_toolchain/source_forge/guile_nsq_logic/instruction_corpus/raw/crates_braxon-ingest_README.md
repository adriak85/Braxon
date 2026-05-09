# Braxon-ingest

`Braxon-ingest` is the dedicated BRAXON crate for chunk-governed model ingress.

It does three specific jobs:
- reads the live donor lane truth from the runtime audit
- keeps the 50 GB chunk window visible for the current ingest lane
- makes the gap between the target 604B lineage and the currently materialized donor lane explicit

This surface is observational and control-facing only.
It does not depend on `aria2c`, and it does not assume a daemonized ingress model.

## Commands

```bash
cargo run -p Braxon-ingest -- status
cargo run -p Braxon-ingest -- json
```
