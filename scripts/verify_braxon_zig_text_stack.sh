#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
source "$ROOT/braxon-text-env"

echo "=== verify Braxon Zig/text stack ==="
clang -dumpmachine
"$ROOT/braxon-python" -c 'import math, _math_integer; print("python native ok")'
"$ROOT/braxon-rustc" --version --verbose

command -v zig >/dev/null && zig version || echo "zig unavailable"
command -v tree-sitter >/dev/null && tree-sitter --version || echo "tree-sitter unavailable"
pkg-config --exists harfbuzz freetype2
pkg-config --modversion harfbuzz
pkg-config --modversion freetype2

echo "BRAXON ZIG TEXT STACK VERIFY OK"
