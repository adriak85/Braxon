#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$HOME/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
LANE="$TC/source_forge/alien_lanes/findutils"
BUILD="$(find "$LANE/build" -maxdepth 1 -type d -name 'findutils-*' | sort | tail -n 1)"
PREFIX="$LANE/prefix"
JOBS="${JOBS:-7}"
STAMP="$(date +%Y%m%d_%H%M%S)"
OUT="$TC/fix_findutils_contract_first_android_$STAMP.log"
REPORT_DIR="$LANE/reports"
LOCK_DIR="$LANE/locks"

mkdir -p "$REPORT_DIR" "$LOCK_DIR" "$PREFIX"

{
  echo "=== BRAXON FINDUTILS CONTRACT-FIRST ANDROID FIX ==="
  date
  echo "build=$BUILD"
  echo "prefix=$PREFIX"
  echo "jobs=$JOBS"
  echo

  cd "$BUILD"

  echo "=== contract rule ==="
  echo "Do not fake yes."
  echo "Native no is acceptable only when classified as:"
  echo "- compat_yes"
  echo "- shim_yes"
  echo "- disabled_by_design"
  echo "- android_boundary"
  echo "- unresolved_blocker"
  echo

  echo "=== clean failed/partial build ==="
  make clean || true
  make distclean || true

  echo
  echo "=== configure cache: disable broken Android SELinux path, keep qsort_r compat ==="
  cat > config.cache <<'EOF'
ac_cv_func_qsort_r=no
gl_cv_func_qsort_r_signature=none
gl_cv_func_qsort_r_works=no

ac_cv_header_selinux_selinux_h=no
ac_cv_header_selinux_context_h=no
ac_cv_header_selinux_label_h=no
ac_cv_header_selinux_flask_h=no
ac_cv_search_setfilecon=no
gl_cv_next_selinux_selinux_h=no
EOF

  export CC="${CC:-/data/data/com.termux/files/usr/bin/clang}"
  export CXX="${CXX:-/data/data/com.termux/files/usr/bin/clang++}"
  export AR="${AR:-/data/data/com.termux/files/usr/bin/llvm-ar}"
  export RANLIB="${RANLIB:-/data/data/com.termux/files/usr/bin/llvm-ranlib}"

  export CPPFLAGS="${CPPFLAGS:-} -I/data/data/com.termux/files/usr/include"
  export CFLAGS="${CFLAGS:-} -O2 -fPIC"
  export LDFLAGS="${LDFLAGS:-} -L/data/data/com.termux/files/usr/lib"
  export LIB_SELINUX=""

  ./configure \
    --cache-file=config.cache \
    --prefix="$PREFIX" \
    --disable-nls \
    --without-selinux

  echo
  echo "=== hard assertions ==="
  grep -E 'HAVE_QSORT_R|qsort_r' config.h config.log || true

  if grep -q '^#define HAVE_QSORT_R 1' config.h; then
    echo "FAIL: qsort_r native path came back"
    exit 1
  fi

  if grep -q '^#define HAVE_SELINUX 1' config.h; then
    echo "FAIL: SELinux native path still enabled"
    exit 1
  fi

  if grep -q 'LIB_SELINUX *= *-lselinux' Makefile gl/lib/Makefile 2>/dev/null; then
    echo "FAIL: -lselinux still wired into Makefiles"
    exit 1
  fi

  echo "qsort_r=native_no_compat_yes"
  echo "selinux=disabled_by_design_android_missing_getfilecon_raw_lgetfilecon_raw"

  echo
  echo "=== build j$JOBS ==="
  if ! make -j "$JOBS"; then
    echo
    echo "=== HARD BLOCK DETECTED ==="
    grep -RInE 'ld\.lld: error|undefined symbol|fatal error|No such file|Error [0-9]' . config.log 2>/dev/null | tail -120 \
      | tee "$REPORT_DIR/findutils_hard_block_$STAMP.txt"
    exit 1
  fi

  echo
  echo "=== install ==="
  make install

  echo
  echo "=== no/bridge contract matrix ==="
  MATRIX="$REPORT_DIR/findutils_android_contract_matrix_$STAMP.tsv"
  {
    printf "item\tclassification\taction\n"
    printf "qsort_r\tcompat_yes\tuse gnulib qsort compatibility path; do not force native yes\n"
    printf "selinux/getfilecon_raw/lgetfilecon_raw\tdisabled_by_design\tAndroid libselinux lacks required raw path; keep disabled unless Braxon provides complete shim contract\n"
    printf "priv.h\tandroid_boundary\tSolaris/Illumos privilege API; do not build unless Braxon designs equivalent privilege contract\n"
    printf "getexecname\tandroid_boundary\tnon-Bionic process-name API; use existing program-name compat path\n"
    printf "secure_getenv/__secure_getenv\tcompat_yes\tuse gnulib/controlled fallback; do not fake glibc semantics\n"
    printf "glibc sys/cdefs\tandroid_boundary\tBionic is not glibc; do not force glibc identity\n"
    printf "sys/inttypes.h sys/bitypes.h\tandroid_boundary\tlegacy system headers absent; stdint/inttypes path is valid\n"
    printf "CoreFoundation APIs\tandroid_boundary\tApple-only APIs; disabled by design\n"
    printf "program_invocation_name/program_invocation_short_name\tcompat_yes\tuse gnulib program name fallback\n"
    printf "rawmemchr\tcompat_yes\tuse gnulib replacement unless Braxon native bridge is explicitly provided\n"
    printf "rpmatch\tcompat_yes\tuse gnulib/locale fallback unless Braxon locale contract is added\n"
    printf "timezone_t\tandroid_boundary\tnot Bionic public API; use gnulib time_rz fallback\n"
    printf "getppriv\tandroid_boundary\tSolaris privilege API; not Android native\n"
    printf "struct random_data/random_r\tcompat_yes\tuse gnulib random_r compatibility path\n"
    printf "sys/mnttab.h sys/mntio.h sys/ucred.h sys/fs_types.h\tandroid_boundary\tBSD/Solaris/Darwin mount/cred headers; classify, do not fake\n"
    printf "pthread robust mutex\tunresolved_blocker_or_android_boundary\tbuild only if Braxon needs exact robust mutex contract; otherwise document disabled boundary\n"
  } | tee "$MATRIX"

  echo
  echo "=== version proof ==="
  "$PREFIX/bin/find" --version | tee "$REPORT_DIR/prefix_find_version_$STAMP.txt"
  "$PREFIX/bin/xargs" --version | tee "$REPORT_DIR/prefix_xargs_version_$STAMP.txt"
  "$PREFIX/bin/find" "$ROOT" -maxdepth 1 -type f | head -30 | tee "$REPORT_DIR/prefix_find_probe_$STAMP.txt"

  echo
  echo "=== lock ==="
  LOCK="$LOCK_DIR/LOCKED_FINDUTILS_ANDROID_CONTRACT_FIRST_$STAMP.txt"
  {
    echo "BRAXON_FINDUTILS_ANDROID_CONTRACT_FIRST_FIXED=1"
    date
    echo "prefix=$PREFIX"
    echo "qsort_r=native_no_compat_yes"
    echo "selinux=disabled_by_design_android_missing_getfilecon_raw_lgetfilecon_raw"
    echo "matrix=$MATRIX"
    "$PREFIX/bin/find" --version | head -1
    "$PREFIX/bin/xargs" --version | head -1
  } > "$LOCK"

  find "$PREFIX/bin" "$REPORT_DIR" "$LOCK_DIR" -type f -print0 | sort -z | xargs -0 sha256sum \
    > "$LOCK_DIR/findutils_contract_first_manifest_$STAMP.sha256"

  ln -sf "$MATRIX" "$REPORT_DIR/findutils_android_contract_matrix_latest.tsv"
  ln -sf "$LOCK" "$LOCK_DIR/LOCKED_FINDUTILS_ANDROID_CONTRACT_FIRST_latest.txt"

  echo
  echo "DONE"
  echo "log=$OUT"
  echo "matrix=$MATRIX"
  echo "lock=$LOCK"
} 2>&1 | tee "$OUT"

ln -sf "$OUT" "$TC/fix_findutils_contract_first_android_latest.log"
