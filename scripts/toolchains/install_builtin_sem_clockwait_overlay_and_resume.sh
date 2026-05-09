#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
cd "$ROOT"

CHAIN="$ROOT/state/full_android_language_toolchain"
RUN="$CHAIN/runs/builtin_sem_clockwait_overlay_$(date +%Y%m%d_%H%M%S)"
REPORT="$RUN/reports"

OVERLAY="$CHAIN/install/braxon_android_overlay"
OVERLAY_INCLUDE="$OVERLAY/include"
OVERLAY_LIB="$OVERLAY/lib"

NATIVE="$CHAIN/native/android_libc_extensions/sem_clockwait"
NATIVE_SRC="$NATIVE/src"
NATIVE_PROOFS="$NATIVE/proofs"

VISIBLE="$ROOT/scripts/toolchains/rebuild_full_android_language_toolchain_visible.sh"
CPY="$CHAIN/src/cpython"

mkdir -p "$REPORT" "$OVERLAY_INCLUDE" "$OVERLAY_LIB" "$NATIVE_SRC" "$NATIVE_PROOFS" scripts/toolchains config/toolchains

LOG="$RUN/builtin_sem_clockwait_overlay.log"
exec > >(tee "$LOG") 2>&1

echo "== Braxon built-in sem_clockwait overlay, not partial sysroot =="
echo "date=$(date -Is)"
echo "root=$ROOT"
echo "chain=$CHAIN"
echo "overlay=$OVERLAY"
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

echo "== hard-block correction =="
echo "Do not pass --sysroot to a partial tree."
echo "Use -isystem overlay/include before Termux headers so include_next can reach real semaphore.h."
echo "Use -L overlay/lib and default LIBS so consumers link the built symbol."
echo

cat > config/toolchains/android_builtin_libc_extensions_overlay.json <<JSON
{
  "schema": "braxon.android.builtin_libc_extensions.overlay.v1",
  "created_at": "$(date -Is)",
  "repo_head": "$(git rev-parse HEAD)",
  "branch": "$(git branch --show-current)",
  "authority": "BRAXON_ANDROID_BUILTIN_LIBC_EXTENSION_OVERLAY",
  "chain_root": "state/full_android_language_toolchain",
  "overlay": "state/full_android_language_toolchain/install/braxon_android_overlay",
  "surfaces": [
    {
      "name": "sem_clockwait",
      "exported_symbol": "sem_clockwait",
      "header": "include/semaphore.h",
      "static_archive": "lib/libbraxon_android_libc_extensions.a",
      "shared_library": "lib/libbraxon_android_libc_extensions.so",
      "default_landing": true,
      "partial_sysroot": false,
      "manual_per_source_patch_required": false,
      "consumer": "cpython/Python/parking_lot.c"
    }
  ],
  "known_hard_blocks": [
    "A partial sysroot masks errno.h/stdint.h/time.h and fails before sem_clockwait is tested.",
    "include_next requires the real Termux/Android header tree after the overlay include path.",
    "This builds a Braxon-chain native extension surface, not a modified Android system libc.",
    "Full libc-native requires a separate custom libc or bionic replacement lane.",
    "CPython development head may expose additional Android incompatibilities after this one."
  ]
}
JSON

cat config/toolchains/android_builtin_libc_extensions_overlay.json | tee "$REPORT/android_builtin_libc_extensions_overlay.json"

echo
echo "== install overlay semaphore.h =="
cat > "$OVERLAY_INCLUDE/semaphore.h" <<'H'
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
echo "== build exported sem_clockwait library =="
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

clang -target aarch64-linux-android24 \
  -O3 \
  -fPIC \
  -fvisibility=hidden \
  -fno-semantic-interposition \
  -isystem "$OVERLAY_INCLUDE" \
  -c "$NATIVE_SRC/sem_clockwait.c" \
  -o "$OVERLAY_LIB/sem_clockwait.o"

llvm-ar rcs "$OVERLAY_LIB/libbraxon_android_libc_extensions.a" "$OVERLAY_LIB/sem_clockwait.o"
llvm-ranlib "$OVERLAY_LIB/libbraxon_android_libc_extensions.a"

clang -target aarch64-linux-android24 \
  -O3 \
  -fPIC \
  -fvisibility=hidden \
  -fno-semantic-interposition \
  -shared \
  -Wl,-soname,libbraxon_android_libc_extensions.so \
  -isystem "$OVERLAY_INCLUDE" \
  "$NATIVE_SRC/sem_clockwait.c" \
  -o "$OVERLAY_LIB/libbraxon_android_libc_extensions.so"

echo
echo "== prove overlay header does not mask real standard headers =="
cat > "$RUN/header_probe.c" <<'C'
#include <errno.h>
#include <semaphore.h>
#include <stdint.h>
#include <stdio.h>
#include <time.h>

int main(void) {
    struct timespec ts;
    ts.tv_sec = 0;
    ts.tv_nsec = 0;
    (void)ts;
    printf("BRAXON_HEADERS_VISIBLE_WITH_SEM_CLOCKWAIT_DECL=%p\n", (void *)&sem_clockwait);
    return EINVAL == 22 ? 0 : 0;
}
C

clang -target aarch64-linux-android24 \
  -O3 \
  -isystem "$OVERLAY_INCLUDE" \
  -L"$OVERLAY_LIB" \
  "$RUN/header_probe.c" \
  -lbraxon_android_libc_extensions \
  -fuse-ld=lld \
  -o "$RUN/header_probe"

LD_LIBRARY_PATH="$OVERLAY_LIB:${LD_LIBRARY_PATH:-}" "$RUN/header_probe" | tee "$NATIVE_PROOFS/header_probe_output.txt"
grep -q "BRAXON_HEADERS_VISIBLE_WITH_SEM_CLOCKWAIT_DECL" "$NATIVE_PROOFS/header_probe_output.txt"

echo
echo "== default landing probe with no partial sysroot =="
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
  -isystem "$OVERLAY_INCLUDE" \
  -L"$OVERLAY_LIB" \
  "$RUN/default_sem_clockwait_probe.c" \
  -lbraxon_android_libc_extensions \
  -fuse-ld=lld \
  -o "$RUN/default_sem_clockwait_probe"

LD_LIBRARY_PATH="$OVERLAY_LIB:${LD_LIBRARY_PATH:-}" "$RUN/default_sem_clockwait_probe" | tee "$NATIVE_PROOFS/default_landing_probe.txt"
grep -q "BRAXON_BUILTIN_SEM_CLOCKWAIT_DEFAULT_OK" "$NATIVE_PROOFS/default_landing_probe.txt"

echo
echo "== symbol proof =="
{
  echo "schema=braxon.android.builtin_sem_clockwait.overlay_symbol_proof.v1"
  echo "date=$(date -Is)"
  echo "overlay=$OVERLAY"
  echo "header=$OVERLAY_INCLUDE/semaphore.h"
  echo "object=$OVERLAY_LIB/sem_clockwait.o"
  echo "archive=$OVERLAY_LIB/libbraxon_android_libc_extensions.a"
  echo "shared=$OVERLAY_LIB/libbraxon_android_libc_extensions.so"
  echo
  file "$OVERLAY_LIB/sem_clockwait.o"
  file "$OVERLAY_LIB/libbraxon_android_libc_extensions.a"
  file "$OVERLAY_LIB/libbraxon_android_libc_extensions.so"
  echo
  echo "== object symbols =="
  llvm-nm "$OVERLAY_LIB/sem_clockwait.o" | grep -E 'sem_clockwait|braxon' || true
  echo
  echo "== archive symbols =="
  llvm-nm "$OVERLAY_LIB/libbraxon_android_libc_extensions.a" | grep -E 'sem_clockwait|braxon' || true
  echo
  echo "== shared exports =="
  readelf -Ws "$OVERLAY_LIB/libbraxon_android_libc_extensions.so" | grep -E 'sem_clockwait|braxon' || true
  echo
  echo "== disassembly =="
  llvm-objdump -d "$OVERLAY_LIB/sem_clockwait.o" | sed -n '1,260p'
} | tee "$NATIVE_PROOFS/builtin_symbol_proof.txt"

llvm-nm "$OVERLAY_LIB/sem_clockwait.o" | grep -q ' T sem_clockwait'

echo
echo "== patch visible rebuild script to use overlay defaults =="
python3 - "$VISIBLE" <<'PY'
from pathlib import Path
import sys

p = Path(sys.argv[1])
s = p.read_text()

# Remove any previous bad partial-sysroot block if it was inserted.
bad_lines = [
    'export BRAXON_BUILTIN_SYSROOT="$CHAIN/install/sysroot"\n',
    'export BRAXON_BUILTIN_SYSROOT_INCLUDE="$BRAXON_BUILTIN_SYSROOT/usr/include"\n',
    'export BRAXON_BUILTIN_SYSROOT_LIB="$BRAXON_BUILTIN_SYSROOT/usr/lib"\n',
]
for line in bad_lines:
    s = s.replace(line, "")

anchor = 'export TMPDIR="$CHAIN/no_tmp_redirect"\nmkdir -p "$TMPDIR"\n'
insert = '''export BRAXON_ANDROID_OVERLAY="$CHAIN/install/braxon_android_overlay"
export BRAXON_ANDROID_OVERLAY_INCLUDE="$BRAXON_ANDROID_OVERLAY/include"
export BRAXON_ANDROID_OVERLAY_LIB="$BRAXON_ANDROID_OVERLAY/lib"
export CPPFLAGS="-isystem $BRAXON_ANDROID_OVERLAY_INCLUDE ${CPPFLAGS:-}"
export CFLAGS="-isystem $BRAXON_ANDROID_OVERLAY_INCLUDE ${CFLAGS:-}"
export CXXFLAGS="-isystem $BRAXON_ANDROID_OVERLAY_INCLUDE ${CXXFLAGS:-}"
export LDFLAGS="-L$BRAXON_ANDROID_OVERLAY_LIB -fuse-ld=lld ${LDFLAGS:-}"
export LIBS="-lbraxon_android_libc_extensions ${LIBS:-}"
export LD_LIBRARY_PATH="$BRAXON_ANDROID_OVERLAY_LIB:${LD_LIBRARY_PATH:-}"
export PKG_CONFIG_PATH="$BRAXON_ANDROID_OVERLAY_LIB/pkgconfig:${PKG_CONFIG_PATH:-}"

echo "BRAXON_ANDROID_OVERLAY=$BRAXON_ANDROID_OVERLAY"
echo "BRAXON_ANDROID_OVERLAY_INCLUDE=$BRAXON_ANDROID_OVERLAY_INCLUDE"
echo "BRAXON_ANDROID_OVERLAY_LIB=$BRAXON_ANDROID_OVERLAY_LIB"
echo "CPPFLAGS=$CPPFLAGS"
echo "LDFLAGS=$LDFLAGS"
echo "LIBS=$LIBS"
echo "LD_LIBRARY_PATH=$LD_LIBRARY_PATH"

'''

if insert not in s:
    if anchor not in s:
        raise SystemExit("FAIL: TMPDIR anchor not found in visible rebuild script")
    s = s.replace(anchor, anchor + insert, 1)

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
echo "== CPython configure proof with overlay defaults =="
cd "$CPY"
make distclean || true
rm -f config.cache config.log config.status Makefile pyconfig.h
find . -name '*.o' -delete 2>/dev/null || true

export BRAXON_ANDROID_OVERLAY="$OVERLAY"
export BRAXON_ANDROID_OVERLAY_INCLUDE="$OVERLAY_INCLUDE"
export BRAXON_ANDROID_OVERLAY_LIB="$OVERLAY_LIB"
export CPPFLAGS="-isystem $OVERLAY_INCLUDE ${CPPFLAGS:-}"
export CFLAGS="-isystem $OVERLAY_INCLUDE ${CFLAGS:-}"
export CXXFLAGS="-isystem $OVERLAY_INCLUDE ${CXXFLAGS:-}"
export LDFLAGS="-L$OVERLAY_LIB -fuse-ld=lld ${LDFLAGS:-}"
export LIBS="-lbraxon_android_libc_extensions ${LIBS:-}"
export LD_LIBRARY_PATH="$OVERLAY_LIB:${LD_LIBRARY_PATH:-}"

CC="$(command -v clang)" \
CXX="$(command -v clang++)" \
./configure \
  --prefix="$CHAIN/install/python" \
  --with-lto \
  | tee "$REPORT/cpython_configure_overlay_sem_clockwait.txt"

grep -n "sem_clockwait" config.log | tee "$NATIVE_PROOFS/cpython_config_log_sem_clockwait.txt" || true

cd "$ROOT"

echo
echo "== surface manifest =="
cat > "$NATIVE/BUILTIN_SEM_CLOCKWAIT_OVERLAY_SURFACE.json" <<JSON
{
  "schema": "braxon.android.builtin_sem_clockwait.overlay_surface.v1",
  "created_at": "$(date -Is)",
  "overlay": "$OVERLAY",
  "header": "$OVERLAY_INCLUDE/semaphore.h",
  "object": "$OVERLAY_LIB/sem_clockwait.o",
  "static_archive": "$OVERLAY_LIB/libbraxon_android_libc_extensions.a",
  "shared_library": "$OVERLAY_LIB/libbraxon_android_libc_extensions.so",
  "visible_rebuild_script_patched": "$VISIBLE",
  "default_methods_land_on_surface": true,
  "partial_sysroot_used": false,
  "standard_headers_visible": true,
  "prefix_modified": false,
  "active_rust_replaced": false
}
JSON

cat "$NATIVE/BUILTIN_SEM_CLOCKWAIT_OVERLAY_SURFACE.json" | tee "$NATIVE_PROOFS/builtin_surface_manifest.txt"

echo
echo "== resume visible full rebuild =="
JOBS="${JOBS:-7}" "$VISIBLE" "$ROOT"
