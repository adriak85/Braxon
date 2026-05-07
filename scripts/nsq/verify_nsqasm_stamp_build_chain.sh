#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
cd "$ROOT"

CFG="config/nsq/nsqasm_stamp_build_chain.json"
SPEC="specs/nsq/NSQASM_STAMP_BUILD_CHAIN.md"
REPORT="state/nsq/stamp_build_chain/verify_$(date +%Y%m%d_%H%M%S).txt"

mkdir -p state/nsq/stamp_build_chain

{
  echo "== verify NSQASM stamp build chain =="
  echo "date=$(date -Is)"
  echo "root=$ROOT"
  echo "head=$(git rev-parse HEAD)"
  echo

  echo "== required files =="
  test -f "$CFG"
  test -f "$SPEC"
  echo "PASS: config and spec exist"
  echo

  echo "== stamp execution meaning =="
  grep -q '"stamp_is_passive_text": false' "$CFG"
  grep -q '"stamp_is_full_payload": false' "$CFG"
  grep -q '"stamp_is_wake_trigger": true' "$CFG"
  grep -q '"stamp_is_address_anchor": true' "$CFG"
  grep -q '"stamp_is_operational_ignition": true' "$CFG"
  grep -q '"stored_operation_required": true' "$CFG"
  grep -q '"wake_packet_required": true' "$CFG"
  grep -q '"runtime_projection_required": true' "$CFG"
  grep -q '"materialization_path_required": true' "$CFG"
  grep -q '"semantic_execution_continuity_required": true' "$CFG"
  echo "PASS: stamp meaning matches runtime contract"
  echo

  echo "== court role placement =="
  grep -q '"court_position": "queen"' "$CFG"
  grep -q '"court_position": "bishop"' "$CFG"
  grep -q '"court_position": "composer"' "$CFG"
  grep -q '"title": "Queen"' "$CFG"
  grep -q '"title": "Bishop"' "$CFG"
  grep -q '"title": "King"' "$CFG"
  echo "PASS: Queen/Bishop/King placement exists"
  echo

  echo "== cross-project database =="
  grep -q '"scope": "cross_project"' "$CFG"
  grep -q '"usable_by_braxon": true' "$CFG"
  grep -q '"usable_by_future_projects": true' "$CFG"
  grep -q '"candidate_records_path": "state/nsq/stamp_build_chain/candidates.jsonl"' "$CFG"
  grep -q '"accepted_records_path": "state/nsq/stamp_build_chain/accepted.jsonl"' "$CFG"
  echo "PASS: shared stamp database paths exist"
  echo

  echo "== generated artifact policy =="
  grep -q '"generated_binaries_committed_by_default": false' "$CFG"
  grep -q '"golden_fixtures_may_be_committed": true' "$CFG"
  grep -q '"golden_fixture_requires_hash": true' "$CFG"
  grep -q '"golden_fixture_requires_source_commit": true' "$CFG"
  grep -q '"golden_fixture_requires_verification_report": true' "$CFG"
  echo "PASS: generated binary policy is explicit"
  echo

  echo "== proof source alignment =="
  grep -q 'Queen validates candidate stamps' "$SPEC"
  grep -q 'Bishop prepares, imbues, elevates, recycles, or reassigns stamp material' "$SPEC"
  grep -q 'King/composer performs final assembly' "$SPEC"
  echo "PASS: spec states court chain"
} | tee "$REPORT"

echo
echo "Report: $REPORT"
