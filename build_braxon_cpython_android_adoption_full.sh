#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
CPY="$TC/src/cpython"
ADOPT_DIR="$TC/adoption/include"
ADOPT="$ADOPT_DIR/braxon_android_posix_adoption_force.h"
LOG="$TC/full_android_adoption_contract_$(date +%Y%m%d_%H%M%S).log"

unalias make clang cc tee rm cp mv 2>/dev/null || true
hash -r 2>/dev/null || true
export PATH="/data/data/com.termux/files/usr/bin:$PATH"

mkdir -p "$ADOPT_DIR" "$TC/profile_catalog/raw"
cd "$CPY"

cat > "$ADOPT" <<'C'
#ifndef BRAXON_ANDROID_POSIX_ADOPTION_FORCE_H
#define BRAXON_ANDROID_POSIX_ADOPTION_FORCE_H 1

#if defined(__ANDROID__)
#ifndef _GNU_SOURCE
#define _GNU_SOURCE 1
#endif

#include <errno.h>
#include <fcntl.h>
#include <grp.h>
#include <pwd.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/syscall.h>
#include <sys/time.h>
#include <sys/types.h>
#include <time.h>
#include <unistd.h>

#ifndef AT_FDCWD
#define AT_FDCWD (-100)
#endif

#ifndef AT_SYMLINK_NOFOLLOW
#define AT_SYMLINK_NOFOLLOW 0x100
#endif

#ifndef SYS_utimensat
#ifdef __NR_utimensat
#define SYS_utimensat __NR_utimensat
#endif
#endif

#ifndef SYS_setns
#ifdef __NR_setns
#define SYS_setns __NR_setns
#endif
#endif

#ifndef SYS_unshare
#ifdef __NR_unshare
#define SYS_unshare __NR_unshare
#endif
#endif

static inline void braxon_android_timeval_to_timespec_pair(
    const struct timeval tv[2],
    struct timespec ts[2]
) {
    ts[0].tv_sec = tv[0].tv_sec;
    ts[0].tv_nsec = tv[0].tv_usec * 1000L;
    ts[1].tv_sec = tv[1].tv_sec;
    ts[1].tv_nsec = tv[1].tv_usec * 1000L;
}

static inline int braxon_android_futimes(int fd, const struct timeval tv[2]) {
#if defined(SYS_utimensat)
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

    braxon_android_timeval_to_timespec_pair(tv, ts);
    return (int)syscall(SYS_utimensat, AT_FDCWD, p, ts, 0);
#else
    (void)fd;
    (void)tv;
    errno = ENOSYS;
    return -1;
#endif
}

static inline int braxon_android_lutimes(const char *path, const struct timeval tv[2]) {
#if defined(SYS_utimensat)
    struct timespec ts[2];

    if (path == NULL) {
        errno = EFAULT;
        return -1;
    }

    if (tv == NULL) {
        return (int)syscall(SYS_utimensat, AT_FDCWD, path, NULL, AT_SYMLINK_NOFOLLOW);
    }

    braxon_android_timeval_to_timespec_pair(tv, ts);
    return (int)syscall(SYS_utimensat, AT_FDCWD, path, ts, AT_SYMLINK_NOFOLLOW);
#else
    (void)path;
    (void)tv;
    errno = ENOSYS;
    return -1;
#endif
}

static inline int braxon_android_setns(int fd, int nstype) {
#if defined(SYS_setns)
    return (int)syscall(SYS_setns, fd, nstype);
#else
    (void)fd;
    (void)nstype;
    errno = ENOSYS;
    return -1;
#endif
}

static inline int braxon_android_unshare(int flags) {
#if defined(SYS_unshare)
    return (int)syscall(SYS_unshare, flags);
#else
    (void)flags;
    errno = ENOSYS;
    return -1;
#endif
}

static struct passwd braxon_android_passwd_entry;
static char braxon_android_pw_name[128];
static char braxon_android_pw_dir[512];
static char braxon_android_pw_shell[512];
static int braxon_android_pw_iter_used = 0;

static inline const char *braxon_android_nonempty_env(const char *key, const char *fallback) {
    const char *v = getenv(key);
    return (v && v[0]) ? v : fallback;
}

static inline struct passwd *braxon_android_current_passwd(void) {
    uid_t uid = getuid();
    gid_t gid = getgid();
    struct passwd *native = getpwuid(uid);

    if (native != NULL) {
        return native;
    }

    snprintf(braxon_android_pw_name, sizeof(braxon_android_pw_name), "u%ld", (long)uid);
    snprintf(braxon_android_pw_dir, sizeof(braxon_android_pw_dir), "%s",
             braxon_android_nonempty_env("HOME", "/data/data/com.termux/files/home"));
    snprintf(braxon_android_pw_shell, sizeof(braxon_android_pw_shell), "%s",
             braxon_android_nonempty_env("SHELL", "/data/data/com.termux/files/usr/bin/sh"));

    memset(&braxon_android_passwd_entry, 0, sizeof(braxon_android_passwd_entry));
    braxon_android_passwd_entry.pw_name = braxon_android_pw_name;
    braxon_android_passwd_entry.pw_passwd = (char *)"x";
    braxon_android_passwd_entry.pw_uid = uid;
    braxon_android_passwd_entry.pw_gid = gid;
    braxon_android_passwd_entry.pw_gecos = braxon_android_pw_name;
    braxon_android_passwd_entry.pw_dir = braxon_android_pw_dir;
    braxon_android_passwd_entry.pw_shell = braxon_android_pw_shell;

    return &braxon_android_passwd_entry;
}

static inline void braxon_android_setpwent(void) {
    braxon_android_pw_iter_used = 0;
}

static inline void braxon_android_endpwent(void) {
    braxon_android_pw_iter_used = 1;
}

static inline struct passwd *braxon_android_getpwent(void) {
    if (braxon_android_pw_iter_used) {
        return NULL;
    }
    braxon_android_pw_iter_used = 1;
    return braxon_android_current_passwd();
}

static struct group braxon_android_group_entry;
static char braxon_android_gr_name[128];
static char *braxon_android_gr_mem[2] = { NULL, NULL };
static int braxon_android_gr_iter_used = 0;

static inline struct group *braxon_android_current_group(void) {
    gid_t gid = getgid();
    struct group *native = getgrgid(gid);

    if (native != NULL) {
        return native;
    }

    snprintf(braxon_android_gr_name, sizeof(braxon_android_gr_name), "g%ld", (long)gid);
    braxon_android_gr_mem[0] = braxon_android_pw_name[0] ? braxon_android_pw_name : (char *)"";

    memset(&braxon_android_group_entry, 0, sizeof(braxon_android_group_entry));
    braxon_android_group_entry.gr_name = braxon_android_gr_name;
    braxon_android_group_entry.gr_passwd = (char *)"x";
    braxon_android_group_entry.gr_gid = gid;
    braxon_android_group_entry.gr_mem = braxon_android_gr_mem;

    return &braxon_android_group_entry;
}

static inline void braxon_android_setgrent(void) {
    braxon_android_gr_iter_used = 0;
}

static inline void braxon_android_endgrent(void) {
    braxon_android_gr_iter_used = 1;
}

static inline struct group *braxon_android_getgrent(void) {
    if (braxon_android_gr_iter_used) {
        return NULL;
    }
    braxon_android_gr_iter_used = 1;
    return braxon_android_current_group();
}

#define futimes  braxon_android_futimes
#define lutimes  braxon_android_lutimes
#define setns    braxon_android_setns
#define unshare  braxon_android_unshare

#define setpwent braxon_android_setpwent
#define getpwent braxon_android_getpwent
#define endpwent braxon_android_endpwent

#define setgrent braxon_android_setgrent
#define getgrent braxon_android_getgrent
#define endgrent braxon_android_endgrent

#endif
#endif
C

# Keep the local Android startup contract patch if already applied.
# It should compare before/after imports instead of failing on startup-preloaded frozen os.
if grep -q "def ensure_lazy_imports" Lib/test/support/import_helper.py; then
  ./python - <<'PY'
from pathlib import Path
import re

p = Path("Lib/test/support/import_helper.py")
s = p.read_text()

if "startup_modules = set(sys.modules)" not in s:
    m = re.search(
        r"def ensure_lazy_imports\(imported_module, modules_to_block, \*, additional_code=None\):\n.*?(?=\ndef [A-Za-z_]|$)",
        s,
        flags=re.S,
    )
    if not m:
        raise SystemExit("could not find ensure_lazy_imports")

    new = '''def ensure_lazy_imports(imported_module, modules_to_block, *, additional_code=None):
    """Check that imported_module does not newly import modules_to_block."""
    from test.support.script_helper import assert_python_ok
    additional_code = additional_code or ""
    script = f"""
import sys
modules_to_block = frozenset({modules_to_block!r})
startup_modules = set(sys.modules)

{additional_code}

import {imported_module}
new_modules = set(sys.modules) - startup_modules
if unexpected := modules_to_block & new_modules:
    after = ", ".join(sorted(unexpected))
    startup = ", ".join(sorted(modules_to_block & startup_modules))
    raise AssertionError(
        f'unexpectedly imported after importing {imported_module}: {{after}}; '
        f'already present at startup: {{startup}}'
    )
"""
    assert_python_ok("-S", "-c", script)
'''
    s = s[:m.start()] + new + s[m.end():]
    p.write_text(s)
    print("patched ensure_lazy_imports")
else:
    print("ensure_lazy_imports already patched")
PY
fi

export CFLAGS_NODIST="${CFLAGS_NODIST:-} -include $ADOPT"
export LLVM_PROFILE_FILE="$TC/profile_catalog/raw/cpython-%p-%m.profraw"
rm -f default.profraw

{
  echo "=== Braxon CPython Android adoption full build ==="
  date
  pwd
  echo "ADOPT=$ADOPT"
  echo "CFLAGS_NODIST=$CFLAGS_NODIST"
  echo "LLVM_PROFILE_FILE=$LLVM_PROFILE_FILE"
  echo "clang=$(command -v clang)"
  echo "make=$(command -v make)"
  echo
} | tee "$LOG"

rm -f \
  Modules/posixmodule.o \
  Modules/pwdmodule.o \
  Modules/grpmodule.o \
  Modules/_multiprocessing/posixshmem_android.o \
  Modules/config.o \
  Programs/python.o \
  python

make -j2 V=1 \
  CFLAGS_NODIST="$CFLAGS_NODIST" \
  Modules/posixmodule.o \
  Modules/pwdmodule.o \
  Modules/grpmodule.o \
  Modules/_multiprocessing/posixshmem_android.o \
  Modules/config.o \
  Programs/python.o \
  python \
  2>&1 | tee -a "$LOG"

echo "=== smoke test ===" | tee -a "$LOG"
./python - <<'PY' 2>&1 | tee -a "$LOG"
import os, sys, pwd, grp
print("python ok:", sys.version.split()[0])
print("pwd current:", pwd.getpwuid(os.getuid()))
print("pwd all sample:", pwd.getpwall()[:3])
print("grp current:", grp.getgrgid(os.getgid()))
print("grp all sample:", grp.getgrall()[:3])
import _posixshmem
print("_posixshmem ok:", _posixshmem)
PY

echo "=== focused functools test ===" | tee -a "$LOG"
./python -m test -v test_functools 2>&1 | tee -a "$LOG"

echo "=== resume full build ===" | tee -a "$LOG"
make -j2 V=1 CFLAGS_NODIST="$CFLAGS_NODIST" 2>&1 | tee -a "$LOG"

STATUS=${PIPESTATUS[0]}
echo "full status: $STATUS" | tee -a "$LOG"
ln -sf "$LOG" "$TC/full_android_adoption_contract_latest.log"
exit "$STATUS"
