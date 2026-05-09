#ifndef BRAXON_ANDROID_BUILTIN_SYS_RANDOM_H
#define BRAXON_ANDROID_BUILTIN_SYS_RANDOM_H
#include <stddef.h>
#ifdef __cplusplus
extern "C" {
#endif
ssize_t getrandom(void *buf, size_t buflen, unsigned int flags);
#ifdef __cplusplus
}
#endif
#endif
