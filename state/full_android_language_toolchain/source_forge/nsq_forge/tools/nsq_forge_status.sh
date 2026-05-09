#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$HOME/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
SRC="$TC/source_forge"
NSQ="$SRC/nsq_forge"

source "$SRC/source_forge_env" 2>/dev/null || true
source "$NSQ/config/nsq_source_forge.env"

echo "=== NSQ forge status ==="
date
echo "source_first=$BRAXON_SOURCE_FIRST"
echo "nsq_forge=$BRAXON_NSQ_FORGE"
echo "watermark=$BRAXON_NSQ_WATERMARK"
echo "active_lever_floor=$BRAXON_NSQ_ACTIVE_LEVER_FLOOR"
echo "proven_effective_positions=$BRAXON_NSQ_PROVEN_EFFECTIVE_POSITIONS"
echo "legacy_1126_only=$BRAXON_NSQ_LEGACY_REFERENCE_1126_ONLY"
echo "not_u8=$BRAXON_NSQ_NOT_U8"
echo "not_bytes=$BRAXON_NSQ_NOT_BYTES"
echo "not_host_width_truth=$BRAXON_NSQ_NOT_HOST_WIDTH_TRUTH"
echo

echo "tool anchors:"
for x in braxon-rustc braxon-cargo braxon-python clang zig tree-sitter rg fd jq; do
  printf "%-16s " "$x"
  command -v "$x" || true
done
echo

echo "cargo packages:"
"$ROOT/braxon-cargo" metadata --no-deps --format-version 1 \
  | "$ROOT/braxon-python" -c 'import json,sys; [print(p["name"]) for p in json.load(sys.stdin)["packages"]]' \
  || true
