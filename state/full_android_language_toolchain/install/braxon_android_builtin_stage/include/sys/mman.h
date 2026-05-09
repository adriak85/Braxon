#ifndef BRAXON_ANDROID_BUILTIN_SYS_MMAN_H
#define BRAXON_ANDROID_BUILTIN_SYS_MMAN_H
#include_next <sys/mman.h>
#ifdef __cplusplus
extern "C" {
#endif
int memfd_create(const char *name, unsigned int flags);
#ifdef __cplusplus
}
#endif
#endif
