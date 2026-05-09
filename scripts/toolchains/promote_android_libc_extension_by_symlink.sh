#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
cd "$ROOT"

STAMP="$(date +%Y%m%d_%H%M%S)"
CHAIN="$ROOT/state/full_android_language_toolchain"
RUN="$CHAIN/runs/symlink_promotion_$STAMP"
REPORT="$RUN/reports"

OVERLAY="$CHAIN/install/braxon_android_overlay"
OVERLAY_INCLUDE="$OVERLAY/include"
OVERLAY_LIB="$OVERLAY/lib"

STAGE="$CHAIN/install/braxon_android_builtin_stage"
STAGE_INCLUDE="$STAGE/include"
STAGE_LIB="$STAGE/lib"

LINKROOT="$CHAIN/install/braxon_android_active_links"
LINK_INCLUDE="$LINKROOT/include"
LINK_LIB="$LINKROOT/lib"

NATIVE="$CHAIN/native/android_libc_extensions"
NATIVE_SRC="$NATIVE/src"
NATIVE_PROOFS="$NATIVE/proofs"

mkdir -p "$RUN" "$REPORT" \
  "$OVERLAY_INCLUDE" "$OVERLAY_LIB" \
  "$STAGE_INCLUDE" "$STAGE_LIB" \
  "$LINK_INCLUDE" "$LINK_LIB" \
  "$NATIVE_SRC" "$NATIVE_PROOFS" \
  scripts/toolchains config/toolchains

LOG="$RUN/symlink_promotion.log"
exec > >(tee "$LOG") 2>&1

echo "== Braxon Android libc extension symlink promotion lane =="
echo "date=$(date -Is)"
echo "root=$ROOT"
echo "chain=$CHAIN"
echo "stage=$STAGE"
echo "linkroot=$LINKROOT"
echo "overlay=$OVERLAY"
echo "prefix=${PREFIX:-unset}"
echo "run=$RUN"
echo

echo "== safety policy =="
echo "No direct write to /system."
echo "No direct overwrite of active Termux headers."
echo "Stage first."
echo "Symlink second."
echo "Probe through symlink third."
echo "Transfer only after proof."
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

echo
echo "== write native sem_clockwait =="
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

echo
echo "== write native pthread_getname_np =="
cat > "$NATIVE_SRC/pthread_getname_np.c" <<'C'
#define _GNU_SOURCE
#include <errno.h>
#include <fcntl.h>
#include <pthread.h>
#include <stdio.h>
#include <string.h>
#include <sys/prctl.h>
#include <unistd.h>

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

    /*
     * Android/Bionic does not expose a stable public pthread_t -> tid map.
     * For non-self pthread names, return a correct unsupported result.
     */
    return ENOSYS;
}
C

echo
echo "== build staged native extension library =="
clang -target aarch64-linux-android24 \
  -O3 \
  -fPIC \
  -fvisibility=hidden \
  -fno-semantic-interposition \
  -isystem "$STAGE_INCLUDE" \
  -c "$NATIVE_SRC/sem_clockwait.c" \
  -o "$STAGE_LIB/sem_clockwait.o"

clang -target aarch64-linux-android24 \
  -O3 \
  -fPIC \
  -fvisibility=hidden \
  -fno-semantic-interposition \
  -isystem "$STAGE_INCLUDE" \
  -c "$NATIVE_SRC/pthread_getname_np.c" \
  -o "$STAGE_LIB/pthread_getname_np.o"

llvm-ar rcs "$STAGE_LIB/libbraxon_android_libc_extensions.a" \
  "$STAGE_LIB/sem_clockwait.o" \
  "$STAGE_LIB/pthread_getname_np.o"

llvm-ranlib "$STAGE_LIB/libbraxon_android_libc_extensions.a"

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
echo "== create controlled symlink active surface =="
rm -f "$LINK_INCLUDE/semaphore.h" \
      "$LINK_INCLUDE/pthread.h" \
      "$LINK_LIB/libbraxon_android_libc_extensions.a" \
      "$LINK_LIB/libbraxon_android_libc_extensions.so"

ln -s "$STAGE_INCLUDE/semaphore.h" "$LINK_INCLUDE/semaphore.h"
ln -s "$STAGE_INCLUDE/pthread.h" "$LINK_INCLUDE/pthread.h"
ln -s "$STAGE_LIB/libbraxon_android_libc_extensions.a" "$LINK_LIB/libbraxon_android_libc_extensions.a"
ln -s "$STAGE_LIB/libbraxon_android_libc_extensions.so" "$LINK_LIB/libbraxon_android_libc_extensions.so"

ls -la "$LINK_INCLUDE" "$LINK_LIB" | tee "$REPORT/symlink_listing.txt"

echo
echo "== probe through symlink include/lib path =="
cat > "$RUN/link_probe.c" <<'C'
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

    int wait_rc = sem_clockwait(&sema, CLOCK_MONOTONIC, &ts);
    int wait_errno = errno;
    sem_destroy(&sema);

    if (wait_rc != -1 || wait_errno != ETIMEDOUT) {
        printf("FAIL sem_clockwait rc=%d errno=%d\n", wait_rc, wait_errno);
        return 4;
    }

    printf("BRAXON_SYMLINK_NATIVE_CHAIN_OK:%s\n", name);
    return 0;
}
C

clang -target aarch64-linux-android24 \
  -O3 \
  -isystem "$LINK_INCLUDE" \
  -L"$LINK_LIB" \
  "$RUN/link_probe.c" \
  -lbraxon_android_libc_extensions \
  -fuse-ld=lld \
  -o "$RUN/link_probe"

LD_LIBRARY_PATH="$LINK_LIB:${LD_LIBRARY_PATH:-}" "$RUN/link_probe" | tee "$REPORT/link_probe_output.txt"
grep -q "BRAXON_SYMLINK_NATIVE_CHAIN_OK" "$REPORT/link_probe_output.txt"

echo
echo "== promote proven symlink targets into overlay =="
rm -f "$OVERLAY_INCLUDE/semaphore.h" \
      "$OVERLAY_INCLUDE/pthread.h" \
      "$OVERLAY_LIB/libbraxon_android_libc_extensions.a" \
      "$OVERLAY_LIB/libbraxon_android_libc_extensions.so"

ln -s "$STAGE_INCLUDE/semaphore.h" "$OVERLAY_INCLUDE/semaphore.h"
ln -s "$STAGE_INCLUDE/pthread.h" "$OVERLAY_INCLUDE/pthread.h"
ln -s "$STAGE_LIB/libbraxon_android_libc_extensions.a" "$OVERLAY_LIB/libbraxon_android_libc_extensions.a"
ln -s "$STAGE_LIB/libbraxon_android_libc_extensions.so" "$OVERLAY_LIB/libbraxon_android_libc_extensions.so"

echo
echo "== prove overlay after symlink transfer =="
clang -target aarch64-linux-android24 \
  -O3 \
  -isystem "$OVERLAY_INCLUDE" \
  -L"$OVERLAY_LIB" \
  "$RUN/link_probe.c" \
  -lbraxon_android_libc_extensions \
  -fuse-ld=lld \
  -o "$RUN/overlay_probe"

LD_LIBRARY_PATH="$OVERLAY_LIB:${LD_LIBRARY_PATH:-}" "$RUN/overlay_probe" | tee "$REPORT/overlay_probe_output.txt"
grep -q "BRAXON_SYMLINK_NATIVE_CHAIN_OK" "$REPORT/overlay_probe_output.txt"

echo
echo "== symbol proof =="
{
  echo "schema=braxon.android.symlink_promoted_libc_extension.symbol_proof.v1"
  echo "date=$(date -Is)"
  echo "stage=$STAGE"
  echo "linkroot=$LINKROOT"
  echo "overlay=$OVERLAY"
  echo
  echo "== realpaths =="
  realpath "$LINK_INCLUDE/semaphore.h"
  realpath "$LINK_INCLUDE/pthread.h"
  realpath "$LINK_LIB/libbraxon_android_libc_extensions.a"
  realpath "$LINK_LIB/libbraxon_android_libc_extensions.so"
  echo
  echo "== archive symbols =="
  llvm-nm "$LINK_LIB/libbraxon_android_libc_extensions.a" | grep -E 'sem_clockwait|pthread_getname_np'
  echo
  echo "== shared exports =="
  readelf -Ws "$LINK_LIB/libbraxon_android_libc_extensions.so" | grep -E 'sem_clockwait|pthread_getname_np'
  echo
  echo "== probe =="
  cat "$REPORT/link_probe_output.txt"
  cat "$REPORT/overlay_probe_output.txt"
} | tee "$NATIVE_PROOFS/symlink_promoted_symbol_proof.txt"

cat > "$NATIVE/SYMLINK_PROMOTED_ANDROID_LIBC_EXTENSION.json" <<JSON
{
  "schema": "braxon.android.symlink_promoted_libc_extension.v1",
  "created_at": "$(date -Is)",
  "stage": "$STAGE",
  "linkroot": "$LINKROOT",
  "overlay": "$OVERLAY",
  "prefix": "${PREFIX:-}",
  "system_write_attempted": false,
  "termux_prefix_overwrite_attempted": false,
  "symlink_probe_passed": true,
  "overlay_probe_passed": true,
  "symbols": [
    "sem_clockwait",
    "pthread_getname_np"
  ],
  "transfer_model": "stage_target_to_active_symlink_to_overlay_symlink",
  "reversible": true,
  "note": "This proves the active Braxon build chain can land on the staged files through symlinked header/library positions before any irreversible placement."
}
JSON

cat "$NATIVE/SYMLINK_PROMOTED_ANDROID_LIBC_EXTENSION.json" | tee "$REPORT/final_manifest.json"

echo
echo "PASS: symlink promotion proved and transferred to Braxon overlay"
echo "RUN=$RUN"
echo "OVERLAY=$OVERLAY"
echo "LINKROOT=$LINKROOT"
echo "STAGE=$STAGE"
