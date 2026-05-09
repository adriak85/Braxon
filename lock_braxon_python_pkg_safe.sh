#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
PREFIX="$TC/install/python"
LOCK="$TC/locks/braxon_python_3.16_native"
BIN="$PREFIX/bin/python3.16"
STAMP="$(date +%Y%m%d_%H%M%S)"

mkdir -p "$LOCK"

echo "=== verify baked Python before lock ==="
export LD_LIBRARY_PATH="$TC/install/braxon_android_overlay/lib:/data/data/com.termux/files/usr/lib:${LD_LIBRARY_PATH:-}"
export PYTHONHOME="$PREFIX"
unset PYTHONPATH

"$BIN" - <<'PY'
import sys, math, _math_integer, cmath
import decimal, _decimal
import tkinter, _tkinter
import _posixshmem
print("python:", sys.version)
print("math:", math.__file__)
print("_math_integer:", _math_integer.__file__)
print("cmath:", cmath.__file__)
print("_decimal:", _decimal.__file__)
print("_tkinter:", _tkinter.__file__)
print("_posixshmem:", _posixshmem)
print("LOCK INPUT OK")
PY

echo "=== write file manifest ==="
cd "$PREFIX"
find . -type f -print0 | sort -z | xargs -0 sha256sum > "$LOCK/manifest_$STAMP.sha256"
ln -sf "$LOCK/manifest_$STAMP.sha256" "$LOCK/manifest_latest.sha256"

echo "=== make frozen tar snapshot ==="
cd "$TC/install"
tar -cpf "$LOCK/braxon_python_3.16_native_$STAMP.tar" python
sha256sum "$LOCK/braxon_python_3.16_native_$STAMP.tar" > "$LOCK/braxon_python_3.16_native_$STAMP.tar.sha256"
ln -sf "$LOCK/braxon_python_3.16_native_$STAMP.tar" "$LOCK/braxon_python_3.16_native_latest.tar"
ln -sf "$LOCK/braxon_python_3.16_native_$STAMP.tar.sha256" "$LOCK/braxon_python_3.16_native_latest.tar.sha256"

echo "=== install guarded launcher ==="
cat > /data/data/com.termux/files/home/Braxon/braxon-python <<EOF
#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail
export TC="$TC"
export PYTHONHOME="$PREFIX"
export LD_LIBRARY_PATH="$TC/install/braxon_android_overlay/lib:/data/data/com.termux/files/usr/lib:\${LD_LIBRARY_PATH:-}"
unset PYTHONPATH
exec "$BIN" "\$@"
EOF
chmod +x /data/data/com.termux/files/home/Braxon/braxon-python

echo "=== optional: stop Termux python package from changing if installed ==="
apt-mark hold python 2>/dev/null || true
apt-mark showhold | grep -E '^python$' || true

echo "LOCKED:"
echo "$LOCK"
echo "/data/data/com.termux/files/home/Braxon/braxon-python"
