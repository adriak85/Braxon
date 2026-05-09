#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
CHAIN="$ROOT/state/full_android_language_toolchain"
STAGE="$CHAIN/install/braxon_android_builtin_stage"
OVERLAY="$CHAIN/install/braxon_android_overlay"
SLIB="$STAGE/lib"
OLIB="$OVERLAY/lib"
RUN="$CHAIN/runs/rebuild_libc_extensions_whitelist_$(date +%Y%m%d_%H%M%S)"

mkdir -p "$RUN" "$OLIB"
chmod -R u+rwX "$STAGE" "$OVERLAY" 2>/dev/null || true

echo "== staged objects =="
ls -1 "$SLIB"/*.o 2>/dev/null | sed 's#^.*/##' | tee "$RUN/staged_objects.txt"

CONTRACT_OBJ=""
for f in \
  "$SLIB/braxon_android_libc_contracts_v2.o" \
  "$SLIB/braxon_android_libc_contracts.o"
do
  if [ -f "$f" ]; then
    CONTRACT_OBJ="$f"
    break
  fi
done

PWD_OBJ=""
for f in \
  "$SLIB/braxon_android_pwd_contracts_v2.o" \
  "$SLIB/braxon_android_pwd_contracts_v1.o"
do
  if [ -f "$f" ]; then
    PWD_OBJ="$f"
    break
  fi
done

if [ -z "$CONTRACT_OBJ" ]; then
  echo "FAIL: no unified libc contract object found"
  exit 1
fi

echo "using_contract_object=$CONTRACT_OBJ"
echo "using_pwd_object=${PWD_OBJ:-none}"

TMP="$RUN/objects"
mkdir -p "$TMP"

cp "$CONTRACT_OBJ" "$TMP/"
if [ -n "$PWD_OBJ" ]; then
  cp "$PWD_OBJ" "$TMP/"
fi

echo "== rebuild archive from whitelist only =="
rm -f "$SLIB/libbraxon_android_libc_extensions.a" \
      "$SLIB/libbraxon_android_libc_extensions.so"

llvm-ar rcs "$SLIB/libbraxon_android_libc_extensions.a" "$TMP"/*.o
llvm-ranlib "$SLIB/libbraxon_android_libc_extensions.a"

echo "== rebuild shared from whitelist only =="
clang -target aarch64-linux-android24 \
  -O3 -fPIC -shared \
  -Wl,-soname,libbraxon_android_libc_extensions.so \
  "$TMP"/*.o \
  -o "$SLIB/libbraxon_android_libc_extensions.so"

ln -sfn "$SLIB/libbraxon_android_libc_extensions.a" "$OLIB/libbraxon_android_libc_extensions.a"
ln -sfn "$SLIB/libbraxon_android_libc_extensions.so" "$OLIB/libbraxon_android_libc_extensions.so"

echo "== duplicate symbol check =="
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

echo "== exported symbols =="
llvm-nm "$SLIB/libbraxon_android_libc_extensions.a" \
  | awk '/ T /{print $3}' \
  | sort \
  | tee "$RUN/exported_symbols.txt"

echo "PASS: whitelist rebuild cleared duplicate exported symbols"
echo "RUN=$RUN"
