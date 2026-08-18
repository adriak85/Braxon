#!/usr/bin/env bash
set -euo pipefail

usage() {
  echo "usage: $0 <bundle-dir> <output-payload> [--install]" >&2
  exit 2
}
[ "$#" -ge 2 ] || usage
BUNDLE=$1
OUTPUT=$2
INSTALL=0
[ "${3:-}" = "--install" ] && INSTALL=1
MANIFEST="$BUNDLE/manifest.json"
PARTS="$BUNDLE/parts"
[ -f "$MANIFEST" ] || { echo "missing manifest.json" >&2; exit 1; }
[ -d "$PARTS" ] || { echo "missing parts directory" >&2; exit 1; }
command -v sha256sum >/dev/null || { echo "sha256sum is required" >&2; exit 1; }
command -v jq >/dev/null || { echo "jq is required" >&2; exit 1; }

schema=$(jq -r '.schema' "$MANIFEST")
[ "$schema" = "braxon.android.split_bundle.v1" ] || { echo "unsupported bundle schema: $schema" >&2; exit 1; }
count=$(jq -r '.part_count' "$MANIFEST")
[ "$count" -gt 0 ] || { echo "invalid part_count" >&2; exit 1; }
expected_payload=$(jq -r '.payload_sha256' "$MANIFEST")
expected_bytes=$(jq -r '.payload_bytes' "$MANIFEST")
actual_part_files=$(find "$PARTS" -maxdepth 1 -type f -name 'part-*.bin' | wc -l)
[ "$actual_part_files" -eq "$count" ] || { echo "part file count mismatch: expected $count, found $actual_part_files" >&2; exit 1; }
while IFS= read -r unexpected; do
  name=$(basename "$unexpected")
  jq -e --arg name "$name" '.parts[] | select(.name == $name)' "$MANIFEST" >/dev/null || { echo "unexpected part file: $name" >&2; exit 1; }
done < <(find "$PARTS" -maxdepth 1 -type f -name 'part-*.bin' | sort)
rm -f "$OUTPUT"
mkdir -p "$(dirname "$OUTPUT")"

for ((i=0; i<count; i++)); do
  name=$(jq -r ".parts[$i].name" "$MANIFEST")
  declared_index=$(jq -r ".parts[$i].index" "$MANIFEST")
  declared_bytes=$(jq -r ".parts[$i].bytes" "$MANIFEST")
  declared_hash=$(jq -r ".parts[$i].sha256" "$MANIFEST")
  [ "$declared_index" -eq "$i" ] || { echo "part index mismatch at $i" >&2; exit 1; }
  part="$PARTS/$name"
  [ -f "$part" ] || { echo "missing part: $name" >&2; exit 1; }
  actual_bytes=$(stat -c '%s' "$part")
  [ "$actual_bytes" -eq "$declared_bytes" ] || { echo "byte mismatch: $name" >&2; exit 1; }
  actual_hash=$(sha256sum "$part" | awk '{print $1}')
  [ "$actual_hash" = "$declared_hash" ] || { echo "hash mismatch: $name" >&2; exit 1; }
  cat "$part" >> "$OUTPUT"
done

actual_bytes=$(stat -c '%s' "$OUTPUT")
[ "$actual_bytes" -eq "$expected_bytes" ] || { echo "payload byte mismatch" >&2; rm -f "$OUTPUT"; exit 1; }
actual_hash=$(sha256sum "$OUTPUT" | awk '{print $1}')
[ "$actual_hash" = "$expected_payload" ] || { echo "payload hash mismatch" >&2; rm -f "$OUTPUT"; exit 1; }

if [ "$INSTALL" -eq 1 ]; then
  case "$OUTPUT" in
    *.apk)
      command -v adb >/dev/null || { echo "adb unavailable; verified APK prepared but not installed" >&2; exit 3; }
      adb install -r -- "$OUTPUT"
      ;;
    *.aab)
      echo "AAB verified; direct adb installation is unsupported. Use bundletool or Play distribution." >&2
      exit 3
      ;;
    *)
      echo "verified payload is not an APK/AAB; no installation attempted" >&2
      exit 3
      ;;
  esac
fi
printf '%s\n' "verified payload: $OUTPUT ($actual_bytes bytes, sha256=$actual_hash)"
