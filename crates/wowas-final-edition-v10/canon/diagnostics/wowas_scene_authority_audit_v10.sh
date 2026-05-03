#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon/wowas}"
cd "$ROOT" || exit 1

OUT="diagnostics/scene_authority_audit_$(date +%Y%m%d_%H%M%S).txt"

{
  echo "WoWaS scene authority audit"
  echo "root=$ROOT"
  echo

  echo "== raw counts by key patterns =="
  printf "SOURCE_DERIVED_RECONSTRUCTION\t"
  rg -c "SOURCE_DERIVED_RECONSTRUCTION" wowas_clean_scene_index_v2.tsv || true
  printf "Reconstructed continuity scene\t"
  rg -c "Reconstructed continuity scene" wowas_clean_scene_index_v2.tsv || true
  printf "Filled to target scene count using uploaded source anchors\t"
  rg -c "Filled to target scene count using uploaded source anchors" wowas_clean_scene_index_v2.tsv || true
  printf "rewritten_beat_end\t"
  rg -c "rewritten_beat_end" wowas_clean_scene_index_v2.tsv || true
  printf "rewritten_from_detail\t"
  rg -c "rewritten_from_detail" wowas_clean_scene_index_v2.tsv || true
  printf "rewritten_book_open\t"
  rg -c "rewritten_book_open" wowas_clean_scene_index_v2.tsv || true
  printf "rewritten_pressure_pattern\t"
  rg -c "rewritten_pressure_pattern" wowas_clean_scene_index_v2.tsv || true
  printf "The-Book-Finds-Its-\t"
  rg -c "The-Book-Finds-Its-" wowas_clean_scene_index_v2.tsv || true
  printf "tone and structure lock\t"
  rg -c "tone and structure lock|Tone and structure lock" wowas_clean_scene_index_v2.tsv || true
  printf "DIRECT_SOURCE.*PLACED_FILE\t"
  rg -c "DIRECT_SOURCE[[:space:]]+PLACED_FILE" wowas_clean_scene_index_v2.tsv || true
  printf "DIRECT_SOURCE.*SCENE_EXPANSION_EXTRACT\t"
  rg -c "DIRECT_SOURCE[[:space:]]+SCENE_EXPANSION_EXTRACT" wowas_clean_scene_index_v2.tsv || true
  printf "ACTUAL_SOURCE\t"
  rg -c "ACTUAL_SOURCE" wowas_clean_scene_index_v2.tsv || true
  printf "COMPILECAT\t"
  rg -c "COMPILECAT" wowas_clean_scene_index_v2.tsv || true

  echo
  echo "== B01 sample of likely scaffold debt =="
  rg -n "^1[[:space:]].*(SOURCE_DERIVED_RECONSTRUCTION|Reconstructed continuity scene|Filled to target scene count using uploaded source anchors)" \
    wowas_clean_scene_index_v2.tsv | head -n 40 || true

  echo
  echo "== B24 sample of likely scaffold debt =="
  rg -n "^24[[:space:]].*(rewritten_|The-Book-Finds-Its-|Nearness = Terminus|Presence ≠ Solved|Ghost = Obligation|The Cello Was Built From Consequence)" \
    wowas_clean_scene_index_v2.tsv | head -n 80 || true

  echo
  echo "== B25 sample of likely scaffold debt =="
  rg -n "^25[[:space:]].*(rewritten_|The-Book-Finds-Its-|Farewell = Instrument|Waking = Proof|Rebirth = Obligation|What Returned Could Not Return Unchanged)" \
    wowas_clean_scene_index_v2.tsv | head -n 80 || true

  echo
  echo "== strongest concrete anchors in B24-B25 =="
  rg -n "^2(4|5)[[:space:]].*(DIRECT_SOURCE[[:space:]]+PLACED_FILE|DIRECT_SOURCE[[:space:]]+SCENE_EXPANSION_EXTRACT|ACTUAL_SOURCE[[:space:]]+M)" \
    wowas_clean_scene_index_v2.tsv | head -n 120 || true

  echo
  echo "== current apply order tail =="
  tail -n 80 CURRENT_APPLY_ORDER_v10.md || true
} > "$OUT"

echo "$OUT"
