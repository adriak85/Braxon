#include <errno.h>
#include <semaphore.h>
#include <stdint.h>
#include <stdio.h>
#include <time.h>

int main(void) {
    struct timespec ts;
    ts.tv_sec = 0;
    ts.tv_nsec = 0;
    (void)ts;
    printf("BRAXON_HEADERS_VISIBLE_WITH_SEM_CLOCKWAIT_DECL=%p\n", (void *)&sem_clockwait);
    return EINVAL == 22 ? 0 : 0;
}
