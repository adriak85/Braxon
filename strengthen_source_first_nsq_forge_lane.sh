#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$HOME/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
SRC="$TC/source_forge"
NSQ="$SRC/nsq_forge"
OUT="$TC/strengthen_source_first_nsq_forge_lane_$(date +%Y%m%d_%H%M%S).log"
JOBS="${JOBS:-7}"

mkdir -p "$SRC"/{downloads,build,install,logs,locks,manifests,tmp}
mkdir -p "$NSQ"/{config,proofs,reports,tools,locks,tmp}

{
  cd "$ROOT"
  source "$SRC/source_forge_env" 2>/dev/null || true
  source "$ROOT/braxon-rust-env" 2>/dev/null || true
  source "$ROOT/braxon-text-env" 2>/dev/null || true

  export JOBS="$JOBS"
  export BRAXON_SOURCE_FIRST=1
  export BRAXON_SOURCE_FORGE="$SRC"
  export BRAXON_NSQ_FORGE="$NSQ"
  export LD_LIBRARY_PATH="$SRC/install/lib:$TC/install/braxon_android_overlay/lib:/data/data/com.termux/files/usr/lib"
  export PATH="$SRC/install/bin:$ROOT:$TC/install/python/bin:/data/data/com.termux/files/usr/opt/rust-nightly/bin:/data/data/com.termux/files/usr/bin:$HOME/.cargo/bin:$HOME/.local/bin:$PATH"

  echo "=== Braxon source-first NSQ forge strengthening ==="
  date
  echo "JOBS=$JOBS"
  echo "SRC=$SRC"
  echo "NSQ=$NSQ"

  echo
  echo "=== write NSQ forge policy ==="
  cat > "$NSQ/config/NSQ_SOURCE_FORGE_POLICY.md" <<'EOF'
# NSQ Source Forge Policy

NSQ is not a generic byte lane.

Rules:
- NSQ is base-eight semantic substrate, not u8 and not bytes.
- Current active lever family must preserve the GE two hundred twenty thousand / proven two hundred twenty five thousand three hundred seventy watermark.
- Legacy eleven twenty six / twenty two fifty four references are legacy references only unless explicitly marked as legacy.
- Source-built tools are staged first, verified second, promoted third.
- Termux package tools are bootstrap/fallback only.
- j7 is the default phone-local build strain.
- Never mark a lane hot-live unless materialized proof exists.
- Missing materialized model/weight artifacts are explicit external boundaries, not fake success.
- State registry is a first-class tool/build surface, not disposable noise.
EOF

  cat > "$NSQ/config/nsq_source_forge.env" <<EOF
export BRAXON_SOURCE_FIRST=1
export BRAXON_NSQ_FORGE="$NSQ"
export BRAXON_NSQ_WATERMARK="BRAXON_NSQ_BASE8_LEVER_SCALE_GE220000_PROVEN225370_V1"
export BRAXON_NSQ_ACTIVE_LEVER_FLOOR="220000"
export BRAXON_NSQ_PROVEN_EFFECTIVE_POSITIONS="225370"
export BRAXON_NSQ_LEGACY_REFERENCE_1126_ONLY="1"
export BRAXON_NSQ_NOT_U8="1"
export BRAXON_NSQ_NOT_BYTES="1"
export BRAXON_NSQ_NOT_HOST_WIDTH_TRUTH="1"
export JOBS="$JOBS"
EOF
  chmod +x "$NSQ/config/nsq_source_forge.env"

  echo
  echo "=== create NSQ-aware forge status tool ==="
  cat > "$NSQ/tools/nsq_forge_status.sh" <<'EOF'
#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$HOME/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
SRC="$TC/source_forge"
NSQ="$SRC/nsq_forge"

source "$SRC/source_forge_env" 2>/dev/null || true
source "$NSQ/config/nsq_source_forge.env"

echo "=== NSQ forge status ==="
date
echo "source_first=$BRAXON_SOURCE_FIRST"
echo "nsq_forge=$BRAXON_NSQ_FORGE"
echo "watermark=$BRAXON_NSQ_WATERMARK"
echo "active_lever_floor=$BRAXON_NSQ_ACTIVE_LEVER_FLOOR"
echo "proven_effective_positions=$BRAXON_NSQ_PROVEN_EFFECTIVE_POSITIONS"
echo "legacy_1126_only=$BRAXON_NSQ_LEGACY_REFERENCE_1126_ONLY"
echo "not_u8=$BRAXON_NSQ_NOT_U8"
echo "not_bytes=$BRAXON_NSQ_NOT_BYTES"
echo "not_host_width_truth=$BRAXON_NSQ_NOT_HOST_WIDTH_TRUTH"
echo

echo "tool anchors:"
for x in braxon-rustc braxon-cargo braxon-python clang zig tree-sitter rg fd jq; do
  printf "%-16s " "$x"
  command -v "$x" || true
done
echo

echo "cargo packages:"
"$ROOT/braxon-cargo" metadata --no-deps --format-version 1 \
  | "$ROOT/braxon-python" -c 'import json,sys; [print(p["name"]) for p in json.load(sys.stdin)["packages"]]' \
  || true
EOF
  chmod +x "$NSQ/tools/nsq_forge_status.sh"

  echo
  echo "=== create NSQ source/manifest scanner ==="
  cat > "$NSQ/tools/nsq_forge_scan.sh" <<'EOF'
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
EOF
  chmod +x "$NSQ/tools/nsq_forge_scan.sh"

  echo
  echo "=== create NSQ fail-closed verifier ==="
  cat > "$ROOT/scripts/verify_braxon_source_first_nsq_forge_lane.sh" <<'EOF'
#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$HOME/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
SRC="$TC/source_forge"
NSQ="$SRC/nsq_forge"

test -f "$SRC/SOURCE_FIRST_POLICY.md"
test -x "$SRC/source_forge_env"
test -f "$NSQ/config/NSQ_SOURCE_FORGE_POLICY.md"
test -x "$NSQ/config/nsq_source_forge.env"
test -x "$NSQ/tools/nsq_forge_status.sh"
test -x "$NSQ/tools/nsq_forge_scan.sh"

source "$SRC/source_forge_env"
source "$NSQ/config/nsq_source_forge.env"

test "$BRAXON_NSQ_WATERMARK" = "BRAXON_NSQ_BASE8_LEVER_SCALE_GE220000_PROVEN225370_V1"
test "$BRAXON_NSQ_ACTIVE_LEVER_FLOOR" = "220000"
test "$BRAXON_NSQ_PROVEN_EFFECTIVE_POSITIONS" = "225370"
test "$BRAXON_NSQ_LEGACY_REFERENCE_1126_ONLY" = "1"
test "$BRAXON_NSQ_NOT_U8" = "1"
test "$BRAXON_NSQ_NOT_BYTES" = "1"

"$NSQ/tools/nsq_forge_status.sh" > "$NSQ/proofs/status_verify.txt"
"$NSQ/tools/nsq_forge_scan.sh" > "$NSQ/proofs/scan_verify.txt"

"$ROOT/braxon-cargo" test -p nsq-core -- --nocapture
"$ROOT/braxon-cargo" test -p Braxon-core -- --nocapture
"$ROOT/braxon-cargo" test -p Braxon-ingest -- --nocapture

echo "BRAXON SOURCE-FIRST NSQ FORGE LANE VERIFY OK"
EOF
  chmod +x "$ROOT/scripts/verify_braxon_source_first_nsq_forge_lane.sh"

  echo
  echo "=== run NSQ status and scan ==="
  "$NSQ/tools/nsq_forge_status.sh"
  "$NSQ/tools/nsq_forge_scan.sh"

  echo
  echo "=== run verifier ==="
  "$ROOT/scripts/verify_braxon_source_first_nsq_forge_lane.sh"

  echo
  echo "=== lock NSQ source-first forge lane ==="
  {
    echo "BRAXON_SOURCE_FIRST_NSQ_FORGE_LANE_LOCK=1"
    date
    echo "JOBS=$JOBS"
    echo "SRC=$SRC"
    echo "NSQ=$NSQ"
    echo "BRAXON_NSQ_WATERMARK=BRAXON_NSQ_BASE8_LEVER_SCALE_GE220000_PROVEN225370_V1"
    "$ROOT/braxon-rustc" --version --verbose
    "$ROOT/braxon-cargo" --version --verbose
    "$NSQ/tools/nsq_forge_status.sh"
  } > "$NSQ/locks/LOCKED_SOURCE_FIRST_NSQ_FORGE_LANE.txt"

  find "$SRC/SOURCE_FIRST_POLICY.md" "$SRC/source_forge_env" "$NSQ" "$ROOT/scripts/verify_braxon_source_first_nsq_forge_lane.sh" \
    -type f -print0 2>/dev/null | sort -z | xargs -0 sha256sum \
    > "$NSQ/locks/manifest.sha256"

  echo
  echo "DONE"
  echo "nsq forge: $NSQ"
  echo "log: $OUT"
  echo "lock: $NSQ/locks/LOCKED_SOURCE_FIRST_NSQ_FORGE_LANE.txt"
} 2>&1 | tee "$OUT"

ln -sf "$OUT" "$TC/strengthen_source_first_nsq_forge_lane_latest.log"
