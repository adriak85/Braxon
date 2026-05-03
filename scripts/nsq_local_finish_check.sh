#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${BRAXON_ROOT:-$HOME/Braxon}"
DL="$HOME/storage/shared/Download"
STAMP="$(date +%Y%m%d_%H%M%S)"
OUT="$DL/nsq_local_finish_check_${STAMP}"

mkdir -p "$OUT"
cd "$ROOT"

{
  echo "== NSQ local finish check =="
  date
  echo "root=$ROOT"
  echo

  echo "== canonical docs =="
  ls -la docs/nsq || true
  echo

  echo "== NSQ doc law snippets =="
  grep -RIn "NSQ is not a byte language\|Multiplication is prime-path vector introduction\|leader-bit\|six hundred ninety-nine\|exponent factor" docs/nsq || true
  echo

  echo "== crate discovery =="
  if command -v fd >/dev/null 2>&1; then
    fd '^Cargo.toml$|nsq' crates docs tools scripts state -HI 2>/dev/null | sed -n '1,300p'
  else
    find crates docs tools scripts state -iname '*nsq*' -o -name Cargo.toml 2>/dev/null | sed -n '1,300p'
  fi
  echo

  echo "== formula verifier demo number 12 =="
  python3 tools/nsq_finish/nsq_formula_verify.py --number 12 --width 33 --groups 17 --orbit-payload 699
  echo

  echo "== model reconstruction manifest template =="
  python3 tools/nsq_finish/nsq_model_reconstruct_scaffold.py --write-template
  echo

  echo "== git status =="
  git status --short || true
  echo

  echo "== cargo metadata/check =="
  if command -v cargo >/dev/null 2>&1 && [ -f Cargo.toml ]; then
    cargo metadata --no-deps --format-version 1 > "$OUT/cargo_metadata.json" 2> "$OUT/cargo_metadata.err" || true

    # Keep this non-destructive and broad. Some local trees are incomplete.
    cargo check --workspace > "$OUT/cargo_check.out" 2> "$OUT/cargo_check.err" || true

    echo "cargo metadata saved to $OUT/cargo_metadata.json"
    echo "cargo check stdout saved to $OUT/cargo_check.out"
    echo "cargo check stderr saved to $OUT/cargo_check.err"
  else
    echo "cargo unavailable or no Cargo.toml at root; skipped cargo check"
  fi

} | tee "$OUT/summary.txt"

echo "report_dir=$OUT"
