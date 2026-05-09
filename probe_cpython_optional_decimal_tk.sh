#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
CPY="$TC/src/cpython"
ADOPT="$TC/adoption/include/braxon_android_posix_adoption_force.h"
LOG="$TC/probe_optional_decimal_tk_$(date +%Y%m%d_%H%M%S).log"

export PATH="/data/data/com.termux/files/usr/bin:$PATH"
cd "$CPY"

{
  echo "=== optional module probe: _decimal / _tkinter ==="
  date
  echo "CPY=$CPY"
  echo "ADOPT=$ADOPT"
  echo
  echo "=== apt candidates ==="
  pkg search 'mpdecimal|libmpdec|tcl|tk|x11' || true
  echo
  echo "=== installed candidates ==="
  pkg list-installed | grep -Ei 'mpdecimal|libmpdec|tcl|tk|x11|xorg' || true
  echo
  echo "=== pkg-config ==="
  pkg-config --list-all 2>/dev/null | grep -Ei 'mpdec|tcl|tk|x11' || true
  echo
  echo "=== headers ==="
  find /data/data/com.termux/files/usr/include -iname 'mpdecimal.h' -o -iname 'tcl.h' -o -iname 'tk.h' 2>/dev/null || true
  echo
  echo "=== libs ==="
  find /data/data/com.termux/files/usr/lib "$TC" -iname 'libmpdec*' -o -iname 'libtcl*' -o -iname 'libtk*' -o -iname 'libX11*' 2>/dev/null || true
  echo
  echo "=== config.log hints ==="
  grep -nEi '_decimal|mpdecimal|mpdec|_tkinter|tcl|tk' config.log 2>/dev/null | tail -n 200 || true
} | tee "$LOG"

echo
echo "probe log: $LOG"
