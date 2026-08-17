#!/usr/bin/env bash
set -euo pipefail
ROOT="${1:-.}"
OUT="${2:-reconstruction/wowas-world/build}"
mkdir -p "$OUT/segments" "$OUT/reports"
: > "$OUT/tsv_manifest.tsv"
: > "$OUT/segment_manifest.tsv"
find "$ROOT" -type f -name '*.tsv' -print0 | sort -z | while IFS= read -r -d '' f; do
  rel="${f#"$ROOT"/}"
  safe=$(printf '%s' "$rel" | tr '/: ' '___')
  lines=$(wc -l < "$f")
  printf '%s\t%s\t%s\n' "$rel" "$lines" "$(sha256sum "$f" | awk '{print $1}')" >> "$OUT/tsv_manifest.tsv"
  n=1; start=1
  while [ "$start" -le "$lines" ]; do
    end=$((start+999)); [ "$end" -gt "$lines" ] && end="$lines"
    sed -n "${start},${end}p" "$f" > "$OUT/segments/${safe}.part${n}.tsv"
    printf '%s\t%s\t%s\t%s\n' "$rel" "$n" "$start" "$end" >> "$OUT/segment_manifest.tsv"
    start=$((end+1)); n=$((n+1))
  done
done
rg -n -i --glob '*.tsv' '(chrono[ _-]?decay|aortic[ _-]?labyrinth|deprecated[ _-]?boojay|hidden[ _-]?old[ _-]?first[ _-]?book)' "$ROOT" > "$OUT/reports/contamination_matches.txt" || true
printf 'TSV corpus segmentation complete\n' > "$OUT/reports/STATUS.txt"
printf 'Every TSV discovered recursively; every file represented by contiguous sed ranges; no source TSV deleted or filtered.\n' >> "$OUT/reports/STATUS.txt"
