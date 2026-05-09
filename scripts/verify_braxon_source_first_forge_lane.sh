#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$HOME/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
SRC="$TC/source_forge"

test -f "$SRC/SOURCE_FIRST_POLICY.md"
test -x "$SRC/source_forge_env"

source "$SRC/source_forge_env"

echo "=== verify Braxon source-first forge lane ==="
echo "BRAXON_SOURCE_FIRST=$BRAXON_SOURCE_FIRST"
echo "BRAXON_SOURCE_FORGE=$BRAXON_SOURCE_FORGE"
echo "PREFIX=$PREFIX"
echo "JOBS=$JOBS"

command -v clang
command -v git
command -v curl
command -v make
command -v cmake
command -v ninja
command -v pkg-config

echo "BRAXON SOURCE-FIRST FORGE LANE VERIFY OK"
