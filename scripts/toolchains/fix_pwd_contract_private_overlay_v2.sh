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
RUN="$CHAIN/runs/pwd_contract_private_overlay_v2_$(date +%Y%m%d_%H%M%S)"
REPORT="$RUN/reports"

mkdir -p "$SINC" "$SLIB" "$SRC" "$RUN" "$REPORT"

# Undo the unsafe directory-symlink shape if it exists.
if [ -L "$OINC" ]; then rm -f "$OINC"; fi
if [ -L "$OLIB" ]; then rm -f "$OLIB"; fi
mkdir -p "$OINC" "$OLIB"

# Reopen staged/overlay permissions for this controlled update.
chmod -R u+rwX "$STAGE" "$OVERLAY" 2>/dev/null || true

cat > "$SINC/pwd.h" <<'H'
#ifndef BRAXON_ANDROID_PWD_CONTRACT_OVERLAY_V2_H
#define BRAXON_ANDROID_PWD_CONTRACT_OVERLAY_V2_H

#include_next <pwd.h>

#ifdef __cplusplus
extern "C" {
#endif

void setpwent(void);
struct passwd *getpwent(void);

/*
 * Do NOT redeclare endpwent here.
 * Android/Bionic already provides it as a static no-op in pwd.h.
 */

#ifdef __cplusplus
}
#endif

#endif
H

cat > "$SRC/braxon_android_pwd_contracts_v2.c" <<'C'
#define _GNU_SOURCE
#include <errno.h>
#include <pwd.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>
#include <unistd.h>

static int braxon_pwd_used = 0;
static struct passwd braxon_pwd_entry;
static char braxon_pwd_name[64];
static char braxon_pwd_dir[256];
static char braxon_pwd_shell[128];

__attribute__((visibility("default")))
void setpwent(void) {
    braxon_pwd_used = 0;
}

__attribute__((visibility("default")))
struct passwd *getpwent(void) {
    if (braxon_pwd_used) {
        return NULL;
    }

    braxon_pwd_used = 1;

    uid_t uid = getuid();
    gid_t gid = getgid();

    struct passwd *real = getpwuid(uid);
    if (real != NULL) {
        return real;
    }

    const char *home = getenv("HOME");
    const char *shell = getenv("SHELL");

    snprintf(braxon_pwd_name, sizeof(braxon_pwd_name), "u%u", (unsigned)uid);
    snprintf(braxon_pwd_dir, sizeof(braxon_pwd_dir), "%s", home ? home : "/");
    snprintf(braxon_pwd_shell, sizeof(braxon_pwd_shell), "%s", shell ? shell : "/system/bin/sh");

    memset(&braxon_pwd_entry, 0, sizeof(braxon_pwd_entry));
    braxon_pwd_entry.pw_name = braxon_pwd_name;
    braxon_pwd_entry.pw_passwd = (char *)"*";
    braxon_pwd_entry.pw_uid = uid;
    braxon_pwd_entry.pw_gid = gid;
    braxon_pwd_entry.pw_dir = braxon_pwd_dir;
    braxon_pwd_entry.pw_shell = braxon_pwd_shell;

    return &braxon_pwd_entry;
}
C

clang -target aarch64-linux-android24 \
  -O3 -fPIC -fvisibility=hidden -fno-semantic-interposition \
  -isystem "$SINC" \
  -c "$SRC/braxon_android_pwd_contracts_v2.c" \
  -o "$SLIB/braxon_android_pwd_contracts_v2.o"

TMP="$RUN/archive_tmp"
mkdir -p "$TMP"
cd "$TMP"

if [ -f "$SLIB/libbraxon_android_libc_extensions.a" ]; then
  llvm-ar x "$SLIB/libbraxon_android_libc_extensions.a"
fi

cp "$SLIB/braxon_android_pwd_contracts_v2.o" .
llvm-ar rcs "$SLIB/libbraxon_android_libc_extensions.a" ./*.o
llvm-ranlib "$SLIB/libbraxon_android_libc_extensions.a"

cd "$ROOT"

clang -target aarch64-linux-android24 \
  -O3 -fPIC -shared \
  -Wl,-soname,libbraxon_android_libc_extensions.so \
  "$SLIB"/*.o \
  -o "$SLIB/libbraxon_android_libc_extensions.so"

ln -sfn "$SINC/pwd.h" "$OINC/pwd.h"
ln -sfn "$SLIB/libbraxon_android_libc_extensions.a" "$OLIB/libbraxon_android_libc_extensions.a"
ln -sfn "$SLIB/libbraxon_android_libc_extensions.so" "$OLIB/libbraxon_android_libc_extensions.so"

cat > "$RUN/pwd_probe.c" <<'C'
#include <pwd.h>
#include <stdio.h>

int main(void) {
    setpwent();
    struct passwd *p = getpwent();
    endpwent();

    if (!p || !p->pw_name) return 1;

    printf("BRAXON_PWD_CONTRACT_V2_OK:%s\n", p->pw_name);
    return 0;
}
C

clang -target aarch64-linux-android24 \
  -O3 \
  -isystem "$OINC" \
  -L"$OLIB" \
  "$RUN/pwd_probe.c" \
  "$OLIB/libbraxon_android_libc_extensions.a" \
  -fuse-ld=lld \
  -o "$RUN/pwd_probe"

LD_LIBRARY_PATH="$OLIB:${LD_LIBRARY_PATH:-}" "$RUN/pwd_probe" | tee "$REPORT/pwd_probe.txt"
grep -q "BRAXON_PWD_CONTRACT_V2_OK" "$REPORT/pwd_probe.txt"

llvm-nm "$OLIB/libbraxon_android_libc_extensions.a" | grep -E 'setpwent|getpwent' | tee "$REPORT/pwd_symbols.txt"

# Harden again after proof.
find "$STAGE" "$OVERLAY" -type f -exec chmod 444 {} +
find "$STAGE" "$OVERLAY" -type d -exec chmod 555 {} +

echo "PASS: pwd contract v2 installed into private overlay"
echo "RUN=$RUN"
