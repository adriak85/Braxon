#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
cd "$ROOT"

ACCEPTED="state/nsq/stamp_build_chain/accepted.jsonl"
CANDIDATES="state/nsq/stamp_build_chain/candidates.jsonl"
REPORT="state/nsq/stamp_build_chain/verify_records_$(date +%Y%m%d_%H%M%S).txt"

echo "== verify NSQASM stamp DB records =="

test -s "$CANDIDATES"
test -s "$ACCEPTED"

python3 - "$CANDIDATES" "$ACCEPTED" "$REPORT" <<'PY'
import json
import sys
from pathlib import Path

candidates = Path(sys.argv[1])
accepted = Path(sys.argv[2])
report = Path(sys.argv[3])

def load_jsonl(path):
    records = []
    with path.open("r", encoding="utf-8") as f:
        for idx, line in enumerate(f, 1):
            line = line.strip()
            if not line:
                continue
            try:
                records.append(json.loads(line))
            except json.JSONDecodeError as e:
                raise SystemExit(f"FAIL: {path}:{idx}: invalid json: {e}")
    return records

candidate_records = load_jsonl(candidates)
accepted_records = load_jsonl(accepted)

if not candidate_records:
    raise SystemExit("FAIL: no candidate records")

if not accepted_records:
    raise SystemExit("FAIL: no accepted records")

candidate_keys = set()
duplicate_candidates = []
for r in candidate_records:
    key = (r.get("source_path"), r.get("start_line"), r.get("end_line"), r.get("language"), r.get("sha256"))
    if key in candidate_keys:
        duplicate_candidates.append(key)
    candidate_keys.add(key)

accepted_keys = set()
duplicate_accepted = []
for r in accepted_records:
    key = (r.get("source_path"), r.get("start_line"), r.get("end_line"), r.get("language"), r.get("sha256"))
    if key in accepted_keys:
        duplicate_accepted.append(key)
    accepted_keys.add(key)

required_true = [
    "stored_operation_required",
    "wake_packet_required",
    "runtime_projection_required",
    "materialization_path_required",
    "semantic_execution_continuity_required",
]

failures = []
for i, r in enumerate(accepted_records, 1):
    if r.get("authority") != "NSQ_COURT":
        failures.append(f"accepted:{i}: authority is not NSQ_COURT")
    if r.get("schema") != "braxon.nsqasm.stamp_record.v1":
        failures.append(f"accepted:{i}: wrong schema")
    if r.get("passive_stamp_only_mode_allowed") is not False:
        failures.append(f"accepted:{i}: passive stamp-only mode not false")
    for k in required_true:
        if r.get(k) is not True:
            failures.append(f"accepted:{i}: {k} is not true")
    route = r.get("court_route") or {}
    if (route.get("validate") or {}).get("court_position") != "queen":
        failures.append(f"accepted:{i}: validate route is not queen")
    if (route.get("prepare") or {}).get("court_position") != "bishop":
        failures.append(f"accepted:{i}: prepare route is not bishop")
    compose = route.get("compose") or {}
    if compose.get("court_position") != "composer" or compose.get("title") != "King":
        failures.append(f"accepted:{i}: compose route is not King/composer")
    if not str(r.get("projection_lane", "")).startswith("current_binary_or_host_language_filtered"):
        failures.append(f"accepted:{i}: projection lane is not current filtered lane")

if duplicate_candidates:
    failures.append(f"candidate duplicate count={len(duplicate_candidates)}")

if duplicate_accepted:
    failures.append(f"accepted duplicate count={len(duplicate_accepted)}")

text = "\n".join([
    "schema=braxon.nsqasm.stamp_database_record_verify.v1",
    "authority=NSQ_COURT",
    f"candidate_records={len(candidate_records)}",
    f"accepted_records={len(accepted_records)}",
    f"candidate_unique_keys={len(candidate_keys)}",
    f"accepted_unique_keys={len(accepted_keys)}",
    f"duplicate_candidates={len(duplicate_candidates)}",
    f"duplicate_accepted={len(duplicate_accepted)}",
    f"failures={len(failures)}",
    "",
    *failures[:200],
    "",
])

report.write_text(text, encoding="utf-8")
print(text)

if failures:
    raise SystemExit("FAIL: accepted stamp DB record verification failed")

print(f"PASS: accepted stamp DB records verified")
print(f"Report: {report}")
PY
