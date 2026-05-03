# NSQ Proof Scoring v0

## Primary metrics
- source_bytes
- artifact_bytes
- decoded_bytes
- decoded_records
- unique_symbols
- structural_edges
- information_density = decoded_records / artifact_bytes
- decoded_bytes_per_artifact_byte = decoded_bytes / artifact_bytes

## Secondary metrics
- parse_ms
- pack_ms
- decode_ms
- deterministic_replay = pass/fail
- corruption_detection = pass/fail

## What counts as proof
NSQ is only considered proven on this lane if:
1. source parses
2. artifact packs
3. decode reproduces all records deterministically
4. metrics are emitted
5. replay hash is stable
