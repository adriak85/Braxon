#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
cd "$ROOT"

OUT_DIR="$ROOT/state/substrate/nsq_court_start"
mkdir -p "$OUT_DIR"

SRC="$ROOT/asm/nsq_court_start/start.S"
OBJ="$OUT_DIR/nsq_court_start.o"
BIN="$OUT_DIR/nsq_court_start"

echo "== assemble NSQ Court start proof =="
clang -target aarch64-linux-android24 -c "$SRC" -o "$OBJ"

echo "== link no-libc / no-startfiles proof binary =="
ld.lld -o "$BIN" "$OBJ"

chmod +x "$BIN"

echo "object=$OBJ"
echo "binary=$BIN"
