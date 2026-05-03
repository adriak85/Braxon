#!/data/data/com.termux/files/usr/bin/bash
# trace_ingress_v2.sh  —  crash-capturing wrapper for any ingress command
#
# Wraps the given command with ps/mem/logcat monitors and captures
# all output to shared Download storage so it survives a Termux crash.
#
# Usage:
#   bash trace_ingress_v2.sh [ROOT] -- COMMAND [ARGS...]
#
# Example (run the GGUF ingress):
#   bash trace_ingress_v2.sh ~/Braxon -- \
#       bash ~/storage/shared/Download/run_gguf_ingress_v2.sh \
#       ~/storage/shared/Download/my_model.gguf
#
# Example (run install_braxon_weights):
#   bash trace_ingress_v2.sh ~/Braxon -- \
#       bash ~/Braxon/scripts/install_braxon_weights.sh
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
shift || true
[ "${1:-}" = "--" ] && shift

if [ "$#" -eq 0 ]; then
    echo "Usage: bash trace_ingress_v2.sh [ROOT] -- COMMAND [ARGS...]" >&2
    exit 2
fi

DL="$HOME/storage/shared/Download"
STAMP="$(date +%Y%m%d_%H%M%S)"
OUT="$DL/ingress_trace_$STAMP"
mkdir -p "$OUT"

LOG="$OUT/ingress.log"
PSLOG="$OUT/ps_watch.log"
MEMLOG="$OUT/mem_watch.log"
CPUINFO="$OUT/cpu_info.txt"
CMDLOG="$OUT/command.txt"

# ── save command being run ──────────────────────────────────────────
printf '%q ' "$@" > "$CMDLOG"; printf '\n' >> "$CMDLOG"

# ── preflight info ──────────────────────────────────────────────────
{
    echo "time_start=$(date -u +%Y-%m-%dT%H:%M:%SZ)"
    echo "root=$ROOT"
    echo "user=${USER:-$(id -un 2>/dev/null || echo unknown)}"
    echo "uname=$(uname -a)"
    echo "cmd=$(cat "$CMDLOG")"
} | tee "$OUT/preflight.env" >> "$LOG"

# ── cpu topology ────────────────────────────────────────────────────
{
    echo "# cpu max-freq"
    for f in /sys/devices/system/cpu/cpu[0-9]*/cpufreq/cpuinfo_max_freq; do
        [ -f "$f" ] && echo "$f $(cat "$f" 2>/dev/null || echo 0)"
    done | sort
    echo
    echo "# meminfo"
    sed -n '1,40p' /proc/meminfo 2>/dev/null || true
    echo
    echo "# df -h"
    df -h 2>/dev/null || true
    echo
    echo "# device"
    getprop ro.product.model   2>/dev/null || true
    getprop ro.build.version.release 2>/dev/null || true
    getprop ro.build.version.sdk    2>/dev/null || true
} > "$CPUINFO"

# ── pick big cores ──────────────────────────────────────────────────
pick_big_cores() {
    for f in /sys/devices/system/cpu/cpu[0-9]*/cpufreq/cpuinfo_max_freq; do
        [ -f "$f" ] || continue
        cpu="$(basename "$(dirname "$(dirname "$f")")")"
        printf '%s\t%s\n' "$(cat "$f" 2>/dev/null || echo 0)" "${cpu#cpu}"
    done | sort -nr | awk 'NR<=2{print $2}' | paste -sd, -
}
BIG_CORES="$(pick_big_cores 2>/dev/null || true)"

# ── background monitors ─────────────────────────────────────────────
watch_ps() {
    while :; do
        {
            echo "=== $(date -u +%Y-%m-%dT%H:%M:%SZ) ==="
            ps -ef 2>/dev/null \
              | grep -E 'termux|cargo|rustc|Braxon|nsq|ingest|aria2|python|bash|gguf' \
              | grep -v grep || true
            echo
        } >> "$PSLOG"
        sleep 5
    done
}

watch_mem() {
    while :; do
        {
            echo "=== $(date -u +%Y-%m-%dT%H:%M:%SZ) ==="
            sed -n '1,30p' /proc/meminfo 2>/dev/null || true
            echo
        } >> "$MEMLOG"
        sleep 5
    done
}

watch_logcat() {
    logcat -v threadtime -b main -b system -b crash 2>/dev/null \
        > "$OUT/logcat_live.txt" || true
}

termux-wake-lock >/dev/null 2>&1 || true
logcat -c >/dev/null 2>&1 || true

watch_ps    & PS_PID=$!
watch_mem   & MEM_PID=$!
watch_logcat & LC_PID=$!

# ── cleanup / capture on exit ───────────────────────────────────────
cleanup() {
    code="$?"
    {
        echo
        echo "time_end=$(date -u +%Y-%m-%dT%H:%M:%SZ)"
        echo "exit_code=$code"
    } >> "$LOG"

    kill "$PS_PID"  2>/dev/null || true
    kill "$MEM_PID" 2>/dev/null || true
    kill "$LC_PID"  2>/dev/null || true

    logcat -d -v threadtime -b crash  > "$OUT/logcat_crash.txt"  2>/dev/null || true
    logcat -d -v threadtime -b main   > "$OUT/logcat_main.txt"   2>/dev/null || true
    logcat -d -v threadtime -b system > "$OUT/logcat_system.txt" 2>/dev/null || true

    {
        echo
        echo "# notable logcat lines"
        grep -Ei 'Braxon|nsq|panic|fatal|abort|signal|segv|killed|oom|lmkd|lowmem' \
            "$OUT/logcat_live.txt" 2>/dev/null | tail -100 || true
    } >> "$LOG"

    echo "OUT=$OUT"
    echo "LOG=$LOG"
}
trap cleanup EXIT INT TERM

# ── environment ─────────────────────────────────────────────────────
export BRAXON_INGEST_USE_DAEMON=0
export BRAXON_ENABLE_INGEST_DAEMON=0
export RUST_BACKTRACE=full
export RUST_LOG="${RUST_LOG:-info}"
export MALLOC_ARENA_MAX=1

cd "$ROOT"

# ── launch ──────────────────────────────────────────────────────────
if command -v taskset >/dev/null 2>&1 && [ -n "${BIG_CORES:-}" ]; then
    echo "pinning to big cores: $BIG_CORES" | tee -a "$LOG"
    exec taskset -c "$BIG_CORES" nice -n 5 "$@" 2>&1 | tee -a "$LOG"
else
    exec nice -n 5 "$@" 2>&1 | tee -a "$LOG"
fi
