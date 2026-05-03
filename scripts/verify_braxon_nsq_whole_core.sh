#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ARTIFACT_PATH="${1:-}"
[ -n "$ARTIFACT_PATH" ] || { printf 'verify=false\nreason=missing_artifact_argument\n'; exit 1; }
[ -f "$ARTIFACT_PATH" ] || { printf 'verify=false\nreason=missing_artifact\n'; exit 1; }
[ -s "$ARTIFACT_PATH" ] || { printf 'verify=false\nreason=empty_artifact\n'; exit 1; }

kind="$(awk -F': ' '/^artifact_kind:/{print $2; exit}' "$ARTIFACT_PATH")"
[ "$kind" = "nsq_whole_core_runtime_bundle" ] || { printf 'verify=false\nreason=unexpected_artifact_kind\n'; exit 1; }

repr="$(awk -F': ' '/^representation_mode:/{print $2; exit}' "$ARTIFACT_PATH")"
mass="$(awk -F': ' '/^runtime_mass_profile:/{print $2; exit}' "$ARTIFACT_PATH")"
manifest="$(awk -F': ' '/^source_manifest:/{print $2; exit}' "$ARTIFACT_PATH")"
srcdir="$(awk -F': ' '/^source_ingest_directory:/{print $2; exit}' "$ARTIFACT_PATH")"
declared_files="$(awk -F': ' '/^source_present_files:/{print $2; exit}' "$ARTIFACT_PATH")"
declared_bytes="$(awk -F': ' '/^source_total_bytes:/{print $2; exit}' "$ARTIFACT_PATH")"

[ "$repr" = "stamp_bound_manifest" ] || { printf 'verify=false\nreason=unexpected_representation_mode\n'; exit 1; }
[ "$mass" = "manifest_and_stamps_only" ] || { printf 'verify=false\nreason=unexpected_runtime_mass_profile\n'; exit 1; }
[ -n "$manifest" ] && [ -f "$manifest" ] || { printf 'verify=false\nreason=missing_source_manifest\n'; exit 1; }
[ -n "$srcdir" ] && [ -d "$srcdir" ] || { printf 'verify=false\nreason=missing_source_ingest_directory\n'; exit 1; }

count=0
bytes=0
while read -r hash rel; do
  [ -n "${hash:-}" ] || continue
  [ -n "${rel:-}" ] || continue
  path="$srcdir/$rel"
  [ -f "$path" ] || { printf 'verify=false\nreason=manifest_entry_missing_on_disk\nmissing=%s\n' "$rel"; exit 1; }
  count=$((count + 1))
  size="$(wc -c < "$path" 2>/dev/null || echo 0)"
  bytes=$((bytes + size))
done < "$manifest"

[ "$count" -gt 0 ] || { printf 'verify=false\nreason=empty_manifest\n'; exit 1; }
[ "$count" = "${declared_files:-0}" ] || {
  printf 'verify=false\nreason=file_count_mismatch\ndeclared=%s\nactual=%s\n' "${declared_files:-0}" "$count"
  exit 1
}
[ "$bytes" = "${declared_bytes:-0}" ] || {
  printf 'verify=false\nreason=byte_count_mismatch\ndeclared=%s\nactual=%s\n' "${declared_bytes:-0}" "$bytes"
  exit 1
}

printf 'verify=true\nreason=manifest_bundle_verified\nfiles=%s\nbytes=%s\n' "$count" "$bytes"
