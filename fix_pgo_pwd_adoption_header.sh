#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
SRC="$TC/src/cpython"
LIBDIR="$TC/install/sysroot/usr/lib"
ADOPT="$TC/adoption/include/braxon_android_posix_adoption_force.h"
LOG="$TC/fix_pgo_pwd_adoption_header_$(date +%Y%m%d_%H%M%S).log"

cd "$SRC"
export PATH="/data/data/com.termux/files/usr/bin:$PATH"
export LD_LIBRARY_PATH="$LIBDIR:${LD_LIBRARY_PATH:-}"

test -f "$ADOPT" || { echo "missing adoption header: $ADOPT"; exit 1; }

export CFLAGS_NODIST="-include $ADOPT ${CFLAGS_NODIST:-}"
export CFLAGS="-include $ADOPT ${CFLAGS:-}"
export CPPFLAGS="-include $ADOPT ${CPPFLAGS:-}"
export LDFLAGS_NODIST="-L$LIBDIR -Wl,-rpath,$LIBDIR ${LDFLAGS_NODIST:-}"
export LIBS="-Wl,--no-as-needed $LIBDIR/libbraxon_android_libc_extensions.so -Wl,--as-needed -ldl -llog -lm ${LIBS:-}"

{
  echo "=== Braxon PGO pwd adoption-header repair ==="
  date
  echo "SRC=$SRC"
  echo "ADOPT=$ADOPT"
  echo "CFLAGS_NODIST=$CFLAGS_NODIST"
  echo "CFLAGS=$CFLAGS"
  echo "CPPFLAGS=$CPPFLAGS"
  echo "LIBS=$LIBS"
  echo
} | tee "$LOG"

rm -f Modules/pwdmodule.o python

make -j2 V=1 \
  CFLAGS_NODIST="$CFLAGS_NODIST" \
  CFLAGS="$CFLAGS" \
  CPPFLAGS="$CPPFLAGS" \
  LDFLAGS_NODIST="$LDFLAGS_NODIST" \
  LIBS="$LIBS" \
  Modules/pwdmodule.o \
  python \
  2>&1 | tee -a "$LOG"

STATUS=${PIPESTATUS[0]}
echo "target status: $STATUS" | tee -a "$LOG"
test "$STATUS" -eq 0 || exit "$STATUS"

make -j2 V=1 \
  CFLAGS_NODIST="$CFLAGS_NODIST" \
  CFLAGS="$CFLAGS" \
  CPPFLAGS="$CPPFLAGS" \
  LDFLAGS_NODIST="$LDFLAGS_NODIST" \
  LIBS="$LIBS" \
  2>&1 | tee -a "$LOG"

STATUS=${PIPESTATUS[0]}
echo "full status: $STATUS" | tee -a "$LOG"
ln -sf "$LOG" "$TC/fix_pgo_pwd_adoption_header_latest.log"
exit "$STATUS"
