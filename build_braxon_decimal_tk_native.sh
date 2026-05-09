#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
CPY="$TC/src/cpython"
ADOPT="$TC/adoption/include/braxon_android_posix_adoption_force.h"
LOG="$TC/build_decimal_tk_native_$(date +%Y%m%d_%H%M%S).log"

export PATH="/data/data/com.termux/files/usr/bin:$PATH"
cd "$CPY"

{
  echo "=== install native Bionic deps ==="
  pkg install -y tcl tk xorgproto libx11 libxext libxft libxrender pkg-config clang make binutils
} 2>&1 | tee "$LOG"

export CFLAGS_NODIST="${CFLAGS_NODIST:-} -include $ADOPT"
export TCLTK_CFLAGS="$(pkg-config --cflags tcl tk)"
export TCLTK_LIBS="$(pkg-config --libs tcl tk x11 xft xrender xext || pkg-config --libs tcl tk)"

echo "=== build internal bundled _decimal manually ===" | tee -a "$LOG"

EXT_SUFFIX="$(./python - <<'PY'
import sysconfig
print(sysconfig.get_config_var("EXT_SUFFIX"))
PY
)"

BUILDLIB="$(cat pybuilddir.txt 2>/dev/null || ./python - <<'PY'
import sysconfig
print(sysconfig.get_config_var("DESTSHARED").rsplit("/", 1)[0])
PY
)"
mkdir -p "$BUILDLIB"

DEC_BUILD="$TC/adoption/build_decimal_native"
rm -rf "$DEC_BUILD"
mkdir -p "$DEC_BUILD"

DEC_CFLAGS="-fPIC -O3 -DNDEBUG -DCONFIG_64 -DANSI -I. -I./Include -I./Include/internal -I./Modules/_decimal -I./Modules/_decimal/libmpdec $CFLAGS_NODIST"

for src in \
  Modules/_decimal/libmpdec/basearith.c \
  Modules/_decimal/libmpdec/constants.c \
  Modules/_decimal/libmpdec/context.c \
  Modules/_decimal/libmpdec/convolute.c \
  Modules/_decimal/libmpdec/crt.c \
  Modules/_decimal/libmpdec/difradix2.c \
  Modules/_decimal/libmpdec/fnt.c \
  Modules/_decimal/libmpdec/fourstep.c \
  Modules/_decimal/libmpdec/io.c \
  Modules/_decimal/libmpdec/mpalloc.c \
  Modules/_decimal/libmpdec/mpdecimal.c \
  Modules/_decimal/libmpdec/mpsignal.c \
  Modules/_decimal/libmpdec/numbertheory.c \
  Modules/_decimal/libmpdec/sixstep.c \
  Modules/_decimal/libmpdec/transpose.c \
  Modules/_decimal/_decimal.c
do
  obj="$DEC_BUILD/$(basename "$src" .c).o"
  clang $DEC_CFLAGS -c "$src" -o "$obj" 2>&1 | tee -a "$LOG"
done

clang -shared -fuse-ld=lld \
  "$DEC_BUILD"/*.o \
  -o "$BUILDLIB/_decimal$EXT_SUFFIX" \
  -lm 2>&1 | tee -a "$LOG"

echo "=== build _tkinter through CPython build system ===" | tee -a "$LOG"

export CFLAGS_NODIST
export TCLTK_CFLAGS
export TCLTK_LIBS

rm -f Modules/_tkinter.o "$BUILDLIB/_tkinter$EXT_SUFFIX"

make -j2 V=1 \
  CFLAGS_NODIST="$CFLAGS_NODIST $TCLTK_CFLAGS" \
  LDFLAGS_NODIST="${LDFLAGS_NODIST:-}" \
  sharedmods \
  2>&1 | tee -a "$LOG"

echo "=== verify ===" | tee -a "$LOG"
./python - <<'PY' 2>&1 | tee -a "$LOG"
import sys
print("python:", sys.version.split()[0])

import decimal, _decimal
print("_decimal:", _decimal.__file__ if hasattr(_decimal, "__file__") else _decimal)

import tkinter, _tkinter
print("_tkinter:", _tkinter.__file__ if hasattr(_tkinter, "__file__") else _tkinter)

print("decimal test:", decimal.Decimal("1.1") + decimal.Decimal("2.2"))
PY

echo "=== optional-module summary ===" | tee -a "$LOG"
./python -E -c 'import sysconfig, sys; print(sysconfig.get_platform(), sys.version)' 2>&1 | tee -a "$LOG"

ln -sf "$LOG" "$TC/build_decimal_tk_native_latest.log"
echo "log: $LOG"
