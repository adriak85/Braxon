#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
PREFIX="$TC/install/python"
PY="$PREFIX/bin/python3.16"
LAUNCH="$ROOT/braxon-python"
LOCK="$TC/locks/braxon_python_3.16_native"
MANIFEST="$TC/locks/braxon_python_3.16_native_manifest.sha256"
VERIFY="$ROOT/scripts/verify_braxon_python_native.sh"
RESEAT="$ROOT/scripts/reseat_braxon_python_native.sh"
DYNLIB="$PREFIX/lib/python3.16/lib-dynload"

mkdir -p "$TC/locks" "$ROOT/scripts" "$DYNLIB"

cat > "$LAUNCH" <<EOF
#!/data/data/com.termux/files/usr/bin/bash
export TC="$TC"
export PYTHONHOME="$PREFIX"
export LD_LIBRARY_PATH="$TC/install/braxon_android_overlay/lib:/data/data/com.termux/files/usr/lib:\${LD_LIBRARY_PATH:-}"
unset PYTHONPATH
exec "$PY" "\$@"
EOF
chmod +x "$LAUNCH"

cat > "$VERIFY" <<'EOF'
#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
PREFIX="$TC/install/python"
PY="$ROOT/braxon-python"
DYNLIB="$PREFIX/lib/python3.16/lib-dynload"

test -x "$PY"
test -d "$DYNLIB"
test ! -L "$DYNLIB/_math_integer.cpython-316-aarch64-linux-android.so"

for m in math _math_integer cmath _decimal _tkinter; do
  ls "$DYNLIB"/"$m".cpython-316-aarch64-linux-android.so >/dev/null
done

"$PY" - <<'PY'
import sys, pathlib, json, time, functools
import math, _math_integer, cmath
import decimal, _decimal
import tkinter, _tkinter
import _posixshmem

assert "3.16.0a0" in sys.version
assert sys.prefix == "/data/data/com.termux/files/home/Braxon/state/full_android_language_toolchain/install/python"

tok = pathlib.Path("/data/data/com.termux/files/home/Braxon/assets/braxon_core/source_ingest/braxon_transport/tokenizer.json")
t0 = time.time()
data = json.loads(tok.read_text(errors="replace"))
vocab = data.get("model", {}).get("vocab", {})
assert len(vocab) > 100000

@functools.lru_cache(maxsize=None)
def f(n):
    return 0 if n <= 0 else f(n-1) + 1

assert f(900) == 900

print("Braxon Python native verify OK")
print("python:", sys.version)
print("math:", math.__file__)
print("_math_integer:", _math_integer.__file__)
print("cmath:", cmath.__file__)
print("_decimal:", _decimal.__file__)
print("_tkinter:", _tkinter.__file__)
print("tokenizer_vocab:", len(vocab), "elapsed:", round(time.time() - t0, 3))
PY
EOF
chmod +x "$VERIFY"

cat > "$RESEAT" <<'EOF'
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
EOF
chmod +x "$RESEAT"

find "$PY" "$LAUNCH" "$PREFIX/lib/python3.16" "$DYNLIB" -type f -print0 | sort -z | xargs -0 sha256sum > "$MANIFEST"

cat > "$LOCK" <<EOF
BRAXON_PYTHON_NATIVE_LOCK=1
PYTHON=$PY
LAUNCHER=$LAUNCH
PREFIX=$PREFIX
DYNLIB=$DYNLIB
MANIFEST=$MANIFEST
VERIFY=$VERIFY
RESEAT=$RESEAT
EOF

"$VERIFY"

echo "LOCKED:"
echo "$LOCK"
echo "$LAUNCH"
echo "$VERIFY"
echo "$RESEAT"
echo "$MANIFEST"
