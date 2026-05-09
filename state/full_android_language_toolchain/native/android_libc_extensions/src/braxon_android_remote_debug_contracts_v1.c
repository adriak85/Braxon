#define _GNU_SOURCE
#include <errno.h>
#include <sys/syscall.h>
#include <sys/types.h>
#include <sys/uio.h>
#include <unistd.h>

#ifndef __NR_process_vm_readv
#define __NR_process_vm_readv 270
#endif

#ifndef __NR_process_vm_writev
#define __NR_process_vm_writev 271
#endif

__attribute__((visibility("default")))
ssize_t process_vm_readv(pid_t pid,
                         const struct iovec *local_iov,
                         unsigned long liovcnt,
                         const struct iovec *remote_iov,
                         unsigned long riovcnt,
                         unsigned long flags) {
    long rc = syscall(__NR_process_vm_readv, pid, local_iov, liovcnt, remote_iov, riovcnt, flags);
    if (rc >= 0) return (ssize_t)rc;
    return -1;
}

__attribute__((visibility("default")))
ssize_t process_vm_writev(pid_t pid,
                          const struct iovec *local_iov,
                          unsigned long liovcnt,
                          const struct iovec *remote_iov,
                          unsigned long riovcnt,
                          unsigned long flags) {
    long rc = syscall(__NR_process_vm_writev, pid, local_iov, liovcnt, remote_iov, riovcnt, flags);
    if (rc >= 0) return (ssize_t)rc;
    return -1;
}
