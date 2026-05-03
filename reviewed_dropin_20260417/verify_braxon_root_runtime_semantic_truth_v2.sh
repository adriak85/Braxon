#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${ROOT:-$HOME/Braxon}"
DL="$HOME/storage/shared/Download"
STAMP="$(date +%Y%m%d_%H%M%S)"
OUT="$DL/verify_BRAXON_root_runtime_semantic_truth_$STAMP"

mkdir -p "$OUT"
cd "$ROOT"

if [ -x "$HOME/.cargo/target-cache/Braxon/release/Braxon" ]; then
  BIN="$HOME/.cargo/target-cache/Braxon/release/Braxon"
else
  BIN="cargo run --release --"
fi

{
  echo "== source truth =="
  rg -n -S \
    -e 'runtime_semantic_truth' \
    -e 'runtime_semantic_consumers_ready' \
    -e 'runtime_semantic_feed_entries' \
    -e 'runtime_compass_seed_tokens' \
    -e 'runtime_semantic_patch_anchor_count' \
    -e 'runtime_semantic_tests_present' \
    crates/Braxon-core/src/lib.rs \
    src/main.rs \
    crates/nsq-runtime/src/lib.rs \
    crates/nsq-runtime/src/semantic_context.rs \
    crates/nsq-runtime/tests || true
} > "$OUT/01_source_truth.txt"

cargo fmt --all > "$OUT/02_fmt.txt" 2>&1 || true
cargo check --workspace --bins --lib --all-targets --all-features --release --keep-going -j6 > "$OUT/03_check.txt" 2>&1 || true
cargo test -p nsq-runtime -- --nocapture > "$OUT/04_nsq_runtime_tests.txt" 2>&1 || true

bash -lc "$BIN status" > "$OUT/05_status.txt" 2>&1 || true
bash -lc "$BIN verify" > "$OUT/06_verify.txt" 2>&1 || true
bash -lc "$BIN plan" > "$OUT/07_plan.txt" 2>&1 || true

{
  echo "out_dir=$OUT"
  echo
  echo "== source truth head =="
  sed -n '1,260p' "$OUT/01_source_truth.txt"
  echo
  echo "== duplicate field scan == "
  rg -n -S "specified more than once|E0062" "$OUT/03_check.txt" || true
  echo
  echo "== cargo check tail == "

  tail -n 140 "$OUT/03_check.txt" || true
  echo
  echo "== nsq-runtime tests tail =="
  tail -n 120 "$OUT/04_nsq_runtime_tests.txt" || true
  echo
  echo "== Braxon semantic lines =="
  rg -n -S \
    -e 'runtime_semantic_consumers_ready' \
    -e 'runtime_semantic_feed_entries' \
    -e 'runtime_compass_seed_tokens' \
    -e 'runtime_semantic_patch_anchor_count' \
    -e 'runtime_semantic_tests_present' \
    "$OUT/05_status.txt" "$OUT/06_verify.txt" "$OUT/07_plan.txt" || true
} > "$OUT/99_summary.txt"

cat "$OUT/99_summary.txt"
