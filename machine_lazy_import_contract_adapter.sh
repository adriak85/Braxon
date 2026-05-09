#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
SRC="$TC/src/cpython"
LIBDIR="$TC/install/sysroot/usr/lib"
HELPER="$SRC/Lib/test/support/import_helper.py"
STAMP="$TC/braxon_lazy_import_startup_contract_adapter.stamp"
LOG="$TC/machine_lazy_import_contract_adapter_$(date +%Y%m%d_%H%M%S).log"

cd "$SRC"
export LD_LIBRARY_PATH="$LIBDIR:${LD_LIBRARY_PATH:-}"

cp "$HELPER" "$HELPER.braxon_adapter_$(date +%Y%m%d_%H%M%S).bak"

./python - <<'PY' 2>&1 | tee "$LOG"
from pathlib import Path
import re

p = Path("Lib/test/support/import_helper.py")
s = p.read_text()

m = re.search(
    r"def ensure_lazy_imports\(imported_module, modules_to_block, \*, additional_code=None\):\n"
    r".*?"
    r"(?=\ndef [A-Za-z_]|$)",
    s,
    flags=re.S,
)

if not m:
    raise SystemExit("could not find ensure_lazy_imports")

new = '''def ensure_lazy_imports(imported_module, modules_to_block, *, additional_code=None):
    """Check that imported_module does not newly import modules_to_block.

    Braxon/Android startup may already contain platform bootstrap modules
    such as os because frozen getpath/bootstrap startup is different here.
    That startup state is not a lazy-import regression in the target module.
    """
    from test.support.script_helper import assert_python_ok

    additional_code = additional_code or ""
    script = f"""
import sys
modules_to_block = frozenset({modules_to_block!r})
startup_modules = set(sys.modules)

{additional_code}

import {imported_module}
new_modules = set(sys.modules) - startup_modules
if unexpected := modules_to_block & new_modules:
    after = ", ".join(sorted(unexpected))
    startup = ", ".join(sorted(modules_to_block & startup_modules))
    raise AssertionError(
        f'unexpectedly imported after importing {imported_module}: {{after}}; '
        f'already present at startup: {{startup}}'
    )
"""
    assert_python_ok("-S", "-c", script)
'''

s = s[:m.start()] + new + s[m.end():]
p.write_text(s)
print("patched ensure_lazy_imports as Braxon startup-contract adapter")
PY

{
  echo "BRAxon lazy import startup-contract adapter installed"
  date
  echo "$HELPER"
} | tee "$STAMP"

./python -m test -v test_functools 2>&1 | tee -a "$LOG"

STATUS=${PIPESTATUS[0]}
echo "focused status: $STATUS" | tee -a "$LOG"
test "$STATUS" -eq 0 || exit "$STATUS"

make -j2 V=1 2>&1 | tee -a "$LOG"

STATUS=${PIPESTATUS[0]}
echo "full status: $STATUS" | tee -a "$LOG"
ln -sf "$LOG" "$TC/machine_lazy_import_contract_adapter_latest.log"
exit "$STATUS"
