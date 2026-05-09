#ifndef BRAXON_ANDROID_BUILTIN_SYS_EVENTFD_H
#define BRAXON_ANDROID_BUILTIN_SYS_EVENTFD_H
#include <stdint.h>
#ifdef __cplusplus
extern "C" {
#endif
typedef uint64_t eventfd_t;
int eventfd(unsigned int initval, int flags);
int eventfd_read(int fd, eventfd_t *value);
int eventfd_write(int fd, eventfd_t value);
#ifdef __cplusplus
}
#endif
#endif
