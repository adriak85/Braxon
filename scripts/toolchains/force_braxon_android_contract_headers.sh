#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
CHAIN="$ROOT/state/full_android_language_toolchain"
OVERLAY="$CHAIN/install/braxon_android_overlay"
STAGE="$CHAIN/install/braxon_android_builtin_stage"
FORCE="$STAGE/include/braxon_android_contracts_force.h"
ENVFILE="$CHAIN/USE_BRAXON_PRIVATE_CC.env"

chmod -R u+rwX "$STAGE" "$OVERLAY" 2>/dev/null || true
mkdir -p "$STAGE/include" "$OVERLAY/include"

cat > "$FORCE" <<'H'
#ifndef BRAXON_ANDROID_CONTRACTS_FORCE_H
#define BRAXON_ANDROID_CONTRACTS_FORCE_H

#include <pwd.h>
#include <unistd.h>
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

ssize_t preadv2(int fd, const struct iovec *iov, int iovcnt, off_t offset, int flags);
ssize_t pwritev2(int fd, const struct iovec *iov, int iovcnt, off_t offset, int flags);

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

ln -sfn "$FORCE" "$OVERLAY/include/braxon_android_contracts_force.h"

cat > "$ENVFILE" <<ENV
export BRAXON_ANDROID_OVERLAY="$OVERLAY"
export PATH="$CHAIN/install/braxon_private_bin:\$PATH"
export CC="braxon-clang"
export CXX="braxon-clang++"
export CPPFLAGS="-isystem $OVERLAY/include -include $OVERLAY/include/braxon_android_contracts_force.h \${CPPFLAGS:-}"
export CFLAGS="-isystem $OVERLAY/include -include $OVERLAY/include/braxon_android_contracts_force.h \${CFLAGS:-}"
export CFLAGS_NODIST="-isystem $OVERLAY/include -include $OVERLAY/include/braxon_android_contracts_force.h \${CFLAGS_NODIST:-}"
export LDFLAGS="-L$OVERLAY/lib \${LDFLAGS:-}"
export LDFLAGS_NODIST="-L$OVERLAY/lib \${LDFLAGS_NODIST:-}"
export LIBS="-ldl -lbraxon_android_libc_extensions -llog"
export LD_LIBRARY_PATH="$OVERLAY/lib:\${LD_LIBRARY_PATH:-}"
ENV

find "$STAGE" "$OVERLAY" -type f -exec chmod 444 {} +
find "$STAGE" "$OVERLAY" -type d -exec chmod 555 {} +

echo "PASS: forced Braxon Android contract header installed"
echo "force_header=$FORCE"
echo "env=$ENVFILE"
