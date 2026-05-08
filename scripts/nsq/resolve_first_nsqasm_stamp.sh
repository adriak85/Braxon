#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
cd "$ROOT"

ACCEPTED="state/nsq/stamp_build_chain/accepted.jsonl"
test -s "$ACCEPTED"

STAMP_ID="$(python3 - "$ACCEPTED" <<'PY'
import json, sys
from pathlib import Path

for line in Path(sys.argv[1]).read_text(encoding="utf-8").splitlines():
    if not line.strip():
        continue
    r = json.loads(line)
    stamp_id = r.get("stamp_id")
    if stamp_id:
        print(stamp_id)
        raise SystemExit(0)

raise SystemExit("no stamp_id found")
PY
)"

echo "== resolve first accepted NSQASM stamp =="
echo "stamp_id=$STAMP_ID"

cargo run -p nsqasm-stamp-db --release -- resolve "$ROOT" "$STAMP_ID"

echo
echo "== verify generated wake dispatch =="
SAFE_NAME="$(python3 - "$STAMP_ID" <<'PY'
import sys
s = sys.argv[1]
print(''.join(c.lower() if c.isalnum() else '_' for c in s).strip('_'))
PY
)"
DISPATCH="state/nsq/stamp_build_chain/resolved/${SAFE_NAME}.wake.json"

test -s "$DISPATCH"
grep -q '"schema": "braxon.nsqasm.stamp_wake_dispatch.v1"' "$DISPATCH"
grep -q '"authority": "NSQ_COURT"' "$DISPATCH"
grep -q '"verified": true' "$DISPATCH"
grep -q '"passive_stamp_only_mode_allowed": false' "$DISPATCH"
grep -q '"court_position": "queen"' "$DISPATCH"
grep -q '"court_position": "bishop"' "$DISPATCH"
grep -q '"title": "King"' "$DISPATCH"

echo "PASS: wake dispatch exists and is court-seated"
echo "dispatch=$DISPATCH"
