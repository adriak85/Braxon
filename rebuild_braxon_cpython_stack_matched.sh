#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
CPY="$TC/src/cpython"
ADOPT="$TC/adoption/include/braxon_android_posix_adoption_force.h"
LOG="$TC/rebuild_stack_matched_$(date +%Y%m%d_%H%M%S).log"

export PATH="/data/data/com.termux/files/usr/bin:$PATH"
export LD_LIBRARY_PATH="$TC/install/braxon_android_overlay/lib:/data/data/com.termux/files/usr/lib:${LD_LIBRARY_PATH:-}"
export CFLAGS_NODIST="${CFLAGS_NODIST:-} -include $ADOPT"
export LLVM_PROFILE_FILE="$TC/profile_catalog/raw/cpython-%p-%m.profraw"

cd "$CPY"

{
  echo "=== Braxon CPython stack-matched rebuild ==="
  date
  echo "CPY=$CPY"
  echo "ADOPT=$ADOPT"
  echo
  echo "=== locating recursion/stack guard code ==="
  grep -RIn '_Py_CheckRecursiveCall\|c_stack\|recursion_headroom\|stack soft\|C_RECURSION\|recursion_limit' Include Python Objects Modules | head -n 300 || true
} | tee "$LOG"

PATCH_FILE="$TC/stack_match_patch_targets_$(date +%Y%m%d_%H%M%S).txt"

python3 - <<'PY' 2>&1 | tee -a "$LOG"
from pathlib import Path
import re, shutil, time

root = Path("/data/data/com.termux/files/home/Braxon/state/full_android_language_toolchain/src/cpython")
stamp = time.strftime("%Y%m%d_%H%M%S")
changed = []

# Conservative Android/Braxon alignment:
# raise CPython internal C-stack safety budget only where this branch exposes it.
# Do not touch Python recursionlimit semantics.
candidates = [
    root / "Include/internal/pycore_ceval.h",
    root / "Include/internal/pycore_pystate.h",
    root / "Python/ceval.c",
    root / "Python/pystate.c",
]

patterns = [
    (re.compile(r'(?P<name>Py_C_RECURSION_LIMIT)\s+(?P<num>[0-9]+)'), 3000),
    (re.compile(r'(?P<name>C_RECURSION_LIMIT)\s+(?P<num>[0-9]+)'), 3000),
    (re.compile(r'(?P<name>PY_C_RECURSION_LIMIT)\s+(?P<num>[0-9]+)'), 3000),
    (re.compile(r'(?P<name>c_stack_soft_limit)\s*=\s*(?P<num>[0-9]+)'), 8 * 1024 * 1024),
    (re.compile(r'(?P<name>c_stack_hard_limit)\s*=\s*(?P<num>[0-9]+)'), 4 * 1024 * 1024),
]

for p in candidates:
    if not p.exists():
        continue
    s = p.read_text(errors="replace")
    ns = s
    for pat, newnum in patterns:
        def repl(m):
            old = int(m.group("num"))
            if old >= newnum:
                return m.group(0)
            changed.append(f"{p.relative_to(root)}: {m.group('name')} {old} -> {newnum}")
            return m.group(0).replace(str(old), str(newnum), 1)
        ns = pat.sub(repl, ns)
    if ns != s:
        shutil.copy2(p, p.with_suffix(p.suffix + f".bak.stackmatch.{stamp}"))
        p.write_text(ns)

out = Path("/data/data/com.termux/files/home/Braxon/state/full_android_language_toolchain") / f"stack_match_changes_{stamp}.txt"
out.write_text("\n".join(changed) + ("\n" if changed else "NO DIRECT CONSTANT PATCHED\n"))
print(out.read_text())
PY

echo "=== rebuild core stack-sensitive objects and python ===" | tee -a "$LOG"
rm -f Python/ceval.o Python/pystate.o Programs/python.o python

make -j2 V=1 \
  CFLAGS_NODIST="$CFLAGS_NODIST" \
  Python/ceval.o Python/pystate.o Programs/python.o python \
  2>&1 | tee -a "$LOG"

echo "=== verify feature surface ===" | tee -a "$LOG"
./python - <<'PY' 2>&1 | tee -a "$LOG"
import os, sys, json, pathlib, time
print("python:", sys.version.split()[0])
import _posixshmem
print("_posixshmem ok")
import decimal, _decimal
print("_decimal ok:", getattr(_decimal, "__file__", "built-in"))
import tkinter, _tkinter
print("_tkinter ok:", getattr(_tkinter, "__file__", "built-in"))

tok = pathlib.Path("/data/data/com.termux/files/home/Braxon/assets/braxon_core/source_ingest/braxon_transport/tokenizer.json")
t0 = time.time()
data = json.loads(tok.read_text(errors="replace"))
print("tokenizer ok:", len(data.get("model", {}).get("vocab", {})), "elapsed", round(time.time() - t0, 3))
PY

echo "=== recursion boundary verification ===" | tee -a "$LOG"
./python - <<'PY' 2>&1 | tee -a "$LOG"
import functools, sys, resource
print("recursionlimit:", sys.getrecursionlimit())
print("stack:", resource.getrlimit(resource.RLIMIT_STACK))

@functools.lru_cache(maxsize=None)
def f(n):
    return 0 if n <= 0 else f(n - 1) + 1

for n in (100, 250, 500, 750, 900):
    f.cache_clear()
    print("lru recursion", n, "=>", f(n))

print("recursion probe survived")
PY

ln -sf "$LOG" "$TC/rebuild_stack_matched_latest.log"
echo "log: $LOG"
