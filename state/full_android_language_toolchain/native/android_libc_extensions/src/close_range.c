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
