#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT_DIR="${BRAXON_HOME:-$(cd "$(dirname "$0")/.." && pwd)}"
STATE_DIR="$ROOT_DIR/state/braxon"
LOG_DIR="$STATE_DIR/logs"
PID_FILE="$STATE_DIR/braxon_weight_ingest.pid"
STATUS_FILE="$STATE_DIR/braxon_weight_ingest.status"
CONTROL_FILE="$STATE_DIR/braxon_weight_ingest.control"
RETIRE_FILE="$STATE_DIR/braxon_weight_ingest.retired"
PIPELINE_STATUS="$STATE_DIR/braxon_nsq_pipeline.status"
SOURCE_INGEST_DIR="$ROOT_DIR/assets/braxon_core/source_ingest/braxon_transport"
INSTALL_SCRIPT="${BRAXON_INSTALL_SCRIPT:-$ROOT_DIR/scripts/install_braxon_weights.sh}"
SEED_SCRIPT="$ROOT_DIR/scripts/seed_braxon_nsq_envelope.sh"
FINALIZE_SCRIPT="$ROOT_DIR/scripts/finalize_braxon_nsq_whole_core.sh"
STALL_SECONDS="${BRAXON_INGEST_STALL_SECONDS:-180}"
POLL_SECONDS="${BRAXON_INGEST_POLL_SECONDS:-30}"
TRANSPORT_LOG="$LOG_DIR/braxon_transport.log"
ALLOW_DAEMON="${BRAXON_ENABLE_INGEST_DAEMON:-0}"

mkdir -p "$LOG_DIR"

pipeline_value() {
  local key="$1"
  local fallback="$2"
  local value=""
  if [ -f "$PIPELINE_STATUS" ]; then
    value="$(awk -F= -v key="$key" '$1 == key { print $2 }' "$PIPELINE_STATUS" | tail -n 1)"
  fi

  if [ -n "$value" ]; then
    printf '%s\n' "$value"
  else
    printf '%s\n' "$fallback"
  fi
}

write_control() {
  local mode="$1"
  local reason="$2"
  local daemon_pid="$3"
  local wake_lock="$4"
  local source_ingest_status
  local nsq_envelope_status
  local nsq_recode_status
  local whole_core_runtime_status

  source_ingest_status="$(pipeline_value "source_ingest_status" "missing")"
  nsq_envelope_status="$(pipeline_value "nsq_envelope_status" "missing")"
  nsq_recode_status="$(pipeline_value "nsq_recode_status" "not_started")"
  whole_core_runtime_status="$(pipeline_value "whole_core_runtime_status" "not_ready")"

  cat > "$CONTROL_FILE" <<EOF
mode=$mode
reason=$reason
job_id=none
daemon_pid=$daemon_pid
wake_lock_requested=$wake_lock
source_ingest_status=$source_ingest_status
nsq_envelope_status=$nsq_envelope_status
nsq_recode_status=$nsq_recode_status
whole_core_runtime_status=$whole_core_runtime_status
EOF
}

current_source_bytes() {
  if [ -d "$SOURCE_INGEST_DIR" ]; then
    find "$SOURCE_INGEST_DIR" -type f -printf '%s\n' | awk '{sum += $1} END {print sum + 0}'
  else
    echo 0
  fi
}

refresh_pipeline() {
  "$SEED_SCRIPT" > "$LOG_DIR/BRAXON_envelope_state.log" || true
  "$FINALIZE_SCRIPT" || true
}

refresh_pipeline

if [ "$(pipeline_value "source_ingest_status" "missing")" = "complete" ]; then
  rm -f "$PID_FILE"
  write_control "retired" "ingress_complete" "none" "no"
  cat > "$RETIRE_FILE" <<EOF
retired=true
reason=ingress_complete
source_ingest_status=complete
nsq_recode_status=$(pipeline_value "nsq_recode_status" "not_started")
whole_core_runtime_status=$(pipeline_value "whole_core_runtime_status" "not_ready")
EOF
  date -u +"tick=%Y-%m-%dT%H:%M:%SZ state=ingress_complete_noop" > "$STATUS_FILE"
  exit 0
fi

if [ "$ALLOW_DAEMON" != "1" ]; then
  rm -f "$PID_FILE"
  write_control "manual_only" "daemon_disabled_for_safety" "none" "no"
  date -u +"tick=%Y-%m-%dT%H:%M:%SZ state=manual_only" > "$STATUS_FILE"
  exit 0
fi

echo "$$" > "$PID_FILE"
trap 'rm -f "$PID_FILE"' EXIT
write_control "active" "ingress_resume" "$$" "yes"

while true; do
  date -u +"tick=%Y-%m-%dT%H:%M:%SZ state=ingress_resume" > "$STATUS_FILE"

  "$INSTALL_SCRIPT" >> "$TRANSPORT_LOG" 2>&1 &
  worker_pid=$!
  last_bytes="$(current_source_bytes)"
  last_change_epoch="$(date +%s)"
  worker_stalled=0

  while kill -0 "$worker_pid" 2>/dev/null; do
    sleep "$POLL_SECONDS"
    current_bytes="$(current_source_bytes)"

    if [ "$current_bytes" -gt "$last_bytes" ]; then
      last_bytes="$current_bytes"
      last_change_epoch="$(date +%s)"
      date -u +"tick=%Y-%m-%dT%H:%M:%SZ state=ingress_progress worker_pid=$worker_pid bytes=$current_bytes" > "$STATUS_FILE"
      continue
    fi

    now_epoch="$(date +%s)"
    stalled_for="$((now_epoch - last_change_epoch))"
    date -u +"tick=%Y-%m-%dT%H:%M:%SZ state=ingress_monitor worker_pid=$worker_pid bytes=$current_bytes stalled_for=$stalled_for" > "$STATUS_FILE"

    if [ "$stalled_for" -ge "$STALL_SECONDS" ]; then
      worker_stalled=1
      date -u +"tick=%Y-%m-%dT%H:%M:%SZ state=stall_restart worker_pid=$worker_pid bytes=$current_bytes stalled_for=$stalled_for" > "$STATUS_FILE"
      kill "$worker_pid" 2>/dev/null || true
      if wait "$worker_pid"; then
        :
      else
        :
      fi
      break
    fi
  done

  if [ "$worker_stalled" -eq 0 ]; then
    if wait "$worker_pid"; then
      date -u +"tick=%Y-%m-%dT%H:%M:%SZ state=worker_exit_ok worker_pid=$worker_pid" > "$STATUS_FILE"
    else
      worker_rc=$?
      date -u +"tick=%Y-%m-%dT%H:%M:%SZ state=retry_pending worker_pid=$worker_pid exit_code=$worker_rc" > "$STATUS_FILE"
    fi
  fi

  refresh_pipeline
  write_control "active" "ingress_monitoring" "$$" "yes"

  source_ingest_status="$(awk -F= '/^source_ingest_status=/{print $2}' "$PIPELINE_STATUS" 2>/dev/null || true)"
  if [ "$source_ingest_status" = "complete" ]; then
    write_control "retired" "ingress_complete" "none" "no"
    cat > "$RETIRE_FILE" <<EOF
retired=true
reason=ingress_complete
source_ingest_status=complete
nsq_recode_status=$(pipeline_value "nsq_recode_status" "not_started")
whole_core_runtime_status=$(pipeline_value "whole_core_runtime_status" "not_ready")
EOF
    date -u +"tick=%Y-%m-%dT%H:%M:%SZ state=ingress_complete" > "$STATUS_FILE"
    break
  fi

  if [ "$worker_stalled" -eq 1 ]; then
    date -u +"tick=%Y-%m-%dT%H:%M:%SZ state=stall_reinitiated" > "$STATUS_FILE"
    sleep 5
  else
    sleep 15
  fi
done
