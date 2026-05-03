#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${ROOT:-$HOME/Braxon}"
DL="$HOME/storage/shared/Download"
STAMP="$(date +%Y%m%d_%H%M%S)"
OUT="$DL/verify_runtime_semantic_consumption_$STAMP"

mkdir -p "$OUT"
cd "$ROOT"

{
  echo "== direct semantic consumers =="
  rg -n -S -i \
    -e 'semantic_feed' \
    -e 'compass_seed' \
    -e 'runtime_semantic_context' \
    -e 'semantic_bias_for_text' \
    crates/nsq-runtime/src crates/nsq-runtime/tests || true
  echo
  echo "== patched anchors =="
  rg -n -S \
    -e 'BRAXON_runtime_semantic_patch::algorithm_lever_from_semantic_text' \
    -e 'BRAXON_runtime_semantic_patch::lane' \
    -e 'BRAXON_runtime_semantic_patch::execute_slice' \
    -e 'BRAXON_runtime_semantic_patch::execute_request' \
    crates/nsq-runtime/src/lib.rs || true
} > "$OUT/01_semantic_consumers.txt"

cargo fmt --all > "$OUT/02_fmt.txt" 2>&1 || true
cargo test -p nsq-runtime -- --nocapture > "$OUT/03_tests.txt" 2>&1 || true

{
  echo "out_dir=$OUT"
  echo
  echo "== semantic consumer head =="
  sed -n '1,220p' "$OUT/01_semantic_consumers.txt"
  echo
  echo "== cargo test tail =="
  tail -n 120 "$OUT/03_tests.txt" || true
} > "$OUT/99_summary.txt"

cat "$OUT/99_summary.txt"
