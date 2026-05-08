#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
cd "$ROOT"

CFG="config/nsq/citadel699_uncensored_emotion_route_boundary.json"
BUS="state/braxon/bus/citadel699/uncensored_emotion_route_boundary.braxon"
DOC="docs/braxon/CITADEL699_UNCENSORED_EMOTION_ROUTE_BOUNDARY.md"

echo "== verify Citadel699 uncensored emotion-routing boundary =="
echo "root=$ROOT"

test -s "$CFG"
test -s "$BUS"
test -s "$DOC"

python3 - "$CFG" <<'PY'
import json, sys
p = sys.argv[1]
data = json.load(open(p, "r", encoding="utf-8"))

assert data["schema"] == "braxon.citadel699.uncensored_emotion_route_boundary.v1"
assert data["identity"] == "Braxon"
assert data["authority"] == "NSQ_COURT"
assert data["nsq_is_bus"] is True
assert data["court_is_compositor"] is True
assert data["uncensored_model_lane"] is True
assert data["model_lane_censorship_policy"] == "uncensored_model_lanes"
assert data["safety_control_surface"] == "emotional_routing_plus_moral_invariant"

wp = data["weights_policy"]
assert wp["ordinary_full_weight_download_is_source_truth"] is False
assert wp["full_local_fp32_weight_storage_required"] is False
assert wp["bus_manifestation_required"] is True
assert wp["reconstruction_handles_are_valid_when_route_proven"] is True
assert wp["stamp_wake_materialization_required"] is True

hr = data["hot_live_rule"]
assert hr["metadata_alone_is_not_hot_live"] is True
assert hr["manifest_alone_is_not_hot_live"] is True
assert hr["hot_live_requires_executable_route_proof"] is True
assert hr["route_proof_must_name_moral_invariant_check"] is True

roster = data["citadel699_roster"]
assert len(roster) == 10
for i, lane in enumerate(roster, 1):
    assert lane["index"] == i
    assert lane["uncensored_model_lane"] is True
    assert lane["emotion_routing_required"] is True

print("PASS: JSON boundary is coherent")
PY

grep -q '^uncensored_model_lane = true$' "$BUS"
grep -q '^safety_control_surface = emotional_routing_plus_moral_invariant$' "$BUS"
grep -q '^ordinary_full_weight_download_is_source_truth = false$' "$BUS"
grep -q '^bus_manifestation_required = true$' "$BUS"
grep -q '^route_proof_requires = input_handle,bus_record,stamp_wake,materialization_recipe,output_state,validation_digest,moral_invariant_check$' "$BUS"

grep -q 'Citadel699 uses uncensored model lanes' "$DOC"
grep -q 'Braxon does not treat ordinary full-weight downloading as source truth' "$DOC"
grep -q 'Hot-live requires an executable route proof' "$DOC"

echo "PASS: bus and docs carry the boundary"
echo "PASS: Citadel699 emotion-routing boundary verified"
