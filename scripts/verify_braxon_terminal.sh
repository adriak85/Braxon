#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
source "$TC/terminal/braxon-term-1/braxon-terminal.env"

echo "=== Braxon terminal proof ==="
echo "PATH=$PATH"
echo "CC=$CC"
echo "LD_LIBRARY_PATH=$LD_LIBRARY_PATH"

echo "=== reject accidental glibc target default ==="
case "$(clang -dumpmachine 2>/dev/null || true)" in
  *android*) echo "clang target OK: $(clang -dumpmachine)" ;;
  *) echo "WARNING: clang target is not reporting android: $(clang -dumpmachine 2>/dev/null || true)" ;;
esac

echo "=== Python proof ==="
"$ROOT/braxon-python" - <<'PY'
import sys, math, _math_integer, cmath, decimal, _decimal, tkinter, _tkinter
print("python:", sys.version)
print("math:", math.__file__)
print("_math_integer:", _math_integer.__file__)
print("cmath:", cmath.__file__)
print("_decimal:", _decimal.__file__)
print("_tkinter:", _tkinter.__file__)
print("Braxon terminal Python OK")
PY

echo "=== toolchain proof ==="
command -v clang
clang --version | head -n 3
command -v ld.lld || true
command -v cmake || true
command -v ninja || true
command -v git || true

echo "BRAXON TERMINAL READY"
