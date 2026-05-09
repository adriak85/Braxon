#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
cd "$ROOT"

STAMP="$(date +%Y%m%d_%H%M%S)"
CHAIN="$ROOT/state/full_android_language_toolchain"
RUN="$CHAIN/runs/android_pwd_contract_overlay_$STAMP"
REPORT="$RUN/reports"

OVERLAY="$CHAIN/install/braxon_android_overlay"
INC="$OVERLAY/include"
LIB="$OVERLAY/lib"

STAGE="$CHAIN/install/braxon_android_builtin_stage"
SINC="$STAGE/include"
SLIB="$STAGE/lib"

SRC="$CHAIN/native/android_libc_extensions/src"
PROOF="$CHAIN/native/android_libc_extensions/proofs"

mkdir -p "$RUN" "$REPORT" "$INC" "$LIB" "$SINC" "$SLIB" "$SRC" "$PROOF"

LOG="$RUN/pwd_contract_overlay.log"
exec > >(tee "$LOG") 2>&1

echo "== Android pwd contract overlay =="
echo "date=$(date -Is)"
echo "overlay=$OVERLAY"
echo "stage=$STAGE"
echo "run=$RUN"
echo

cat > "$SINC/pwd.h" <<'H'
#ifndef BRAXON_ANDROID_PWD_OVERLAY_V1_H
#define BRAXON_ANDROID_PWD_OVERLAY_V1_H

#include_next <pwd.h>

#ifdef __cplusplus
extern "C" {
#endif

void setpwent(void);
struct passwd *getpwent(void);
void endpwent(void);

#ifdef __cplusplus
}
#endif

#endif
H

cat > "$SRC/braxon_android_pwd_contracts_v1.c" <<'C'
#define _GNU_SOURCE
#include <errno.h>
#include <pwd.h>
#include <stddef.h>
#include <string.h>
#include <sys/types.h>
#include <unistd.h>

static int braxon_pwd_used = 0;
static struct passwd braxon_pwd_entry;
static char braxon_pwd_name[64];
static char braxon_pwd_dir[256];
static char braxon_pwd_shell[128];

void setpwent(void) {
    braxon_pwd_used = 0;
}

void endpwent(void) {
    braxon_pwd_used = 1;
}

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

    snprintf(braxon_pwd_name, sizeof(braxon_pwd_name), "u%u", (unsigned)uid);
    snprintf(braxon_pwd_dir, sizeof(braxon_pwd_dir), "%s", getenv("HOME") ? getenv("HOME") : "/");
    snprintf(braxon_pwd_shell, sizeof(braxon_pwd_shell), "%s", getenv("SHELL") ? getenv("SHELL") : "/system/bin/sh");

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

echo "== build pwd object =="
clang -target aarch64-linux-android24 \
  -O3 -fPIC -fvisibility=hidden -fno-semantic-interposition \
  -isystem "$SINC" \
  -c "$SRC/braxon_android_pwd_contracts_v1.c" \
  -o "$SLIB/braxon_android_pwd_contracts_v1.o"

echo "== merge into existing extension archive =="
TMP="$RUN/archive_tmp"
mkdir -p "$TMP"
cd "$TMP"

if [ -f "$SLIB/libbraxon_android_libc_extensions.a" ]; then
  llvm-ar x "$SLIB/libbraxon_android_libc_extensions.a"
fi

cp "$SLIB/braxon_android_pwd_contracts_v1.o" .

llvm-ar rcs "$SLIB/libbraxon_android_libc_extensions.a" ./*.o
llvm-ranlib "$SLIB/libbraxon_android_libc_extensions.a"

cd "$ROOT"

echo "== rebuild shared extension with all staged objects =="
clang -target aarch64-linux-android24 \
  -O3 -fPIC -shared \
  -Wl,-soname,libbraxon_android_libc_extensions.so \
  "$SLIB"/*.o \
  -o "$SLIB/libbraxon_android_libc_extensions.so"

echo "== refresh overlay links =="
rm -f "$INC/pwd.h" \
      "$LIB/libbraxon_android_libc_extensions.a" \
      "$LIB/libbraxon_android_libc_extensions.so"

ln -s "$SINC/pwd.h" "$INC/pwd.h"
ln -s "$SLIB/libbraxon_android_libc_extensions.a" "$LIB/libbraxon_android_libc_extensions.a"
ln -s "$SLIB/libbraxon_android_libc_extensions.so" "$LIB/libbraxon_android_libc_extensions.so"

cat > "$RUN/pwd_probe.c" <<'C'
#include <pwd.h>
#include <stdio.h>

int main(void) {
    setpwent();
    struct passwd *p = getpwent();
    endpwent();

    if (p == 0 || p->pw_name == 0) {
        return 1;
    }

    printf("BRAXON_ANDROID_PWD_CONTRACT_OK:%s\n", p->pw_name);
    return 0;
}
C

clang -target aarch64-linux-android24 \
  -O3 \
  -isystem "$INC" \
  -L"$LIB" \
  "$RUN/pwd_probe.c" \
  -lbraxon_android_libc_extensions \
  -fuse-ld=lld \
  -o "$RUN/pwd_probe"

LD_LIBRARY_PATH="$LIB:${LD_LIBRARY_PATH:-}" "$RUN/pwd_probe" | tee "$REPORT/pwd_probe.txt"
grep -q "BRAXON_ANDROID_PWD_CONTRACT_OK" "$REPORT/pwd_probe.txt"

{
  echo "schema=braxon.android.pwd_contract_overlay_v1.symbol_proof"
  echo "date=$(date -Is)"
  echo "overlay=$OVERLAY"
  echo
  llvm-nm "$LIB/libbraxon_android_libc_extensions.a" | grep -E 'setpwent|getpwent|endpwent'
  echo
  readelf -Ws "$LIB/libbraxon_android_libc_extensions.so" | grep -E 'setpwent|getpwent|endpwent'
} | tee "$PROOF/pwd_contract_overlay_v1_symbol_proof.txt"

echo
echo "PASS: pwd contract overlay built and proved"
echo "RUN=$RUN"
