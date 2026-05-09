#!/data/data/com.termux/files/usr/bin/bash
set -u

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
SRC="$TC/src/cpython"
LIBDIR="$TC/install/sysroot/usr/lib"
OUT="$TC/startup_os_trace_$(date +%Y%m%d_%H%M%S)"

mkdir -p "$OUT"
cd "$SRC" || exit 1

export LD_LIBRARY_PATH="$LIBDIR:${LD_LIBRARY_PATH:-}"

echo "=== direct startup module check ===" | tee "$OUT/summary.log"
./python -I -S -c '
import sys
print("blocked:", sorted({"os","weakref","typing","annotationlib","warnings"} & sys.modules.keys()))
print("os module:", sys.modules.get("os"))
print("module count:", len(sys.modules))
for name in sorted(sys.modules):
    if name in {"os","posixpath","genericpath","stat","site","warnings","weakref","typing","annotationlib"}:
        print("present:", name, sys.modules[name])
' 2>&1 | tee "$OUT/direct_check.log"

echo "=== verbose import trace ===" | tee -a "$OUT/summary.log"
./python -I -S -v -c 'import sys; print("READY")' \
  > "$OUT/verbose_stdout.log" \
  2> "$OUT/verbose_stderr.log"

grep -nE "import 'os'|import os|# .*os|posixpath|genericpath|site|warnings|weakref|typing|annotationlib" \
  "$OUT/verbose_stderr.log" "$OUT/verbose_stdout.log" \
  > "$OUT/suspect_lines.log" || true

echo "=== suspect lines ==="
cat "$OUT/suspect_lines.log"

echo
echo "trace saved:"
echo "$OUT"
