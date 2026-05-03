#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

DL="$HOME/storage/shared/Download"
V8_ZIP_DEFAULT="$DL/wowas_final_edition_BRAXON_ready_v8.zip"
V9_TAR_DEFAULT="$DL/wowas_v9_polish_patch.tar.gz"
V10_TAR_DEFAULT="$DL/wowas_update10_bundle.tar.gz"
V10_DIR_DEFAULT="$DL/wowas_update10_bundle"
OUT="$DL/wowas_final_edition_BRAXON_ready_v10"

V8_ZIP="${1:-$V8_ZIP_DEFAULT}"
V9_TAR="${2:-$V9_TAR_DEFAULT}"
V10_SRC="${3:-}"

mkdir -p "$DL"
rm -rf "$OUT"
mkdir -p "$OUT" "$OUT/patches/v9" "$OUT/patches/v10"

if [ ! -f "$V8_ZIP" ]; then
  echo "Missing v8 zip: $V8_ZIP" >&2
  exit 1
fi

# Extract base v8 bundle into the normalized v10 folder root.
unzip -q "$V8_ZIP" -d "$OUT/.stage_v8"
if [ -d "$OUT/.stage_v8/wowas_final_edition_BRAXON_ready_v8" ]; then
  cp -a "$OUT/.stage_v8/wowas_final_edition_BRAXON_ready_v8/." "$OUT/"
else
  echo "Unexpected v8 archive layout." >&2
  exit 1
fi
rm -rf "$OUT/.stage_v8"

# Stage v9 polish layer if present.
if [ -f "$V9_TAR" ]; then
  tar -xzf "$V9_TAR" -C "$OUT/patches/v9"
else
  echo "Warning: v9 patch tar not found at $V9_TAR" >&2
fi

# Determine v10 source.
if [ -z "$V10_SRC" ]; then
  if [ -d "$V10_DIR_DEFAULT" ]; then
    V10_SRC="$V10_DIR_DEFAULT"
  elif [ -f "$V10_TAR_DEFAULT" ]; then
    V10_SRC="$V10_TAR_DEFAULT"
  else
    echo "Missing Update 10 source. Place wowas_update10_bundle.tar.gz or wowas_update10_bundle in $DL, or pass a third argument." >&2
    exit 1
  fi
fi

if [ -d "$V10_SRC" ]; then
  cp -a "$V10_SRC/." "$OUT/patches/v10/"
elif [ -f "$V10_SRC" ]; then
  tar -xzf "$V10_SRC" -C "$OUT/patches/v10"
else
  echo "Invalid Update 10 source: $V10_SRC" >&2
  exit 1
fi

# Copy the key update10 authority docs to root for easier ingest discovery.
for f in   wowas_final_canon_control_bundle_v10_addendum.md   wowas_scene_patch_v10.tsv   wowas_character_timeline_lattice_patch_v10.tsv   wowas_orbit_patch_v10.tsv   wowas_endgame_judgment_matrix_v10.tsv   wowas_magic_system_patch_v10.md   BRAXON_ready_manifest_v10.md   CURRENT_APPLY_ORDER_v10.md   README_v10.md
  do
  if [ -f "$OUT/patches/v10/$f" ]; then
    cp -f "$OUT/patches/v10/$f" "$OUT/$f"
  fi
done

cat > "$OUT/INSTALL_SUMMARY_v10.txt" <<EOF
install_target=$OUT
base_v8_zip=$V8_ZIP
v9_patch=$V9_TAR
v10_source=$V10_SRC
status=installed
next_step=point_BRAXON_ingest_at_CURRENT_APPLY_ORDER_v10.md
EOF

echo
echo "Installed WoWaS v8 + v9 + v10 at: $OUT"
echo "Key authority file: $OUT/CURRENT_APPLY_ORDER_v10.md"
echo "Install summary: $OUT/INSTALL_SUMMARY_v10.txt"
