#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
CPY="$TC/src/cpython"
ADOPT="$TC/adoption/include/braxon_android_posix_adoption_force.h"
MPDEC="$TC/adoption/mpdecimal"
LOG="$TC/fix_decimal_tk_detect_$(date +%Y%m%d_%H%M%S).log"

export PATH="/data/data/com.termux/files/usr/bin:$PATH"
cd "$CPY"

{
  echo "=== install Tcl/Tk headers/libs ==="
  pkg install -y tcl tk xorgproto libx11 libxext libxft libxrender pkg-config make clang binutils
} 2>&1 | tee "$LOG"

echo "=== build local Bionic mpdecimal from bundled source ===" | tee -a "$LOG"
rm -rf "$MPDEC"
mkdir -p "$MPDEC"

cd "$CPY/Modules/_decimal/libmpdec"

if [ -x ./configure ]; then
  ./configure --prefix="$MPDEC" 2>&1 | tee -a "$LOG"
  make -j2 2>&1 | tee -a "$LOG"
  make install 2>&1 | tee -a "$LOG"
else
  echo "No libmpdec configure script found. Listing source:" | tee -a "$LOG"
  find . -maxdepth 2 -type f | sort | tee -a "$LOG"
  exit 1
fi

cd "$CPY"

echo "=== detect Tcl/Tk pkg-config names ===" | tee -a "$LOG"
TCL_PC=""
TK_PC=""

for n in tcl8.6 tcl86 tcl; do
  if pkg-config --exists "$n"; then TCL_PC="$n"; break; fi
done

for n in tk8.6 tk86 tk; do
  if pkg-config --exists "$n"; then TK_PC="$n"; break; fi
done

test -n "$TCL_PC" || { echo "could not find Tcl pkg-config file"; exit 1; }
test -n "$TK_PC" || { echo "could not find Tk pkg-config file"; exit 1; }

export LIBMPDEC_CFLAGS="-I$MPDEC/include"
export LIBMPDEC_LIBS="-L$MPDEC/lib -lmpdec -lm"
export TCLTK_CFLAGS="$(pkg-config --cflags "$TCL_PC" "$TK_PC")"
export TCLTK_LIBS="$(pkg-config --libs "$TCL_PC" "$TK_PC")"
export CFLAGS_NODIST="${CFLAGS_NODIST:-} -include $ADOPT"
export LDFLAGS_NODIST="${LDFLAGS_NODIST:-} -L$MPDEC/lib"

{
  echo "LIBMPDEC_CFLAGS=$LIBMPDEC_CFLAGS"
  echo "LIBMPDEC_LIBS=$LIBMPDEC_LIBS"
  echo "TCLTK_CFLAGS=$TCLTK_CFLAGS"
  echo "TCLTK_LIBS=$TCLTK_LIBS"
  echo "CFLAGS_NODIST=$CFLAGS_NODIST"
  echo "LDFLAGS_NODIST=$LDFLAGS_NODIST"
} | tee -a "$LOG"

echo "=== rerun configure with prior args plus detected deps ===" | tee -a "$LOG"
CONFIG_ARGS="$(./config.status --config 2>/dev/null || true)"
test -n "$CONFIG_ARGS" || { echo "could not recover configure args from config.status"; exit 1; }

bash -lc "
cd '$CPY'
LIBMPDEC_CFLAGS='$LIBMPDEC_CFLAGS' \
LIBMPDEC_LIBS='$LIBMPDEC_LIBS' \
TCLTK_CFLAGS='$TCLTK_CFLAGS' \
TCLTK_LIBS='$TCLTK_LIBS' \
CFLAGS_NODIST='$CFLAGS_NODIST' \
LDFLAGS_NODIST='$LDFLAGS_NODIST' \
./configure $CONFIG_ARGS
" 2>&1 | tee -a "$LOG"

echo "=== rebuild extension state ===" | tee -a "$LOG"
rm -f pybuilddir.txt platform
make -j2 V=1 CFLAGS_NODIST="$CFLAGS_NODIST" LDFLAGS_NODIST="$LDFLAGS_NODIST" 2>&1 | tee -a "$LOG"

echo "=== verify imports ===" | tee -a "$LOG"
LD_LIBRARY_PATH="$MPDEC/lib:${LD_LIBRARY_PATH:-}" ./python - <<'PY' 2>&1 | tee -a "$LOG"
import decimal, _decimal
print("_decimal ok:", _decimal)
try:
    import tkinter, _tkinter
    print("_tkinter ok:", _tkinter)
except Exception as e:
    print("_tkinter import failed:", repr(e))
    raise
PY

ln -sf "$LOG" "$TC/fix_decimal_tk_detect_latest.log"
echo "log: $LOG"
