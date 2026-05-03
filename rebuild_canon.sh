#!/usr/bin/env bash
set -Eeuo pipefail
shopt -s nullglob globstar

ROOT="${1:-$(pwd)}"
CRATE="$ROOT/crates/wowas-final-edition-v10"
CANON="$CRATE/canon"
ACTIVE="$CANON/active"
CONTROL="$CANON/control"
TREE="$CANON/canonical_story_tree"
SUPPORT="$TREE/_support"
CHAR_DIR="$TREE/characters"
WORLD_DIR="$TREE/world"
BOOKS_DIR="$TREE/books"
FINAL="$CANON/final"
PACKAGE_DIR="$ROOT/dist"

[[ -d "$CRATE" ]] || { echo "missing crate: $CRATE" >&2; exit 1; }
[[ -d "$CANON" ]] || { echo "missing canon: $CANON" >&2; exit 1; }

mkdir -p "$ACTIVE" "$CONTROL" "$SUPPORT" "$CHAR_DIR" "$WORLD_DIR" "$BOOKS_DIR" "$FINAL" "$PACKAGE_DIR"
mkdir -p "$ACTIVE/generated" "$ACTIVE/transmedia" "$ACTIVE/guides" "$ACTIVE/indexes"

move_if_exists() {
  local src="$1" dst="$2"
  if [[ -f "$src" ]]; then
    mkdir -p "$(dirname "$dst")"
    mv -f "$src" "$dst"
  fi
}

copy_if_exists() {
  local src="$1" dst="$2"
  if [[ -f "$src" ]]; then
    mkdir -p "$(dirname "$dst")"
    cp -f "$src" "$dst"
  fi
}

merge_unique() {
  local dst="$1"; shift
  local tmp="$dst.tmp.$$"
  : > "$tmp"
  if [[ -f "$dst" ]]; then cat "$dst" >> "$tmp"; fi
  for src in "$@"; do
    [[ -f "$src" ]] || continue
    cat "$src" >> "$tmp"
  done
  awk 'NF && !seen[$0]++' "$tmp" > "$dst"
  rm -f "$tmp"
}

merge_tsv_keep_one_header() {
  local dst="$1"; shift
  local tmp="$dst.tmp.$$"
  : > "$tmp"
  local wrote_header=0
  if [[ -f "$dst" ]]; then
    cat "$dst" >> "$tmp"
    wrote_header=1
  fi
  for src in "$@"; do
    [[ -f "$src" ]] || continue
    if [[ $wrote_header -eq 0 ]]; then
      cat "$src" >> "$tmp"
      wrote_header=1
    else
      tail -n +2 "$src" >> "$tmp"
    fi
  done
  awk 'NF && !seen[$0]++' "$tmp" > "$dst"
  rm -f "$tmp"
}

remove_if_exists() {
  for target in "$@"; do
    [[ -e "$target" ]] && rm -rf "$target"
  done
}

# final control surfaces: highest known versions become unversioned usable files
move_if_exists "$CANON/WOWAS_CANON_AUTHORITY_v14.md" "$CONTROL/WOWAS_CANON_AUTHORITY.md"
move_if_exists "$CONTROL/WOWAS_SOURCE_OF_TRUTH_REGISTRY_v14.md" "$CONTROL/WOWAS_SOURCE_OF_TRUTH_REGISTRY.md"
move_if_exists "$CONTROL/character_generation_review_v14.tsv" "$CONTROL/character_generation_review.tsv"
move_if_exists "$CONTROL/prose_and_tone_guide_v14.json" "$CONTROL/prose_and_tone_guide.json"
move_if_exists "$CANON/wowas_final_authority_manifest_v13.json" "$CONTROL/final_authority_manifest.json"
move_if_exists "$CANON/wowas_final_authority_system_v13.md" "$CONTROL/final_authority_system.md"

# final active canon surfaces
move_if_exists "$CANON/wowas_canon_v1.md" "$ACTIVE/wowas_canon_seed.md"
move_if_exists "$CANON/timeline_lattice.tsv" "$ACTIVE/timeline_lattice.tsv"
move_if_exists "$CANON/wowas_american_morphed_life_lattice_v2.tsv" "$ACTIVE/american_morphed_life_lattice.tsv"
move_if_exists "$CANON/wowas_arc_insertion_registry_v7.tsv" "$ACTIVE/arc_insertion_registry.tsv"
move_if_exists "$CANON/wowas_conflict_ledger_v6.tsv" "$ACTIVE/conflict_ledger.tsv"
move_if_exists "$CANON/wowas_corridor_encounter_pressure_patch_v8.tsv" "$ACTIVE/corridor_encounter_pressure.tsv"
move_if_exists "$CANON/wowas_county_corridor_pressure_map_v2.tsv" "$ACTIVE/county_corridor_pressure_map.tsv"
move_if_exists "$CANON/wowas_county_creature_patch_v6.tsv" "$ACTIVE/county_creature_registry.tsv"
move_if_exists "$CANON/wowas_ecology_pressure_rules_v2.md" "$ACTIVE/guides/ecology_pressure_rules.md"
move_if_exists "$CANON/wowas_endgame_judgment_matrix_v10.tsv" "$ACTIVE/endgame_judgment_matrix.tsv"
move_if_exists "$CANON/wowas_magic_system_patch_v10.md" "$ACTIVE/guides/magic_system.md"
move_if_exists "$CANON/wowas_monster_count_alignment_v8.md" "$ACTIVE/guides/monster_count_alignment.md"
move_if_exists "$CANON/wowas_monster_species_registry_v8.tsv" "$ACTIVE/monster_species_registry.tsv"
move_if_exists "$CANON/wowas_protected_support_cast_v7.tsv" "$ACTIVE/protected_support_cast.tsv"

# character lattice: unified v14 is the authority and must feed active/index/generator surfaces
copy_if_exists "$CANON/wowas_character_timeline_lattice_UNIFIED_v14.tsv" "$ACTIVE/character_timeline_lattice.tsv"
copy_if_exists "$CANON/wowas_character_timeline_lattice_UNIFIED_v14.tsv" "$SUPPORT/character_timeline_lattice.tsv"
copy_if_exists "$ACTIVE/character_timeline_lattice_v14_33.tsv" "$SUPPORT/character_timeline_lattice_33.tsv"
merge_tsv_keep_one_header "$ACTIVE/character_timeline_lattice.tsv" \
  "$CANON/wowas_character_timeline_lattice_v2.tsv" \
  "$CANON/wowas_character_timeline_lattice_patch_v6.tsv" \
  "$CANON/wowas_character_timeline_lattice_patch_v10.tsv" \
  "$ACTIVE/character_timeline_lattice_v14_33.tsv"
cp -f "$ACTIVE/character_timeline_lattice.tsv" "$SUPPORT/character_timeline_lattice.tsv" 2>/dev/null || true

# orbit lattice: base + patches become one usable file
merge_tsv_keep_one_header "$ACTIVE/orbit_file.tsv" \
  "$CANON/wowas_orbit_file_v2.tsv" \
  "$CANON/wowas_orbit_patch_v6.tsv" \
  "$CANON/wowas_orbit_patch_v10.tsv"

# scene index: base + clean index + known patches become one usable active scene index
merge_tsv_keep_one_header "$ACTIVE/scene_index.tsv" \
  "$CANON/wowas_clean_scene_index_v2.tsv" \
  "$CANON/wowas_scene_patch_v6.tsv" \
  "$CANON/wowas_scene_patch_v10.tsv" \
  "$CANON/wowas_scene_patch_v11.tsv" \
  "$TREE/_scene_heading_index.tsv"
cp -f "$ACTIVE/scene_index.tsv" "$TREE/_scene_index.tsv" 2>/dev/null || true

# generated character/creature surfaces remain final active data, not farming-only scratch
copy_if_exists "$ACTIVE/generated/wowas_generated_characters_5000.tsv" "$SUPPORT/generated_characters_5000.tsv"
copy_if_exists "$ACTIVE/generated/wowas_generated_creatures_5000.tsv" "$SUPPORT/generated_creatures_5000.tsv"
copy_if_exists "$ACTIVE/creature_registry_target_5000.tsv" "$SUPPORT/creature_registry_target_5000.tsv"
copy_if_exists "$ACTIVE/commerce_politics_and_chow_layer.tsv" "$SUPPORT/commerce_politics_and_chow_layer.tsv"
copy_if_exists "$ACTIVE/book_spine_33.tsv" "$SUPPORT/book_spine.tsv"
copy_if_exists "$ACTIVE/canon_blocklist.tsv" "$SUPPORT/canon_blocklist.tsv"
copy_if_exists "$ACTIVE/pip_unbrictionable_word_law.tsv" "$SUPPORT/pip_unbrictionable_word_law.tsv"
copy_if_exists "$ACTIVE/blood_cello_resolution_state.tsv" "$SUPPORT/blood_cello_resolution_state.tsv"

# unversion character/world engine guides where applicable
move_if_exists "$CHAR_DIR/00_ENGINE_RULES.md" "$ACTIVE/guides/character_engine_rules.md"
move_if_exists "$CHAR_DIR/01_NAMED_CAST_TOP300.md" "$ACTIVE/guides/named_cast_top300.md"
move_if_exists "$CHAR_DIR/02_CHARACTER_PLACEMENT_BY_BOOK.md" "$ACTIVE/guides/character_placement_by_book.md"
move_if_exists "$CHAR_DIR/03_WORLD_POPULATION_CANON.md" "$ACTIVE/guides/world_population_canon.md"
move_if_exists "$CHAR_DIR/04_SOURCE_HERO_ENGINE.md" "$ACTIVE/guides/source_hero_engine.md"
move_if_exists "$CHAR_DIR/05_SELF_CORRECTING_CANON_RULES.md" "$ACTIVE/guides/self_correcting_canon_rules.md"
copy_if_exists "$CHAR_DIR/06_CHARACTER_REGISTRY.json" "$ACTIVE/character_registry.json"

# remove stale or superseded version/control/farming files after absorption
remove_if_exists \
  "$CANON"/CURRENT_APPLY_ORDER_v*.md \
  "$CANON"/INSTALL_SUMMARY_v*.txt \
  "$CANON"/README_v*.md \
  "$CANON"/SHA256SUMS_v*.txt \
  "$CANON"/BRAXON_ready_manifest_v*.md \
  "$CANON"/wowas_final_canon_control_bundle_v*.md \
  "$CANON"/wowas_patch_absorption_registry_v*.md \
  "$CANON"/wowas_character_timeline_lattice_v*.tsv \
  "$CANON"/wowas_character_timeline_lattice_patch_v*.tsv \
  "$CANON"/wowas_orbit_file_v*.tsv \
  "$CANON"/wowas_orbit_patch_v*.tsv \
  "$CANON"/wowas_scene_patch_v*.tsv \
  "$CANON"/wowas_clean_scene_index_v*.tsv \
  "$CANON"/wowas_monster_species_registry_v*.tsv \
  "$CANON"/wowas_monster_count_alignment_v*.md \
  "$CANON"/wowas_county_corridor_pressure_map_v*.tsv \
  "$CANON"/wowas_county_creature_patch_v*.tsv \
  "$CANON"/wowas_corridor_encounter_pressure_patch_v*.tsv \
  "$CANON"/wowas_ecology_pressure_rules_v*.md \
  "$CANON"/wowas_endgame_judgment_matrix_v*.tsv \
  "$CANON"/wowas_magic_system_patch_v*.md \
  "$CANON"/wowas_arc_insertion_registry_v*.tsv \
  "$CANON"/wowas_protected_support_cast_v*.tsv \
  "$CANON"/wowas_american_morphed_life_lattice_v*.tsv \
  "$CANON"/wowas_final_authority_manifest_v*.json \
  "$CANON"/wowas_final_authority_system_v*.md

# purge backups, duplicate backup hubs, build/review/farming fragments, and mutation scripts from active canon tree
find "$CANON" -depth \( -iname '*.bak' -o -iname '*.backup' -o -iname '*.backup.*' -o -iname '*~' -o -iname '*.orig' -o -iname '*.rej' \) -type f -delete
find "$CANON" -depth \( -iname '*backup*' -o -iname '*BACKUP*' \) -exec rm -rf {} +
find "$CANON" -depth -type d \( -iname 'review' -o -iname 'patches' -o -iname 'diagnostics' -o -iname 'scratch' -o -iname 'tmp' -o -iname 'temp' \) -exec rm -rf {} +
find "$CANON" -depth -type f \( -iname '*apply_order*' -o -iname '*install_summary*' -o -iname '*patch_absorption*' -o -iname '*review_chunk*' -o -iname '*direct_review*' -o -iname '*scene_ladder*' \) -delete

# remove empty directories left by consolidation
find "$CANON" -depth -type d -empty -delete
mkdir -p "$ACTIVE" "$CONTROL" "$SUPPORT" "$BOOKS_DIR" "$WORLD_DIR" "$PACKAGE_DIR"

# executable marker and final package
chmod +x "$ROOT/rebuild_canon.sh" 2>/dev/null || true
TAR_OUT="$PACKAGE_DIR/wowas-final-canon-state.tar.gz"
tar --exclude='.git' --exclude='target' --exclude='dist' -czf "$TAR_OUT" -C "$ROOT" "crates/wowas-final-edition-v10" "rebuild_canon.sh"

printf 'final_canon_state=%s\n' "$TAR_OUT"
