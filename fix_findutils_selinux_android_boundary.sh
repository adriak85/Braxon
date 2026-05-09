#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$HOME/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
LANE="$TC/source_forge/alien_lanes/findutils"
BUILD="$(find "$LANE/build" -maxdepth 1 -type d -name 'findutils-*' | sort | tail -n 1)"
PREFIX="$LANE/prefix"
STAMP="$(date +%Y%m%d_%H%M%S)"
OUT="$TC/fix_findutils_selinux_android_boundary_$STAMP.log"
JOBS="${JOBS:-7}"

mkdir -p "$LANE/reports" "$LANE/locks"

{
  echo "=== fix findutils SELinux Android boundary ==="
  date
  echo "build=$BUILD"
  echo "prefix=$PREFIX"

  cd "$BUILD"

  echo
  echo "=== previous failure proof ==="
  grep -n 'getfilecon_raw\|lgetfilecon_raw\|libselinux\|-lselinux\|selinux' config.log Makefile gl/lib/Makefile 2>/dev/null | head -200 || true

  echo
  echo "=== clean failed build ==="
  make clean || true

  export CC="${CC:-/data/data/com.termux/files/usr/bin/clang}"
  export CXX="${CXX:-/data/data/com.termux/files/usr/bin/clang++}"
  export AR="${AR:-/data/data/com.termux/files/usr/bin/llvm-ar}"
  export RANLIB="${RANLIB:-/data/data/com.termux/files/usr/bin/llvm-ranlib}"
  export CFLAGS="-O2 -fPIC"
  export CPPFLAGS="-I/data/data/com.termux/files/usr/include"
  export LDFLAGS="-L/data/data/com.termux/files/usr/lib"

  echo
  echo "=== reconfigure: qsort_r compat + selinux disabled ==="
  ac_cv_func_qsort_r=no \
  gl_cv_func_qsort_r_signature=none \
  gl_cv_func_qsort_r_works=no \
  ac_cv_header_selinux_selinux_h=no \
  ac_cv_header_selinux_context_h=no \
  ac_cv_header_selinux_label_h=no \
  ac_cv_lib_selinux_setfilecon=no \
  ac_cv_search_setfilecon=no \
  ./configure \
    --prefix="$PREFIX" \
    --host=aarch64-linux-android \
    --build=aarch64-linux-android \
    --disable-nls \
    --without-selinux

  echo
  echo "=== hard asserts ==="
  grep -n 'HAVE_QSORT_R\|qsort_r\|HAVE_SELINUX\|selinux\|getfilecon\|setfilecon' config.h config.log \
    | tee "$LANE/reports/qsort_selinux_contract_after_reconfigure.txt" || true

  if grep -q '^#define HAVE_QSORT_R 1' config.h; then
    echo "FAIL: qsort_r native path came back"
    exit 1
  fi

  if grep -q '^#define HAVE_SELINUX' config.h; then
    echo "FAIL: SELinux native path still enabled"
    exit 1
  fi

  echo
  echo "=== build j$JOBS ==="
  make -j "$JOBS"

  echo
  echo "=== install prefix ==="
  make install

  echo
  echo "=== prefix proof ==="
  "$PREFIX/bin/find" --version | tee "$LANE/reports/prefix_find_version_selinux_fixed.txt"
  "$PREFIX/bin/xargs" --version | tee "$LANE/reports/prefix_xargs_version_selinux_fixed.txt"
  "$PREFIX/bin/find" "$ROOT" -maxdepth 1 -type f | head -30 | tee "$LANE/reports/prefix_find_probe_selinux_fixed.txt"

  echo
  echo "=== lock ==="
  {
    echo "BRAXON_FINDUTILS_ANDROID_SELINUX_BOUNDARY_FIXED=1"
    date
    echo "qsort_r=native_no_compat_yes"
    echo "selinux=disabled_for_android_missing_getfilecon_raw_lgetfilecon_raw"
    echo "prefix=$PREFIX"
    "$PREFIX/bin/find" --version | head -1
    "$PREFIX/bin/xargs" --version | head -1
  } > "$LANE/locks/LOCKED_FINDUTILS_ANDROID_SELINUX_BOUNDARY_FIXED.txt"

  find "$PREFIX/bin" "$LANE/reports" "$LANE/locks" -type f -print0 | sort -z | xargs -0 sha256sum \
    > "$LANE/locks/selinux_boundary_fixed_manifest.sha256"

  echo
  echo "DONE"
  echo "log=$OUT"
} 2>&1 | tee "$OUT"

ln -sf "$OUT" "$TC/fix_findutils_selinux_android_boundary_latest.log"
