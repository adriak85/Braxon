#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
CPY="$TC/src/cpython"
ADOPT="$TC/adoption/include/braxon_android_posix_adoption_force.h"
LOG="$TC/build_tkinter_native_$(date +%Y%m%d_%H%M%S).log"

export PATH="/data/data/com.termux/files/usr/bin:$PATH"
cd "$CPY"

{
  echo "=== install native Tk deps ==="
  pkg install -y tcl tk xorgproto libx11 libxext libxft libxrender libxss pkg-config clang make binutils
} 2>&1 | tee "$LOG"

EXT_SUFFIX="$(./python - <<'PY'
import sysconfig
print(sysconfig.get_config_var("EXT_SUFFIX"))
PY
)"

BUILDLIB="$(cat pybuilddir.txt)"
mkdir -p "$BUILDLIB"

TCLTK_CFLAGS="$(pkg-config --cflags tcl tk x11 xft xrender xext 2>/dev/null || pkg-config --cflags tcl tk)"
TCLTK_LIBS="$(pkg-config --libs tcl tk x11 xft xrender xext 2>/dev/null || pkg-config --libs tcl tk)"

echo "EXT_SUFFIX=$EXT_SUFFIX" | tee -a "$LOG"
echo "BUILDLIB=$BUILDLIB" | tee -a "$LOG"
echo "TCLTK_CFLAGS=$TCLTK_CFLAGS" | tee -a "$LOG"
echo "TCLTK_LIBS=$TCLTK_LIBS" | tee -a "$LOG"

rm -f Modules/_tkinter.o "$BUILDLIB/_tkinter$EXT_SUFFIX"

clang \
  -fPIC -O3 -DNDEBUG \
  -I. -I./Include -I./Include/internal \
  -include "$ADOPT" \
  $TCLTK_CFLAGS \
  -c Modules/_tkinter.c \
  -o Modules/_tkinter.o \
  2>&1 | tee -a "$LOG"

clang \
  -shared -fuse-ld=lld \
  Modules/_tkinter.o \
  -o "$BUILDLIB/_tkinter$EXT_SUFFIX" \
  $TCLTK_LIBS \
  -lm \
  2>&1 | tee -a "$LOG"

echo "=== verify _tkinter ===" | tee -a "$LOG"
LD_LIBRARY_PATH="/data/data/com.termux/files/usr/lib:${LD_LIBRARY_PATH:-}" ./python - <<'PY' 2>&1 | tee -a "$LOG"
import _tkinter
print("_tkinter ok:", _tkinter)
print("TclVersion:", _tkinter.TCL_VERSION)
print("TkVersion:", _tkinter.TK_VERSION)

import tkinter
print("tkinter ok:", tkinter.TkVersion)
PY

ln -sf "$LOG" "$TC/build_tkinter_native_latest.log"
echo "log: $LOG"
