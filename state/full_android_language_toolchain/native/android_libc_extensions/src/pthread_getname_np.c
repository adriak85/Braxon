#define _GNU_SOURCE
#include <errno.h>
#include <pthread.h>
#include <stddef.h>
#include <string.h>
#include <sys/prctl.h>

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

    return ENOSYS;
}
