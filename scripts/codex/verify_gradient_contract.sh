#!/usr/bin/env bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"

cd "$ROOT"

TARGET="state/nsq/gradient/eight_dimensional_gradient_contract.json"

echo
echo "== VERIFYING EIGHT DIMENSIONAL GRADIENT CONTRACT =="
echo

test -f "$TARGET"

grep -q '"canonical_gradient_topology": true' "$TARGET"
grep -q '"semantic_coordinate_fields_required": true' "$TARGET"
grep -q '"persistent_semantic_state_required": true' "$TARGET"
grep -q '"gradient_execution_causality_required": true' "$TARGET"
grep -q '"wake_execution_required": true' "$TARGET"
grep -q '"runtime_materialization_required": true' "$TARGET"

echo
echo "EIGHT DIMENSIONAL GRADIENT CONTRACT VERIFIED"
echo
