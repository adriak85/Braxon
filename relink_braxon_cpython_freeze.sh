#!/data/data/com.termux/files/usr/bin/bash
set -u

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
SRC="$TC/src/cpython"
LIBDIR="$TC/install/sysroot/usr/lib"
LIB="$LIBDIR/libbraxon_android_libc_extensions.so"
ADOPT="$TC/adoption/include/braxon_android_posix_adoption_force.h"
LOG="$TC/relink_freeze_$(date +%Y%m%d_%H%M%S).log"

export PATH="/data/data/com.termux/files/usr/bin:$PATH"
export LD_LIBRARY_PATH="$LIBDIR:${LD_LIBRARY_PATH:-}"

test -d "$SRC" || { echo "missing source dir: $SRC"; exit 1; }
test -f "$LIB" || { echo "missing lib: $LIB"; exit 1; }
test -f "$ADOPT" || { echo "missing adoption header: $ADOPT"; exit 1; }

export CFLAGS_NODIST="${CFLAGS_NODIST:-} -include $ADOPT"
export LDFLAGS_NODIST="-L$LIBDIR -Wl,-rpath,$LIBDIR ${LDFLAGS_NODIST:-}"
export LIBS="-Wl,--no-as-needed $LIB -Wl,--as-needed -ldl -llog -lm ${LIBS:-}"

{
  echo "=== Braxon CPython freeze relink ==="
  date
  echo "SRC=$SRC"
  echo "LIB=$LIB"
  echo "ADOPT=$ADOPT"
  echo "LIBS=$LIBS"
  echo
} | tee "$LOG"

make -C "$SRC" -j2 V=1 \
  CFLAGS_NODIST="$CFLAGS_NODIST" \
  LDFLAGS_NODIST="$LDFLAGS_NODIST" \
  LIBS="$LIBS" \
  Programs/_freeze_module \
  python \
  2>&1 | tee -a "$LOG"

STATUS=${PIPESTATUS[0]}
echo "relink status: $STATUS" | tee -a "$LOG"
test "$STATUS" -eq 0 || exit "$STATUS"

make -C "$SRC" -j2 V=1 \
  CFLAGS_NODIST="$CFLAGS_NODIST" \
  LDFLAGS_NODIST="$LDFLAGS_NODIST" \
  LIBS="$LIBS" \
  2>&1 | tee -a "$LOG"

STATUS=${PIPESTATUS[0]}
echo "full status: $STATUS" | tee -a "$LOG"
ln -sf "$LOG" "$TC/relink_freeze_latest.log"
exit "$STATUS"
