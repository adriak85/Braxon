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
