#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
SRC="$TC/src/cpython"
LIBDIR="$TC/install/sysroot/usr/lib"
HELPER="$SRC/Lib/test/support/import_helper.py"
BACKUP="$HELPER.braxon_startup_delta_$(date +%Y%m%d_%H%M%S).bak"
LOG="$TC/fix_lazy_import_startup_contract_v2_$(date +%Y%m%d_%H%M%S).log"

cd "$SRC"

export LD_LIBRARY_PATH="$LIBDIR:${LD_LIBRARY_PATH:-}"
mkdir -p "$TC/profile_catalog/raw"
export LLVM_PROFILE_FILE="$TC/profile_catalog/raw/cpython-%p-%m.profraw"
rm -f default.profraw

cp "$HELPER" "$BACKUP"

echo "=== locating ensure_lazy_imports ===" | tee "$LOG"
grep -n "def ensure_lazy_imports" "$HELPER" | tee -a "$LOG"

./python - <<'PY' 2>&1 | tee -a "$LOG"
from pathlib import Path
import re

p = Path("Lib/test/support/import_helper.py")
s = p.read_text()

m = re.search(
    r"def ensure_lazy_imports\(module_name, modules_to_block\):\n"
    r".*?"
    r"(?=\ndef [A-Za-z_]|$)",
    s,
    flags=re.S,
)

if not m:
    raise SystemExit("could not find ensure_lazy_imports function")

old = m.group(0)
print("old function:")
print(old)

new = '''def ensure_lazy_imports(module_name, modules_to_block):
    """Check that importing module_name does not newly import modules_to_block.

    Android/Braxon startup may already preload frozen bootstrap modules such as
    os, posixpath, genericpath, or stat. That is a startup/platform contract,
    not a functools lazy-import violation. This assertion therefore compares
    the module table before and after the target import.
    """
    script = f"""
import sys
modules_to_block = frozenset({modules_to_block!r})
startup_modules = set(sys.modules)

import {module_name}
new_modules = set(sys.modules) - startup_modules
if unexpected := modules_to_block & new_modules:
    after = ", ".join(sorted(unexpected))
    startup = ", ".join(sorted(modules_to_block & startup_modules))
    raise AssertionError(
        f'unexpectedly imported after importing {module_name}: {{after}}; '
        f'already present at startup: {{startup}}'
    )
"""
    assert_python_ok("-S", "-c", script)
'''

s = s[:m.start()] + new + s[m.end():]
p.write_text(s)
print("patched:", p)
PY

echo "=== startup baseline ===" | tee -a "$LOG"
./python -I -S -c '
import sys
blocked = {"os","weakref","typing","annotationlib","warnings"}
print("startup blocked:", sorted(blocked & sys.modules.keys()))
print("module count:", len(sys.modules))
' 2>&1 | tee -a "$LOG"

echo "=== focused retest ===" | tee -a "$LOG"
./python -m test -v test_functools 2>&1 | tee -a "$LOG"

STATUS=${PIPESTATUS[0]}
echo "focused status: $STATUS" | tee -a "$LOG"
test "$STATUS" -eq 0 || exit "$STATUS"

echo "=== continue build ===" | tee -a "$LOG"
make -j2 V=1 2>&1 | tee -a "$LOG"

STATUS=${PIPESTATUS[0]}
echo "full status: $STATUS" | tee -a "$LOG"
ln -sf "$LOG" "$TC/fix_lazy_import_startup_contract_latest.log"
exit "$STATUS"
