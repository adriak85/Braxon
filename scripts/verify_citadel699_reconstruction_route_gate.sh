#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
cd "$ROOT"

CFG="config/nsq/citadel699_reconstruction_route_gate.json"
BUS="state/braxon/bus/citadel699/reconstruction_route_gate.braxon"
DOC="docs/braxon/CITADEL699_RECONSTRUCTION_ROUTE_GATE.md"

echo "== verify Citadel699 reconstruction route gate =="
echo "root=$ROOT"

test -s "$CFG"
test -s "$BUS"
test -s "$DOC"

python3 - "$CFG" <<'PY'
import json
import sys

path = sys.argv[1]
data = json.load(open(path, "r", encoding="utf-8"))

assert data["schema"] == "braxon.citadel699.reconstruction_route_gate.v1"
assert data["identity"] == "Braxon"
assert data["authority"] == "NSQ_COURT"
assert data["nsq_is_bus"] is True
assert data["court_is_compositor"] is True
assert data["ordinary_full_weight_download_is_source_truth"] is False
assert data["full_local_fp32_weight_storage_required"] is False
assert data["bus_manifestation_required"] is True
assert data["pointer_like_small_files_are_not_automatic_failure"] is True

rule = data["reconstruction_handle_rule"]
assert rule["inert_pointer_stub_only_if_no_route_evidence_exists"] is True

required_chain = [
    "input_handle",
    "bus_record",
    "stamp_wake",
    "stored_operation_or_framework",
    "materialization_recipe",
    "output_state",
    "validation_digest",
    "moral_invariant_check",
    "identity_preservation_check",
]
assert data["route_proof_chain"] == required_chain

assert data["status_ladder"] == [
    "inert_pointer_stub_or_catalog_only",
    "reconstruction_handle_unverified",
    "reconstruction_route_verified_not_hot_live",
    "reconstruction_route_executed_not_runtime_bound",
    "hot_live_verified",
]

hot = data["hot_live_rule"]
assert hot["metadata_alone_is_not_hot_live"] is True
assert hot["manifest_alone_is_not_hot_live"] is True
assert hot["reserved_artifact_name_alone_is_not_hot_live"] is True
assert hot["hot_live_requires_executable_route_proof"] is True

assert data["citadel699_roster_required_count"] == 10
assert data["uncensored_lane_emotion_routing_required"] is True
assert data["moral_invariant_preservation_required"] is True

print("PASS: JSON route gate is coherent")
PY

grep -q '^ordinary_full_weight_download_is_source_truth = false$' "$BUS"
grep -q '^full_local_fp32_weight_storage_required = false$' "$BUS"
grep -q '^bus_manifestation_required = true$' "$BUS"
grep -q '^pointer_like_small_files_are_not_automatic_failure = true$' "$BUS"
grep -q '^inert_pointer_stub_only_if_no_route_evidence_exists = true$' "$BUS"
grep -q '^runtime_rule = do_not_claim_hot_live_without_executable_route_proof$' "$BUS"
grep -q '^uncensored_lane_emotion_routing_required = true$' "$BUS"
grep -q '^moral_invariant_preservation_required = true$' "$BUS"

grep -q 'Citadel699 does not require ordinary full local FP32 weight storage as the source truth' "$DOC"
grep -q 'Pointer-looking files are not automatically failure' "$DOC"
grep -q 'Hot-live requires executable route proof' "$DOC"
grep -q 'Citadel699 is the ten-lane uncensored model stack' "$DOC"
grep -q 'reconstruction_route_executed_not_runtime_bound' "$DOC"

echo "PASS: bus and docs carry the reconstruction route gate"
echo "PASS: Citadel699 reconstruction route gate verified"
