#!/usr/bin/env bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
SOURCE="${2:-crates/braxon-core/src/watermarked_file_operation.rs}"

cd "$ROOT"

printf '\n== VERIFYING FUNCTIONAL WATERMARK OPERATION ==\n\n'
test -f "config/nsq/watermarked_file_operation_contract.json"
test -f "crates/braxon-core/src/watermarked_file_operation.rs"
test -f "$SOURCE"

receipt="$(mktemp)"
trap 'rm -f "$receipt"' EXIT
cargo run --locked --offline -- watermark verify "$SOURCE" >"$receipt"

# These conditions are emitted only after the functional operation tokenizes
# the request, commits the source watermark through the Kinetic Reflexor, and
# completes the Parameter-Citadel invariants. No native compiler execution is
# requested on this host; target materialization remains explicitly bounded.
grep -q '"capability": "feature:watermark.file_operation"' "$receipt"
grep -q '"routed": true' "$receipt"
grep -q '"source_watermark_committed": true' "$receipt"
grep -q '"parameter_invariants_passed": true' "$receipt"
grep -q '"model_weight_execution_claimed": false' "$receipt"
grep -q '"no_resident_runtime": true' "$receipt"
grep -q '"hidden_download_allowed": false' "$receipt"
grep -q 'intent→kinetic_reflexor_route→functional_source_watermark→native_compiler_boundary→artifact_watermark→recovery_baseline' "$receipt"

printf '\nFUNCTIONAL WATERMARK OPERATION VERIFIED\n'
printf 'source=%s\n' "$SOURCE"
printf 'receipt=%s\n' "$receipt"
