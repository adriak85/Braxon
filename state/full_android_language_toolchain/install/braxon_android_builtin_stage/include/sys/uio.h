#ifndef BRAXON_ANDROID_SYS_UIO_OVERLAY_V2_H
#define BRAXON_ANDROID_SYS_UIO_OVERLAY_V2_H
#include_next <sys/uio.h>
#include <sys/types.h>
#ifdef __cplusplus
extern "C" {
#endif
#ifndef RWF_HIPRI
#define RWF_HIPRI 0x00000001
#endif
#ifndef RWF_DSYNC
#define RWF_DSYNC 0x00000002
#endif
#ifndef RWF_SYNC
#define RWF_SYNC 0x00000004
#endif
#ifndef RWF_NOWAIT
#define RWF_NOWAIT 0x00000008
#endif
#ifndef RWF_APPEND
#define RWF_APPEND 0x00000010
#endif
ssize_t preadv2(int fd, const struct iovec *iov, int iovcnt, off_t offset, int flags);
ssize_t pwritev2(int fd, const struct iovec *iov, int iovcnt, off_t offset, int flags);
#ifdef __cplusplus
}
#endif
#endif
