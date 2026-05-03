#!/data/data/com.termux/files/usr/bin/bash
# run_gguf_ingress_v2.sh  —  safe single-model ingress with crash recovery
#
# Usage:
#   bash run_gguf_ingress_v2.sh  MODEL.gguf  [ROOT]
#
# Outputs (all in ROOT/state/braxon/ingress/<model-stem>/):
#   manifest.tsv   — tensor index (crash-resumed)
#   manifest.ckpt  — checkpoint (atomic, survives crash)
#   manifest.sum   — GGUF header summary
#   ingress.log    — stderr + progress
#
# Re-run after a crash: the checkpoint is read and the manifest is
# appended from where it left off.  Safe to call repeatedly.
set -euo pipefail

MODEL="${1:-}"
ROOT="${2:-$HOME/Braxon}"

if [ -z "$MODEL" ] || [ ! -f "$MODEL" ]; then
    echo "Usage: bash run_gguf_ingress_v2.sh  MODEL.gguf  [ROOT]" >&2
    echo "  MODEL.gguf must exist" >&2
    exit 2
fi

DL="$HOME/storage/shared/Download"
BIN="$DL/gguf_ingress_c_v2"

if [ ! -x "$BIN" ]; then
    echo "Binary not found: $BIN" >&2
    echo "Run build_gguf_ingress_v2.sh first." >&2
    exit 1
fi

# ── output directory ────────────────────────────────────────────────
STEM="$(basename "$MODEL" .gguf)"
OUTDIR="$ROOT/state/braxon/ingress/$STEM"
mkdir -p "$OUTDIR"

MANIFEST="$OUTDIR/manifest.tsv"
CKPT="$OUTDIR/manifest.ckpt"
SUMMARY="$OUTDIR/manifest.sum"
LOG="$OUTDIR/ingress.log"

echo "model=$MODEL"
echo "outdir=$OUTDIR"
echo "time_start=$(date -u +%Y-%m-%dT%H:%M:%SZ)"

# ── resume state ────────────────────────────────────────────────────
if [ -f "$CKPT" ]; then
    NEXT="$(grep '^next_index=' "$CKPT" | cut -d= -f2 || echo 0)"
    TOTAL="$(grep '^total_tensors=' "$CKPT" | cut -d= -f2 || echo '?')"
    echo "resuming from tensor $NEXT / $TOTAL"
else
    echo "starting fresh"
fi

# ── keep phone awake ────────────────────────────────────────────────
termux-wake-lock >/dev/null 2>&1 || true

# ── choose big cores if available ──────────────────────────────────
pick_big_cores() {
    for f in /sys/devices/system/cpu/cpu[0-9]*/cpufreq/cpuinfo_max_freq; do
        [ -f "$f" ] || continue
        cpu="$(basename "$(dirname "$(dirname "$f")")")"
        printf '%s\t%s\n' "$(cat "$f" 2>/dev/null || echo 0)" "${cpu#cpu}"
    done | sort -nr | awk 'NR<=2{print $2}' | paste -sd, -
}
BIG="$(pick_big_cores 2>/dev/null || true)"

run_ingress() {
    "$BIN" \
        --input        "$MODEL"   \
        --out-manifest "$MANIFEST" \
        --checkpoint   "$CKPT"    \
        --summary      "$SUMMARY" \
        --sample-bytes 4096       \
        --report-every 50
}

{
    echo "=== $(date -u +%Y-%m-%dT%H:%M:%SZ) ==="
    if [ -n "${BIG:-}" ]; then
        echo "pinning to cores $BIG"
        taskset -c "$BIG" nice -n 5 run_ingress
    else
        nice -n 5 run_ingress
    fi
    echo "exit_code=$?"
    echo "time_end=$(date -u +%Y-%m-%dT%H:%M:%SZ)"
} 2>&1 | tee -a "$LOG"

echo "manifest=$MANIFEST"
echo "log=$LOG"
