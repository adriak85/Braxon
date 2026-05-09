#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$HOME/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
SRC="$TC/source_forge"
LANE="$SRC/alien_lanes/findutils"
BUILD="$(find "$LANE/build" -maxdepth 1 -type d -name 'findutils-*' | sort | tail -n 1)"
PREFIX="$LANE/prefix"
TERMUX_BIN="/data/data/com.termux/files/usr/bin"
STAMP="$(date +%Y%m%d_%H%M%S)"
OUT="$TC/resume_findutils_contract_completion_$STAMP.log"
JOBS="${JOBS:-7}"

mkdir -p "$LANE/reports" "$LANE/locks" "$LANE/backups"

{
  echo "=== resume findutils with contract completion ==="
  date
  echo "build=$BUILD"
  echo "prefix=$PREFIX"

  cd "$BUILD"

  echo
  echo "=== backup failed build config ==="
  cp -f config.log "$LANE/reports/config.log.before_qsort_contract_$STAMP" 2>/dev/null || true
  cp -f config.h "$LANE/reports/config.h.before_qsort_contract_$STAMP" 2>/dev/null || true

  echo
  echo "=== force qsort_r to compat lane, not undeclared native ==="
  make clean || true

  export CC="${CC:-/data/data/com.termux/files/usr/bin/clang}"
  export CXX="${CXX:-/data/data/com.termux/files/usr/bin/clang++}"
  export AR="${AR:-/data/data/com.termux/files/usr/bin/llvm-ar}"
  export RANLIB="${RANLIB:-/data/data/com.termux/files/usr/bin/llvm-ranlib}"
  export CFLAGS="-O2 -fPIC"
  export CPPFLAGS="-I/data/data/com.termux/files/usr/include"
  export LDFLAGS="-L/data/data/com.termux/files/usr/lib"

  ac_cv_func_qsort_r=no \
  gl_cv_func_qsort_r_signature=none \
  gl_cv_func_qsort_r_works=no \
  ./configure \
    --prefix="$PREFIX" \
    --host=aarch64-linux-android \
    --build=aarch64-linux-android \
    --disable-nls

  echo
  echo "=== hard assert qsort_r is not treated as native ==="
  perl -0pi -e 's/#define HAVE_QSORT_R 1/\/* #undef HAVE_QSORT_R *\//g' config.h
  grep -n 'HAVE_QSORT_R\|qsort_r' config.h config.log | tee "$LANE/reports/qsort_r_contract_after_configure.txt" || true

  echo
  echo "=== build j$JOBS ==="
  make -j "$JOBS"

  echo
  echo "=== install prefix ==="
  make install

  echo
  echo "=== prove prefix ==="
  "$PREFIX/bin/find" --version | tee "$LANE/reports/prefix_find_version_after_contract.txt"
  "$PREFIX/bin/xargs" --version | tee "$LANE/reports/prefix_xargs_version_after_contract.txt"
  "$PREFIX/bin/find" "$ROOT" -maxdepth 1 -type f | head -30 | tee "$LANE/reports/prefix_find_probe_after_contract.txt"

  echo
  echo "=== backup current Termux find/xargs before overlay ==="
  BACKUP="$LANE/backups/termux-findutils-before-contract-overlay-$STAMP.tar.gz.bak"
  tar -czf "$BACKUP" -C "$TERMUX_BIN" find xargs
  sha256sum "$BACKUP" > "$BACKUP.sha256"

  echo
  echo "=== overlay proven find/xargs ==="
  install -m 0755 "$PREFIX/bin/find" "$TERMUX_BIN/find"
  install -m 0755 "$PREFIX/bin/xargs" "$TERMUX_BIN/xargs"

  echo
  echo "=== post-overlay proof ==="
  find --version | tee "$LANE/reports/overlay_find_version_after_contract.txt"
  xargs --version | tee "$LANE/reports/overlay_xargs_version_after_contract.txt"
  find "$ROOT" -maxdepth 1 -type f | head -30 | tee "$LANE/reports/overlay_find_probe_after_contract.txt"

  echo
  echo "=== no-to-net-yes contract report ==="
  {
    echo "BRAXON_FINDUTILS_CONTRACT_COMPLETION=1"
    echo "timestamp=$STAMP"
    echo "qsort_r=native_no_compat_completed"
    echo "rule=no fake yes; native no becomes net yes only when compat/replacement builds and runtime proof passes"
    echo
    echo "proofs:"
    echo "- prefix find version proved"
    echo "- prefix xargs version proved"
    echo "- overlay find version proved"
    echo "- overlay xargs version proved"
    echo "- qsort_r undeclared build failure resolved by compat contract"
  } > "$LANE/reports/NO_TO_NET_YES_CONTRACT_REPORT.txt"

  echo
  echo "=== lock ==="
  {
    echo "BRAXON_ALIEN_FINDUTILS_CONTRACT_COMPLETED_LOCK=1"
    date
    echo "prefix=$PREFIX"
    echo "backup=$BACKUP"
    find --version | head -1
    xargs --version | head -1
  } > "$LANE/locks/LOCKED_FINDUTILS_CONTRACT_COMPLETED.txt"

  find "$PREFIX/bin" "$LANE/reports" "$LANE/locks" -type f -print0 | sort -z | xargs -0 sha256sum \
    > "$LANE/locks/contract_completed_manifest.sha256"

  echo
  echo "DONE"
  echo "backup=$BACKUP"
  echo "restore:"
  echo "tar -xzf \"$BACKUP\" -C \"$TERMUX_BIN\""
  echo "log=$OUT"
} 2>&1 | tee "$OUT"

ln -sf "$OUT" "$TC/resume_findutils_contract_completion_latest.log"
