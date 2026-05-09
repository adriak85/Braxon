#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
OUT="$TC/clean_braxon_warning_noise_$(date +%Y%m%d_%H%M%S).log"
LOCKDIR="$TC/locks/braxon_warning_noise_clean"

mkdir -p "$TC/tmp" "$LOCKDIR" "$ROOT/scripts"

{
  cd "$ROOT"
  source "$ROOT/braxon-rust-env" 2>/dev/null || true

  export PATH="$ROOT:$TC/install/python/bin:/data/data/com.termux/files/usr/opt/rust-nightly/bin:/data/data/com.termux/files/usr/bin:$PATH"
  export LD_LIBRARY_PATH="$TC/install/braxon_android_overlay/lib:/data/data/com.termux/files/usr/lib:${LD_LIBRARY_PATH:-}"

  echo "=== inspect current warning sources ==="
  sed -n '1,40p' crates/braxon-core/src/wowas_rescue.rs
  sed -n '1,80p' crates/braxon-ingest/src/lib.rs

  echo
  echo "=== remove unused imports only ==="
  "$ROOT/braxon-python" - <<'PY'
from pathlib import Path

p = Path("/data/data/com.termux/files/home/Braxon/crates/braxon-core/src/wowas_rescue.rs")
s = p.read_text()

for line in [
    "use serde::{Deserialize, Serialize};\n",
    "use std::collections::HashMap;\n",
]:
    s = s.replace(line, "")

p.write_text(s)
PY

  echo
  echo "=== preserve intentional non-snake-case public identity ==="
  "$ROOT/braxon-python" - <<'PY'
from pathlib import Path

targets = [
    Path("/data/data/com.termux/files/home/Braxon/crates/braxon-core/src/lib.rs"),
    Path("/data/data/com.termux/files/home/Braxon/crates/braxon-ingest/src/lib.rs"),
]

for p in targets:
    if not p.exists():
        continue
    s = p.read_text()
    if "#![allow(non_snake_case)]" not in s:
        s = "#![allow(non_snake_case)]\n" + s
        p.write_text(s)
PY

  echo
  echo "=== format ==="
  "$ROOT/braxon-cargo" fmt --all

  echo
  echo "=== verify core tests ==="
  "$ROOT/braxon-cargo" test -p nsq-core -- --nocapture
  "$ROOT/braxon-cargo" test -p Braxon-core -- --nocapture
  "$ROOT/braxon-cargo" test -p Braxon-ingest -- --nocapture

  echo
  echo "=== run fastest_status ==="
  "$ROOT/fastest_status"

  echo
  echo "=== warning scan ==="
  WARNLOG="$TC/tmp/braxon_warning_scan.log"
  {
    "$ROOT/braxon-cargo" check -p nsq-core
    "$ROOT/braxon-cargo" check -p Braxon-core
    "$ROOT/braxon-cargo" check -p Braxon-ingest
  } 2>&1 | tee "$WARNLOG"

  if grep -E "warning:" "$WARNLOG"; then
    echo "WARNINGS REMAIN"
    exit 3
  fi

  echo
  echo "=== lock clean warning state ==="
  {
    echo "BRAXON_WARNING_NOISE_CLEAN_LOCK=1"
    date
    "$ROOT/braxon-rustc" --version --verbose
    "$ROOT/braxon-cargo" --version --verbose
    "$ROOT/braxon-cargo" metadata --no-deps --format-version 1 \
      | "$ROOT/braxon-python" -c 'import json,sys; [print(p["name"]) for p in json.load(sys.stdin)["packages"]]'
  } > "$LOCKDIR/LOCKED_WARNING_NOISE_CLEAN.txt"

  find \
    crates/braxon-core/src/wowas_rescue.rs \
    crates/braxon-core/src/lib.rs \
    crates/braxon-ingest/src/lib.rs \
    "$ROOT/fastest_status" \
    -type f -print0 | sort -z | xargs -0 sha256sum > "$LOCKDIR/manifest.sha256"

  echo
  echo "DONE"
  echo "log: $OUT"
  echo "lock: $LOCKDIR/LOCKED_WARNING_NOISE_CLEAN.txt"
} 2>&1 | tee "$OUT"

ln -sf "$OUT" "$TC/clean_braxon_warning_noise_latest.log"
