#define _GNU_SOURCE 1
#include <errno.h>
#include <fcntl.h>
#include <pwd.h>
#include <stdio.h>
#include <string.h>
#include <sys/syscall.h>
#include <sys/time.h>
#include <time.h>
#include <unistd.h>

#ifndef AT_FDCWD
#define AT_FDCWD (-100)
#endif

#ifndef AT_SYMLINK_NOFOLLOW
#define AT_SYMLINK_NOFOLLOW 0x100
#endif

static void braxon_timeval_to_timespec_pair(const struct timeval tv[2], struct timespec ts[2]) {
    ts[0].tv_sec = tv[0].tv_sec;
    ts[0].tv_nsec = tv[0].tv_usec * 1000L;
    ts[1].tv_sec = tv[1].tv_sec;
    ts[1].tv_nsec = tv[1].tv_usec * 1000L;
}

int braxon_android_futimes(int fd, const struct timeval tv[2]) {
    char p[64];
    struct timespec ts[2];

    if (fd < 0) {
        errno = EBADF;
        return -1;
    }

    snprintf(p, sizeof(p), "/proc/self/fd/%d", fd);

    if (tv == NULL) {
        return (int)syscall(SYS_utimensat, AT_FDCWD, p, NULL, 0);
    }

    braxon_timeval_to_timespec_pair(tv, ts);
    return (int)syscall(SYS_utimensat, AT_FDCWD, p, ts, 0);
}

int braxon_android_lutimes(const char *path, const struct timeval tv[2]) {
    struct timespec ts[2];

    if (path == NULL) {
        errno = EFAULT;
        return -1;
    }

    if (tv == NULL) {
        return (int)syscall(SYS_utimensat, AT_FDCWD, path, NULL, AT_SYMLINK_NOFOLLOW);
    }

    braxon_timeval_to_timespec_pair(tv, ts);
    return (int)syscall(SYS_utimensat, AT_FDCWD, path, ts, AT_SYMLINK_NOFOLLOW);
}

int braxon_android_setns(int fd, int nstype) {
#ifdef SYS_setns
    return (int)syscall(SYS_setns, fd, nstype);
#else
    (void)fd;
    (void)nstype;
    errno = ENOSYS;
    return -1;
#endif
}

int braxon_android_unshare(int flags) {
#ifdef SYS_unshare
    return (int)syscall(SYS_unshare, flags);
#else
    (void)flags;
    errno = ENOSYS;
    return -1;
#endif
}

/* Android/Bionic-safe passwd enumeration bridge. */
static int braxon_pwd_used = 0;
static struct passwd braxon_pwd_entry;
static char braxon_name[64];
static char braxon_dir[256];
static char braxon_shell[128];

void braxon_android_setpwent(void) {
    braxon_pwd_used = 0;
}

void braxon_android_endpwent(void) {
    braxon_pwd_used = 1;
}

struct passwd *braxon_android_getpwent(void) {
    if (braxon_pwd_used) {
        return NULL;
    }

    braxon_pwd_used = 1;

    uid_t uid = getuid();
    struct passwd *real = getpwuid(uid);
    if (real != NULL) {
        return real;
    }

    snprintf(braxon_name, sizeof(braxon_name), "u%ld", (long)uid);
    snprintf(braxon_dir, sizeof(braxon_dir), "%s", "/data/data/com.termux/files/home");
    snprintf(braxon_shell, sizeof(braxon_shell), "%s", "/data/data/com.termux/files/usr/bin/sh");

    memset(&braxon_pwd_entry, 0, sizeof(braxon_pwd_entry));
    braxon_pwd_entry.pw_name = braxon_name;
    braxon_pwd_entry.pw_passwd = (char *)"x";
    braxon_pwd_entry.pw_uid = uid;
    braxon_pwd_entry.pw_gid = getgid();
    braxon_pwd_entry.pw_dir = braxon_dir;
    braxon_pwd_entry.pw_shell = braxon_shell;

    return &braxon_pwd_entry;
}
