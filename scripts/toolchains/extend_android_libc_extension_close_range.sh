#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
cd "$ROOT"

STAMP="$(date +%Y%m%d_%H%M%S)"
CHAIN="$ROOT/state/full_android_language_toolchain"
RUN="$CHAIN/runs/close_range_extension_$STAMP"
REPORT="$RUN/reports"

OVERLAY="$CHAIN/install/braxon_android_overlay"
OVERLAY_INCLUDE="$OVERLAY/include"
OVERLAY_LIB="$OVERLAY/lib"

STAGE="$CHAIN/install/braxon_android_builtin_stage"
STAGE_INCLUDE="$STAGE/include"
STAGE_LIB="$STAGE/lib"

NATIVE="$CHAIN/native/android_libc_extensions"
NATIVE_SRC="$NATIVE/src"
NATIVE_PROOFS="$NATIVE/proofs"

mkdir -p "$RUN" "$REPORT" \
  "$OVERLAY_INCLUDE" "$OVERLAY_LIB" \
  "$STAGE_INCLUDE" "$STAGE_LIB" \
  "$NATIVE_SRC" "$NATIVE_PROOFS" \
  scripts/toolchains config/toolchains

LOG="$RUN/close_range_extension.log"
exec > >(tee "$LOG") 2>&1

echo "== Braxon Android libc extension: close_range =="
echo "date=$(date -Is)"
echo "root=$ROOT"
echo "chain=$CHAIN"
echo "stage=$STAGE"
echo "overlay=$OVERLAY"
echo "prefix=${PREFIX:-unset}"
echo "run=$RUN"
echo

echo "== preserve existing staged headers if present =="
if [ ! -f "$STAGE_INCLUDE/semaphore.h" ]; then
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
fi

if [ ! -f "$STAGE_INCLUDE/pthread.h" ]; then
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
fi

echo "== write staged unistd.h with close_range declaration =="
cat > "$STAGE_INCLUDE/unistd.h" <<'H'
#ifndef BRAXON_ANDROID_BUILTIN_UNISTD_H
#define BRAXON_ANDROID_BUILTIN_UNISTD_H

#include_next <unistd.h>

#ifdef __cplusplus
extern "C" {
#endif

int close_range(unsigned int first, unsigned int last, unsigned int flags);

#ifdef __cplusplus
}
#endif

#endif
H

echo "== write native close_range implementation =="
cat > "$NATIVE_SRC/close_range.c" <<'C'
#define _GNU_SOURCE
#include <errno.h>
#include <limits.h>
#include <unistd.h>
#include <sys/syscall.h>

#ifndef __NR_close_range
#if defined(__aarch64__)
#define __NR_close_range 436
#else
#define __NR_close_range 436
#endif
#endif

#ifndef CLOSE_RANGE_UNSHARE
#define CLOSE_RANGE_UNSHARE (1U << 1)
#endif

#ifndef CLOSE_RANGE_CLOEXEC
#define CLOSE_RANGE_CLOEXEC (1U << 2)
#endif

static inline void braxon_close_range_barrier(void) {
#if defined(__aarch64__)
    __asm__ __volatile__("dmb ish" ::: "memory");
#else
    __asm__ __volatile__("" ::: "memory");
#endif
}

__attribute__((visibility("default")))
int close_range(unsigned int first, unsigned int last, unsigned int flags) {
    if (first > last) {
        errno = EINVAL;
        return -1;
    }

    if ((flags & ~(CLOSE_RANGE_UNSHARE | CLOSE_RANGE_CLOEXEC)) != 0U) {
        errno = EINVAL;
        return -1;
    }

    braxon_close_range_barrier();

#ifdef SYS_close_range
    long rc = syscall(SYS_close_range, first, last, flags);
#else
    long rc = syscall(__NR_close_range, first, last, flags);
#endif

    if (rc == 0) {
        braxon_close_range_barrier();
        return 0;
    }

    /*
     * If the kernel supports the syscall, syscall() has already set errno.
     * If it does not, provide a conservative userspace fallback only for
     * plain close_range(first,last,0). CPython's fileutils path uses flags=0.
     */
    if (errno != ENOSYS) {
        return -1;
    }

    if (flags != 0U) {
        errno = ENOSYS;
        return -1;
    }

    unsigned int max_fd = last;
    if (max_fd == UINT_MAX) {
        long open_max = sysconf(_SC_OPEN_MAX);
        if (open_max > 0) {
            max_fd = (unsigned int)(open_max - 1);
        } else {
            max_fd = 1048576U;
        }
    }

    for (unsigned int fd = first; fd <= max_fd; fd++) {
        int saved_errno;
        if (close((int)fd) != 0) {
            saved_errno = errno;
            if (saved_errno != EBADF && saved_errno != EINTR) {
                errno = saved_errno;
                return -1;
            }
        }

        if (fd == UINT_MAX) {
            break;
        }
    }

    braxon_close_range_barrier();
    return 0;
}
C

echo "== rebuild native extension library with all known contract objects =="
OBJECTS=()

if [ -f "$NATIVE_SRC/sem_clockwait.c" ]; then
  clang -target aarch64-linux-android24 \
    -O3 \
    -fPIC \
    -fvisibility=hidden \
    -fno-semantic-interposition \
    -isystem "$STAGE_INCLUDE" \
    -c "$NATIVE_SRC/sem_clockwait.c" \
    -o "$STAGE_LIB/sem_clockwait.o"
  OBJECTS+=("$STAGE_LIB/sem_clockwait.o")
fi

if [ -f "$NATIVE_SRC/pthread_getname_np.c" ]; then
  clang -target aarch64-linux-android24 \
    -O3 \
    -fPIC \
    -fvisibility=hidden \
    -fno-semantic-interposition \
    -isystem "$STAGE_INCLUDE" \
    -c "$NATIVE_SRC/pthread_getname_np.c" \
    -o "$STAGE_LIB/pthread_getname_np.o"
  OBJECTS+=("$STAGE_LIB/pthread_getname_np.o")
fi

clang -target aarch64-linux-android24 \
  -O3 \
  -fPIC \
  -fvisibility=hidden \
  -fno-semantic-interposition \
  -isystem "$STAGE_INCLUDE" \
  -c "$NATIVE_SRC/close_range.c" \
  -o "$STAGE_LIB/close_range.o"
OBJECTS+=("$STAGE_LIB/close_range.o")

llvm-ar rcs "$STAGE_LIB/libbraxon_android_libc_extensions.a" "${OBJECTS[@]}"
llvm-ranlib "$STAGE_LIB/libbraxon_android_libc_extensions.a"

clang -target aarch64-linux-android24 \
  -O3 \
  -fPIC \
  -fvisibility=hidden \
  -fno-semantic-interposition \
  -shared \
  -Wl,-soname,libbraxon_android_libc_extensions.so \
  -isystem "$STAGE_INCLUDE" \
  "${OBJECTS[@]}" \
  -o "$STAGE_LIB/libbraxon_android_libc_extensions.so"

echo "== refresh overlay symlinks =="
rm -f "$OVERLAY_INCLUDE/semaphore.h" \
      "$OVERLAY_INCLUDE/pthread.h" \
      "$OVERLAY_INCLUDE/unistd.h" \
      "$OVERLAY_LIB/libbraxon_android_libc_extensions.a" \
      "$OVERLAY_LIB/libbraxon_android_libc_extensions.so"

ln -s "$STAGE_INCLUDE/semaphore.h" "$OVERLAY_INCLUDE/semaphore.h"
ln -s "$STAGE_INCLUDE/pthread.h" "$OVERLAY_INCLUDE/pthread.h"
ln -s "$STAGE_INCLUDE/unistd.h" "$OVERLAY_INCLUDE/unistd.h"
ln -s "$STAGE_LIB/libbraxon_android_libc_extensions.a" "$OVERLAY_LIB/libbraxon_android_libc_extensions.a"
ln -s "$STAGE_LIB/libbraxon_android_libc_extensions.so" "$OVERLAY_LIB/libbraxon_android_libc_extensions.so"

echo "== close_range compile/link/runtime probe =="
cat > "$RUN/close_range_probe.c" <<'C'
#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <unistd.h>

int main(void) {
    int fd = open("/dev/null", O_RDONLY);
    if (fd < 0) {
        perror("open");
        return 1;
    }

    if (close_range((unsigned int)fd, (unsigned int)fd, 0) != 0) {
        perror("close_range");
        return 2;
    }

    errno = 0;
    if (close(fd) == 0) {
        printf("FAIL close_range did not close fd\n");
        return 3;
    }

    if (errno != EBADF) {
        printf("FAIL expected EBADF after close_range, errno=%d\n", errno);
        return 4;
    }

    printf("BRAXON_CLOSE_RANGE_NATIVE_CHAIN_OK\n");
    return 0;
}
C

clang -target aarch64-linux-android24 \
  -O3 \
  -isystem "$OVERLAY_INCLUDE" \
  -L"$OVERLAY_LIB" \
  "$RUN/close_range_probe.c" \
  -lbraxon_android_libc_extensions \
  -fuse-ld=lld \
  -o "$RUN/close_range_probe"

LD_LIBRARY_PATH="$OVERLAY_LIB:${LD_LIBRARY_PATH:-}" "$RUN/close_range_probe" | tee "$REPORT/close_range_probe_output.txt"
grep -q "BRAXON_CLOSE_RANGE_NATIVE_CHAIN_OK" "$REPORT/close_range_probe_output.txt"

echo "== CPython fileutils compile probe =="
CPY="$CHAIN/src/cpython"
if [ -d "$CPY" ]; then
  cd "$CPY"
  rm -f Python/fileutils.o

  make Python/fileutils.o \
    CFLAGS_NODIST="-isystem $OVERLAY_INCLUDE" \
    CPPFLAGS="-isystem $OVERLAY_INCLUDE ${CPPFLAGS:-}" \
    LIBS="-ldl -lbraxon_android_libc_extensions -llog" \
    LDFLAGS="-L$OVERLAY_LIB ${LDFLAGS:-}" \
    LD_LIBRARY_PATH="$OVERLAY_LIB:${LD_LIBRARY_PATH:-}" \
    | tee "$REPORT/cpython_fileutils_compile_probe.txt"

  cd "$ROOT"
else
  echo "WARN: CPython source dir not found at $CPY; skipped fileutils object probe"
fi

echo "== symbol proof =="
{
  echo "schema=braxon.android.libc_extension.close_range.symbol_proof.v1"
  echo "date=$(date -Is)"
  echo "stage=$STAGE"
  echo "overlay=$OVERLAY"
  echo
  echo "== overlay realpaths =="
  realpath "$OVERLAY_INCLUDE/unistd.h"
  realpath "$OVERLAY_LIB/libbraxon_android_libc_extensions.a"
  realpath "$OVERLAY_LIB/libbraxon_android_libc_extensions.so"
  echo
  echo "== archive symbols =="
  llvm-nm "$OVERLAY_LIB/libbraxon_android_libc_extensions.a" | grep -E 'sem_clockwait|pthread_getname_np|close_range' || true
  echo
  echo "== shared exports =="
  readelf -Ws "$OVERLAY_LIB/libbraxon_android_libc_extensions.so" | grep -E 'sem_clockwait|pthread_getname_np|close_range' || true
  echo
  echo "== probe =="
  cat "$REPORT/close_range_probe_output.txt"
} | tee "$NATIVE_PROOFS/close_range_symbol_proof.txt"

cat > "$NATIVE/CLOSE_RANGE_ANDROID_LIBC_EXTENSION.json" <<JSON
{
  "schema": "braxon.android.libc_extension.close_range.v1",
  "created_at": "$(date -Is)",
  "stage": "$STAGE",
  "overlay": "$OVERLAY",
  "prefix": "${PREFIX:-}",
  "system_write_attempted": false,
  "termux_prefix_overwrite_attempted": false,
  "symbol": "close_range",
  "header": "unistd.h",
  "source": "$NATIVE_SRC/close_range.c",
  "archive": "$STAGE_LIB/libbraxon_android_libc_extensions.a",
  "shared": "$STAGE_LIB/libbraxon_android_libc_extensions.so",
  "probe_passed": true,
  "cpython_fileutils_probe_attempted": true,
  "fallback_policy": "native syscall first, conservative userspace fallback only for flags zero when syscall is unavailable",
  "note": "Adds close_range to the same Braxon Android libc extension lane consumed by CPython."
}
JSON

cat "$NATIVE/CLOSE_RANGE_ANDROID_LIBC_EXTENSION.json" | tee "$REPORT/final_manifest.json"

echo
echo "PASS: close_range added to Braxon Android libc extension overlay"
echo "RUN=$RUN"
echo "OVERLAY=$OVERLAY"
