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
