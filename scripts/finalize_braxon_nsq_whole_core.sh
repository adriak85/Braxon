#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT_DIR="${BRAXON_HOME:-$(cd "$(dirname "$0")/.." && pwd)}"
STATE_DIR="$ROOT_DIR/state/braxon"
TARGET_DIR="$ROOT_DIR/assets/braxon_core/weights/nsq"
RESERVED_ARTIFACT_PATH="$TARGET_DIR/Braxon-27B_extended.nsqb"
ENVELOPE_PATH="$TARGET_DIR/Braxon-27B_extended.nsqb.meta"
FINALIZE_STATUS="$STATE_DIR/BRAXON_whole_core_finalize.status"
PIPELINE_STATUS="$STATE_DIR/braxon_nsq_pipeline.status"
READY_MARKER="$TARGET_DIR/Braxon-27B_extended.ready"
VERIFY_SCRIPT="$ROOT_DIR/scripts/verify_braxon_nsq_whole_core.sh"
VERIFY_LOG="$STATE_DIR/BRAXON_whole_core_verify.last"

mkdir -p "$STATE_DIR"

if [ -f "$RESERVED_ARTIFACT_PATH" ] && grep -Eq '^artifact_kind: nsq_(whole_core_envelope|source_ingest_envelope)$' "$RESERVED_ARTIFACT_PATH"; then
  mv -f "$RESERVED_ARTIFACT_PATH" "$ENVELOPE_PATH"
fi

source_ingest_status="missing"
nsq_envelope_status="missing"
source_required_files="0"
source_present_files="0"
source_total_bytes="0"
source_blake3_manifest=""
source_blake3_recorded_files="0"
source_blake3_status="missing"
source_authority_lane=""
source_authority_state="missing"
nsq_artifact_state="absent_checkout"
runtime_authority_lane="none_bound"
runtime_authority_state="unbound"
runtime_authority_bound="false"
source_required_shards="0"
source_present_shards="0"
source_materialized_shards="0"
source_pointer_stub_files="0"
source_text_stub_files="0"

if [ -f "$ENVELOPE_PATH" ]; then
  source_ingest_status="$(awk -F': ' '/^source_ingest_status:/{print $2}' "$ENVELOPE_PATH")"
  nsq_envelope_status="$(awk -F': ' '/^nsq_envelope_status:/{print $2}' "$ENVELOPE_PATH")"
  source_required_files="$(awk -F': ' '/^source_required_files:/{print $2}' "$ENVELOPE_PATH")"
  source_present_files="$(awk -F': ' '/^source_present_files:/{print $2}' "$ENVELOPE_PATH")"
  source_total_bytes="$(awk -F': ' '/^source_total_bytes:/{print $2}' "$ENVELOPE_PATH")"
  source_blake3_manifest="$(awk -F': ' '/^source_blake3_manifest:/{print $2}' "$ENVELOPE_PATH")"
  source_blake3_recorded_files="$(awk -F': ' '/^source_blake3_recorded_files:/{print $2}' "$ENVELOPE_PATH")"
  source_blake3_status="$(awk -F': ' '/^source_blake3_status:/{print $2}' "$ENVELOPE_PATH")"
  source_authority_lane="$(awk -F': ' '/^source_authority_lane:/{print $2}' "$ENVELOPE_PATH")"
  source_authority_state="$(awk -F': ' '/^source_authority_state:/{print $2}' "$ENVELOPE_PATH")"
  nsq_artifact_state="$(awk -F': ' '/^nsq_artifact_state:/{print $2}' "$ENVELOPE_PATH")"
  runtime_authority_lane="$(awk -F': ' '/^runtime_authority_lane:/{print $2}' "$ENVELOPE_PATH")"
  runtime_authority_state="$(awk -F': ' '/^runtime_authority_state:/{print $2}' "$ENVELOPE_PATH")"
  runtime_authority_bound="$(awk -F': ' '/^runtime_authority_bound:/{print $2}' "$ENVELOPE_PATH")"
  source_required_shards="$(awk -F': ' '/^source_required_shards:/{print $2}' "$ENVELOPE_PATH")"
  source_present_shards="$(awk -F': ' '/^source_present_shards:/{print $2}' "$ENVELOPE_PATH")"
  source_materialized_shards="$(awk -F': ' '/^source_materialized_shards:/{print $2}' "$ENVELOPE_PATH")"
  source_pointer_stub_files="$(awk -F': ' '/^source_pointer_stub_files:/{print $2}' "$ENVELOPE_PATH")"
  source_text_stub_files="$(awk -F': ' '/^source_text_stub_files:/{print $2}' "$ENVELOPE_PATH")"
fi

nsq_recode_status="not_started"
whole_core_runtime_status="not_ready"
whole_core_ready="false"
verification_state="not_run"
artifact_verification_status="not_run"
reserved_runtime_artifact_present="no"

if [ -f "$RESERVED_ARTIFACT_PATH" ]; then
  reserved_runtime_artifact_present="yes"

  if [ ! -x "$VERIFY_SCRIPT" ]; then
    nsq_recode_status="failed"
    artifact_verification_status="verifier_missing"
    verification_state="failed"
    printf 'verify=false\nreason=verifier_missing\n' > "$VERIFY_LOG"
  elif "$VERIFY_SCRIPT" "$RESERVED_ARTIFACT_PATH" > "$VERIFY_LOG" 2>&1; then
    nsq_recode_status="manifest_bundle_only"
    whole_core_ready="false"
    whole_core_runtime_status="manifest_verified_not_hot_live"
    artifact_verification_status="manifest_bundle_verified"
    verification_state="manifest_only"
    nsq_artifact_state="manifest_bundle_only"
  else
    nsq_recode_status="failed"
    verification_state="failed"
    artifact_verification_status="$(awk -F= '/^reason=/{print $2}' "$VERIFY_LOG" | tail -n 1)"
    if [ -z "$artifact_verification_status" ]; then
      artifact_verification_status="failed"
    fi
  fi
else
  rm -f "$VERIFY_LOG"
fi

cat > "$PIPELINE_STATUS" <<EOF
source_ingest_status=$source_ingest_status
source_authority_lane=$source_authority_lane
source_authority_state=$source_authority_state
nsq_artifact_state=$nsq_artifact_state
runtime_authority_lane=$runtime_authority_lane
runtime_authority_state=$runtime_authority_state
runtime_authority_bound=$runtime_authority_bound
nsq_envelope_status=$nsq_envelope_status
nsq_recode_status=$nsq_recode_status
whole_core_runtime_status=$whole_core_runtime_status
verification_state=$verification_state
artifact_verification_status=$artifact_verification_status
source_required_files=$source_required_files
source_present_files=$source_present_files
source_required_shards=$source_required_shards
source_present_shards=$source_present_shards
source_materialized_shards=$source_materialized_shards
source_pointer_stub_files=$source_pointer_stub_files
source_text_stub_files=$source_text_stub_files
source_total_bytes=$source_total_bytes
source_blake3_manifest=$source_blake3_manifest
source_blake3_recorded_files=$source_blake3_recorded_files
source_blake3_status=$source_blake3_status
nsq_envelope_artifact=$ENVELOPE_PATH
reserved_runtime_artifact=$RESERVED_ARTIFACT_PATH
reserved_runtime_artifact_present=$reserved_runtime_artifact_present
EOF

cat > "$FINALIZE_STATUS" <<EOF
whole_core_ready=$whole_core_ready
source_ingest_status=$source_ingest_status
source_authority_lane=$source_authority_lane
source_authority_state=$source_authority_state
nsq_artifact_state=$nsq_artifact_state
runtime_authority_lane=$runtime_authority_lane
runtime_authority_state=$runtime_authority_state
runtime_authority_bound=$runtime_authority_bound
nsq_envelope_status=$nsq_envelope_status
nsq_recode_status=$nsq_recode_status
whole_core_runtime_status=$whole_core_runtime_status
verification_state=$verification_state
artifact_verification_status=$artifact_verification_status
source_required_files=$source_required_files
source_present_files=$source_present_files
source_required_shards=$source_required_shards
source_present_shards=$source_present_shards
source_materialized_shards=$source_materialized_shards
source_pointer_stub_files=$source_pointer_stub_files
source_text_stub_files=$source_text_stub_files
source_total_bytes=$source_total_bytes
source_blake3_manifest=$source_blake3_manifest
source_blake3_recorded_files=$source_blake3_recorded_files
source_blake3_status=$source_blake3_status
nsq_envelope_artifact=$ENVELOPE_PATH
reserved_runtime_artifact=$RESERVED_ARTIFACT_PATH
reserved_runtime_artifact_present=$reserved_runtime_artifact_present
EOF

if { [ "$whole_core_runtime_status" = "hot_live_active" ] || [ "$whole_core_runtime_status" = "hot_live_verified" ]; } && [ "$whole_core_ready" = "true" ]; then
  printf 'ready=true\nartifact=%s\nverification_state=%s\n' "$RESERVED_ARTIFACT_PATH" "$verification_state" > "$READY_MARKER"
else
  rm -f "$READY_MARKER"
fi
