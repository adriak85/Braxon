#ifndef BRAXON_ANDROID_SYS_STAT_OVERLAY_V2_H
#define BRAXON_ANDROID_SYS_STAT_OVERLAY_V2_H
#include_next <sys/stat.h>
#include <linux/stat.h>
#ifdef __cplusplus
extern "C" {
#endif
#ifndef AT_FDCWD
#define AT_FDCWD -100
#endif
#ifndef AT_EMPTY_PATH
#define AT_EMPTY_PATH 0x1000
#endif
int statx(int dirfd, const char *pathname, int flags, unsigned int mask, struct statx *statxbuf);
#ifdef __cplusplus
}
#endif
#endif
