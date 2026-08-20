#!/usr/bin/env bash
set -euo pipefail

ROOT="${1:-.}"
CANON="$ROOT/crates/wowas-final-edition-v10/canon"
ACTIVE="$CANON/active"
TREE="$CANON/canonical_story_tree"
WORK="$ROOT/reconstruction/wowas-world/build/reconcile"
mkdir -p "$WORK"

SPINE="$ACTIVE/book_spine_33.tsv"
BLOCK="$ACTIVE/canon_blocklist.tsv"
SCENES="$CANON/wowas_clean_scene_index_v2.tsv"
TREE_INDEX="$TREE/_scene_heading_index.tsv"

for f in "$SPINE" "$BLOCK" "$SCENES" "$TREE_INDEX"; do test -f "$f" || { echo "MISSING $f"; exit 2; }; done

# Work only on active canonical representations. Legacy/history material is never
# rewritten by this pass. Large files are transformed through bounded sed windows.
rewrite_bounded() {
  local src="$1" dst="$2" lines start end chunk
  lines=$(wc -l < "$src")
  : > "$dst"
  start=1
  while [ "$start" -le "$lines" ]; do
    end=$((start + 499))
    [ "$end" -gt "$lines" ] && end="$lines"
    chunk=$(sed -n "${start},${end}p" "$src")
    # Proven, canonical renames only. Do not erase a row merely because it
    # contains historical language; unresolved source remains auditable.
    chunk=$(printf '%s\n' "$chunk" | sed \
      -e 's/The Fifteenth Birthday/Choices Make World/g' \
      -e 's/Boojay/Rylos Vayne Johnson/g')
    printf '%s\n' "$chunk" >> "$dst"
    start=$((end + 1))
  done
}

rewrite_bounded "$SCENES" "$WORK/wowas_clean_scene_index_v2.tsv"
rewrite_bounded "$TREE_INDEX" "$WORK/_scene_heading_index.tsv"

# Reject forbidden causal concepts from active output. This is deliberately a
# gate, not a destructive rewrite: any remaining occurrence must be resolved
# by a narrative replacement pass before the result can be promoted.
for forbidden in 'Chrono Decay' 'Chrono Coral' 'Chrono Chasm' 'Heart Chronometer' 'Aortic Labyrinth'; do
  if grep -Fqi "$forbidden" "$WORK/wowas_clean_scene_index_v2.tsv"; then
    echo "UNRESOLVED_DEPRECATED_ACTIVE_SCENE: $forbidden" >&2
    exit 10
  fi
done

# Hard spine/character locks.
grep -Fq $'1\t1\tB01\tChoices Make World' "$SPINE"
grep -Fq $'12\t2\tB12\tStasis' "$SPINE"
grep -Fq $'25\t3\tB25\tDeath Is Rebirth' "$SPINE"
grep -Fq $'33\t3\tB33\tWillow and Stone' "$SPINE"

# Blood Cello may culminate only in B25 active scene material.
if grep -Fqi 'Blood Cello' "$WORK/wowas_clean_scene_index_v2.tsv"; then
  if grep -Fi 'Blood Cello' "$WORK/wowas_clean_scene_index_v2.tsv" | grep -v $'B25\t' >/dev/null; then
    echo 'BLOOD_CELLO_OUTSIDE_B25' >&2
    exit 11
  fi
fi

# Character identity locks in active scene material.
for lock in 'Daisy May' 'Majiskii' 'Xethrolund' 'Mack' 'Rolzen' 'Rylos Vayne Johnson'; do
  grep -Fqi "$lock" "$WORK/wowas_clean_scene_index_v2.tsv" || {
    echo "MISSING_REQUIRED_CHARACTER: $lock" >&2; exit 12;
  }
done

# Promote only after every deterministic gate passes.
cp "$WORK/wowas_clean_scene_index_v2.tsv" "$SCENES"
cp "$WORK/_scene_heading_index.tsv" "$TREE_INDEX"

echo 'PROMOTED_CANONICAL_RECONCILIATION'
