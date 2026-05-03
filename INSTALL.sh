#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

SRC_DIR="$(cd "$(dirname "$0")" && pwd)"
TARGET_DIR="${1:-$HOME/Braxon}"

echo "== BRAXON ROOT INSTALL =="
echo "source: $SRC_DIR"
echo "target: $TARGET_DIR"

mkdir -p "$TARGET_DIR"

if command -v rsync >/dev/null 2>&1; then
  rsync -a \
    --exclude '.git' \
    --exclude 'target' \
    --exclude '.cargo' \
    "$SRC_DIR"/ "$TARGET_DIR"/
else
  cp -a "$SRC_DIR"/. "$TARGET_DIR"/
fi

chmod +x \
  "$TARGET_DIR"/INSTALL.sh \
  "$TARGET_DIR"/bin/* \
  "$TARGET_DIR"/scripts/* || true

mkdir -p "$HOME/bin"

cat > "$HOME/bin/Braxon-root" <<'SH'
#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail
ROOT_DIR="${BRAXON_HOME:-$HOME/Braxon}"
if [ ! -d "$ROOT_DIR" ]; then
  echo "missing Braxon dir: $ROOT_DIR" >&2
  exit 2
fi
. "$ROOT_DIR/env/BRAXON_env.sh"
exec "$ROOT_DIR/bin/Braxon-capsule-status" "$@"
SH
chmod +x "$HOME/bin/Braxon-root"

if ! grep -q 'BRAXON_HOME' "$HOME/.bashrc" 2>/dev/null; then
  {
    echo
    echo "# BRAXON ROOT"
    echo "export BRAXON_HOME=\"$TARGET_DIR\""
    echo "[ -f \"$TARGET_DIR/env/BRAXON_env.sh\" ] && . \"$TARGET_DIR/env/BRAXON_env.sh\""
  } >> "$HOME/.bashrc"
fi

echo
echo "== installed =="
echo "BRAXON_root: $TARGET_DIR"
echo
echo "next:"
echo "  source \"$TARGET_DIR/env/BRAXON_env.sh\""
echo "  \"$TARGET_DIR/bin/Braxon-capsule-status\""
echo "  \"$TARGET_DIR/bin/Braxon-capsule-doctor\""
