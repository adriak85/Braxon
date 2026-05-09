#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

SRC="/data/data/com.termux/files/home/Braxon/state/full_android_language_toolchain/src/cpython"
cd "$SRC"

echo "=== latest backups ==="
ls -1t Lib/test/support/import_helper.py.braxon_startup_delta_*.bak 2>/dev/null | head -5 || true

BACKUP="$(ls -1t Lib/test/support/import_helper.py.braxon_startup_delta_*.bak 2>/dev/null | head -1 || true)"
if [ -n "$BACKUP" ]; then
  cp "$BACKUP" Lib/test/support/import_helper.py
  echo "restored helper from: $BACKUP"
else
  echo "no backup found; continuing with in-place repair"
fi

python - <<'PY'
from pathlib import Path
import re

p = Path("Lib/test/support/import_helper.py")
s = p.read_text()

# Remove any bad doubled namespace damage.
s = s.replace("script_helper.script_helper.assert_python_ok", "script_helper.assert_python_ok")
s = s.replace("assert_python_ok(\"-S\", \"-c\", script)", "script_helper.assert_python_ok(\"-S\", \"-c\", script)")

# Ensure the module is imported.
if "from test.support import script_helper" not in s:
    marker = "from test import support\n"
    if marker in s:
        s = s.replace(marker, marker + "from test.support import script_helper\n", 1)
    else:
        s = "from test.support import script_helper\n" + s

p.write_text(s)
print("helper repaired")
PY

echo "=== verify helper references ==="
grep -n "script_helper" Lib/test/support/import_helper.py

echo "=== retest ==="
./python -m test -v test_functools

echo "=== continue build ==="
make -j2 V=1
