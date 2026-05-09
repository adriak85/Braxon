#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
LOCKDIR="$TC/locks/braxon_python_3.16_native"
LOCKFILE="$LOCKDIR/LOCKED_NATIVE_STATE.txt"
MANIFEST="$LOCKDIR/manifest.sha256"

mkdir -p "$LOCKDIR"

cat > "$LOCKFILE" <<EOF
BRAXON_PYTHON_NATIVE_LOCK=1
PYTHON=$TC/install/python/bin/python3.16
LAUNCHER=$ROOT/braxon-python
PREFIX=$TC/install/python
DYNLIB=$TC/install/python/lib/python3.16/lib-dynload
VERIFY=$ROOT/scripts/verify_braxon_python_native.sh
RESEAT=$ROOT/scripts/reseat_braxon_python_native.sh
EOF

find \
  "$TC/install/python/bin/python3.16" \
  "$ROOT/braxon-python" \
  "$TC/install/python/lib/python3.16" \
  -type f -print0 | sort -z | xargs -0 sha256sum > "$MANIFEST"

"$ROOT/scripts/verify_braxon_python_native.sh"

echo "LOCKED:"
echo "$LOCKDIR"
echo "$LOCKFILE"
echo "$MANIFEST"
echo "$ROOT/braxon-python"
