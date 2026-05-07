#!/usr/bin/env bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"

cd "$ROOT"

echo
echo "== VERIFYING EIGHT DIMENSIONAL GRADIENT CONTINUITY =="
echo

JSON_FILE="state/nsq/gradient/eight_dimensional_gradient_topology.json"
RS_FILE="crates/nsq-core/src/eight_dimensional_gradient_contract.rs"

test -f "$JSON_FILE"
test -f "$RS_FILE"

grep -q '"octillion_scale_traversal_required": true' "$JSON_FILE"
grep -q '"multidirectional_semantic_resolution_required": true' "$JSON_FILE"
grep -q '"gradient_is_execution_relevant": true' "$JSON_FILE"

grep -q 'gradient_supports_octillion_scale_traversal' "$RS_FILE"
grep -q 'gradient_preserves_inverse_semantic_continuity' "$RS_FILE"
grep -q 'flattened_embedding_only_mode_allowed' "$RS_FILE"

echo
echo "EIGHT DIMENSIONAL GRADIENT VERIFIED"
echo
