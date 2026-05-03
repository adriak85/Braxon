#!/data/data/com.termux/files/usr/bin/bash
# transport_only_1x1_v2.sh  —  single-connection weight transport
#
# Forces ARIA2C_SPLIT=1, ARIA2C_MAX_CONNECTIONS=1, and disables the
# ingest daemon so nothing runs in parallel.  Uses nice -n 5 (not 15)
# and pins to big cores when available.
#
# Usage:  bash transport_only_1x1_v2.sh [ROOT]
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
LOGDIR="$ROOT/state/braxon/logs"
LOG="$LOGDIR/install_braxon_weights.transport_1x1.log"
INSTALL_SCRIPT="$ROOT/scripts/install_braxon_weights.sh"

mkdir -p "$LOGDIR"

if [ ! -f "$INSTALL_SCRIPT" ]; then
    echo "ERROR: not found: $INSTALL_SCRIPT" >&2
    exit 1
fi

export BRAXON_INGEST_USE_DAEMON=0
export BRAXON_ENABLE_INGEST_DAEMON=0
export ARIA2C_SPLIT=1
export ARIA2C_MAX_CONNECTIONS=1
export MALLOC_ARENA_MAX=1

termux-wake-lock >/dev/null 2>&1 || true

# ── big cores ───────────────────────────────────────────────────────
pick_big_cores() {
    for f in /sys/devices/system/cpu/cpu[0-9]*/cpufreq/cpuinfo_max_freq; do
        [ -f "$f" ] || continue
        cpu="$(basename "$(dirname "$(dirname "$f")")")"
        printf '%s\t%s\n' "$(cat "$f" 2>/dev/null || echo 0)" "${cpu#cpu}"
    done | sort -nr | awk 'NR<=2{print $2}' | paste -sd, -
}
BIG="$(pick_big_cores 2>/dev/null || true)"

echo "root=$ROOT"
echo "log=$LOG"
echo "mode=transport_only_1x1"
echo "time_start=$(date -u +%Y-%m-%dT%H:%M:%SZ)"

cd "$ROOT"

if command -v taskset >/dev/null 2>&1 && [ -n "${BIG:-}" ]; then
    echo "pinning to big cores: $BIG"
    taskset -c "$BIG" nice -n 5 \
        /data/data/com.termux/files/usr/bin/bash "$INSTALL_SCRIPT" \
        2>&1 | tee -a "$LOG"
else
    nice -n 5 \
        /data/data/com.termux/files/usr/bin/bash "$INSTALL_SCRIPT" \
        2>&1 | tee -a "$LOG"
fi

echo "time_end=$(date -u +%Y-%m-%dT%H:%M:%SZ)"
tail -n 80 "$LOG" || true
