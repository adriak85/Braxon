#!/usr/bin/env bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"

cd "$ROOT"

TARGET="state/nsq/runtime/stamp_runtime_contract.json"

echo
echo "== VERIFYING STAMP RUNTIME CONTRACT =="
echo

test -f "$TARGET"

grep -q '"stamp_is_operational_wake_trigger": true' "$TARGET"
grep -q '"metadata_only_execution": false' "$TARGET"
grep -q '"runtime_truth_required": true' "$TARGET"
grep -q '"materialization_proof_required": true' "$TARGET"
grep -q '"execution_proof_required": true' "$TARGET"
grep -q '"wake_packet_required": true' "$TARGET"
grep -q '"semantic_routing_required": true' "$TARGET"

echo
echo "STAMP RUNTIME CONTRACT VERIFIED"
echo
