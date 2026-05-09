#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
CHAIN="$ROOT/state/full_android_language_toolchain"
STAGE="$CHAIN/install/braxon_android_builtin_stage"
OVERLAY="$CHAIN/install/braxon_android_overlay"
SINC="$STAGE/include"
SLIB="$STAGE/lib"
OINC="$OVERLAY/include"
OLIB="$OVERLAY/lib"
SRC="$CHAIN/native/android_libc_extensions/src"
FORCE="$SINC/braxon_android_contracts_force.h"
RUN="$CHAIN/runs/remote_debug_contracts_v1_$(date +%Y%m%d_%H%M%S)"

mkdir -p "$SINC" "$SLIB" "$OINC" "$OLIB" "$SRC" "$RUN"
chmod -R u+rwX "$STAGE" "$OVERLAY" 2>/dev/null || true

cat > "$FORCE" <<'H'
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
H

ln -sfn "$FORCE" "$OINC/braxon_android_contracts_force.h"

cat > "$SRC/braxon_android_remote_debug_contracts_v1.c" <<'C'
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
C

clang -target aarch64-linux-android24 \
  -O3 -fPIC -fvisibility=hidden -fno-semantic-interposition \
  -isystem "$SINC" \
  -c "$SRC/braxon_android_remote_debug_contracts_v1.c" \
  -o "$SLIB/braxon_android_remote_debug_contracts_v1.o"

CONTRACT_OBJ=""
for f in "$SLIB/braxon_android_libc_contracts_v2.o" "$SLIB/braxon_android_libc_contracts.o"; do
  [ -f "$f" ] && CONTRACT_OBJ="$f" && break
done

PWD_OBJ=""
for f in "$SLIB/braxon_android_pwd_contracts_v2.o" "$SLIB/braxon_android_pwd_contracts_v1.o"; do
  [ -f "$f" ] && PWD_OBJ="$f" && break
done

if [ -z "$CONTRACT_OBJ" ]; then
  echo "FAIL: missing base contract object"
  exit 1
fi

TMP="$RUN/objects"
mkdir -p "$TMP"
cp "$CONTRACT_OBJ" "$TMP/"
[ -n "$PWD_OBJ" ] && cp "$PWD_OBJ" "$TMP/"
cp "$SLIB/braxon_android_remote_debug_contracts_v1.o" "$TMP/"

rm -f "$SLIB/libbraxon_android_libc_extensions.a" "$SLIB/libbraxon_android_libc_extensions.so"

llvm-ar rcs "$SLIB/libbraxon_android_libc_extensions.a" "$TMP"/*.o
llvm-ranlib "$SLIB/libbraxon_android_libc_extensions.a"

clang -target aarch64-linux-android24 \
  -O3 -fPIC -shared \
  -Wl,-soname,libbraxon_android_libc_extensions.so \
  "$TMP"/*.o \
  -o "$SLIB/libbraxon_android_libc_extensions.so"

ln -sfn "$SLIB/libbraxon_android_libc_extensions.a" "$OLIB/libbraxon_android_libc_extensions.a"
ln -sfn "$SLIB/libbraxon_android_libc_extensions.so" "$OLIB/libbraxon_android_libc_extensions.so"

llvm-nm "$SLIB/libbraxon_android_libc_extensions.a" \
  | awk '/ T /{print $3}' \
  | sort \
  | uniq -d \
  | tee "$RUN/duplicate_symbols.txt"

if [ -s "$RUN/duplicate_symbols.txt" ]; then
  echo "FAIL: duplicate exported symbols remain"
  cat "$RUN/duplicate_symbols.txt"
  exit 1
fi

llvm-nm "$SLIB/libbraxon_android_libc_extensions.a" \
  | awk '/ T /{print $3}' \
  | sort \
  | tee "$RUN/exported_symbols.txt"

find "$STAGE" "$OVERLAY" -type f -exec chmod 444 {} +
find "$STAGE" "$OVERLAY" -type d -exec chmod 555 {} +

echo "PASS: remote debugging contracts installed"
echo "RUN=$RUN"
