#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT_DIR="${BRAXON_HOME:-$(cd "$(dirname "$0")/.." && pwd)}"
SOURCE_DIR="$ROOT_DIR/assets/braxon_core/source_ingest/braxon_transport"
TARGET_DIR="$ROOT_DIR/assets/braxon_core/weights/nsq"
STATE_DIR="$ROOT_DIR/state/braxon"
RESERVED_ARTIFACT_PATH="$TARGET_DIR/Braxon-27B_extended.nsqb"
ENVELOPE_PATH="$TARGET_DIR/Braxon-27B_extended.nsqb.meta"
PIPELINE_STATUS="$STATE_DIR/braxon_nsq_pipeline.status"
FINALIZE_STATUS="$STATE_DIR/BRAXON_whole_core_finalize.status"
BLAKE3_MANIFEST="$SOURCE_DIR/BLAKE3SUMS"

REQUIRED_FILES=(
  README.md
  added_tokens.json
  chat_template.jinja
  chat_template.json
  config.json
  generation_config.json
  merges.txt
  model-00001-of-00014.safetensors
  model-00002-of-00014.safetensors
  model-00003-of-00014.safetensors
  model-00004-of-00014.safetensors
  model-00005-of-00014.safetensors
  model-00006-of-00014.safetensors
  model-00007-of-00014.safetensors
  model-00008-of-00014.safetensors
  model-00009-of-00014.safetensors
  model-00010-of-00014.safetensors
  model-00011-of-00014.safetensors
  model-00012-of-00014.safetensors
  model-00013-of-00014.safetensors
  model-00014-of-00014.safetensors
  model.safetensors.index.json
  preprocessor_config.json
  special_tokens_map.json
  tokenizer.json
  tokenizer_config.json
  video_preprocessor_config.json
  vocab.json
)

is_pointer_stub() {
  local file="$1"
  [ -f "$file" ] || return 1
  head -c 512 "$file" 2>/dev/null | grep -aEq \
    'version https://git-lfs.github.com/spec/v1|oid sha256:'
}

is_text_stub() {
  local file="$1"
  [ -f "$file" ] || return 1
  local head_len printable
  head_len="$(head -c 512 "$file" 2>/dev/null | wc -c | awk '{print $1}')"
  [ "${head_len:-0}" -gt 0 ] || return 1
  printable="$(
    head -c 512 "$file" 2>/dev/null \
      | LC_ALL=C tr -cd '\11\12\15\40-\176' \
      | wc -c \
      | awk '{print $1}'
  )"
  [ $(( printable * 100 / head_len )) -ge 95 ]
}

state_value() {
  local path="$1"
  local key="$2"
  local fallback="$3"
  local value=""

  if [ -f "$path" ]; then
    value="$(awk -F= -v key="$key" '$1 == key { print $2 }' "$path" | tail -n 1)"
  fi

  if [ -n "$value" ]; then
    printf '%s\n' "$value"
  else
    printf '%s\n' "$fallback"
  fi
}

mkdir -p "$TARGET_DIR" "$STATE_DIR"

# Migrate any historical envelope accidentally written to the reserved whole-core path.
if [ -f "$RESERVED_ARTIFACT_PATH" ] && grep -Eq '^artifact_kind: nsq_(whole_core_envelope|source_ingest_envelope)$' "$RESERVED_ARTIFACT_PATH"; then
  mv -f "$RESERVED_ARTIFACT_PATH" "$ENVELOPE_PATH"
fi

present_count=0
required_shards=0
present_shards=0
materialized_shards=0
pointer_shards=0
text_stub_shards=0
blake3_recorded_count=0
source_total_bytes=0
for file in "${REQUIRED_FILES[@]}"; do
  if [ -f "$SOURCE_DIR/$file" ]; then
    present_count=$((present_count + 1))
    file_size="$(wc -c "$SOURCE_DIR/$file" | awk '{print $1}')"
    source_total_bytes=$((source_total_bytes + file_size))
    if [ -f "$BLAKE3_MANIFEST" ] && grep -Fq "  $file" "$BLAKE3_MANIFEST"; then
      blake3_recorded_count=$((blake3_recorded_count + 1))
    fi
  fi
  case "$file" in
    model-*.safetensors)
      required_shards=$((required_shards + 1))
      if [ -f "$SOURCE_DIR/$file" ]; then
        present_shards=$((present_shards + 1))
        if is_pointer_stub "$SOURCE_DIR/$file"; then
          pointer_shards=$((pointer_shards + 1))
        elif is_text_stub "$SOURCE_DIR/$file"; then
          text_stub_shards=$((text_stub_shards + 1))
        else
          materialized_shards=$((materialized_shards + 1))
        fi
      fi
      ;;
  esac
done

source_ingest_status="missing"
if [ "$present_count" -gt 0 ]; then
  source_ingest_status="partial"
fi
if [ "$required_shards" -gt 0 ]; then
  if [ "$present_shards" -lt "$required_shards" ]; then
    source_ingest_status="materialization_incomplete_missing_shards"
  elif [ "$materialized_shards" -eq "$required_shards" ]; then
    source_ingest_status="complete"
  elif [ "$pointer_shards" -eq "$required_shards" ]; then
    source_ingest_status="catalog_complete_pointer_stubs_only"
  elif [ "$materialized_shards" -gt 0 ]; then
    source_ingest_status="partial_materialization"
  elif [ "$text_stub_shards" -gt 0 ]; then
    source_ingest_status="text_stub_invalid"
  fi
fi

nsq_envelope_status="seeded"
if [ -f "$ENVELOPE_PATH" ]; then
  nsq_envelope_status="updated"
fi

source_blake3_status="missing"
if [ "$blake3_recorded_count" -gt 0 ]; then
  source_blake3_status="partial"
fi
if [ "$present_count" -gt 0 ] && [ "$blake3_recorded_count" -eq "$present_count" ]; then
  source_blake3_status="verified"
fi

reserved_runtime_artifact_present="no"
if [ -f "$RESERVED_ARTIFACT_PATH" ]; then
  reserved_runtime_artifact_present="yes"
fi

source_authority_lane="$SOURCE_DIR"
source_authority_state="$source_ingest_status"
nsq_artifact_state="absent_checkout"
if [ "$reserved_runtime_artifact_present" = "yes" ]; then
  if head -c 512 "$RESERVED_ARTIFACT_PATH" 2>/dev/null | grep -aEq 'version https://git-lfs.github.com/spec/v1|oid sha256:'; then
    nsq_artifact_state="pointer_mask_invalid"
  elif grep -Eq '^runtime_mass_profile: manifest_and_stamps_only$' "$RESERVED_ARTIFACT_PATH"; then
    nsq_artifact_state="manifest_bundle_only"
  else
    nsq_artifact_state="artifact_present_unbound"
  fi
fi
runtime_authority_lane="none_bound"
runtime_authority_state="unbound"
runtime_authority_bound="false"

nsq_recode_status="$(state_value "$FINALIZE_STATUS" "nsq_recode_status" "not_started")"
whole_core_runtime_status="$(state_value "$FINALIZE_STATUS" "whole_core_runtime_status" "not_ready")"
artifact_verification_status="$(state_value "$FINALIZE_STATUS" "artifact_verification_status" "not_run")"
verification_state="$(state_value "$FINALIZE_STATUS" "verification_state" "not_run")"

if [ "$reserved_runtime_artifact_present" = "no" ]; then
  nsq_recode_status="not_started"
  whole_core_runtime_status="not_ready"
  artifact_verification_status="not_run"
  verification_state="not_run"
fi

cat > "$ENVELOPE_PATH" <<EOF
artifact_kind: nsq_source_ingest_envelope
artifact_name: Braxon-27B_extended.nsqb.meta
reserved_runtime_artifact: $RESERVED_ARTIFACT_PATH
reserved_runtime_artifact_present: $reserved_runtime_artifact_present
hot_live_parameter_embodiment: false
delta_expansion_state: not_implemented
tokenizer_runtime_unification: not_proven
model_label: BRAXON
launch_form: hot_whole_core
runtime_load_policy: whole_core_only
zlm_binding_mode: whole_core_session_surface
grid_26d_mode: sealed_reference_structure
grid_26d_activation_mode: semantic_score_alignment
supermodel_extension_mode: sealed_reference_structure
supermodel_extension_activation_mode: semantic_score_alignment
delta_extension_mode: sealed_reference_structure
delta_extension_activation_mode: semantic_score_alignment
live_grid_loading: false
live_delta_loading: false
source_ingest_directory: $SOURCE_DIR
source_required_files: ${#REQUIRED_FILES[@]}
source_present_files: $present_count
source_total_bytes: $source_total_bytes
source_blake3_manifest: $BLAKE3_MANIFEST
source_blake3_recorded_files: $blake3_recorded_count
source_blake3_status: $source_blake3_status
artifact_verification_status: $artifact_verification_status
verification_state: $verification_state
source_ingest_status: $source_ingest_status
source_authority_lane: $source_authority_lane
source_authority_state: $source_authority_state
nsq_artifact_state: $nsq_artifact_state
runtime_authority_lane: $runtime_authority_lane
runtime_authority_state: $runtime_authority_state
runtime_authority_bound: $runtime_authority_bound
source_required_shards: $required_shards
source_present_shards: $present_shards
source_materialized_shards: $materialized_shards
source_pointer_stub_files: $pointer_shards
source_text_stub_files: $text_stub_shards
nsq_envelope_status: $nsq_envelope_status
nsq_recode_status: $nsq_recode_status
whole_core_runtime_status: $whole_core_runtime_status
generated_at_utc: $(date -u +"%Y-%m-%dT%H:%M:%SZ")
EOF

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
artifact_verification_status=$artifact_verification_status
verification_state=$verification_state
source_required_files=${#REQUIRED_FILES[@]}
source_present_files=$present_count
source_required_shards=$required_shards
source_present_shards=$present_shards
source_materialized_shards=$materialized_shards
source_pointer_stub_files=$pointer_shards
source_text_stub_files=$text_stub_shards
source_total_bytes=$source_total_bytes
source_blake3_manifest=$BLAKE3_MANIFEST
source_blake3_recorded_files=$blake3_recorded_count
source_blake3_status=$source_blake3_status
nsq_envelope_artifact=$ENVELOPE_PATH
reserved_runtime_artifact=$RESERVED_ARTIFACT_PATH
reserved_runtime_artifact_present=$reserved_runtime_artifact_present
EOF

printf 'artifact=%s\n' "$ENVELOPE_PATH"
printf 'source_present_files=%s\n' "$present_count"
printf 'source_ingest_status=%s\n' "$source_ingest_status"
printf 'source_authority_state=%s\n' "$source_authority_state"
printf 'nsq_artifact_state=%s\n' "$nsq_artifact_state"
printf 'nsq_envelope_status=%s\n' "$nsq_envelope_status"
printf 'nsq_recode_status=%s\n' "$nsq_recode_status"
printf 'whole_core_runtime_status=%s\n' "$whole_core_runtime_status"
printf 'artifact_verification_status=%s\n' "$artifact_verification_status"
printf 'source_blake3_recorded_files=%s\n' "$blake3_recorded_count"
printf 'source_blake3_status=%s\n' "$source_blake3_status"
