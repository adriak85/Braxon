#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
CPY="$TC/src/cpython"
PREFIX="$TC/install/python"
DYNLIB="$PREFIX/lib/python3.16/lib-dynload"
BUILDLIB="$(cat "$CPY/pybuilddir.txt")"

mkdir -p "$DYNLIB"
rm -f "$DYNLIB/_math_integer.cpython-316-aarch64-linux-android.so"

find "$CPY/$BUILDLIB" "$CPY/Modules" -maxdepth 1 -type f -name '*.cpython-316-aarch64-linux-android.so' -exec cp -av {} "$DYNLIB"/ \;

"$ROOT/scripts/verify_braxon_python_native.sh"
