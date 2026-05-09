#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
CPY="$TC/src/cpython"
ADOPT="$TC/adoption/include/braxon_android_posix_adoption_force.h"
OVERLAY="$TC/install/braxon_android_overlay"
LOG="$TC/link_braxon_cpython_deep_$(date +%Y%m%d_%H%M%S).log"

export PATH="/data/data/com.termux/files/usr/bin:$PATH"
export LD_LIBRARY_PATH="$OVERLAY/lib:/data/data/com.termux/files/usr/lib:${LD_LIBRARY_PATH:-}"
export CFLAGS_NODIST="${CFLAGS_NODIST:-} -include $ADOPT"
export LDFLAGS_NODIST="${LDFLAGS_NODIST:-} -L$OVERLAY/lib -Wl,-rpath,$OVERLAY/lib"
export LLVM_PROFILE_FILE="$TC/profile_catalog/raw/cpython-%p-%m.profraw"

cd "$CPY"

{
  echo "=== Braxon-linked CPython deep seating ==="
  date
  echo "CPY=$CPY"
  echo "ADOPT=$ADOPT"
  echo "OVERLAY=$OVERLAY"
  echo "LD_LIBRARY_PATH=$LD_LIBRARY_PATH"
  echo "CFLAGS_NODIST=$CFLAGS_NODIST"
  echo "LDFLAGS_NODIST=$LDFLAGS_NODIST"
  echo
  echo "=== Braxon overlay libs ==="
  find "$OVERLAY/lib" -maxdepth 1 -type f -name '*.so*' -print | sort || true
  echo
} | tee "$LOG"

echo "=== relink python against Braxon overlay ===" | tee -a "$LOG"
rm -f python Programs/python.o

make -j2 V=1 \
  CFLAGS_NODIST="$CFLAGS_NODIST" \
  LDFLAGS_NODIST="$LDFLAGS_NODIST" \
  Programs/python.o python \
  2>&1 | tee -a "$LOG"

echo "=== runtime dependency check ===" | tee -a "$LOG"
ldd ./python 2>&1 | tee -a "$LOG" || true

echo "=== Braxon contract smoke ===" | tee -a "$LOG"
./python - <<'PY' 2>&1 | tee -a "$LOG"
import os, sys, sysconfig, pwd, grp, json, pathlib, time
print("python:", sys.version)
print("platform:", sysconfig.get_platform())
print("executable:", sys.executable)
print("pwd:", pwd.getpwuid(os.getuid()))
print("grp:", grp.getgrgid(os.getgid()))

mods = ["_posixshmem", "_decimal", "_tkinter"]
for m in mods:
    try:
        mod = __import__(m)
        print(m, "ok", getattr(mod, "__file__", "built-in"))
    except Exception as e:
        print(m, "missing/fail:", repr(e))

tokenizer = pathlib.Path("/data/data/com.termux/files/home/Braxon/assets/braxon_core/source_ingest/braxon_transport/tokenizer.json")
if tokenizer.exists():
    t0 = time.time()
    data = json.loads(tokenizer.read_text(errors="replace"))
    vocab = data.get("model", {}).get("vocab", {})
    print("tokenizer ok:", tokenizer.stat().st_size, "bytes", "vocab", len(vocab), "elapsed", round(time.time() - t0, 3))
else:
    print("tokenizer missing:", tokenizer)
PY

echo "=== deep recursion boundary probe, non-fatal ===" | tee -a "$LOG"
(
  ulimit -s 65532 || true
  ./python - <<'PY'
import functools, sys, resource
print("recursionlimit:", sys.getrecursionlimit())
try:
    print("stack:", resource.getrlimit(resource.RLIMIT_STACK))
except Exception as e:
    print("stack query failed:", e)

@functools.lru_cache(maxsize=None)
def f(n):
    return 0 if n <= 0 else f(n-1) + 1

for n in (100, 250, 500, 750, 900):
    try:
        f.cache_clear()
        print("lru recursion", n, "=>", f(n))
    except BaseException as e:
        print("lru recursion failed at", n, repr(e))
        break
PY
) 2>&1 | tee -a "$LOG" || true

ln -sf "$LOG" "$TC/link_braxon_cpython_deep_latest.log"
echo "log: $LOG"
