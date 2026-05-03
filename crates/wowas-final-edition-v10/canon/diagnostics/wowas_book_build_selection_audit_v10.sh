#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon/wowas}"
cd "$ROOT" || exit 1

OUT="diagnostics/book_build_selection_audit_$(date +%Y%m%d_%H%M%S).txt"

{
  echo "WoWaS book build selection audit"
  echo "root=$ROOT"
  echo

  echo "== B24 probable over-selected shorthand families =="
  rg -n "^24[[:space:]].*(rewritten_beat_end|rewritten_from_detail|rewritten_pressure_pattern|The-Book-Finds-Its-|Nearness = Terminus|Presence ≠ Solved|Ghost = Obligation)" \
    wowas_clean_scene_index_v2.tsv | head -n 160 || true

  echo
  echo "== B24 strongest concrete anchors =="
  rg -n "^24[[:space:]].*(DIRECT_SOURCE[[:space:]]+PLACED_FILE|DIRECT_SOURCE[[:space:]]+SCENE_EXPANSION_EXTRACT.*(Neiths Full Revelation|Rylos And Pip Final Reckoning|Neith Delivers Mortal Wound|Pip Learns Superposition|The Impossible Done)|ACTUAL_SOURCE[[:space:]]+M)" \
    wowas_clean_scene_index_v2.tsv | head -n 120 || true

  echo
  echo "== B25 probable shorthand families =="
  rg -n "^25[[:space:]].*(rewritten_|The-Book-Finds-Its-|Farewell = Instrument|Waking = Proof|Rebirth = Obligation|What Returned Could Not Return Unchanged)" \
    wowas_clean_scene_index_v2.tsv | head -n 160 || true

  echo
  echo "== B25 strongest concrete anchors =="
  rg -n "^25[[:space:]].*(DIRECT_SOURCE[[:space:]]+PLACED_FILE|DIRECT_SOURCE[[:space:]]+SCENE_EXPANSION_EXTRACT|ACTUAL_SOURCE[[:space:]]+M)" \
    wowas_clean_scene_index_v2.tsv | head -n 120 || true

  echo
  echo "== B01 likely reconstruction flood =="
  rg -n "^1[[:space:]].*(SOURCE_DERIVED_RECONSTRUCTION|Reconstructed continuity scene|Filled to target scene count using uploaded source anchors)" \
    wowas_clean_scene_index_v2.tsv | head -n 120 || true

  echo
  echo "== B01 strongest concrete anchors =="
  rg -n "^1[[:space:]].*(DIRECT_SOURCE[[:space:]]+PLACED_FILE|DIRECT_SOURCE[[:space:]]+SCENE_EXPANSION_EXTRACT|ACTUAL_SOURCE[[:space:]]+M|ACTUAL_SOURCE[[:space:]]+S)" \
    wowas_clean_scene_index_v2.tsv | head -n 120 || true

  echo
  echo "== current apply order tail =="
  tail -n 100 CURRENT_APPLY_ORDER_v10.md || true
} > "$OUT"

echo "$OUT"
