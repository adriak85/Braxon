#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$HOME/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
SRC="$TC/source_forge"
PACKAGED="$ROOT/packaged"
BACKBONE="$ROOT/state"
STAMP="$(date +%Y%m%d_%H%M%S)"

cd "$ROOT"

mkdir -p \
  "$PACKAGED/backbone-links" \
  "$PACKAGED/bake-over" \
  "$PACKAGED/manifests" \
  "$PACKAGED/review-notes"

echo "=== link package to real backbone, do not duplicate backbone ==="

ln -sfn "$BACKBONE" "$PACKAGED/backbone-links/state_backbone"
ln -sfn "$SRC" "$PACKAGED/backbone-links/source_forge"
ln -sfn "$TC" "$PACKAGED/backbone-links/full_android_language_toolchain"

cat > "$PACKAGED/backbone-links/BACKBONE_LINK_POLICY.md" <<EOF
# Backbone Link Policy

Status: source-review staging, not release.

This package does not store or duplicate the full backbone.

It links to the actual Braxon backbone:

- state_backbone -> $BACKBONE
- source_forge -> $SRC
- full_android_language_toolchain -> $TC

Rule:
The package carries identity, manifests, proofs, scripts, source-lane reports, and bake-over instructions.
The real source/state backbone remains first-class and external to the review package unless explicitly selected.
EOF

cat > "$PACKAGED/bake-over/BAXON_SOURCE_BAKE_OVER_PLAN.md" <<EOF
# Braxon Source Bake-Over Plan

Bake-over status: prepared, not executed.

Purpose:
Use the package as a clean source-facing index while preserving the actual backbone as the source of truth.

Bake-over includes:
- identity law
- resolver strategies
- source-first lane scripts
- verifier scripts
- source-lane reports
- source-lane locks
- backbone symlink map
- manifest hashes

Bake-over excludes by default:
- private credentials
- full generated build trees
- unreviewed state registry bulk
- fake hot-live claims
- model payload claims without proof

Backbone link root:
$BACKBONE

Source forge:
$SRC

Created:
$STAMP
EOF

{
  echo "BRAXON_SOURCE_BACKBONE_LINK_MANIFEST=1"
  echo "timestamp=$STAMP"
  echo "root=$ROOT"
  echo "backbone=$BACKBONE"
  echo "source_forge=$SRC"
  echo
  find "$PACKAGED" -maxdepth 4 -type l -exec sh -c 'for x; do printf "%s -> %s\n" "$x" "$(readlink "$x")"; done' sh {} +
  echo
  find "$PACKAGED" -type f ! -path "*/tarballs/*" | sort
} > "$PACKAGED/manifests/backbone_link_manifest.txt"

find "$PACKAGED" -type f ! -path "*/tarballs/*" -print0 | sort -z | xargs -0 sha256sum \
  > "$PACKAGED/manifests/package_manifest.sha256"

git add packaged/backbone-links packaged/bake-over packaged/manifests
git status --short
