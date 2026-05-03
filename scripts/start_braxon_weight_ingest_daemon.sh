#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT_DIR="${BRAXON_HOME:-$(cd "$(dirname "$0")/.." && pwd)}"
STATE_DIR="$ROOT_DIR/state/braxon"
PIPELINE_STATUS="$STATE_DIR/braxon_nsq_pipeline.status"
CONTROL_FILE="$STATE_DIR/braxon_weight_ingest.control"
STATUS_FILE="$STATE_DIR/braxon_weight_ingest.status"
RETIRE_FILE="$STATE_DIR/braxon_weight_ingest.retired"
PID_FILE="$STATE_DIR/braxon_weight_ingest.pid"

mkdir -p "$STATE_DIR"

"$ROOT_DIR/scripts/seed_braxon_nsq_envelope.sh" >/dev/null
"$ROOT_DIR/scripts/finalize_braxon_nsq_whole_core.sh" >/dev/null

source_ingest_status="$(awk -F= '/^source_ingest_status=/{print $2}' "$PIPELINE_STATUS" 2>/dev/null | tail -n 1)"
nsq_envelope_status="$(awk -F= '/^nsq_envelope_status=/{print $2}' "$PIPELINE_STATUS" 2>/dev/null | tail -n 1)"
nsq_recode_status="$(awk -F= '/^nsq_recode_status=/{print $2}' "$PIPELINE_STATUS" 2>/dev/null | tail -n 1)"
whole_core_runtime_status="$(awk -F= '/^whole_core_runtime_status=/{print $2}' "$PIPELINE_STATUS" 2>/dev/null | tail -n 1)"

rm -f "$PID_FILE"

if [ "$source_ingest_status" = "complete" ]; then
  cat > "$CONTROL_FILE" <<EOF
mode=retired
reason=ingress_complete
job_id=none
daemon_pid=none
wake_lock_requested=no
source_ingest_status=$source_ingest_status
nsq_envelope_status=$nsq_envelope_status
nsq_recode_status=$nsq_recode_status
whole_core_runtime_status=$whole_core_runtime_status
EOF
  cat > "$RETIRE_FILE" <<EOF
retired=true
reason=ingress_complete
source_ingest_status=$source_ingest_status
nsq_recode_status=$nsq_recode_status
whole_core_runtime_status=$whole_core_runtime_status
EOF
  date -u +"tick=%Y-%m-%dT%H:%M:%SZ state=ingress_complete_noop" > "$STATUS_FILE"
  echo "daemon_start=skipped"
  echo "reason=ingress_complete"
  exit 0
fi

cat > "$CONTROL_FILE" <<EOF
mode=manual_only
reason=daemon_disabled_for_safety
job_id=none
daemon_pid=none
wake_lock_requested=no
source_ingest_status=${source_ingest_status:-missing}
nsq_envelope_status=${nsq_envelope_status:-missing}
nsq_recode_status=${nsq_recode_status:-not_started}
whole_core_runtime_status=${whole_core_runtime_status:-not_ready}
EOF

rm -f "$RETIRE_FILE"
date -u +"tick=%Y-%m-%dT%H:%M:%SZ state=manual_only" > "$STATUS_FILE"

echo "daemon_start=skipped"
echo "reason=daemon_disabled_for_safety"
echo "manual_ingress_command=$ROOT_DIR/scripts/install_braxon_weights.sh"
