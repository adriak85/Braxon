#ifndef BRAXON_ANDROID_CONTRACTS_FORCE_H
#define BRAXON_ANDROID_CONTRACTS_FORCE_H

#include <pwd.h>
#include <unistd.h>
#include <stddef.h>
#include <sys/types.h>
#include <sys/uio.h>
#include <sys/stat.h>

#ifdef __cplusplus
extern "C" {
#endif

void setpwent(void);
struct passwd *getpwent(void);

int close_range(unsigned int first, unsigned int last, unsigned int flags);
int fexecve(int fd, char *const argv[], char *const envp[]);
int getlogin_r(char *name, size_t namesize);
int getloadavg(double loadavg[], int nelem);

ssize_t preadv(int fd, const struct iovec *iov, int iovcnt, off_t offset);
ssize_t pwritev(int fd, const struct iovec *iov, int iovcnt, off_t offset);
ssize_t preadv2(int fd, const struct iovec *iov, int iovcnt, off_t offset, int flags);
ssize_t pwritev2(int fd, const struct iovec *iov, int iovcnt, off_t offset, int flags);

ssize_t process_vm_readv(pid_t pid,
                         const struct iovec *local_iov,
                         unsigned long liovcnt,
                         const struct iovec *remote_iov,
                         unsigned long riovcnt,
                         unsigned long flags);

ssize_t process_vm_writev(pid_t pid,
                          const struct iovec *local_iov,
                          unsigned long liovcnt,
                          const struct iovec *remote_iov,
                          unsigned long riovcnt,
                          unsigned long flags);

#ifndef STATX_BASIC_STATS
#define STATX_BASIC_STATS 0x000007ffU
#endif

#ifndef AT_EMPTY_PATH
#define AT_EMPTY_PATH 0x1000
#endif

int statx(int dirfd, const char *pathname, int flags, unsigned int mask, struct statx *statxbuf);

#ifdef __cplusplus
}
#endif

#endif
