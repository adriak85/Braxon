#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT_DIR="${BRAXON_HOME:-$(cd "$(dirname "$0")/.." && pwd)}"
STATE_DIR="$ROOT_DIR/state/braxon"
LOG_DIR="$STATE_DIR/logs"
PID_FILE="$STATE_DIR/braxon_weight_ingest.pid"
STATUS_FILE="$STATE_DIR/braxon_weight_ingest.status"
LOG_FILE="$LOG_DIR/braxon_weight_ingest.log"
PIPELINE_STATUS="$STATE_DIR/braxon_nsq_pipeline.status"
FINALIZE_STATUS="$STATE_DIR/BRAXON_whole_core_finalize.status"
RETIRE_FILE="$STATE_DIR/braxon_weight_ingest.retired"
CONTROL_FILE="$STATE_DIR/braxon_weight_ingest.control"
ENVELOPE_PATH="$ROOT_DIR/assets/braxon_core/weights/nsq/Braxon-27B_extended.nsqb.meta"
RESERVED_ARTIFACT_PATH="$ROOT_DIR/assets/braxon_core/weights/nsq/Braxon-27B_extended.nsqb"
JOB_ID="${BRAXON_WEIGHT_INGEST_JOB_ID:-320032}"

"$ROOT_DIR/scripts/seed_braxon_nsq_envelope.sh" >/dev/null
"$ROOT_DIR/scripts/finalize_braxon_nsq_whole_core.sh" >/dev/null

if [ -f "$PID_FILE" ]; then
  pid="$(cat "$PID_FILE" 2>/dev/null || true)"
else
  pid=""
fi

if [ -n "${pid:-}" ] && kill -0 "$pid" 2>/dev/null; then
  echo "daemon_pid=$pid"
  echo "daemon_state=running"
else
  echo "daemon_pid=${pid:-none}"
  echo "daemon_state=stopped"
fi

if [ -f "$STATUS_FILE" ]; then
  cat "$STATUS_FILE"
else
  echo "tick=none state=unknown"
fi

echo "reserved_runtime_artifact_present=$([ -f "$RESERVED_ARTIFACT_PATH" ] && echo yes || echo no)"
echo "nsq_envelope_present=$([ -f "$ENVELOPE_PATH" ] && echo yes || echo no)"

if [ -f "$PIPELINE_STATUS" ]; then
  echo "---"
  cat "$PIPELINE_STATUS"
fi

if [ -f "$FINALIZE_STATUS" ]; then
  echo "---"
  cat "$FINALIZE_STATUS"
fi

if [ -f "$RETIRE_FILE" ]; then
  echo "---"
  cat "$RETIRE_FILE"
fi

if [ -f "$CONTROL_FILE" ]; then
  echo "---"
  cat "$CONTROL_FILE"
fi

if [ -f "$ENVELOPE_PATH" ]; then
  echo "---"
  sed -n '1,24p' "$ENVELOPE_PATH"
fi

if command -v termux-job-scheduler >/dev/null 2>&1; then
  echo "---"
  termux-job-scheduler -p | sed -n "/job-id: $JOB_ID/,+8p" || true
fi

if [ -f "$LOG_FILE" ]; then
  echo "---"
  tail -n 20 "$LOG_FILE"
fi
