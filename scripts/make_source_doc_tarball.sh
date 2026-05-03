#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
. "$ROOT/env/BRAXON_env.sh"

STAMP="$(date +%Y%m%d_%H%M%S)"
OUT_DIR="$HOME/storage/shared/Download"
PKG_DIR="$OUT_DIR/BRAXON_DEV_CAPSULE_$STAMP"
TAR_PATH="$OUT_DIR/BRAXON_DEV_CAPSULE_$STAMP.tar.gz"

rm -rf "$PKG_DIR"
mkdir -p "$PKG_DIR"

rsync -a \
  --exclude '.git' \
  --exclude 'target' \
  --exclude '.cargo' \
  --exclude '.backup' \
  --exclude 'tmp' \
  "$ROOT"/ "$PKG_DIR"/

cat > "$PKG_DIR/MANIFEST.json" <<JSON
{
  "version": 1,
  "name": "BRAXON_DEV_CAPSULE",
  "kind": "development_source_unit",
  "created_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "contains": [
    "installer",
    "environment contract",
    "AI operator brief",
    "hook matrix",
    "dialect matrix",
    "source tree",
    "proof scripts",
    "bench scripts"
  ],
  "ownership_note": "Development-facing source unit. Not a public distribution release."
}
JSON

tar -C "$OUT_DIR" -czf "$TAR_PATH" "$(basename "$PKG_DIR")"

echo "made:"
echo "  $TAR_PATH"
