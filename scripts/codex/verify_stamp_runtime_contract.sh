#!/usr/bin/env bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"

cd "$ROOT"

TARGET="state/nsq/stamps/stamp_execution_topology.json"

echo
echo "== VERIFYING STAMP RUNTIME CONTRACT =="
echo

test -f "$TARGET"

grep -q '"stamp_is_wake_trigger": true' "$TARGET"
grep -q '"passive_stamp_only_mode_allowed": false' "$TARGET"
grep -q '"stored_operation_required": true' "$TARGET"
grep -q '"wake_packet_required": true' "$TARGET"
grep -q '"runtime_projection_required": true' "$TARGET"
grep -q '"materialization_path_required": true' "$TARGET"
grep -q '"semantic_execution_continuity_required": true' "$TARGET"
grep -q '"semantic_routing_required": true' "$TARGET"

echo
echo "STAMP RUNTIME CONTRACT VERIFIED"
echo
