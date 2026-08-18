#!/usr/bin/env bash
set -euo pipefail

usage() {
  echo "usage: $0 <payload-file-or-directory> <output-dir> [part-size-mib]" >&2
  exit 2
}
[ "$#" -ge 2 ] || usage
INPUT=$1
OUT=$2
PART_MIB=${3:-64}
case "$PART_MIB" in (*[!0-9]*|'') echo "part size must be a positive integer MiB" >&2; exit 2;; esac
[ "$PART_MIB" -gt 0 ] || { echo "part size must be positive" >&2; exit 2; }
[ -e "$INPUT" ] || { echo "payload does not exist: $INPUT" >&2; exit 1; }
command -v sha256sum >/dev/null || { echo "sha256sum is required" >&2; exit 1; }
command -v tar >/dev/null || { echo "tar is required" >&2; exit 1; }
command -v split >/dev/null || { echo "split is required" >&2; exit 1; }

ROOT=$(cd "$(dirname "$0")/../.." && pwd)
mkdir -p "$OUT"
WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT
PAYLOAD="$WORK/payload.bin"
if [ -d "$INPUT" ]; then
  tar -C "$INPUT" --sort=name --mtime='UTC 1970-01-01' --owner=0 --group=0 --numeric-owner -cf "$PAYLOAD" .
  KIND=directory_tar
else
  cp --reflink=auto -- "$INPUT" "$PAYLOAD"
  KIND=file
fi
SIZE=$(stat -c '%s' "$PAYLOAD")
DIGEST=$(sha256sum "$PAYLOAD" | awk '{print $1}')
PREFIX="$WORK/part-"
split -b "$((PART_MIB * 1024 * 1024))" -d -a 6 -- "$PAYLOAD" "$PREFIX"

rm -rf "$OUT/parts"
mkdir -p "$OUT/parts"
index=0
parts_json=''
for part in "$PREFIX"*; do
  [ -f "$part" ] || continue
  name=$(printf 'part-%06d.bin' "$index")
  cp --reflink=auto -- "$part" "$OUT/parts/$name"
  bytes=$(stat -c '%s' "$OUT/parts/$name")
  hash=$(sha256sum "$OUT/parts/$name" | awk '{print $1}')
  [ "$index" -eq 0 ] && comma='' || comma=','
  parts_json+="${comma}"$'\n'"    {\"index\":${index},\"name\":\"${name}\",\"bytes\":${bytes},\"sha256\":\"${hash}\"}"
  index=$((index + 1))
done
[ "$index" -gt 0 ] || { echo "no payload parts produced" >&2; exit 1; }
cat > "$OUT/manifest.json" <<EOF
{
  "schema": "braxon.android.split_bundle.v1",
  "payload_kind": "$KIND",
  "android_min_api": 35,
  "android_target_api": 36,
  "android_compatibility": [35, 36],
  "part_size_mib": $PART_MIB,
  "payload_bytes": $SIZE,
  "payload_sha256": "$DIGEST",
  "part_count": $index,
  "parts": [${parts_json}
  ],
  "install_policy": "reconstruct_verify_then_install",
  "nsq_role": "android_is_entrance_and_bridge; nsq_is_runtime_authority",
  "hardware_acceptance": "not_proven_until_real_device_install_and_render_test"
}
EOF
printf '%s\n' "split bundle created: $index parts, $SIZE bytes, sha256=$DIGEST"
