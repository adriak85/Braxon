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
