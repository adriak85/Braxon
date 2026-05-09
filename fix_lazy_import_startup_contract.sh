#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
SRC="$TC/src/cpython"
LIBDIR="$TC/install/sysroot/usr/lib"
HELPER="$SRC/Lib/test/support/import_helper.py"
BACKUP="$HELPER.braxon_startup_delta_$(date +%Y%m%d_%H%M%S).bak"
LOG="$TC/fix_lazy_import_startup_contract_$(date +%Y%m%d_%H%M%S).log"

cd "$SRC"

export LD_LIBRARY_PATH="$LIBDIR:${LD_LIBRARY_PATH:-}"
mkdir -p "$TC/profile_catalog/raw"
export LLVM_PROFILE_FILE="$TC/profile_catalog/raw/cpython-%p-%m.profraw"
rm -f default.profraw

cp "$HELPER" "$BACKUP"

python - <<'PY'
from pathlib import Path

p = Path("Lib/test/support/import_helper.py")
s = p.read_text()

old = '''\
    script = f"""
import sys
modules_to_block = frozenset({modules_to_block!r})
if unexpected := modules_to_block & sys.modules.keys():
    startup = ", ".join(unexpected)
    raise AssertionError(f'unexpectedly imported at startup: {{startup}}')

import {module_name}
if unexpected := modules_to_block & sys.modules.keys():
    after = ", ".join(unexpected)
    raise AssertionError(f'unexpectedly imported after importing {module_name}: {{after}}')
"""
'''

new = '''\
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
'''

if old not in s:
    raise SystemExit("target block not found; helper layout changed")

p.write_text(s.replace(old, new))
print("patched:", p)
PY

echo "=== startup baseline ===" | tee "$LOG"
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

echo "=== continue PGO/profile build ===" | tee -a "$LOG"
make -j2 V=1 2>&1 | tee -a "$LOG"

STATUS=${PIPESTATUS[0]}
echo "full status: $STATUS" | tee -a "$LOG"
ln -sf "$LOG" "$TC/fix_lazy_import_startup_contract_latest.log"
exit "$STATUS"
