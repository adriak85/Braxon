#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
cd "$ROOT"

OUT_DIR="$ROOT/state/perf"
STAMP="$(date +%Y%m%d_%H%M%S)"
REPORT="$OUT_DIR/hot_substrate_gate_$STAMP.txt"
mkdir -p "$OUT_DIR"

measure() {
  label="$1"
  shift
  start_ns="$(date +%s%N)"
  "$@"
  end_ns="$(date +%s%N)"
  elapsed_ns="$((end_ns - start_ns))"
  elapsed_ms="$((elapsed_ns / 1000000))"
  echo "$label elapsed_ms=$elapsed_ms"
}

{
  echo "== Braxon hot substrate gate timing =="
  echo "date=$(date -Is)"
  echo "branch=$(git branch --show-current)"
  echo "head=$(git rev-parse HEAD)"
  echo

  measure "substrate_proof" scripts/substrate/verify_nsq_court_start_proof.sh "$ROOT"
  echo

  measure "workspace_nextest_release_gate" cargo nextest run --workspace --bins --lib --all-targets --all-features --all --release --no-fail-fast -j7
  echo

  echo "== final status =="
  git status --branch --short
} | tee "$REPORT"

echo
echo "Report: $REPORT"
