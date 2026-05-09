#!/data/data/com.termux/files/usr/bin/bash
set -u

unalias make clang cc tee rm cp mv readelf nm 2>/dev/null || true
hash -r 2>/dev/null || true

export PATH="/data/data/com.termux/files/usr/bin:$PATH"

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
SRC="$TC/src/cpython"
ADOPT="$TC/adoption/include/braxon_android_posix_adoption_force.h"
EXT_SRC="$TC/adoption/src/braxon_android_libc_extensions.c"
LIBDIR="$TC/install/sysroot/usr/lib"
LIB="$LIBDIR/libbraxon_android_libc_extensions.so"
LOG="$TC/fix_extension_link_$(date +%Y%m%d_%H%M%S).log"

cd "$SRC" || exit 1
mkdir -p "$LIBDIR"

export LD_LIBRARY_PATH="$LIBDIR:${LD_LIBRARY_PATH:-}"
export CFLAGS_NODIST="${CFLAGS_NODIST:-} -include $ADOPT"
export LDFLAGS_NODIST="${LDFLAGS_NODIST:-} -L$LIBDIR -Wl,-rpath,$LIBDIR"
export LIBS="${LIBS:-} -lbraxon_android_libc_extensions"

{
  echo "=== Braxon CPython extension link repair ==="
  date
  pwd
  echo "ADOPT=$ADOPT"
  echo "EXT_SRC=$EXT_SRC"
  echo "LIB=$LIB"
  echo "LD_LIBRARY_PATH=$LD_LIBRARY_PATH"
  echo
} | tee "$LOG"

clang -shared -fPIC -O3 -Wall -Wextra \
  "$EXT_SRC" \
  -Wl,-soname,libbraxon_android_libc_extensions.so \
  -o "$LIB" 2>&1 | tee -a "$LOG"

echo "=== exported symbols ===" | tee -a "$LOG"
readelf -Ws "$LIB" 2>/dev/null | grep -E 'braxon_android_(futimes|lutimes|setns|unshare|setpwent|getpwent|endpwent)' | tee -a "$LOG"

for sym in \
  braxon_android_futimes \
  braxon_android_lutimes \
  braxon_android_setns \
  braxon_android_unshare \
  braxon_android_setpwent \
  braxon_android_getpwent \
  braxon_android_endpwent
do
  readelf -Ws "$LIB" | grep -q " $sym$" || {
    echo "missing exported symbol: $sym" | tee -a "$LOG"
    exit 1
  }
done

rm -f Programs/_freeze_module python

command make -j2 V=1 \
  CFLAGS_NODIST="$CFLAGS_NODIST" \
  LDFLAGS_NODIST="$LDFLAGS_NODIST" \
  LIBS="$LIBS" \
  Programs/_freeze_module \
  python \
  2>&1 | tee -a "$LOG"

STATUS=${PIPESTATUS[0]}
echo "target relink status: $STATUS" | tee -a "$LOG"
test "$STATUS" -eq 0 || exit "$STATUS"

./python - <<'PY' 2>&1 | tee -a "$LOG"
import sys, os, pwd, _posixshmem
print("python:", sys.version)
print("_posixshmem:", _posixshmem)
print("pwd:", pwd.getpwuid(os.getuid()))
PY

ln -sf "$LOG" "$TC/fix_extension_link_latest.log"
exit 0
