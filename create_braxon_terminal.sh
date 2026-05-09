#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
TERMROOT="$TC/terminal/braxon-term-1"
BIN="$TERMROOT/bin"
ENVFILE="$TERMROOT/braxon-terminal.env"
LAUNCH="$ROOT/braxon-terminal"
VERIFY="$ROOT/scripts/verify_braxon_terminal.sh"

mkdir -p "$BIN" "$ROOT/scripts" "$TERMROOT/state" "$TERMROOT/logs"

cat > "$ENVFILE" <<EOF
export ROOT="$ROOT"
export TC="$TC"
export BRAXON_TERMINAL="$TERMROOT"
export PYTHONHOME="$TC/install/python"
export PATH="$BIN:$ROOT:$TC/install/python/bin:/data/data/com.termux/files/usr/bin:\$PATH"
export LD_LIBRARY_PATH="$TC/install/braxon_android_overlay/lib:/data/data/com.termux/files/usr/lib:\${LD_LIBRARY_PATH:-}"
export CC="/data/data/com.termux/files/usr/bin/clang"
export CXX="/data/data/com.termux/files/usr/bin/clang++"
export AR="/data/data/com.termux/files/usr/bin/llvm-ar"
export RANLIB="/data/data/com.termux/files/usr/bin/llvm-ranlib"
export LD="/data/data/com.termux/files/usr/bin/ld.lld"
export CFLAGS="-include $TC/adoption/include/braxon_android_posix_adoption_force.h"
export CXXFLAGS="-include $TC/adoption/include/braxon_android_posix_adoption_force.h"
export LDFLAGS="-L$TC/install/braxon_android_overlay/lib -lbraxon_android_libc_extensions"
unset PYTHONPATH
EOF

cat > "$LAUNCH" <<EOF
#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail
source "$ENVFILE"
exec /data/data/com.termux/files/usr/bin/bash --noprofile --norc "\$@"
EOF
chmod +x "$LAUNCH"

cat > "$VERIFY" <<'EOF'
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
EOF
chmod +x "$VERIFY"

"$VERIFY"

echo "READY:"
echo "$LAUNCH"
echo "$ENVFILE"
echo "$VERIFY"
