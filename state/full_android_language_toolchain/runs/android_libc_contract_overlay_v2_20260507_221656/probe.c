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
