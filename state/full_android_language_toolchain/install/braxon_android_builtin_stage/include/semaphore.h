#ifndef BRAXON_ANDROID_SEMAPHORE_OVERLAY_V2_H
#define BRAXON_ANDROID_SEMAPHORE_OVERLAY_V2_H
#include_next <semaphore.h>
#include <time.h>
#ifdef __cplusplus
extern "C" {
#endif
int sem_clockwait(sem_t *sem, clockid_t clockid, const struct timespec *abstime);
#ifdef __cplusplus
}
#endif
#endif
