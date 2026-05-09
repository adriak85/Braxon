#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$HOME/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
SRC="$TC/source_forge"
NSQ="$SRC/nsq_forge"
STAMP="$(date +%Y%m%d_%H%M%S)"
REPORT="$NSQ/reports/nsq_scan_${STAMP}.txt"

source "$SRC/source_forge_env" 2>/dev/null || true
source "$NSQ/config/nsq_source_forge.env"

{
  echo "=== NSQ source/manifest scan ==="
  date
  echo

  echo "=== NSQ crates/files ==="
  rg --files "$ROOT/crates" "$ROOT/tests" "$ROOT/state" \
    -g '!target' -g '!.git' \
    | rg '(^|/)(nsq|NSQ|braxon|Braxon|BRAXON)' \
    | head -n 2000
  echo

  echo "=== watermark hits ==="
  rg -n --hidden -g '!.git' -g '!target' "$BRAXON_NSQ_WATERMARK|225370|220000|1126|2254|not_u8|not_bytes|base8|base 8" "$ROOT" \
    | head -n 3000 || true
  echo

  echo "=== Cargo.toml NSQ/Braxon metadata hits ==="
  rg -n --hidden -g 'Cargo.toml' 'BRAXON_NSQ|225370|220000|1126|2254|metadata|watermark|nsq' "$ROOT" || true
  echo

  echo "=== tests mentioning NSQ family ==="
  rg -n --hidden -g '*.rs' '225370|220000|1126|2254|BRAXON_NSQ|not_u8|not_bytes|base8|base 8' "$ROOT/tests" "$ROOT/crates" || true
} > "$REPORT"

echo "NSQ scan report: $REPORT"
