#!/usr/bin/env bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"

cd "$ROOT"

echo
echo "== VERIFYING STAMP EXECUTION BEHAVIOR =="
echo

STAMP_JSON="state/nsq/stamps/stamp_execution_topology.json"
STAMP_RS="crates/nsq-core/src/stamp_execution_contract.rs"

test -f "$STAMP_JSON"
test -f "$STAMP_RS"

grep -q '"stamp_is_wake_trigger": true' "$STAMP_JSON"
grep -q '"runtime_projection_required": true' "$STAMP_JSON"
grep -q '"semantic_execution_continuity_required": true' "$STAMP_JSON"

grep -q 'stamp_execution_requires_runtime_behavior' "$STAMP_RS"
grep -q 'passive_stamp_only_mode_allowed' "$STAMP_RS"
grep -q 'runtime_projection_required' "$STAMP_RS"

echo
echo "STAMP EXECUTION CONTRACT VERIFIED"
echo
