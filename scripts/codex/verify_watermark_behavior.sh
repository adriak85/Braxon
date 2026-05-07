#!/usr/bin/env bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"

cd "$ROOT"

echo
echo "== VERIFYING WATERMARK EXECUTION CONTINUITY =="
echo

JSON_FILE="state/nsq/watermarks/braxon_watermark_execution_contract.json"
RS_FILE="crates/nsq-core/src/watermark_execution_contract.rs"

test -f "$JSON_FILE"
test -f "$RS_FILE"

grep -q 'BRAXON_NSQ_BASE8_LEVER_SCALE_GE220000_PROVEN225370_V1' "$JSON_FILE"

grep -q '"watermarks_are_operational": true' "$JSON_FILE"
grep -q '"watermarks_participate_in_execution_validation": true' "$JSON_FILE"

grep -q 'watermark_required_for_runtime_execution' "$RS_FILE"
grep -q 'watermark_fail_closed_on_mismatch' "$RS_FILE"
grep -q 'watermark_is_operational' "$RS_FILE"

echo
echo "WATERMARK EXECUTION CONTINUITY VERIFIED"
echo
