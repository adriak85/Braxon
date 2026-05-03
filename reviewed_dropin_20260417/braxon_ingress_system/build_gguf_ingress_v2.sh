#!/data/data/com.termux/files/usr/bin/bash
# build_gguf_ingress_v2.sh  —  compile gguf_ingress_c_v2 on-device
# Usage:  bash build_gguf_ingress_v2.sh [ROOT]
# ROOT defaults to ~/Braxon; the binary lands in ~/storage/shared/Download/
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
DL="$HOME/storage/shared/Download"
SRC="$DL/gguf_ingress_c_v2.c"
BIN="$DL/gguf_ingress_c_v2"

if [ ! -f "$SRC" ]; then
    echo "ERROR: source not found: $SRC" >&2
    echo "Copy gguf_ingress_c_v2.c to $DL first." >&2
    exit 1
fi

echo "Building $BIN ..."
cc -O2 -std=c11 \
   -D_FILE_OFFSET_BITS=64 \
   -Wall -Wextra \
   -o "$BIN" "$SRC"

# Strip debug symbols — saves ~40 % on ARM64 for phone storage
strip "$BIN" 2>/dev/null || true

chmod 755 "$BIN"
echo "built=$BIN"
ls -lh "$BIN"
