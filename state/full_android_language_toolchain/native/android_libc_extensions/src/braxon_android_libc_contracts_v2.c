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
