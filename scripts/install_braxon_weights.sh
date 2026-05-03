#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT_DIR="${BRAXON_HOME:-$(cd "$(dirname "$0")/.." && pwd)}"
SOURCE_INGEST_DIR="$ROOT_DIR/assets/braxon_core/source_ingest/braxon_transport"
BASE_URL="https://huggingface.co/huihui-ai/Huihui-Qwen3.5-27B-abliterated/resolve/main"
ARIA2_BIN="${ARIA2_BIN:-$(command -v aria2c 2>/dev/null || true)}"
B3SUM_BIN="${B3SUM_BIN:-$(command -v b3sum 2>/dev/null || true)}"
WGET_BIN="${WGET_BIN:-$(command -v wget 2>/dev/null || true)}"
BLAKE3_MANIFEST="$SOURCE_INGEST_DIR/BLAKE3SUMS"

mkdir -p "$SOURCE_INGEST_DIR"
cd "$SOURCE_INGEST_DIR"

if [ -z "$ARIA2_BIN" ] && [ -z "$WGET_BIN" ]; then
  echo "missing aria2c and wget binaries" >&2
  exit 127
fi

if [ -z "$B3SUM_BIN" ]; then
  echo "missing b3sum binary" >&2
  exit 127
fi

# Provider-format files are source ingress transport only.
# This script does not produce the native NSQ whole-core artifact.
# Current envelope/status output is reserved to:
#   assets/braxon_core/weights/nsq/Braxon-27B_extended.nsqb.meta
# Final runtime launch remains reserved for the future real recoded artifact:
#   assets/braxon_core/weights/nsq/Braxon-27B_extended.nsqb

ARIA2_FLAGS=(
  --continue=true
  --allow-overwrite=true
  --auto-file-renaming=false
  --file-allocation=none
  --max-tries=0
  --retry-wait=5
  --timeout=30
  --connect-timeout=30
  --summary-interval=0
  --console-log-level=warn
  --download-result=hide
  --max-connection-per-server=4
  --split=4
)

WGET_FLAGS=(
  -c
  --progress=dot:giga
  --retry-connrefused
  --waitretry=5
  --read-timeout=30
  --timeout=30
  --tries=0
)

FILES=(
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

requires_materialized_payload() {
  case "$1" in
    *.safetensors) return 0 ;;
    *) return 1 ;;
  esac
}

is_lfs_pointer_stub() {
  local file="$1"
  [ -f "$file" ] || return 1
  head -c 512 "$file" 2>/dev/null | grep -aEq \
    'version https://git-lfs.github.com/spec/v1|oid sha256:'
}

ensure_materialized_payload() {
  local file="$1"
  if requires_materialized_payload "$file" && is_lfs_pointer_stub "$file"; then
    echo "pointer_stub_invalid:$file" >&2
    rm -f "$file" "$file.aria2"
    return 1
  fi
  return 0
}

manifest_entry_for() {
  local file="$1"
  if [ -f "$BLAKE3_MANIFEST" ]; then
    grep -F "  $file" "$BLAKE3_MANIFEST" | tail -n 1 || true
  fi
}

has_recorded_blake3() {
  local file="$1"
  [ -n "$(manifest_entry_for "$file")" ]
}

record_blake3() {
  local file="$1"
  local temp_manifest="$BLAKE3_MANIFEST.tmp"
  if [ -f "$BLAKE3_MANIFEST" ]; then
    grep -v "  $file$" "$BLAKE3_MANIFEST" > "$temp_manifest" || true
  else
    : > "$temp_manifest"
  fi
  "$B3SUM_BIN" "$file" >> "$temp_manifest"
  mv "$temp_manifest" "$BLAKE3_MANIFEST"
}

verify_recorded_blake3() {
  local file="$1"
  local recorded_line recorded_hash actual_hash
  recorded_line="$(manifest_entry_for "$file")"
  if [ -z "$recorded_line" ]; then
    return 1
  fi
  recorded_hash="${recorded_line%% *}"
  actual_hash="$("$B3SUM_BIN" "$file" | awk '{print $1}')"
  [ "$recorded_hash" = "$actual_hash" ]
}

for file in "${FILES[@]}"; do
  if [ -f "$file" ] && ! ensure_materialized_payload "$file"; then
    echo "removing_pointer_stub:$file"
  fi

  if [ -f "$file" ] && has_recorded_blake3 "$file" && ! verify_recorded_blake3 "$file"; then
    echo "blake3_mismatch:$file"
    rm -f "$file" "$file.aria2"
  elif [ -f "$file" ] && ! has_recorded_blake3 "$file"; then
    if ! ensure_materialized_payload "$file"; then
      echo "redownload_required:$file"
    else
      record_blake3 "$file"
      echo "blake3_recorded:$file:$("$B3SUM_BIN" "$file" | awk '{print $1}')"
      continue
    fi
  fi

  recorded_blake3=""
  if [ -f "$BLAKE3_MANIFEST" ] && [ -f "$file" ] && [ -n "${B3SUM_BIN:-}" ]; then
    recorded_blake3="$(grep -F "  $file" "$BLAKE3_MANIFEST" | tail -n 1 | awk '{print $1}' || true)"
    if [ -n "$recorded_blake3" ]; then
      actual_blake3="$("$B3SUM_BIN" "$file" | awk '{print $1}')"
      if [ "$actual_blake3" = "$recorded_blake3" ]; then
        if ensure_materialized_payload "$file"; then
          echo "blake3_verified:$file:$actual_blake3"
          continue
        fi
      else
        echo "blake3_mismatch_refetch:$file:$actual_blake3:$recorded_blake3"
        rm -f "$file" "$file.aria2"
      fi
    fi
  fi
  echo "downloading:$file"
  if [ -n "$ARIA2_BIN" ]; then
    "$ARIA2_BIN" "${ARIA2_FLAGS[@]}" --out="$file" "$BASE_URL/$file?download=1"
  else
    "$WGET_BIN" "${WGET_FLAGS[@]}" -O "$file" "$BASE_URL/$file?download=1"
  fi

  ensure_materialized_payload "$file"
  record_blake3 "$file"
  echo "blake3_verified:$file:$("$B3SUM_BIN" "$file" | awk '{print $1}')"
done
