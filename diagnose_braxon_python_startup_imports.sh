#!/data/data/com.termux/files/usr/bin/bash
set -u

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
SRC="$TC/src/cpython"
LIBDIR="$TC/install/sysroot/usr/lib"
OUT="$TC/startup_import_diagnosis_$(date +%Y%m%d_%H%M%S)"

mkdir -p "$OUT"
cd "$SRC" || exit 1
export LD_LIBRARY_PATH="$LIBDIR:${LD_LIBRARY_PATH:-}"

echo "=== startup module list ===" | tee "$OUT/summary.log"
./python -I -S - <<'PY' 2>&1 | tee "$OUT/startup_modules.log"
import sys
blocked = {"os", "weakref", "typing", "annotationlib", "warnings"}
print("blocked present:", sorted(blocked & sys.modules.keys()))
print("all startup modules:")
for name in sorted(sys.modules):
    print(name)
PY

echo "=== verbose import trace ===" | tee -a "$OUT/summary.log"
./python -I -S -v -c 'import sys; print("READY")' \
  >"$OUT/verbose_stdout.log" \
  2>"$OUT/verbose_stderr.log"

grep -nE "import .*os|import 'os'|# .*os|warnings|typing|weakref|annotationlib" \
  "$OUT/verbose_stderr.log" \
  "$OUT/verbose_stdout.log" \
  > "$OUT/suspect_import_lines.log" || true

echo "=== environment suspects ===" | tee "$OUT/env_suspects.log"
env | grep -E 'PYTHON|LD_|CLANG|CFLAGS|LDFLAGS|TERMUX|ANDROID' | sort | tee -a "$OUT/env_suspects.log"

echo
echo "diagnosis saved:"
echo "$OUT"
echo
echo "suspect lines:"
cat "$OUT/suspect_import_lines.log"
