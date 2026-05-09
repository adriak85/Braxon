#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
cd "$ROOT"

CHAIN="$ROOT/state/full_android_language_toolchain"
RUN="$CHAIN/runs/builtin_sem_clockwait_$(date +%Y%m%d_%H%M%S)"
REPORT="$RUN/reports"

SYSROOT="$CHAIN/install/sysroot"
SYS_INCLUDE="$SYSROOT/usr/include"
SYS_LIB="$SYSROOT/usr/lib"

NATIVE="$CHAIN/native/android_libc_extensions/sem_clockwait"
NATIVE_SRC="$NATIVE/src"
NATIVE_PROOFS="$NATIVE/proofs"

VISIBLE="$ROOT/scripts/toolchains/rebuild_full_android_language_toolchain_visible.sh"
CPY="$CHAIN/src/cpython"

mkdir -p "$REPORT" "$SYS_INCLUDE" "$SYS_LIB" "$NATIVE_SRC" "$NATIVE_PROOFS" scripts/toolchains config/toolchains

LOG="$RUN/builtin_sem_clockwait_sysroot.log"
exec > >(tee "$LOG") 2>&1

echo "== Braxon built-in Android sysroot sem_clockwait surface =="
echo "date=$(date -Is)"
echo "root=$ROOT"
echo "chain=$CHAIN"
echo "sysroot=$SYSROOT"
echo "run=$RUN"
echo

if [ ! -d "$CPY" ]; then
  echo "FAIL: CPython source not found: $CPY"
  exit 1
fi

if [ ! -f "$VISIBLE" ]; then
  echo "FAIL: visible rebuild script not found: $VISIBLE"
  exit 1
fi

echo "== rule =="
echo "sem_clockwait must be built into the chain sysroot so default compiler/configure methods land on it."
echo "No one-off CPPFLAGS/LIBS dependency should be required after this script patches the rebuild lane."
echo "This does not modify Android /system or Termux PREFIX."
echo

cat > config/toolchains/android_builtin_libc_extensions.json <<JSON
{
  "schema": "braxon.android.builtin_libc_extensions.v1",
  "created_at": "$(date -Is)",
  "repo_head": "$(git rev-parse HEAD)",
  "branch": "$(git branch --show-current)",
  "authority": "BRAXON_ANDROID_BUILTIN_LIBC_EXTENSION_SYSROOT",
  "chain_root": "state/full_android_language_toolchain",
  "sysroot": "state/full_android_language_toolchain/install/sysroot",
  "surfaces": [
    {
      "name": "sem_clockwait",
      "exported_symbol": "sem_clockwait",
      "header": "usr/include/semaphore.h",
      "static_archive": "usr/lib/libbraxon_android_libc_extensions.a",
      "shared_library": "usr/lib/libbraxon_android_libc_extensions.so",
      "default_landing": true,
      "manual_overlay_required_after_install": false,
      "consumer": "cpython/Python/parking_lot.c"
    }
  ],
  "non_claims": [
    "does not replace Android system libc",
    "does not write to /data/data/com.termux/files/usr",
    "does not replace active Rust",
    "does not claim complete libc until all required symbols have equivalent proof"
  ]
}
JSON

cat config/toolchains/android_builtin_libc_extensions.json | tee "$REPORT/android_builtin_libc_extensions.json"

echo
echo "== install semaphore.h into chain sysroot =="
cat > "$SYS_INCLUDE/semaphore.h" <<'H'
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

echo
echo "== build direct exported sem_clockwait into chain sysroot lib =="
cat > "$NATIVE_SRC/sem_clockwait.c" <<'C'
#define _GNU_SOURCE
#include <errno.h>
#include <semaphore.h>
#include <time.h>

#ifndef CLOCK_MONOTONIC
#define CLOCK_MONOTONIC 1
#endif

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

static inline void braxon_native_clock_barrier(void) {
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

    braxon_native_clock_barrier();

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
        braxon_native_clock_barrier();

        return sem_timedwait(sem, &real_deadline);
    }

    errno = EINVAL;
    return -1;
}
C

clang -target aarch64-linux-android24 \
  -O3 \
  -fPIC \
  -fvisibility=hidden \
  -fno-semantic-interposition \
  -isystem "$SYS_INCLUDE" \
  -c "$NATIVE_SRC/sem_clockwait.c" \
  -o "$SYS_LIB/sem_clockwait.o"

llvm-ar rcs "$SYS_LIB/libbraxon_android_libc_extensions.a" "$SYS_LIB/sem_clockwait.o"
llvm-ranlib "$SYS_LIB/libbraxon_android_libc_extensions.a"

clang -target aarch64-linux-android24 \
  -O3 \
  -fPIC \
  -fvisibility=hidden \
  -fno-semantic-interposition \
  -shared \
  -Wl,-soname,libbraxon_android_libc_extensions.so \
  -isystem "$SYS_INCLUDE" \
  "$NATIVE_SRC/sem_clockwait.c" \
  -o "$SYS_LIB/libbraxon_android_libc_extensions.so"

echo
echo "== prove symbol is built in and direct =="
{
  echo "schema=braxon.android.builtin_sem_clockwait.symbol_proof.v1"
  echo "date=$(date -Is)"
  echo "sysroot=$SYSROOT"
  echo "header=$SYS_INCLUDE/semaphore.h"
  echo "object=$SYS_LIB/sem_clockwait.o"
  echo "archive=$SYS_LIB/libbraxon_android_libc_extensions.a"
  echo "shared=$SYS_LIB/libbraxon_android_libc_extensions.so"
  echo
  file "$SYS_LIB/sem_clockwait.o"
  file "$SYS_LIB/libbraxon_android_libc_extensions.a"
  file "$SYS_LIB/libbraxon_android_libc_extensions.so"
  echo
  echo "== object symbols =="
  llvm-nm "$SYS_LIB/sem_clockwait.o" | grep -E 'sem_clockwait|braxon' || true
  echo
  echo "== archive symbols =="
  llvm-nm "$SYS_LIB/libbraxon_android_libc_extensions.a" | grep -E 'sem_clockwait|braxon' || true
  echo
  echo "== shared exports =="
  readelf -Ws "$SYS_LIB/libbraxon_android_libc_extensions.so" | grep -E 'sem_clockwait|braxon' || true
  echo
  echo "== disassembly =="
  llvm-objdump -d "$SYS_LIB/sem_clockwait.o" | sed -n '1,260p'
} | tee "$NATIVE_PROOFS/builtin_symbol_proof.txt"

llvm-nm "$SYS_LIB/sem_clockwait.o" | grep -q ' T sem_clockwait'

echo
echo "== default landing probe: no manual source include path =="
cat > "$RUN/default_sem_clockwait_probe.c" <<'C'
#include <errno.h>
#include <semaphore.h>
#include <stdio.h>
#include <string.h>
#include <time.h>

int main(void) {
    sem_t sem;
    if (sem_init(&sem, 0, 0) != 0) {
        perror("sem_init");
        return 10;
    }

    struct timespec deadline;
    if (clock_gettime(CLOCK_MONOTONIC, &deadline) != 0) {
        perror("clock_gettime");
        return 11;
    }

    deadline.tv_nsec += 1000000L;
    if (deadline.tv_nsec >= 1000000000L) {
        deadline.tv_sec += 1;
        deadline.tv_nsec -= 1000000000L;
    }

    errno = 0;
    int rc = sem_clockwait(&sem, CLOCK_MONOTONIC, &deadline);
    int err = errno;
    sem_destroy(&sem);

    if (rc == -1 && err == ETIMEDOUT) {
        puts("BRAXON_BUILTIN_SEM_CLOCKWAIT_DEFAULT_OK");
        return 0;
    }

    printf("FAIL rc=%d errno=%d %s\n", rc, err, strerror(err));
    return 12;
}
C

clang -target aarch64-linux-android24 \
  -O3 \
  --sysroot="$SYSROOT" \
  "$RUN/default_sem_clockwait_probe.c" \
  -lbraxon_android_libc_extensions \
  -fuse-ld=lld \
  -o "$RUN/default_sem_clockwait_probe"

"$RUN/default_sem_clockwait_probe" | tee "$NATIVE_PROOFS/default_landing_probe.txt"
grep -q "BRAXON_BUILTIN_SEM_CLOCKWAIT_DEFAULT_OK" "$NATIVE_PROOFS/default_landing_probe.txt"

echo
echo "== patch visible rebuild script so default methods land on built-in sysroot =="
python3 - "$VISIBLE" <<'PY'
from pathlib import Path
import sys

p = Path(sys.argv[1])
s = p.read_text()

anchor = 'export TMPDIR="$CHAIN/no_tmp_redirect"\nmkdir -p "$TMPDIR"\n'
insert = '''export BRAXON_BUILTIN_SYSROOT="$CHAIN/install/sysroot"
export BRAXON_BUILTIN_SYSROOT_INCLUDE="$BRAXON_BUILTIN_SYSROOT/usr/include"
export BRAXON_BUILTIN_SYSROOT_LIB="$BRAXON_BUILTIN_SYSROOT/usr/lib"
export CPPFLAGS="-isystem $BRAXON_BUILTIN_SYSROOT_INCLUDE ${CPPFLAGS:-}"
export CFLAGS="-isystem $BRAXON_BUILTIN_SYSROOT_INCLUDE ${CFLAGS:-}"
export CXXFLAGS="-isystem $BRAXON_BUILTIN_SYSROOT_INCLUDE ${CXXFLAGS:-}"
export LDFLAGS="-L$BRAXON_BUILTIN_SYSROOT_LIB -fuse-ld=lld ${LDFLAGS:-}"
export LIBS="-lbraxon_android_libc_extensions ${LIBS:-}"
export PKG_CONFIG_PATH="$BRAXON_BUILTIN_SYSROOT_LIB/pkgconfig:${PKG_CONFIG_PATH:-}"

echo "BRAXON_BUILTIN_SYSROOT=$BRAXON_BUILTIN_SYSROOT"
echo "BRAXON_BUILTIN_SYSROOT_INCLUDE=$BRAXON_BUILTIN_SYSROOT_INCLUDE"
echo "BRAXON_BUILTIN_SYSROOT_LIB=$BRAXON_BUILTIN_SYSROOT_LIB"
echo "CPPFLAGS=$CPPFLAGS"
echo "LDFLAGS=$LDFLAGS"
echo "LIBS=$LIBS"

'''

if insert in s:
    print("PASS: visible rebuild already has built-in sysroot landing block")
else:
    if anchor not in s:
        raise SystemExit("FAIL: TMPDIR anchor not found in visible rebuild script")
    s = s.replace(anchor, anchor + insert, 1)

# Make CPython configure inherit defaults instead of overriding them into a one-off lane.
old = '''      CC="$(command -v clang)" \\
      CXX="$(command -v clang++)" \\
      LDFLAGS="-fuse-ld=lld"'''
new = '''      CC="$(command -v clang)" \\
      CXX="$(command -v clang++)"'''
if old in s:
    s = s.replace(old, new, 1)

p.write_text(s)
PY

echo
echo "== CPython configure proof using patched default landing =="
cd "$CPY"
make distclean || true
rm -f config.cache config.log config.status Makefile pyconfig.h
find . -name '*.o' -delete 2>/dev/null || true

BRAXON_BUILTIN_SYSROOT="$SYSROOT"
BRAXON_BUILTIN_SYSROOT_INCLUDE="$SYS_INCLUDE"
BRAXON_BUILTIN_SYSROOT_LIB="$SYS_LIB"
export BRAXON_BUILTIN_SYSROOT BRAXON_BUILTIN_SYSROOT_INCLUDE BRAXON_BUILTIN_SYSROOT_LIB
export CPPFLAGS="-isystem $SYS_INCLUDE ${CPPFLAGS:-}"
export CFLAGS="-isystem $SYS_INCLUDE ${CFLAGS:-}"
export CXXFLAGS="-isystem $SYS_INCLUDE ${CXXFLAGS:-}"
export LDFLAGS="-L$SYS_LIB -fuse-ld=lld ${LDFLAGS:-}"
export LIBS="-lbraxon_android_libc_extensions ${LIBS:-}"

CC="$(command -v clang)" \
CXX="$(command -v clang++)" \
./configure \
  --prefix="$CHAIN/install/python" \
  --enable-optimizations \
  --with-lto \
  | tee "$REPORT/cpython_configure_builtin_sem_clockwait.txt"

grep -n "sem_clockwait" config.log | tee "$NATIVE_PROOFS/cpython_config_log_sem_clockwait.txt" || true

cd "$ROOT"

echo
echo "== built-in surface manifest =="
cat > "$NATIVE/BUILTIN_SEM_CLOCKWAIT_SYSROOT_SURFACE.json" <<JSON
{
  "schema": "braxon.android.builtin_sem_clockwait.sysroot_surface.v1",
  "created_at": "$(date -Is)",
  "sysroot": "$SYSROOT",
  "header": "$SYS_INCLUDE/semaphore.h",
  "object": "$SYS_LIB/sem_clockwait.o",
  "static_archive": "$SYS_LIB/libbraxon_android_libc_extensions.a",
  "shared_library": "$SYS_LIB/libbraxon_android_libc_extensions.so",
  "visible_rebuild_script_patched": "$VISIBLE",
  "default_methods_land_on_surface": true,
  "manual_cppflags_required_after_patch": false,
  "manual_libs_required_after_patch": false,
  "prefix_modified": false,
  "active_rust_replaced": false
}
JSON

cat "$NATIVE/BUILTIN_SEM_CLOCKWAIT_SYSROOT_SURFACE.json" | tee "$NATIVE_PROOFS/builtin_surface_manifest.txt"

echo
echo "== resume visible full rebuild =="
JOBS="${JOBS:-7}" "$VISIBLE" "$ROOT"
