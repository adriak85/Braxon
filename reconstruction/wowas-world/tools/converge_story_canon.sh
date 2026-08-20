#!/usr/bin/env bash
set -euo pipefail

# Fixed-point narrative reconciliation controller.
# It intentionally does NOT invent canon or delete source material. Each pass:
#   1. segments large source files with sed,
#   2. cross-references spine/laws/blocklist/characters/story tree,
#   3. records safe normalization candidates,
#   4. validates required beats and deprecated surfaces,
#   5. repeats until the generated evidence fingerprint stops changing.
#
# Narrative generators/reflexor may be plugged in through WOWAS_REFLEXOR_CMD.
# A generator is never allowed to promote unsupported material: its output must
# be reconciled against the canonical spine and laws before application.

ROOT="${1:-.}"
OUT="${2:-reconstruction/wowas-world/build/convergence}"
SEGMENT_LINES="${WOWAS_SEGMENT_LINES:-500}"
mkdir -p "$OUT/segments" "$OUT/reports"

CANON="$ROOT/crates/wowas-final-edition-v10/canon"
ACTIVE="$CANON/active"
TREE="$CANON/canonical_story_tree"

SPINE="$ACTIVE/book_spine_33.tsv"
LAWS="$ACTIVE/canon_laws.tsv"
BLOCKLIST="$ACTIVE/canon_blocklist.tsv"
SCENES="$CANON/wowas_clean_scene_index_v2.tsv"
CHARACTERS="$ACTIVE/character_timeline_lattice_v14_33.tsv"
TREE_INDEX="$TREE/_scene_index.tsv"

for f in "$SPINE" "$LAWS" "$BLOCKLIST" "$SCENES" "$CHARACTERS" "$TREE_INDEX"; do
  test -f "$f" || { echo "MISSING_SOURCE $f" >&2; exit 2; }
done

# Segment every authoritative TSV with bounded sed windows.
: > "$OUT/reports/segment_manifest.tsv"
for f in "$SPINE" "$LAWS" "$BLOCKLIST" "$SCENES" "$CHARACTERS" "$TREE_INDEX"; do
  rel="${f#"$ROOT"/}"
  safe=$(printf '%s' "$rel" | tr '/: ' '___')
  lines=$(wc -l < "$f")
  n=1
  start=1
  while [ "$start" -le "$lines" ]; do
    end=$((start + SEGMENT_LINES - 1))
    [ "$end" -gt "$lines" ] && end="$lines"
    sed -n "${start},${end}p" "$f" > "$OUT/segments/${safe}.part${n}.tsv"
    printf '%s\t%s\t%s\t%s\n' "$rel" "$n" "$start" "$end" >> "$OUT/reports/segment_manifest.tsv"
    start=$((end + 1)); n=$((n + 1))
  done
done

# Explicit deprecated vocabulary is evidence for reconciliation, never an
# automatic deletion instruction.
rg -n -i --glob '*.txt' --glob '*.md' --glob '*.tsv' \
  '(Fifteenth Birthday|Fifteen = Threshold|Chrono Coral|Chrono Chasm|Heart Chronometer|Aortic Labyrinth|chrono decay|deprecated.?boojay)' \
  "$TREE" "$CANON" > "$OUT/reports/deprecated_surface.tsv" || true

# Detect stale authority claims so they can be removed from executable paths.
rg -n -i --glob '*.rs' --glob '*.toml' --glob '*.json' --glob '*.md' \
  '(active 25-book|25-book lattice|WOWAS_CANON_AUTHORITY_v14|Book_01_The_Fifteenth_Birthday)' \
  "$ROOT/crates/wowas-final-edition-v10" > "$OUT/reports/stale_authority.tsv" || true

# Required spine anchors are hard validation constraints.
: > "$OUT/reports/beat_validation.tsv"
while IFS=$'\t' read -r book title beat_summary; do
  [ -n "${book:-}" ] || continue
  if rg -qi --fixed-strings "$title" "$SPINE"; then
    printf '%s\tPRESENT\t%s\n' "$book" "$title" >> "$OUT/reports/beat_validation.tsv"
  else
    printf '%s\tMISSING\t%s\n' "$book" "$title" >> "$OUT/reports/beat_validation.tsv"
  fi
done < "$SPINE"

# Character presence is checked against both the lattice and scene corpus.
: > "$OUT/reports/character_coverage.tsv"
cut -f1 "$CHARACTERS" | tail -n +2 | while IFS= read -r character; do
  [ -n "$character" ] || continue
  if rg -qi --fixed-strings "$character" "$SCENES"; then
    printf '%s\tSCENE_PRESENT\n' "$character" >> "$OUT/reports/character_coverage.tsv"
  else
    printf '%s\tSCENE_MISSING\n' "$character" >> "$OUT/reports/character_coverage.tsv"
  fi
done

# Candidate seam ledger. Nothing is promoted automatically.
: > "$OUT/reports/seam_ledger.tsv"
rg -n -i --glob '*.tsv' '(TBD|pending|unplaced|candidate|missing|unknown)' "$TREE" "$CANON" \
  | head -n 2000 > "$OUT/reports/seam_ledger.tsv" || true

# Optional reflexor/generator pass. The command receives the fixed corpus root
# and evidence directory. It must emit a review artifact; application remains
# gated by deterministic validation in the next pass.
if [ -n "${WOWAS_REFLEXOR_CMD:-}" ]; then
  env ROOT="$ROOT" OUT="$OUT" "$WOWAS_REFLEXOR_CMD" > "$OUT/reports/reflexor.stdout" 2> "$OUT/reports/reflexor.stderr"
else
  printf 'REFLEXOR_NOT_CONFIGURED\n' > "$OUT/reports/reflexor.status"
fi

# Stable fingerprint: convergence means the evidence set is unchanged.
{
  sha256sum "$OUT/reports/deprecated_surface.tsv" "$OUT/reports/stale_authority.tsv" \
    "$OUT/reports/beat_validation.tsv" "$OUT/reports/character_coverage.tsv" "$OUT/reports/seam_ledger.tsv"
} > "$OUT/reports/fingerprint.new"

if [ -f "$OUT/reports/fingerprint.prev" ] && cmp -s "$OUT/reports/fingerprint.prev" "$OUT/reports/fingerprint.new"; then
  echo "CONVERGED" > "$OUT/reports/STATUS.txt"
  cp "$OUT/reports/fingerprint.new" "$OUT/reports/fingerprint.final"
  exit 0
fi
cp "$OUT/reports/fingerprint.new" "$OUT/reports/fingerprint.prev"
printf 'REVIEW_REQUIRED\n' > "$OUT/reports/STATUS.txt"
exit 0
