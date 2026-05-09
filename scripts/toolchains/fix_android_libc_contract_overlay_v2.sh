#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
cd "$ROOT"

STAMP="$(date +%Y%m%d_%H%M%S)"
CHAIN="$ROOT/state/full_android_language_toolchain"
RUN="$CHAIN/runs/android_libc_contract_overlay_v2_$STAMP"
REPORT="$RUN/reports"

OVERLAY="$CHAIN/install/braxon_android_overlay"
INC="$OVERLAY/include"
LIB="$OVERLAY/lib"

STAGE="$CHAIN/install/braxon_android_builtin_stage"
SINC="$STAGE/include"
SLIB="$STAGE/lib"

SRC="$CHAIN/native/android_libc_extensions/src"
PROOF="$CHAIN/native/android_libc_extensions/proofs"

mkdir -p "$RUN" "$REPORT" "$INC/sys" "$LIB" "$SINC/sys" "$SLIB" "$SRC" "$PROOF"

LOG="$RUN/overlay_v2.log"
exec > >(tee "$LOG") 2>&1

echo "== Android libc contract overlay v2 =="
echo "date=$(date -Is)"
echo "overlay=$OVERLAY"
echo "stage=$STAGE"
echo "run=$RUN"
echo

cat > "$SINC/unistd.h" <<'H'
#ifndef BRAXON_ANDROID_UNISTD_OVERLAY_V2_H
#define BRAXON_ANDROID_UNISTD_OVERLAY_V2_H
#include_next <unistd.h>
#include <stddef.h>
#include <sys/types.h>
#ifdef __cplusplus
extern "C" {
#endif
int close_range(unsigned int first, unsigned int last, unsigned int flags);
ssize_t copy_file_range(int fd_in, loff_t *off_in, int fd_out, loff_t *off_out, size_t len, unsigned int flags);
int pipe2(int pipefd[2], int flags);
int dup3(int oldfd, int newfd, int flags);
int fexecve(int fd, char *const argv[], char *const envp[]);
int getlogin_r(char *name, size_t namesize);
int getloadavg(double loadavg[], int nelem);
#ifdef __cplusplus
}
#endif
#endif
H

cat > "$SINC/sys/stat.h" <<'H'
#ifndef BRAXON_ANDROID_SYS_STAT_OVERLAY_V2_H
#define BRAXON_ANDROID_SYS_STAT_OVERLAY_V2_H
#include_next <sys/stat.h>
#include <linux/stat.h>
#ifdef __cplusplus
extern "C" {
#endif
#ifndef AT_FDCWD
#define AT_FDCWD -100
#endif
#ifndef AT_EMPTY_PATH
#define AT_EMPTY_PATH 0x1000
#endif
int statx(int dirfd, const char *pathname, int flags, unsigned int mask, struct statx *statxbuf);
#ifdef __cplusplus
}
#endif
#endif
H

cat > "$SINC/sys/uio.h" <<'H'
#ifndef BRAXON_ANDROID_SYS_UIO_OVERLAY_V2_H
#define BRAXON_ANDROID_SYS_UIO_OVERLAY_V2_H
#include_next <sys/uio.h>
#include <sys/types.h>
#ifdef __cplusplus
extern "C" {
#endif
#ifndef RWF_HIPRI
#define RWF_HIPRI 0x00000001
#endif
#ifndef RWF_DSYNC
#define RWF_DSYNC 0x00000002
#endif
#ifndef RWF_SYNC
#define RWF_SYNC 0x00000004
#endif
#ifndef RWF_NOWAIT
#define RWF_NOWAIT 0x00000008
#endif
#ifndef RWF_APPEND
#define RWF_APPEND 0x00000010
#endif
ssize_t preadv2(int fd, const struct iovec *iov, int iovcnt, off_t offset, int flags);
ssize_t pwritev2(int fd, const struct iovec *iov, int iovcnt, off_t offset, int flags);
#ifdef __cplusplus
}
#endif
#endif
H

cat > "$SINC/semaphore.h" <<'H'
#ifndef BRAXON_ANDROID_SEMAPHORE_OVERLAY_V2_H
#define BRAXON_ANDROID_SEMAPHORE_OVERLAY_V2_H
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

cat > "$SINC/pthread.h" <<'H'
#ifndef BRAXON_ANDROID_PTHREAD_OVERLAY_V2_H
#define BRAXON_ANDROID_PTHREAD_OVERLAY_V2_H
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

cat > "$SRC/braxon_android_libc_contracts_v2.c" <<'C'
#define _GNU_SOURCE
#include <errno.h>
#include <fcntl.h>
#include <limits.h>
#include <pthread.h>
#include <semaphore.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include <linux/stat.h>
#include <sys/prctl.h>
#include <sys/stat.h>
#include <sys/syscall.h>
#include <sys/types.h>
#include <sys/uio.h>

#ifndef PR_GET_NAME
#define PR_GET_NAME 16
#endif
#ifndef __NR_close_range
#define __NR_close_range 436
#endif
#ifndef __NR_statx
#define __NR_statx 291
#endif
#ifndef __NR_copy_file_range
#define __NR_copy_file_range 285
#endif
#ifndef __NR_preadv2
#define __NR_preadv2 286
#endif
#ifndef __NR_pwritev2
#define __NR_pwritev2 287
#endif
#ifndef __NR_pipe2
#define __NR_pipe2 59
#endif
#ifndef __NR_dup3
#define __NR_dup3 24
#endif

static inline void braxon_barrier(void) {
#if defined(__aarch64__)
    __asm__ __volatile__("dmb ish" ::: "memory");
#else
    __asm__ __volatile__("" ::: "memory");
#endif
}

int sem_clockwait(sem_t *sem, clockid_t clockid, const struct timespec *abstime) {
    if (!sem || !abstime || abstime->tv_nsec < 0 || abstime->tv_nsec >= 1000000000L) {
        errno = EINVAL;
        return -1;
    }

    if (clockid == CLOCK_REALTIME) {
        return sem_timedwait(sem, abstime);
    }

    if (clockid != CLOCK_MONOTONIC) {
        errno = EINVAL;
        return -1;
    }

    struct timespec mono_now, real_now;
    if (clock_gettime(CLOCK_MONOTONIC, &mono_now) != 0) return -1;
    if (clock_gettime(CLOCK_REALTIME, &real_now) != 0) return -1;

    struct timespec rem;
    rem.tv_sec = abstime->tv_sec - mono_now.tv_sec;
    rem.tv_nsec = abstime->tv_nsec - mono_now.tv_nsec;
    if (rem.tv_nsec < 0) {
        rem.tv_sec--;
        rem.tv_nsec += 1000000000L;
    }
    if (rem.tv_sec < 0) {
        errno = ETIMEDOUT;
        return -1;
    }

    struct timespec rt;
    rt.tv_sec = real_now.tv_sec + rem.tv_sec;
    rt.tv_nsec = real_now.tv_nsec + rem.tv_nsec;
    if (rt.tv_nsec >= 1000000000L) {
        rt.tv_sec++;
        rt.tv_nsec -= 1000000000L;
    }

    braxon_barrier();
    return sem_timedwait(sem, &rt);
}

int pthread_getname_np(pthread_t thread, char *name, size_t len) {
    if (!name || len == 0) return ERANGE;
    name[0] = '\0';

    if (!pthread_equal(thread, pthread_self())) {
        return ENOSYS;
    }

    char local[16] = {0};
    if (prctl(PR_GET_NAME, (unsigned long)local, 0UL, 0UL, 0UL) != 0) {
        return errno ? errno : ENOSYS;
    }

    size_t n = strnlen(local, sizeof(local));
    if (n >= len) {
        memcpy(name, local, len - 1);
        name[len - 1] = '\0';
        return ERANGE;
    }

    memcpy(name, local, n);
    name[n] = '\0';
    return 0;
}

int close_range(unsigned int first, unsigned int last, unsigned int flags) {
    if (first > last) {
        errno = EINVAL;
        return -1;
    }

    long rc = syscall(__NR_close_range, first, last, flags);
    if (rc == 0) return 0;
    if (errno != ENOSYS || flags != 0) return -1;

    unsigned int max_fd = last;
    if (max_fd == UINT_MAX) {
        long open_max = sysconf(_SC_OPEN_MAX);
        max_fd = open_max > 0 ? (unsigned int)(open_max - 1) : 1048576U;
    }

    for (unsigned int fd = first; fd <= max_fd; fd++) {
        if (close((int)fd) != 0 && errno != EBADF && errno != EINTR) return -1;
        if (fd == UINT_MAX) break;
    }
    return 0;
}

int statx(int dirfd, const char *pathname, int flags, unsigned int mask, struct statx *statxbuf) {
    if (!pathname || !statxbuf) {
        errno = EINVAL;
        return -1;
    }
    long rc = syscall(__NR_statx, dirfd, pathname, flags, mask, statxbuf);
    if (rc == 0) return 0;
    return -1;
}

ssize_t copy_file_range(int fd_in, loff_t *off_in, int fd_out, loff_t *off_out, size_t len, unsigned int flags) {
    long rc = syscall(__NR_copy_file_range, fd_in, off_in, fd_out, off_out, len, flags);
    if (rc >= 0) return (ssize_t)rc;
    return -1;
}

int pipe2(int pipefd[2], int flags) {
    long rc = syscall(__NR_pipe2, pipefd, flags);
    if (rc == 0) return 0;
    return -1;
}

int dup3(int oldfd, int newfd, int flags) {
    if (oldfd == newfd) {
        errno = EINVAL;
        return -1;
    }
    long rc = syscall(__NR_dup3, oldfd, newfd, flags);
    if (rc >= 0) return (int)rc;
    return -1;
}

int fexecve(int fd, char *const argv[], char *const envp[]) {
    char path[64];
    snprintf(path, sizeof(path), "/proc/self/fd/%d", fd);
    execve(path, argv, envp);
    return -1;
}

int getlogin_r(char *name, size_t namesize) {
    if (!name || namesize == 0) return ERANGE;
    const char *u = getlogin();
    if (!u) u = getenv("USER");
    if (!u) u = getenv("LOGNAME");
    if (!u) return ENXIO;
    size_t n = strlen(u);
    if (n >= namesize) return ERANGE;
    memcpy(name, u, n + 1);
    return 0;
}

ssize_t preadv2(int fd, const struct iovec *iov, int iovcnt, off_t offset, int flags) {
    long rc = syscall(__NR_preadv2, fd, iov, iovcnt, offset, flags);
    if (rc >= 0) return (ssize_t)rc;
    if (errno == ENOSYS && flags == 0) return preadv(fd, iov, iovcnt, offset);
    return -1;
}

ssize_t pwritev2(int fd, const struct iovec *iov, int iovcnt, off_t offset, int flags) {
    long rc = syscall(__NR_pwritev2, fd, iov, iovcnt, offset, flags);
    if (rc >= 0) return (ssize_t)rc;
    if (errno == ENOSYS && flags == 0) return pwritev(fd, iov, iovcnt, offset);
    return -1;
}

int getloadavg(double loadavg[], int nelem) {
    if (!loadavg || nelem < 0) {
        errno = EINVAL;
        return -1;
    }
    if (nelem > 3) nelem = 3;

    FILE *f = fopen("/proc/loadavg", "r");
    if (!f) return -1;

    double a = 0.0, b = 0.0, c = 0.0;
    int got = fscanf(f, "%lf %lf %lf", &a, &b, &c);
    fclose(f);

    if (got <= 0) {
        errno = EIO;
        return -1;
    }

    double vals[3] = {a, b, c};
    int n = got < nelem ? got : nelem;
    for (int i = 0; i < n; i++) loadavg[i] = vals[i];
    return n;
}
C

echo "== build v2 native library =="
clang -target aarch64-linux-android24 \
  -O3 -fPIC -fvisibility=hidden -fno-semantic-interposition \
  -isystem "$SINC" \
  -c "$SRC/braxon_android_libc_contracts_v2.c" \
  -o "$SLIB/braxon_android_libc_contracts_v2.o"

llvm-ar rcs "$SLIB/libbraxon_android_libc_extensions.a" "$SLIB/braxon_android_libc_contracts_v2.o"
llvm-ranlib "$SLIB/libbraxon_android_libc_extensions.a"

clang -target aarch64-linux-android24 \
  -O3 -fPIC -shared \
  -Wl,-soname,libbraxon_android_libc_extensions.so \
  "$SLIB/braxon_android_libc_contracts_v2.o" \
  -o "$SLIB/libbraxon_android_libc_extensions.so"

echo "== refresh overlay links =="
rm -f "$INC/unistd.h" "$INC/semaphore.h" "$INC/pthread.h" \
      "$INC/sys/stat.h" "$INC/sys/uio.h" \
      "$LIB/libbraxon_android_libc_extensions.a" \
      "$LIB/libbraxon_android_libc_extensions.so"

ln -s "$SINC/unistd.h" "$INC/unistd.h"
ln -s "$SINC/semaphore.h" "$INC/semaphore.h"
ln -s "$SINC/pthread.h" "$INC/pthread.h"
ln -s "$SINC/sys/stat.h" "$INC/sys/stat.h"
ln -s "$SINC/sys/uio.h" "$INC/sys/uio.h"
ln -s "$SLIB/libbraxon_android_libc_extensions.a" "$LIB/libbraxon_android_libc_extensions.a"
ln -s "$SLIB/libbraxon_android_libc_extensions.so" "$LIB/libbraxon_android_libc_extensions.so"

cat > "$RUN/probe.c" <<'C'
#include <errno.h>
#include <fcntl.h>
#include <pthread.h>
#include <semaphore.h>
#include <stdio.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include <sys/stat.h>
#include <sys/uio.h>

int main(void) {
    char n[64] = {0};
    if (pthread_getname_np(pthread_self(), n, sizeof(n)) != 0) return 1;

    sem_t s;
    if (sem_init(&s, 0, 0) != 0) return 2;

    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    ts.tv_nsec += 1000000L;
    if (ts.tv_nsec >= 1000000000L) {
        ts.tv_sec++;
        ts.tv_nsec -= 1000000000L;
    }

    int wr = sem_clockwait(&s, CLOCK_MONOTONIC, &ts);
    int we = errno;
    sem_destroy(&s);
    if (wr != -1 || we != ETIMEDOUT) return 3;

    struct statx sx;
    memset(&sx, 0, sizeof(sx));
    if (statx(AT_FDCWD, ".", 0, STATX_BASIC_STATS, &sx) != 0) return 4;

    double la[3];
    if (getloadavg(la, 3) < 1) return 5;

    printf("BRAXON_ANDROID_LIBC_CONTRACT_OVERLAY_V2_OK:%s\n", n);
    return 0;
}
C

clang -target aarch64-linux-android24 \
  -O3 -isystem "$INC" -L"$LIB" \
  "$RUN/probe.c" \
  -lbraxon_android_libc_extensions \
  -fuse-ld=lld \
  -o "$RUN/probe"

LD_LIBRARY_PATH="$LIB:${LD_LIBRARY_PATH:-}" "$RUN/probe" | tee "$REPORT/probe.txt"
grep -q "BRAXON_ANDROID_LIBC_CONTRACT_OVERLAY_V2_OK" "$REPORT/probe.txt"

{
  echo "schema=braxon.android.libc_contract_overlay_v2.symbol_proof"
  echo "date=$(date -Is)"
  echo "overlay=$OVERLAY"
  echo
  llvm-nm "$LIB/libbraxon_android_libc_extensions.a" | grep -E 'sem_clockwait|pthread_getname_np|close_range|statx|copy_file_range|pipe2|dup3|fexecve|getlogin_r|preadv2|pwritev2|getloadavg'
  echo
  readelf -Ws "$LIB/libbraxon_android_libc_extensions.so" | grep -E 'sem_clockwait|pthread_getname_np|close_range|statx|copy_file_range|pipe2|dup3|fexecve|getlogin_r|preadv2|pwritev2|getloadavg'
} | tee "$PROOF/overlay_v2_symbol_proof.txt"

echo
echo "PASS: overlay v2 built, linked, and probed"
echo "RUN=$RUN"
echo "OVERLAY=$OVERLAY"
