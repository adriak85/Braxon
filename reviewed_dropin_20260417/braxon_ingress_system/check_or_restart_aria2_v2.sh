#!/data/data/com.termux/files/usr/bin/bash
# check_or_restart_aria2_v2.sh  —  check aria2c status; restart if dead
#
# Prints a compact status report and, if aria2c is not running, launches
# transport_only_1x1_v2.sh in nohup background.
#
# Usage:  bash check_or_restart_aria2_v2.sh [ROOT]
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
DL="$HOME/storage/shared/Download"
SRC="$ROOT/assets/braxon_core/source_ingest/braxon_transport"
LOGDIR="$ROOT/state/braxon/logs"
LOG="$LOGDIR/install_braxon_weights.transport_1x1.log"
STARTER="$DL/transport_only_1x1_v2.sh"

mkdir -p "$LOGDIR"

echo "time=$(date -u +%Y-%m-%dT%H:%M:%SZ)"
echo "root=$ROOT"
echo "src=$SRC"
echo "log=$LOG"
echo

# ── report current download state ──────────────────────────────────
echo "# active transport processes"
pgrep -af 'aria2c|install_braxon_weights' 2>/dev/null || echo "(none)"
echo

echo "# unfinished aria2 sidecars"
find "$SRC" -maxdepth 1 -name '*.aria2' -printf '%f\t%s\n' 2>/dev/null \
    | sort || echo "(none)"
echo

echo "# shard files (first 40)"
find "$SRC" -maxdepth 1 -type f -printf '%f\t%s\n' 2>/dev/null \
    | sort | head -40 || echo "(none)"
echo

# ── decide whether to restart ───────────────────────────────────────
if pgrep -f 'aria2c' >/dev/null 2>&1; then
    echo "status=aria2c already running — no action needed"
    exit 0
fi

echo "status=aria2c not running"

if [ ! -f "$STARTER" ]; then
    echo "error=starter script missing: $STARTER" >&2
    echo "Copy transport_only_1x1_v2.sh to $DL first." >&2
    exit 1
fi

if [ ! -x "$STARTER" ]; then
    chmod 755 "$STARTER"
fi

echo "action=launching transport-only 1x1 in background"
nohup /data/data/com.termux/files/usr/bin/bash "$STARTER" "$ROOT" \
    >> "$LOG" 2>&1 < /dev/null &

sleep 3

echo
echo "# post-restart processes"
pgrep -af 'aria2c|install_braxon_weights' 2>/dev/null || echo "(none)"
echo
echo "# log tail"
tail -n 60 "$LOG" 2>/dev/null || true
