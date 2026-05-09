#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
cd "$ROOT"

STAMP="$(date +%Y%m%d_%H%M%S)"
CHAIN="$ROOT/state/full_android_language_toolchain"
RUN="$CHAIN/runs/cpython_native_contract_resolve_$STAMP"
REPORT="$RUN/reports"

SRC="$CHAIN/src"
CPY="$SRC/cpython"
INSTALL="$CHAIN/install"
PY_INSTALL="$INSTALL/python"

OVERLAY="$INSTALL/braxon_android_overlay"
OVERLAY_INCLUDE="$OVERLAY/include"
OVERLAY_LIB="$OVERLAY/lib"

STAGE="$INSTALL/braxon_android_builtin_stage"
STAGE_INCLUDE="$STAGE/include"
STAGE_LIB="$STAGE/lib"

LINKROOT="$INSTALL/braxon_android_active_links"
LINK_INCLUDE="$LINKROOT/include"
LINK_LIB="$LINKROOT/lib"

NATIVE="$CHAIN/native/android_libc_extensions"
NATIVE_SRC="$NATIVE/src"
NATIVE_PROOFS="$NATIVE/proofs"

mkdir -p \
  "$RUN" "$REPORT" \
  "$CPY" "$PY_INSTALL" \
  "$OVERLAY_INCLUDE" "$OVERLAY_LIB" \
  "$STAGE_INCLUDE" "$STAGE_LIB" \
  "$LINK_INCLUDE" "$LINK_LIB" \
  "$NATIVE_SRC" "$NATIVE_PROOFS" \
  scripts/toolchains config/toolchains

LOG="$RUN/resolve_cpython_android_native_contracts_full.log"
exec > >(tee "$LOG") 2>&1

run_logged() {
  label="$1"
  logfile="$2"
  shift 2

  echo
  echo "== $label =="
  echo "logfile=$logfile"
  printf 'command='
  printf '%q ' "$@"
  echo

  if command -v stdbuf >/dev/null 2>&1; then
    stdbuf -oL -eL "$@" 2>&1 | tee "$logfile"
  else
    "$@" 2>&1 | tee "$logfile"
  fi
}

echo "== Braxon CPython Android native contract resolver =="
echo "date=$(date -Is)"
echo "root=$ROOT"
echo "chain=$CHAIN"
echo "run=$RUN"
echo "cpython=$CPY"
echo "overlay=$OVERLAY"
echo "stage=$STAGE"
echo "linkroot=$LINKROOT"
echo "prefix=${PREFIX:-unset}"
echo "head=$(git rev-parse HEAD 2>/dev/null || true)"
echo "branch=$(git branch --show-current 2>/dev/null || true)"
echo

echo "== policy =="
echo "No /system write."
echo "No Termux prefix header overwrite."
echo "Build missing Android libc contract symbols as native extension library."
echo "Expose declarations by staged headers using include_next."
echo "Probe through active symlinks."
echo "Promote only after proof."
echo "Rebuild CPython with overlay include/lib injected."
echo "Progress remains visible."
echo

export TMPDIR="$CHAIN/no_tmp_redirect"
mkdir -p "$TMPDIR"

echo "== required tools =="
missing=0
for t in clang clang++ llvm-ar llvm-ranlib llvm-nm readelf file make git grep sed awk tee sha256sum; do
  if command -v "$t" >/dev/null 2>&1; then
    printf "OK: %-16s %s\n" "$t" "$(command -v "$t")"
  else
    echo "MISSING: $t"
    missing=1
  fi
done | tee "$REPORT/required_tools.txt"

if [ "$missing" = "1" ]; then
  echo "FAIL: missing required tools"
  exit 1
fi

if [ ! -d "$CPY" ] || [ ! -f "$CPY/configure" ]; then
  echo "FAIL: CPython source tree missing or incomplete at: $CPY"
  echo "Expected: $CPY/configure"
  exit 1
fi

echo
echo "== manifest =="
cat > config/toolchains/cpython_android_native_contract_resolve.json <<JSON
{
  "schema": "braxon.cpython_android_native_contract_resolve.v1",
  "created_at": "$(date -Is)",
  "root": "$ROOT",
  "chain": "$CHAIN",
  "cpython": "$CPY",
  "overlay": "$OVERLAY",
  "stage": "$STAGE",
  "linkroot": "$LINKROOT",
  "native_extension_symbols": [
    "sem_clockwait",
    "pthread_getname_np"
  ],
  "system_write_attempted": false,
  "termux_prefix_overwrite_attempted": false,
  "active_custom_rust_replaced": false,
  "purpose": "Resolve CPython Android/Termux missing libc contract declarations and symbols through a native build-chain extension, then rebuild CPython against that extension."
}
JSON
cat config/toolchains/cpython_android_native_contract_resolve.json | tee "$REPORT/manifest.json"

echo
echo "== write staged semaphore.h =="
cat > "$STAGE_INCLUDE/semaphore.h" <<'H'
#ifndef BRAXON_ANDROID_BUILTIN_SEMAPHORE_H
#define BRAXON_ANDROID_BUILTIN_SEMAPHORE_H

#include_next <semaphore.h>
#include <time.h>

#ifdef __cplusplus
extern "C" {
#endif

int sem_clockwait(sem_t *sem, clockid_t clockid, const struct timespec *abstime);

#ifdef __cplusplus
}
#endif

#endif
H
cat "$STAGE_INCLUDE/semaphore.h" | tee "$REPORT/staged_semaphore_h.txt"

echo
echo "== write staged pthread.h =="
cat > "$STAGE_INCLUDE/pthread.h" <<'H'
#ifndef BRAXON_ANDROID_BUILTIN_PTHREAD_H
#define BRAXON_ANDROID_BUILTIN_PTHREAD_H

#include_next <pthread.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

int pthread_getname_np(pthread_t thread, char *name, size_t len);

#ifdef __cplusplus
}
#endif

#endif
H
cat "$STAGE_INCLUDE/pthread.h" | tee "$REPORT/staged_pthread_h.txt"

echo
echo "== write native sem_clockwait implementation =="
cat > "$NATIVE_SRC/sem_clockwait.c" <<'C'
#define _GNU_SOURCE
#include <errno.h>
#include <semaphore.h>
#include <time.h>

static inline int braxon_valid_timespec(const struct timespec *ts) {
    return ts != 0 && ts->tv_nsec >= 0 && ts->tv_nsec < 1000000000L;
}

static inline struct timespec braxon_timespec_add(struct timespec a, struct timespec b) {
    struct timespec out;
    out.tv_sec = a.tv_sec + b.tv_sec;
    out.tv_nsec = a.tv_nsec + b.tv_nsec;
    if (out.tv_nsec >= 1000000000L) {
        out.tv_sec += 1;
        out.tv_nsec -= 1000000000L;
    }
    return out;
}

static inline struct timespec braxon_timespec_sub(struct timespec a, struct timespec b) {
    struct timespec out;
    out.tv_sec = a.tv_sec - b.tv_sec;
    out.tv_nsec = a.tv_nsec - b.tv_nsec;
    if (out.tv_nsec < 0) {
        out.tv_sec -= 1;
        out.tv_nsec += 1000000000L;
    }
    return out;
}

static inline int braxon_timespec_negative(struct timespec ts) {
    return ts.tv_sec < 0;
}

static inline void braxon_clock_order_barrier(void) {
#if defined(__aarch64__)
    __asm__ __volatile__("isb" ::: "memory");
#else
    __asm__ __volatile__("" ::: "memory");
#endif
}

__attribute__((visibility("default")))
int sem_clockwait(sem_t *sem, clockid_t clockid, const struct timespec *abstime) {
    if (sem == 0 || !braxon_valid_timespec(abstime)) {
        errno = EINVAL;
        return -1;
    }

    braxon_clock_order_barrier();

    if (clockid == CLOCK_REALTIME) {
        return sem_timedwait(sem, abstime);
    }

    if (clockid == CLOCK_MONOTONIC) {
        struct timespec mono_now;
        struct timespec real_now;

        if (clock_gettime(CLOCK_MONOTONIC, &mono_now) != 0) {
            return -1;
        }

        if (clock_gettime(CLOCK_REALTIME, &real_now) != 0) {
            return -1;
        }

        struct timespec remaining = braxon_timespec_sub(*abstime, mono_now);
        if (braxon_timespec_negative(remaining)) {
            errno = ETIMEDOUT;
            return -1;
        }

        struct timespec real_deadline = braxon_timespec_add(real_now, remaining);
        braxon_clock_order_barrier();

        return sem_timedwait(sem, &real_deadline);
    }

    errno = EINVAL;
    return -1;
}
C
cat "$NATIVE_SRC/sem_clockwait.c" | tee "$REPORT/native_sem_clockwait_c.txt"

echo
echo "== write native pthread_getname_np implementation =="
cat > "$NATIVE_SRC/pthread_getname_np.c" <<'C'
#define _GNU_SOURCE
#include <errno.h>
#include <pthread.h>
#include <stddef.h>
#include <string.h>
#include <sys/prctl.h>

#ifndef PR_GET_NAME
#define PR_GET_NAME 16
#endif

static int braxon_copy_thread_name(char *dst, size_t len, const char *src) {
    if (dst == 0 || len == 0) {
        return ERANGE;
    }

    if (src == 0) {
        dst[0] = '\0';
        return 0;
    }

    size_t n = strnlen(src, len);
    if (n >= len) {
        memcpy(dst, src, len - 1);
        dst[len - 1] = '\0';
        return ERANGE;
    }

    memcpy(dst, src, n);
    dst[n] = '\0';
    return 0;
}

__attribute__((visibility("default")))
int pthread_getname_np(pthread_t thread, char *name, size_t len) {
    if (name == 0 || len == 0) {
        return ERANGE;
    }

    name[0] = '\0';

    if (pthread_equal(thread, pthread_self())) {
        char local[16];
        memset(local, 0, sizeof(local));

        if (prctl(PR_GET_NAME, (unsigned long)local, 0UL, 0UL, 0UL) == 0) {
            return braxon_copy_thread_name(name, len, local);
        }

        return errno ? errno : ENOSYS;
    }

    return ENOSYS;
}
C
cat "$NATIVE_SRC/pthread_getname_np.c" | tee "$REPORT/native_pthread_getname_np_c.txt"

echo
echo "== compile native extension objects =="
run_logged "compile sem_clockwait.o" "$REPORT/compile_sem_clockwait_o.txt" \
  clang -target aarch64-linux-android24 \
    -O3 \
    -fPIC \
    -fvisibility=hidden \
    -fno-semantic-interposition \
    -isystem "$STAGE_INCLUDE" \
    -c "$NATIVE_SRC/sem_clockwait.c" \
    -o "$STAGE_LIB/sem_clockwait.o"

run_logged "compile pthread_getname_np.o" "$REPORT/compile_pthread_getname_np_o.txt" \
  clang -target aarch64-linux-android24 \
    -O3 \
    -fPIC \
    -fvisibility=hidden \
    -fno-semantic-interposition \
    -isystem "$STAGE_INCLUDE" \
    -c "$NATIVE_SRC/pthread_getname_np.c" \
    -o "$STAGE_LIB/pthread_getname_np.o"

echo
echo "== create static and shared native extension libraries =="
run_logged "archive native extension" "$REPORT/archive_native_extension.txt" \
  llvm-ar rcs "$STAGE_LIB/libbraxon_android_libc_extensions.a" \
    "$STAGE_LIB/sem_clockwait.o" \
    "$STAGE_LIB/pthread_getname_np.o"

run_logged "ranlib native extension" "$REPORT/ranlib_native_extension.txt" \
  llvm-ranlib "$STAGE_LIB/libbraxon_android_libc_extensions.a"

run_logged "shared native extension" "$REPORT/shared_native_extension.txt" \
  clang -target aarch64-linux-android24 \
    -O3 \
    -fPIC \
    -fvisibility=hidden \
    -fno-semantic-interposition \
    -shared \
    -Wl,-soname,libbraxon_android_libc_extensions.so \
    -isystem "$STAGE_INCLUDE" \
    "$NATIVE_SRC/sem_clockwait.c" \
    "$NATIVE_SRC/pthread_getname_np.c" \
    -o "$STAGE_LIB/libbraxon_android_libc_extensions.so"

echo
echo "== create controlled active symlinks =="
rm -f \
  "$LINK_INCLUDE/semaphore.h" \
  "$LINK_INCLUDE/pthread.h" \
  "$LINK_LIB/libbraxon_android_libc_extensions.a" \
  "$LINK_LIB/libbraxon_android_libc_extensions.so"

ln -s "$STAGE_INCLUDE/semaphore.h" "$LINK_INCLUDE/semaphore.h"
ln -s "$STAGE_INCLUDE/pthread.h" "$LINK_INCLUDE/pthread.h"
ln -s "$STAGE_LIB/libbraxon_android_libc_extensions.a" "$LINK_LIB/libbraxon_android_libc_extensions.a"
ln -s "$STAGE_LIB/libbraxon_android_libc_extensions.so" "$LINK_LIB/libbraxon_android_libc_extensions.so"

ls -la "$LINK_INCLUDE" "$LINK_LIB" | tee "$REPORT/active_symlink_listing.txt"

echo
echo "== build link probe through symlink surface =="
cat > "$RUN/native_contract_probe.c" <<'C'
#include <errno.h>
#include <pthread.h>
#include <semaphore.h>
#include <stdio.h>
#include <string.h>
#include <time.h>

int main(void) {
    char name[64];
    memset(name, 0, sizeof(name));

    int name_rc = pthread_getname_np(pthread_self(), name, sizeof(name));
    if (name_rc != 0) {
        printf("FAIL pthread_getname_np rc=%d errno=%d\n", name_rc, errno);
        return 1;
    }

    sem_t sema;
    if (sem_init(&sema, 0, 0) != 0) {
        perror("sem_init");
        return 2;
    }

    struct timespec ts;
    if (clock_gettime(CLOCK_MONOTONIC, &ts) != 0) {
        perror("clock_gettime");
        return 3;
    }

    ts.tv_nsec += 1000000L;
    if (ts.tv_nsec >= 1000000000L) {
        ts.tv_sec += 1;
        ts.tv_nsec -= 1000000000L;
    }

    errno = 0;
    int wait_rc = sem_clockwait(&sema, CLOCK_MONOTONIC, &ts);
    int wait_errno = errno;
    sem_destroy(&sema);

    if (wait_rc != -1 || wait_errno != ETIMEDOUT) {
        printf("FAIL sem_clockwait rc=%d errno=%d\n", wait_rc, wait_errno);
        return 4;
    }

    printf("BRAXON_NATIVE_CONTRACT_OK:%s\n", name);
    return 0;
}
C

run_logged "compile symlink native contract probe" "$REPORT/compile_symlink_probe.txt" \
  clang -target aarch64-linux-android24 \
    -O3 \
    -isystem "$LINK_INCLUDE" \
    -L"$LINK_LIB" \
    "$RUN/native_contract_probe.c" \
    -lbraxon_android_libc_extensions \
    -fuse-ld=lld \
    -o "$RUN/symlink_native_contract_probe"

LD_LIBRARY_PATH="$LINK_LIB:${LD_LIBRARY_PATH:-}" "$RUN/symlink_native_contract_probe" | tee "$REPORT/symlink_probe_output.txt"
grep -q "BRAXON_NATIVE_CONTRACT_OK" "$REPORT/symlink_probe_output.txt"

echo
echo "== promote proven symlink targets into overlay =="
rm -f \
  "$OVERLAY_INCLUDE/semaphore.h" \
  "$OVERLAY_INCLUDE/pthread.h" \
  "$OVERLAY_LIB/libbraxon_android_libc_extensions.a" \
  "$OVERLAY_LIB/libbraxon_android_libc_extensions.so"

ln -s "$STAGE_INCLUDE/semaphore.h" "$OVERLAY_INCLUDE/semaphore.h"
ln -s "$STAGE_INCLUDE/pthread.h" "$OVERLAY_INCLUDE/pthread.h"
ln -s "$STAGE_LIB/libbraxon_android_libc_extensions.a" "$OVERLAY_LIB/libbraxon_android_libc_extensions.a"
ln -s "$STAGE_LIB/libbraxon_android_libc_extensions.so" "$OVERLAY_LIB/libbraxon_android_libc_extensions.so"

ls -la "$OVERLAY_INCLUDE" "$OVERLAY_LIB" | tee "$REPORT/overlay_symlink_listing.txt"

echo
echo "== prove overlay compile/link/run =="
run_logged "compile overlay native contract probe" "$REPORT/compile_overlay_probe.txt" \
  clang -target aarch64-linux-android24 \
    -O3 \
    -isystem "$OVERLAY_INCLUDE" \
    -L"$OVERLAY_LIB" \
    "$RUN/native_contract_probe.c" \
    -lbraxon_android_libc_extensions \
    -fuse-ld=lld \
    -o "$RUN/overlay_native_contract_probe"

LD_LIBRARY_PATH="$OVERLAY_LIB:${LD_LIBRARY_PATH:-}" "$RUN/overlay_native_contract_probe" | tee "$REPORT/overlay_probe_output.txt"
grep -q "BRAXON_NATIVE_CONTRACT_OK" "$REPORT/overlay_probe_output.txt"

echo
echo "== symbol proof =="
{
  echo "schema=braxon.android.native_contract.symbol_proof.v1"
  echo "date=$(date -Is)"
  echo "stage=$STAGE"
  echo "linkroot=$LINKROOT"
  echo "overlay=$OVERLAY"
  echo
  echo "== archive symbols =="
  llvm-nm "$OVERLAY_LIB/libbraxon_android_libc_extensions.a" | grep -E 'sem_clockwait|pthread_getname_np'
  echo
  echo "== shared exports =="
  readelf -Ws "$OVERLAY_LIB/libbraxon_android_libc_extensions.so" | grep -E 'sem_clockwait|pthread_getname_np'
  echo
  echo "== realpaths =="
  realpath "$OVERLAY_INCLUDE/semaphore.h"
  realpath "$OVERLAY_INCLUDE/pthread.h"
  realpath "$OVERLAY_LIB/libbraxon_android_libc_extensions.a"
  realpath "$OVERLAY_LIB/libbraxon_android_libc_extensions.so"
  echo
  echo "== probe outputs =="
  cat "$REPORT/symlink_probe_output.txt"
  cat "$REPORT/overlay_probe_output.txt"
} | tee "$NATIVE_PROOFS/cpython_native_contract_symbol_proof.txt"

echo
echo "== CPython source status before rebuild =="
(
  cd "$CPY"
  git rev-parse HEAD 2>/dev/null || true
  git status --short 2>/dev/null || true
) | tee "$REPORT/cpython_source_status_before.txt"

echo
echo "== clean prior CPython generated build outputs =="
(
  cd "$CPY"
  make distclean 2>&1 || true
  make clean 2>&1 || true
) | tee "$REPORT/cpython_clean.txt"

echo
echo "== configure CPython with native contract overlay =="
cd "$CPY"

export CPPFLAGS="-isystem $OVERLAY_INCLUDE ${CPPFLAGS:-}"
export CFLAGS="-isystem $OVERLAY_INCLUDE ${CFLAGS:-}"
export LDFLAGS="-L$OVERLAY_LIB -fuse-ld=lld ${LDFLAGS:-}"
export LIBS="-lbraxon_android_libc_extensions ${LIBS:-}"
export LD_LIBRARY_PATH="$OVERLAY_LIB:${LD_LIBRARY_PATH:-}"

echo "CPPFLAGS=$CPPFLAGS" | tee "$REPORT/cpython_env.txt"
echo "CFLAGS=$CFLAGS" | tee -a "$REPORT/cpython_env.txt"
echo "LDFLAGS=$LDFLAGS" | tee -a "$REPORT/cpython_env.txt"
echo "LIBS=$LIBS" | tee -a "$REPORT/cpython_env.txt"
echo "LD_LIBRARY_PATH=$LD_LIBRARY_PATH" | tee -a "$REPORT/cpython_env.txt"

run_logged "configure CPython with overlay" "$REPORT/cpython_configure_overlay.txt" \
  ./configure \
    --prefix="$PY_INSTALL" \
    --enable-optimizations \
    --with-lto \
    CC="$(command -v clang)" \
    CXX="$(command -v clang++)"

echo
echo "== verify configure detected required functions or overlay declarations =="
{
  grep -n "sem_clockwait" config.log || true
  grep -n "pthread_getname_np" config.log || true
  grep -n "HAVE_SEM_CLOCKWAIT" pyconfig.h || true
  grep -n "HAVE_PTHREAD_GETNAME_NP" pyconfig.h || true
} | tee "$REPORT/cpython_config_detection.txt"

echo
echo "== build CPython with overlay =="
run_logged "build CPython with overlay" "$REPORT/cpython_build_overlay.txt" \
  make -j"${JOBS:-7}"

echo
echo "== install CPython with overlay =="
run_logged "install CPython with overlay" "$REPORT/cpython_install_overlay.txt" \
  make install

echo
echo "== verify installed CPython =="
"$PY_INSTALL/bin/python3" --version | tee "$REPORT/cpython_version.txt"

"$PY_INSTALL/bin/python3" - <<'PY' | tee "$REPORT/cpython_runtime_probe.txt"
import sys
import ssl
import sqlite3
import ctypes
print("BRAXON_CPYTHON_RUNTIME_OK")
print(sys.version)
print(ssl.OPENSSL_VERSION)
print(sqlite3.sqlite_version)
print(ctypes.sizeof(ctypes.c_void_p))
PY

grep -q "BRAXON_CPYTHON_RUNTIME_OK" "$REPORT/cpython_runtime_probe.txt"

echo
echo "== compile final native contract probe using installed overlay =="
cd "$ROOT"

run_logged "compile final native contract probe" "$REPORT/final_native_contract_probe_compile.txt" \
  clang -target aarch64-linux-android24 \
    -O3 \
    -isystem "$OVERLAY_INCLUDE" \
    -L"$OVERLAY_LIB" \
    "$RUN/native_contract_probe.c" \
    -lbraxon_android_libc_extensions \
    -fuse-ld=lld \
    -o "$RUN/final_native_contract_probe"

LD_LIBRARY_PATH="$OVERLAY_LIB:${LD_LIBRARY_PATH:-}" "$RUN/final_native_contract_probe" | tee "$REPORT/final_native_contract_probe_output.txt"
grep -q "BRAXON_NATIVE_CONTRACT_OK" "$REPORT/final_native_contract_probe_output.txt"

echo
echo "== hash proof outputs =="
(
  cd "$CHAIN"
  find \
    "install/braxon_android_builtin_stage" \
    "install/braxon_android_active_links" \
    "install/braxon_android_overlay" \
    "install/python" \
    "native/android_libc_extensions" \
    -type f -print0 | sort -z | xargs -0 sha256sum
) | tee "$RUN/CPYTHON_NATIVE_CONTRACT_SHA256SUMS.txt"

echo
echo "== final report =="
cat > "$RUN/CPYTHON_ANDROID_NATIVE_CONTRACT_RESOLVE_REPORT.txt" <<REPORTTXT
schema=braxon.cpython_android_native_contract_resolve.report.v1
date=$(date -Is)
root=$ROOT
chain=$CHAIN
run=$RUN
cpython=$CPY
python_install=$PY_INSTALL
overlay=$OVERLAY
stage=$STAGE
linkroot=$LINKROOT
native_symbol_proof=$NATIVE_PROOFS/cpython_native_contract_symbol_proof.txt
hash_manifest=$RUN/CPYTHON_NATIVE_CONTRACT_SHA256SUMS.txt
system_write_attempted=false
termux_prefix_overwrite_attempted=false
active_custom_rust_replaced=false
sem_clockwait_native_extension=true
pthread_getname_np_native_extension=true
symlink_probe_passed=true
overlay_probe_passed=true
cpython_rebuild_attempted=true
cpython_runtime_probe_passed=true
REPORTTXT

cat "$RUN/CPYTHON_ANDROID_NATIVE_CONTRACT_RESOLVE_REPORT.txt" | tee "$REPORT/final_report.txt"

echo
echo "PASS: CPython Android native contracts resolved through Braxon native extension overlay"
echo "RUN=$RUN"
echo "REPORT=$REPORT"
echo "PY_INSTALL=$PY_INSTALL"
echo "OVERLAY=$OVERLAY"
