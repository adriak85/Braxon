#!/usr/bin/env bash
set -euo pipefail
ROOT="${1:-.}"
OUT="${2:-reconstruction/wowas-world/build/finalization}"
CANON="$ROOT/crates/wowas-final-edition-v10/canon"
ACTIVE="$CANON/active"
TREE="$CANON/canonical_story_tree"
SPINE="$ACTIVE/book_spine_33.tsv"
MANIFEST="$ACTIVE/novel_manifest_33.tsv"
SCENES="$CANON/wowas_clean_scene_index_v2.tsv"
LAWS="$ACTIVE/canon_laws.tsv"
BLOCK="$ACTIVE/canon_blocklist.tsv"

for f in "$SPINE" "$MANIFEST" "$SCENES" "$LAWS" "$BLOCK"; do test -s "$f" || exit 1; done

books=$(awk -F'\t' 'NF && $1 ~ /^[0-9]+$/ {print $1}' "$SPINE" | sort -n | uniq | wc -l)
[ "$books" -eq 33 ] || exit 1

# Finalization cannot be asserted while any canonical book is explicitly
# contract-only or lacks verified prose. This prevents a structural manifest
# from being mistaken for a finished manuscript.
awk -F'\t' 'NR>1 && ($1 ~ /^[0-9]+$/) && ($4 != "true" || $6 != "prose_validated") {bad=1; print > "/dev/stderr"} END {exit bad}' "$MANIFEST"

for term in 'Chrono Coral' 'Chrono Chasm' 'Heart Chronometer' 'Aortic Labyrinth' 'Fifteenth Birthday'; do
  if grep -RniF --exclude-dir=legacy --exclude-dir=quarantine "$term" "$TREE" "$ACTIVE" >/dev/null 2>&1; then exit 1; fi
done

for term in 'Choices Make World' 'Stasis' 'Death Is Rebirth' 'Willow and Stone' 'Daisy May' 'Majiskii' 'Xethrolund' 'Mack' 'Rolzen' 'Rylos Vayne Johnson'; do
  grep -RqiF --exclude-dir=legacy --exclude-dir=quarantine "$term" "$TREE" "$ACTIVE" || exit 1
done

if grep -RniF --exclude-dir=legacy --exclude-dir=quarantine 'Boojay' "$TREE" "$ACTIVE" >/dev/null 2>&1; then exit 1; fi

for n in $(seq -w 1 33); do
  find "$TREE/books" -maxdepth 1 -type d -iname "Book_${n}_*" -print -quit | grep -q . || exit 1
done

mkdir -p "$OUT"
{
  echo 'WOWAS_FINAL_VALIDATION=PASS'
  echo "BOOK_COUNT=$books"
  git -C "$ROOT" rev-parse HEAD
} > "$OUT/final_validation.txt"
exit 0
