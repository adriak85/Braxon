#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT_DIR="${BRAXON_HOME:-$(cd "$(dirname "$0")/.." && pwd)}"
SOURCE_DIR="$ROOT_DIR/assets/braxon_core/source_ingest/braxon_transport"
PIPELINE_STATUS="$ROOT_DIR/state/braxon/braxon_nsq_pipeline.status"

required_files=(
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

if [ ! -d "$SOURCE_DIR" ]; then
  echo "source_dir_missing=$SOURCE_DIR"
  exit 1
fi

present=0
required_shards=0
present_shards=0
materialized_shards=0
pointer_shards=0
text_stub_shards=0
for file in "${required_files[@]}"; do
  if [ -f "$SOURCE_DIR/$file" ]; then
    present=$((present + 1))
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

legacy_leftovers=()
while IFS= read -r legacy; do
  [ -n "$legacy" ] && legacy_leftovers+=("$legacy")
done < <(find "$SOURCE_DIR" -maxdepth 1 -type f \( \
  -name 'model.safetensors-00001-of-00011.safetensors' -o \
  -name 'model.safetensors-00002-of-00011.safetensors' -o \
  -name 'model.safetensors-00003-of-00011.safetensors' -o \
  -name '*.gitattributes' \
\) -printf '%f\n' | sort)

source_status="missing"
if [ "$present" -gt 0 ]; then
  source_status="partial"
fi
if [ "$required_shards" -gt 0 ]; then
  if [ "$present_shards" -lt "$required_shards" ]; then
    source_status="materialization_incomplete_missing_shards"
  elif [ "$materialized_shards" -eq "$required_shards" ]; then
    source_status="complete"
  elif [ "$pointer_shards" -eq "$required_shards" ]; then
    source_status="catalog_complete_pointer_stubs_only"
  elif [ "$materialized_shards" -gt 0 ]; then
    source_status="partial_materialization"
  elif [ "$text_stub_shards" -gt 0 ]; then
    source_status="text_stub_invalid"
  fi
fi

blake_status="missing"
if [ -f "$SOURCE_DIR/BLAKE3SUMS" ]; then
  blake_status="present"
fi

pipeline_source_status="missing"
pipeline_blake_status="missing"
if [ -f "$PIPELINE_STATUS" ]; then
  pipeline_source_status="$(awk -F= '/^source_ingest_status=/{print $2}' "$PIPELINE_STATUS" | tail -n 1)"
  pipeline_blake_status="$(awk -F= '/^source_blake3_status=/{print $2}' "$PIPELINE_STATUS" | tail -n 1)"
fi

echo "source_dir=$SOURCE_DIR"
echo "required_files=${#required_files[@]}"
echo "present_files=$present"
echo "required_shards=$required_shards"
echo "present_shards=$present_shards"
echo "materialized_shards=$materialized_shards"
echo "pointer_shards=$pointer_shards"
echo "text_stub_shards=$text_stub_shards"
echo "source_status=$source_status"
echo "blake3_manifest=$blake_status"
echo "pipeline_source_status=$pipeline_source_status"
echo "pipeline_blake3_status=$pipeline_blake_status"
echo "legacy_leftover_count=${#legacy_leftovers[@]}"
for item in "${legacy_leftovers[@]}"; do
  echo "legacy_leftover=$item"
done
