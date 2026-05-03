#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail
ROOT="${1:-$HOME/Braxon}"
BIN="$HOME/.cargo/target-cache/Braxon/debug/Braxon"

echo "BRAXON CURRENT TRUTH"
echo "root=$ROOT"
echo

if [ -x "$BIN" ]; then
  echo "== verify =="
  "$BIN" verify 2>/dev/null | sed -n '1,40p' || true
  echo
  echo "== status =="
  "$BIN" status 2>/dev/null | sed -n '1,40p' || true
  echo
fi

echo "== control =="
[ -f "$ROOT/state/braxon/braxon_weight_ingest.control" ] && sed -n '1,60p' "$ROOT/state/braxon/braxon_weight_ingest.control" || echo missing
echo

echo "== pipeline =="
[ -f "$ROOT/state/braxon/braxon_nsq_pipeline.status" ] && sed -n '1,80p' "$ROOT/state/braxon/braxon_nsq_pipeline.status" || echo missing
echo

echo "== runtime artifact truth =="
echo "real_nsqb_present=$([ -f "$ROOT/assets/braxon_core/weights/nsq/Braxon-27B_extended.nsqb" ] && echo yes || echo no)"
echo "envelope_present=$([ -f "$ROOT/assets/braxon_core/weights/nsq/Braxon-27B_extended.nsqb.meta" ] && echo yes || echo no)"
echo "source_ingest_dir_present=$([ -d "$ROOT/assets/braxon_core/source_ingest/braxon_transport" ] && echo yes || echo no)"
