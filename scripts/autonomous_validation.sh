#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

JOBS="${BRAXON_JOBS:-1}"
ROUNDS="${BRAXON_VALIDATION_ROUNDS:-3000}"

printf '%s\n' '[1/3] workspace tests'
cargo test --workspace --all-targets --jobs "$JOBS"

printf '%s\n' '[2/3] deterministic engagement smoke tests'
questions=(
  "What has not been seen yet?"
  "Explain the strongest contradiction in the current state."
  "What should be preserved before changing direction?"
  "Find the smallest useful next action."
  "Compare two competing interpretations."
  "What evidence would change the conclusion?"
  "Explore the boundary of this intent."
  "Return to the earliest unresolved assumption."
)
for q in "${questions[@]}"; do
  cargo run -q -p nsq-citadel --bin engage -- "$q" >/dev/null
done

printf '%s\n' '[3/3] thousands-of-operations deterministic corpus'
python3 - "$ROUNDS" <<'PY'
import hashlib
import json
import subprocess
import sys

rounds = int(sys.argv[1])
operations = []
for i in range(rounds):
    operations.append(f"operation {i}: inspect intent, route through Council Ten, preserve invariants, and identify the next useful state")

failures = []
for i, op in enumerate(operations):
    p = subprocess.run(
        ["cargo", "run", "-q", "-p", "nsq-citadel", "--bin", "engage", "--", op],
        text=True, capture_output=True,
    )
    if p.returncode:
        failures.append({"index": i, "operation": op, "stderr": p.stderr[-2000:]})
        break
    required = ("input_slots=", "capitals=", "poles=", "lead_pole=", "pressure=", "logical_complete=")
    if not all(x in p.stdout for x in required):
        failures.append({"index": i, "operation": op, "reason": "missing routing invariant"})
        break

result = {
    "rounds_requested": rounds,
    "rounds_completed": rounds if not failures else failures[0]["index"],
    "passed": not failures,
    "corpus_digest": hashlib.sha256("\n".join(operations).encode()).hexdigest(),
    "failures": failures,
}
print(json.dumps(result, indent=2))
if failures:
    raise SystemExit(1)
PY

printf '%s\n' 'AUTONOMOUS VALIDATION COMPLETE'
