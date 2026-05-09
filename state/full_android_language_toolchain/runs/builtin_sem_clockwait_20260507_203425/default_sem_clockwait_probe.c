#include <errno.h>
#include <semaphore.h>
#include <stdio.h>
#include <string.h>
#include <time.h>

int main(void) {
    sem_t sem;
    if (sem_init(&sem, 0, 0) != 0) {
        perror("sem_init");
        return 10;
    }

    struct timespec deadline;
    if (clock_gettime(CLOCK_MONOTONIC, &deadline) != 0) {
        perror("clock_gettime");
        return 11;
    }

    deadline.tv_nsec += 1000000L;
    if (deadline.tv_nsec >= 1000000000L) {
        deadline.tv_sec += 1;
        deadline.tv_nsec -= 1000000000L;
    }

    errno = 0;
    int rc = sem_clockwait(&sem, CLOCK_MONOTONIC, &deadline);
    int err = errno;
    sem_destroy(&sem);

    if (rc == -1 && err == ETIMEDOUT) {
        puts("BRAXON_BUILTIN_SEM_CLOCKWAIT_DEFAULT_OK");
        return 0;
    }

    printf("FAIL rc=%d errno=%d %s\n", rc, err, strerror(err));
    return 12;
}
