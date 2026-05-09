#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
cd "$ROOT"

STAMP="$(date +%Y%m%d_%H%M%S)"
CHAIN="$ROOT/state/full_android_language_toolchain"
RUN="$CHAIN/runs/unified_android_libc_contracts_$STAMP"
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

CPY="$CHAIN/src/cpython"

mkdir -p "$RUN" "$REPORT" \
  "$OVERLAY_INCLUDE" "$OVERLAY_LIB" \
  "$STAGE_INCLUDE" "$STAGE_LIB" \
  "$NATIVE_SRC" "$NATIVE_PROOFS" \
  scripts/toolchains config/toolchains

LOG="$RUN/unified_android_libc_contracts.log"
exec > >(tee "$LOG") 2>&1

echo "== Braxon unified Android libc contract overlay =="
echo "date=$(date -Is)"
echo "root=$ROOT"
echo "chain=$CHAIN"
echo "stage=$STAGE"
echo "overlay=$OVERLAY"
echo "prefix=${PREFIX:-unset}"
echo "run=$RUN"
echo

echo "== policy =="
echo "No direct /system write."
echo "No direct Termux prefix overwrite."
echo "Stage first, symlink overlay second, probe third."
echo "Native syscall first, conservative fallback only where safe."
echo

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

cat > "$STAGE_INCLUDE/unistd.h" <<'H'
#ifndef BRAXON_ANDROID_BUILTIN_UNISTD_H
#define BRAXON_ANDROID_BUILTIN_UNISTD_H
#include_next <unistd.h>
#include <stddef.h>
#include <sys/types.h>
#ifdef __cplusplus
extern "C" {
#endif
#ifndef RWF_HIPRI
typedef long long loff_t;
#endif
int close_range(unsigned int first, unsigned int last, unsigned int flags);
ssize_t copy_file_range(int fd_in, loff_t *off_in, int fd_out, loff_t *off_out, size_t len, unsigned int flags);
int pipe2(int pipefd[2], int flags);
int dup3(int oldfd, int newfd, int flags);
#ifdef __cplusplus
}
#endif
#endif
H

cat > "$STAGE_INCLUDE/sys_stat.h" <<'H'
#ifndef BRAXON_ANDROID_BUILTIN_SYS_STAT_ALIAS_H
#define BRAXON_ANDROID_BUILTIN_SYS_STAT_ALIAS_H
#include_next <sys/stat.h>
#endif
H

mkdir -p "$STAGE_INCLUDE/sys"

cat > "$STAGE_INCLUDE/sys/stat.h" <<'H'
#ifndef BRAXON_ANDROID_BUILTIN_SYS_STAT_H
#define BRAXON_ANDROID_BUILTIN_SYS_STAT_H
#include_next <sys/stat.h>
#include <stdint.h>
#ifdef __cplusplus
extern "C" {
#endif

#ifndef AT_FDCWD
#define AT_FDCWD -100
#endif

#ifndef AT_SYMLINK_NOFOLLOW
#define AT_SYMLINK_NOFOLLOW 0x100
#endif

#ifndef AT_EMPTY_PATH
#define AT_EMPTY_PATH 0x1000
#endif

#ifndef STATX_TYPE
#define STATX_TYPE 0x00000001U
#define STATX_MODE 0x00000002U
#define STATX_NLINK 0x00000004U
#define STATX_UID 0x00000008U
#define STATX_GID 0x00000010U
#define STATX_ATIME 0x00000020U
#define STATX_MTIME 0x00000040U
#define STATX_CTIME 0x00000080U
#define STATX_INO 0x00000100U
#define STATX_SIZE 0x00000200U
#define STATX_BLOCKS 0x00000400U
#define STATX_BASIC_STATS 0x000007ffU
#define STATX_BTIME 0x00000800U
#define STATX_ALL 0x00000fffU
#endif

struct statx_timestamp {
    int64_t tv_sec;
    uint32_t tv_nsec;
    int32_t __reserved;
};

struct statx {
    uint32_t stx_mask;
    uint32_t stx_blksize;
    uint64_t stx_attributes;
    uint32_t stx_nlink;
    uint32_t stx_uid;
    uint32_t stx_gid;
    uint16_t stx_mode;
    uint16_t __spare0[1];
    uint64_t stx_ino;
    uint64_t stx_size;
    uint64_t stx_blocks;
    uint64_t stx_attributes_mask;
    struct statx_timestamp stx_atime;
    struct statx_timestamp stx_btime;
    struct statx_timestamp stx_ctime;
    struct statx_timestamp stx_mtime;
    uint32_t stx_rdev_major;
    uint32_t stx_rdev_minor;
    uint32_t stx_dev_major;
    uint32_t stx_dev_minor;
    uint64_t __spare2[14];
};

int statx(int dirfd, const char *pathname, int flags, unsigned int mask, struct statx *statxbuf);

#ifdef __cplusplus
}
#endif
#endif
H

cat > "$STAGE_INCLUDE/sys/random.h" <<'H'
#ifndef BRAXON_ANDROID_BUILTIN_SYS_RANDOM_H
#define BRAXON_ANDROID_BUILTIN_SYS_RANDOM_H
#include <stddef.h>
#ifdef __cplusplus
extern "C" {
#endif
ssize_t getrandom(void *buf, size_t buflen, unsigned int flags);
#ifdef __cplusplus
}
#endif
#endif
H

cat > "$STAGE_INCLUDE/sys/mman.h" <<'H'
#ifndef BRAXON_ANDROID_BUILTIN_SYS_MMAN_H
#define BRAXON_ANDROID_BUILTIN_SYS_MMAN_H
#include_next <sys/mman.h>
#ifdef __cplusplus
extern "C" {
#endif
int memfd_create(const char *name, unsigned int flags);
#ifdef __cplusplus
}
#endif
#endif
H

cat > "$STAGE_INCLUDE/sys/eventfd.h" <<'H'
#ifndef BRAXON_ANDROID_BUILTIN_SYS_EVENTFD_H
#define BRAXON_ANDROID_BUILTIN_SYS_EVENTFD_H
#include <stdint.h>
#ifdef __cplusplus
extern "C" {
#endif
typedef uint64_t eventfd_t;
int eventfd(unsigned int initval, int flags);
int eventfd_read(int fd, eventfd_t *value);
int eventfd_write(int fd, eventfd_t value);
#ifdef __cplusplus
}
#endif
#endif
H

cat > "$STAGE_INCLUDE/sys/socket.h" <<'H'
#ifndef BRAXON_ANDROID_BUILTIN_SYS_SOCKET_H
#define BRAXON_ANDROID_BUILTIN_SYS_SOCKET_H
#include_next <sys/socket.h>
#ifdef __cplusplus
extern "C" {
#endif
int accept4(int sockfd, struct sockaddr *addr, socklen_t *addrlen, int flags);
#ifdef __cplusplus
}
#endif
#endif
H

cat > "$NATIVE_SRC/braxon_android_libc_contracts.c" <<'C'
#define _GNU_SOURCE

#include <errno.h>
#include <fcntl.h>
#include <limits.h>
#include <pthread.h>
#include <semaphore.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include <sys/prctl.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <sys/syscall.h>
#include <sys/types.h>

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
#ifndef __NR_getrandom
#define __NR_getrandom 278
#endif
#ifndef __NR_memfd_create
#define __NR_memfd_create 279
#endif
#ifndef __NR_eventfd2
#define __NR_eventfd2 19
#endif
#ifndef __NR_pipe2
#define __NR_pipe2 59
#endif
#ifndef __NR_dup3
#define __NR_dup3 24
#endif
#ifndef __NR_accept4
#define __NR_accept4 242
#endif

#ifndef CLOSE_RANGE_UNSHARE
#define CLOSE_RANGE_UNSHARE (1U << 1)
#endif
#ifndef CLOSE_RANGE_CLOEXEC
#define CLOSE_RANGE_CLOEXEC (1U << 2)
#endif

#ifndef GRND_NONBLOCK
#define GRND_NONBLOCK 0x0001
#endif
#ifndef GRND_RANDOM
#define GRND_RANDOM 0x0002
#endif

#ifndef MFD_CLOEXEC
#define MFD_CLOEXEC 0x0001U
#endif
#ifndef MFD_ALLOW_SEALING
#define MFD_ALLOW_SEALING 0x0002U
#endif

typedef uint64_t eventfd_t;

struct braxon_statx_timestamp {
    int64_t tv_sec;
    uint32_t tv_nsec;
    int32_t __reserved;
};

struct braxon_statx {
    uint32_t stx_mask;
    uint32_t stx_blksize;
    uint64_t stx_attributes;
    uint32_t stx_nlink;
    uint32_t stx_uid;
    uint32_t stx_gid;
    uint16_t stx_mode;
    uint16_t __spare0[1];
    uint64_t stx_ino;
    uint64_t stx_size;
    uint64_t stx_blocks;
    uint64_t stx_attributes_mask;
    struct braxon_statx_timestamp stx_atime;
    struct braxon_statx_timestamp stx_btime;
    struct braxon_statx_timestamp stx_ctime;
    struct braxon_statx_timestamp stx_mtime;
    uint32_t stx_rdev_major;
    uint32_t stx_rdev_minor;
    uint32_t stx_dev_major;
    uint32_t stx_dev_minor;
    uint64_t __spare2[14];
};

static inline void braxon_barrier(void) {
#if defined(__aarch64__)
    __asm__ __volatile__("dmb ish" ::: "memory");
#else
    __asm__ __volatile__("" ::: "memory");
#endif
}

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

__attribute__((visibility("default")))
int sem_clockwait(sem_t *sem, clockid_t clockid, const struct timespec *abstime) {
    if (sem == 0 || !braxon_valid_timespec(abstime)) {
        errno = EINVAL;
        return -1;
    }

    braxon_barrier();

    if (clockid == CLOCK_REALTIME) {
        return sem_timedwait(sem, abstime);
    }

    if (clockid == CLOCK_MONOTONIC) {
        struct timespec mono_now;
        struct timespec real_now;

        if (clock_gettime(CLOCK_MONOTONIC, &mono_now) != 0) return -1;
        if (clock_gettime(CLOCK_REALTIME, &real_now) != 0) return -1;

        struct timespec remaining = braxon_timespec_sub(*abstime, mono_now);
        if (remaining.tv_sec < 0) {
            errno = ETIMEDOUT;
            return -1;
        }

        struct timespec real_deadline = braxon_timespec_add(real_now, remaining);
        braxon_barrier();
        return sem_timedwait(sem, &real_deadline);
    }

    errno = EINVAL;
    return -1;
}

static int braxon_copy_thread_name(char *dst, size_t len, const char *src) {
    if (dst == 0 || len == 0) return ERANGE;
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
    if (name == 0 || len == 0) return ERANGE;
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

    braxon_barrier();

    long rc = syscall(__NR_close_range, first, last, flags);
    if (rc == 0) {
        braxon_barrier();
        return 0;
    }

    if (errno != ENOSYS) return -1;

    if (flags != 0U) {
        errno = ENOSYS;
        return -1;
    }

    unsigned int max_fd = last;
    if (max_fd == UINT_MAX) {
        long open_max = sysconf(_SC_OPEN_MAX);
        max_fd = open_max > 0 ? (unsigned int)(open_max - 1) : 1048576U;
    }

    for (unsigned int fd = first; fd <= max_fd; fd++) {
        if (close((int)fd) != 0 && errno != EBADF && errno != EINTR) {
            return -1;
        }
        if (fd == UINT_MAX) break;
    }

    braxon_barrier();
    return 0;
}

__attribute__((visibility("default")))
int statx(int dirfd, const char *pathname, int flags, unsigned int mask, void *statxbuf) {
    if (pathname == 0 || statxbuf == 0) {
        errno = EINVAL;
        return -1;
    }

    braxon_barrier();

    long rc = syscall(__NR_statx, dirfd, pathname, flags, mask, statxbuf);
    if (rc == 0) {
        braxon_barrier();
        return 0;
    }

    return -1;
}

__attribute__((visibility("default")))
ssize_t copy_file_range(int fd_in, loff_t *off_in, int fd_out, loff_t *off_out, size_t len, unsigned int flags) {
    if (flags != 0U) {
        errno = EINVAL;
        return -1;
    }

    braxon_barrier();
    long rc = syscall(__NR_copy_file_range, fd_in, off_in, fd_out, off_out, len, flags);
    if (rc >= 0) {
        braxon_barrier();
        return (ssize_t)rc;
    }

    return -1;
}

__attribute__((visibility("default")))
ssize_t getrandom(void *buf, size_t buflen, unsigned int flags) {
    if (buf == 0 && buflen != 0) {
        errno = EFAULT;
        return -1;
    }

    long rc = syscall(__NR_getrandom, buf, buflen, flags);
    if (rc >= 0) return (ssize_t)rc;
    return -1;
}

__attribute__((visibility("default")))
int memfd_create(const char *name, unsigned int flags) {
    if (name == 0) {
        errno = EFAULT;
        return -1;
    }

    long rc = syscall(__NR_memfd_create, name, flags);
    if (rc >= 0) return (int)rc;
    return -1;
}

__attribute__((visibility("default")))
int eventfd(unsigned int initval, int flags) {
    long rc = syscall(__NR_eventfd2, initval, flags);
    if (rc >= 0) return (int)rc;
    return -1;
}

__attribute__((visibility("default")))
int eventfd_read(int fd, eventfd_t *value) {
    if (value == 0) {
        errno = EINVAL;
        return -1;
    }

    ssize_t got = read(fd, value, sizeof(*value));
    if (got == (ssize_t)sizeof(*value)) return 0;
    if (got >= 0) errno = EINVAL;
    return -1;
}

__attribute__((visibility("default")))
int eventfd_write(int fd, eventfd_t value) {
    ssize_t wrote = write(fd, &value, sizeof(value));
    if (wrote == (ssize_t)sizeof(value)) return 0;
    if (wrote >= 0) errno = EINVAL;
    return -1;
}

__attribute__((visibility("default")))
int pipe2(int pipefd[2], int flags) {
    if (pipefd == 0) {
        errno = EFAULT;
        return -1;
    }

    long rc = syscall(__NR_pipe2, pipefd, flags);
    if (rc == 0) return 0;
    return -1;
}

__attribute__((visibility("default")))
int dup3(int oldfd, int newfd, int flags) {
    if (oldfd == newfd) {
        errno = EINVAL;
        return -1;
    }

    long rc = syscall(__NR_dup3, oldfd, newfd, flags);
    if (rc >= 0) return (int)rc;
    return -1;
}

__attribute__((visibility("default")))
int accept4(int sockfd, struct sockaddr *addr, socklen_t *addrlen, int flags) {
    long rc = syscall(__NR_accept4, sockfd, addr, addrlen, flags);
    if (rc >= 0) return (int)rc;
    return -1;
}
C

echo "== build unified native object/library =="
clang -target aarch64-linux-android24 \
  -O3 \
  -fPIC \
  -fvisibility=hidden \
  -fno-semantic-interposition \
  -isystem "$STAGE_INCLUDE" \
  -c "$NATIVE_SRC/braxon_android_libc_contracts.c" \
  -o "$STAGE_LIB/braxon_android_libc_contracts.o"

llvm-ar rcs "$STAGE_LIB/libbraxon_android_libc_extensions.a" \
  "$STAGE_LIB/braxon_android_libc_contracts.o"

llvm-ranlib "$STAGE_LIB/libbraxon_android_libc_extensions.a"

clang -target aarch64-linux-android24 \
  -O3 \
  -fPIC \
  -fvisibility=hidden \
  -fno-semantic-interposition \
  -shared \
  -Wl,-soname,libbraxon_android_libc_extensions.so \
  "$STAGE_LIB/braxon_android_libc_contracts.o" \
  -o "$STAGE_LIB/libbraxon_android_libc_extensions.so"

echo "== refresh overlay symlinks =="
rm -f "$OVERLAY_INCLUDE/semaphore.h" \
      "$OVERLAY_INCLUDE/pthread.h" \
      "$OVERLAY_INCLUDE/unistd.h" \
      "$OVERLAY_INCLUDE/sys/stat.h" \
      "$OVERLAY_INCLUDE/sys/random.h" \
      "$OVERLAY_INCLUDE/sys/mman.h" \
      "$OVERLAY_INCLUDE/sys/eventfd.h" \
      "$OVERLAY_INCLUDE/sys/socket.h" \
      "$OVERLAY_LIB/libbraxon_android_libc_extensions.a" \
      "$OVERLAY_LIB/libbraxon_android_libc_extensions.so"

mkdir -p "$OVERLAY_INCLUDE/sys" "$OVERLAY_LIB"

ln -s "$STAGE_INCLUDE/semaphore.h" "$OVERLAY_INCLUDE/semaphore.h"
ln -s "$STAGE_INCLUDE/pthread.h" "$OVERLAY_INCLUDE/pthread.h"
ln -s "$STAGE_INCLUDE/unistd.h" "$OVERLAY_INCLUDE/unistd.h"
ln -s "$STAGE_INCLUDE/sys/stat.h" "$OVERLAY_INCLUDE/sys/stat.h"
ln -s "$STAGE_INCLUDE/sys/random.h" "$OVERLAY_INCLUDE/sys/random.h"
ln -s "$STAGE_INCLUDE/sys/mman.h" "$OVERLAY_INCLUDE/sys/mman.h"
ln -s "$STAGE_INCLUDE/sys/eventfd.h" "$OVERLAY_INCLUDE/sys/eventfd.h"
ln -s "$STAGE_INCLUDE/sys/socket.h" "$OVERLAY_INCLUDE/sys/socket.h"
ln -s "$STAGE_LIB/libbraxon_android_libc_extensions.a" "$OVERLAY_LIB/libbraxon_android_libc_extensions.a"
ln -s "$STAGE_LIB/libbraxon_android_libc_extensions.so" "$OVERLAY_LIB/libbraxon_android_libc_extensions.so"

echo "== unified probe =="
cat > "$RUN/unified_contract_probe.c" <<'C'
#include <errno.h>
#include <fcntl.h>
#include <pthread.h>
#include <semaphore.h>
#include <stdio.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include <sys/stat.h>
#include <sys/random.h>

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

    int fd = open("/dev/null", O_RDONLY);
    if (fd < 0) {
        perror("open");
        return 5;
    }

    if (close_range((unsigned int)fd, (unsigned int)fd, 0) != 0) {
        perror("close_range");
        return 6;
    }

    errno = 0;
    if (close(fd) == 0 || errno != EBADF) {
        printf("FAIL close_range verification errno=%d\n", errno);
        return 7;
    }

    struct statx sx;
    memset(&sx, 0, sizeof(sx));
    if (statx(AT_FDCWD, ".", 0, STATX_BASIC_STATS, &sx) != 0) {
        perror("statx");
        return 8;
    }

    unsigned char b[8];
    ssize_t gr = getrandom(b, sizeof(b), 0);
    if (gr != (ssize_t)sizeof(b)) {
        perror("getrandom");
        return 9;
    }

    printf("BRAXON_UNIFIED_ANDROID_LIBC_CONTRACTS_OK:%s\n", name);
    return 0;
}
C

clang -target aarch64-linux-android24 \
  -O3 \
  -isystem "$OVERLAY_INCLUDE" \
  -L"$OVERLAY_LIB" \
  "$RUN/unified_contract_probe.c" \
  -lbraxon_android_libc_extensions \
  -fuse-ld=lld \
  -o "$RUN/unified_contract_probe"

LD_LIBRARY_PATH="$OVERLAY_LIB:${LD_LIBRARY_PATH:-}" "$RUN/unified_contract_probe" | tee "$REPORT/unified_contract_probe_output.txt"
grep -q "BRAXON_UNIFIED_ANDROID_LIBC_CONTRACTS_OK" "$REPORT/unified_contract_probe_output.txt"

echo "== scan CPython for likely undeclared Linux contracts =="
if [ -d "$CPY" ]; then
  {
    echo "schema=braxon.cpython.android_contract_scan.v1"
    echo "date=$(date -Is)"
    echo "cpython=$CPY"
    echo
    grep -RInE '\b(sem_clockwait|pthread_getname_np|close_range|statx|copy_file_range|getrandom|memfd_create|eventfd|eventfd_read|eventfd_write|pipe2|dup3|accept4)\b' \
      "$CPY/Python" "$CPY/Modules" "$CPY/Include" 2>/dev/null || true
  } | tee "$REPORT/cpython_contract_scan.txt"
fi

echo "== symbol proof =="
{
  echo "schema=braxon.android.unified_libc_contracts.symbol_proof.v1"
  echo "date=$(date -Is)"
  echo "stage=$STAGE"
  echo "overlay=$OVERLAY"
  echo
  echo "== archive symbols =="
  llvm-nm "$OVERLAY_LIB/libbraxon_android_libc_extensions.a" | grep -E 'sem_clockwait|pthread_getname_np|close_range|statx|copy_file_range|getrandom|memfd_create|eventfd|eventfd_read|eventfd_write|pipe2|dup3|accept4'
  echo
  echo "== shared exports =="
  readelf -Ws "$OVERLAY_LIB/libbraxon_android_libc_extensions.so" | grep -E 'sem_clockwait|pthread_getname_np|close_range|statx|copy_file_range|getrandom|memfd_create|eventfd|eventfd_read|eventfd_write|pipe2|dup3|accept4'
  echo
  echo "== probe =="
  cat "$REPORT/unified_contract_probe_output.txt"
} | tee "$NATIVE_PROOFS/unified_android_libc_contracts_symbol_proof.txt"

cat > "$NATIVE/UNIFIED_ANDROID_LIBC_CONTRACTS.json" <<JSON
{
  "schema": "braxon.android.unified_libc_contracts.v1",
  "created_at": "$(date -Is)",
  "stage": "$STAGE",
  "overlay": "$OVERLAY",
  "prefix": "${PREFIX:-}",
  "system_write_attempted": false,
  "termux_prefix_overwrite_attempted": false,
  "symbols": [
    "sem_clockwait",
    "pthread_getname_np",
    "close_range",
    "statx",
    "copy_file_range",
    "getrandom",
    "memfd_create",
    "eventfd",
    "eventfd_read",
    "eventfd_write",
    "pipe2",
    "dup3",
    "accept4"
  ],
  "headers": [
    "semaphore.h",
    "pthread.h",
    "unistd.h",
    "sys/stat.h",
    "sys/random.h",
    "sys/mman.h",
    "sys/eventfd.h",
    "sys/socket.h"
  ],
  "probe_passed": true,
  "contract_model": "staged native object plus overlay symlink headers and library",
  "note": "Unified Android/Bionic contract surface for CPython build gaps."
}
JSON

cat "$NATIVE/UNIFIED_ANDROID_LIBC_CONTRACTS.json" | tee "$REPORT/final_manifest.json"

echo
echo "PASS: unified Android libc contract overlay built and proved"
echo "RUN=$RUN"
echo "OVERLAY=$OVERLAY"
