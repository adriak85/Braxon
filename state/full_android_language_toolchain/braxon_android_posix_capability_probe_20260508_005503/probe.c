#define _GNU_SOURCE
#define _POSIX_C_SOURCE 200809L
#include <stdio.h>
#include <errno.h>
#include <string.h>
#include <sys/time.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>
#include <sys/syscall.h>

#ifndef SYS_setns
#define SYS_setns -1
#endif
#ifndef SYS_unshare
#define SYS_unshare -1
#endif

int main(void) {
    printf("__ANDROID__=%s\n",
#ifdef __ANDROID__
    "1"
#else
    "0"
#endif
    );

    printf("SYS_setns=%ld\n", (long)SYS_setns);
    printf("SYS_unshare=%ld\n", (long)SYS_unshare);

#ifdef futimes
    printf("futimes_macro=1\n");
#else
    printf("futimes_macro=0\n");
#endif

#ifdef lutimes
    printf("lutimes_macro=1\n");
#else
    printf("lutimes_macro=0\n");
#endif

    errno = 0;
    if (SYS_unshare != -1) {
        long r = syscall(SYS_unshare, 0);
        printf("syscall_unshare_0=%ld errno=%d %s\n", r, errno, strerror(errno));
    }

    errno = 0;
    if (SYS_setns != -1) {
        long r = syscall(SYS_setns, -1, 0);
        printf("syscall_setns_badfd=%ld errno=%d %s\n", r, errno, strerror(errno));
    }

    return 0;
}
