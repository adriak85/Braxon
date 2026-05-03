#!/data/data/com.termux/files/usr/bin/bash
# strace_braxon_weights_v2.sh  —  strace (or bash -x fallback) wrapper
#                               for install_braxon_weights.sh
#
# Kills any existing ingest workers first, then traces one clean run.
# All output is tarred to shared Download storage so it survives a crash.
#
# Usage:  bash strace_braxon_weights_v2.sh [ROOT]
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
DL="$HOME/storage/shared/Download"
STAMP="$(date +%Y%m%d_%H%M%S)"
OUT="$DL/braxon_weights_strace_$STAMP"
TAR="$OUT.tar.gz"

INSTALL_SCRIPT="$ROOT/scripts/install_braxon_weights.sh"

if [ ! -f "$INSTALL_SCRIPT" ]; then
    echo "ERROR: install script not found: $INSTALL_SCRIPT" >&2
    exit 1
fi

mkdir -p "$OUT"

STATUS="$OUT/status.txt"
STRACE_META="$OUT/strace_meta.txt"
STDOUT="$OUT/run.stdout.log"
STDERR="$OUT/run.stderr.log"
BASH_X="$OUT/bash_xtrace.log"

{
    echo "time_start=$(date -u +%Y-%m-%dT%H:%M:%SZ)"
    echo "root=$ROOT"
    echo "script=$INSTALL_SCRIPT"
    echo
    echo "# df -h"
    df -h 2>/dev/null || true
    echo
    echo "# meminfo"
    sed -n '1,40p' /proc/meminfo 2>/dev/null || true
} > "$STATUS"

# ── kill existing workers ───────────────────────────────────────────
echo "# pre-kill processes:" >> "$STATUS"
pgrep -af 'aria2c|install_braxon_weights|braxon_weight_ingest' >> "$STATUS" 2>/dev/null || true

pkill -f 'install_braxon_weights.sh'   2>/dev/null || true
pkill -f 'aria2c.*qwen_transport'   2>/dev/null || true
pkill -f 'braxon_weight_ingest'        2>/dev/null || true
sleep 2

echo "# post-kill processes:" >> "$STATUS"
pgrep -af 'aria2c|install_braxon_weights|braxon_weight_ingest' >> "$STATUS" 2>/dev/null || true

# ── environment ─────────────────────────────────────────────────────
export BRAXON_INGEST_USE_DAEMON=0
export BRAXON_ENABLE_INGEST_DAEMON=0
export ARIA2C_SPLIT=1
export ARIA2C_MAX_CONNECTIONS=1
export RUST_BACKTRACE=full
export RUST_LOG="${RUST_LOG:-info}"
export MALLOC_ARENA_MAX=1

cd "$ROOT"

# ── cleanup / tar on exit ───────────────────────────────────────────
cleanup() {
    code="$?"
    {
        echo
        echo "time_end=$(date -u +%Y-%m-%dT%H:%M:%SZ)"
        echo "exit_code=$code"
        echo "# final processes:"
        pgrep -af 'aria2c|install_braxon_weights|braxon_weight_ingest' || true
    } >> "$STATUS"
    tar -czf "$TAR" -C "$DL" "$(basename "$OUT")" 2>/dev/null || true
    echo "OUT=$OUT"
    echo "TAR=$TAR"
}
trap cleanup EXIT INT TERM

termux-wake-lock >/dev/null 2>&1 || true

# ── strace or bash -x ───────────────────────────────────────────────
if command -v strace >/dev/null 2>&1; then
    {
        echo "mode=strace"
        echo "strace=$(command -v strace)"
        echo "target=$INSTALL_SCRIPT"
    } > "$STRACE_META"

    strace -ff -tt -T -s 256 -yy \
        -o "$OUT/strace" \
        /data/data/com.termux/files/usr/bin/bash "$INSTALL_SCRIPT" \
        > >(tee -a "$STDOUT") \
        2> >(tee -a "$STDERR" >&2)
else
    {
        echo "mode=bash_xtrace"
        echo "reason=strace not in PATH"
        echo "target=$INSTALL_SCRIPT"
    } > "$STRACE_META"

    /data/data/com.termux/files/usr/bin/bash -x "$INSTALL_SCRIPT" \
        > >(tee -a "$STDOUT") \
        2> >(tee -a "$BASH_X" | tee -a "$STDERR" >&2)
fi
