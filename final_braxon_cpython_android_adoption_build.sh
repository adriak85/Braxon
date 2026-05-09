#!/data/data/com.termux/files/usr/bin/bash
set -u

unalias make clang cc tee rm cp mv ldd readelf 2>/dev/null || true
hash -r 2>/dev/null || true

export PATH="/data/data/com.termux/files/usr/bin:$PATH"

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
SRC="$TC/src/cpython"
ADOPT_DIR="$TC/adoption/include"
ADOPT="$ADOPT_DIR/braxon_android_posix_adoption_force.h"
EXT_SRC_DIR="$TC/adoption/src"
EXT_SRC="$EXT_SRC_DIR/braxon_android_libc_extensions.c"
LIBDIR="$TC/install/sysroot/usr/lib"
BINDIR="$TC/install/sysroot/usr/bin"
LIB="$LIBDIR/libbraxon_android_libc_extensions.so"
LOG="$TC/final_android_adoption_build_$(date +%Y%m%d_%H%M%S).log"

mkdir -p "$ADOPT_DIR" "$EXT_SRC_DIR" "$LIBDIR" "$BINDIR"

cat > "$ADOPT" <<'C'
#ifndef BRAXON_ANDROID_POSIX_ADOPTION_FORCE_H
#define BRAXON_ANDROID_POSIX_ADOPTION_FORCE_H 1

#if defined(__ANDROID__)
#ifndef _GNU_SOURCE
#define _GNU_SOURCE 1
#endif

#include <errno.h>
#include <fcntl.h>
#include <pwd.h>
#include <stdio.h>
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

int braxon_android_futimes(int fd, const struct timeval tv[2]);
int braxon_android_lutimes(const char *path, const struct timeval tv[2]);
int braxon_android_setns(int fd, int nstype);
int braxon_android_unshare(int flags);

void braxon_android_setpwent(void);
struct passwd *braxon_android_getpwent(void);
void braxon_android_endpwent(void);

#define futimes braxon_android_futimes
#define lutimes braxon_android_lutimes
#define setns braxon_android_setns
#define unshare braxon_android_unshare
#define setpwent braxon_android_setpwent
#define getpwent braxon_android_getpwent
#define endpwent braxon_android_endpwent

#endif
#endif
C

cat > "$EXT_SRC" <<'C'
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
C

clang -shared -fPIC -O3 -Wall -Wextra \
  "$EXT_SRC" \
  -o "$LIB"

cat > "$BINDIR/ldd" <<'SH2'
#!/data/data/com.termux/files/usr/bin/bash
set -u
for f in "$@"; do
  echo "$f:"
  if command -v readelf >/dev/null 2>&1; then
    readelf -d "$f" 2>/dev/null | grep 'NEEDED\|RUNPATH\|RPATH' || true
  else
    echo "readelf not found"
  fi
done
SH2
chmod 755 "$BINDIR/ldd"

cd "$SRC" || exit 1

export LD_LIBRARY_PATH="$LIBDIR:${LD_LIBRARY_PATH:-}"
export CFLAGS_NODIST="${CFLAGS_NODIST:-} -include $ADOPT"
export LDFLAGS_NODIST="${LDFLAGS_NODIST:-} -L$LIBDIR -lbraxon_android_libc_extensions"

{
  echo "=== BRAxon final Android CPython adoption build ==="
  date
  pwd
  echo "ADOPT=$ADOPT"
  echo "LIB=$LIB"
  echo "LD_LIBRARY_PATH=$LD_LIBRARY_PATH"
  echo "clang=$(command -v clang)"
  echo "make=$(command -v make)"
  echo "ldd=$BINDIR/ldd"
  echo
} | tee "$LOG"

rm -f \
  Modules/posixmodule.o \
  Modules/pwdmodule.o \
  Modules/config.o \
  Programs/python.o \
  Modules/_multiprocessing/posixshmem_android.o \
  python

command make -j2 V=1 \
  CFLAGS_NODIST="$CFLAGS_NODIST" \
  LDFLAGS_NODIST="$LDFLAGS_NODIST" \
  Modules/posixmodule.o \
  Modules/pwdmodule.o \
  Modules/_multiprocessing/posixshmem_android.o \
  Modules/config.o \
  Programs/python.o \
  python \
  2>&1 | tee -a "$LOG"

STATUS=${PIPESTATUS[0]}
echo "targeted build status: $STATUS" | tee -a "$LOG"
test "$STATUS" -eq 0 || exit "$STATUS"

"$BINDIR/ldd" ./python 2>&1 | tee -a "$LOG"

./python - <<'PY' 2>&1 | tee -a "$LOG"
import os, pwd, sys
print("python:", sys.version)
print("_posixshmem builtin/import check:")
import _posixshmem
print("_posixshmem ok:", _posixshmem)
print("pwd entry:", pwd.getpwuid(os.getuid()).pw_name)
PY

command make -j2 V=1 \
  CFLAGS_NODIST="$CFLAGS_NODIST" \
  LDFLAGS_NODIST="$LDFLAGS_NODIST" \
  2>&1 | tee -a "$LOG"

STATUS=${PIPESTATUS[0]}
echo "full build status: $STATUS" | tee -a "$LOG"
ln -sf "$LOG" "$TC/final_android_adoption_build_latest.log"
exit "$STATUS"
