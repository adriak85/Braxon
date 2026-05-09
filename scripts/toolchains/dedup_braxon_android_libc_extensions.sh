#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
CHAIN="$ROOT/state/full_android_language_toolchain"
STAGE="$CHAIN/install/braxon_android_builtin_stage"
OVERLAY="$CHAIN/install/braxon_android_overlay"
SLIB="$STAGE/lib"
OLIB="$OVERLAY/lib"
RUN="$CHAIN/runs/dedup_android_libc_extensions_$(date +%Y%m%d_%H%M%S)"
mkdir -p "$RUN" "$OLIB"

chmod -R u+rwX "$STAGE" "$OVERLAY" 2>/dev/null || true

echo "== remove duplicate split objects =="
rm -f \
  "$SLIB/sem_clockwait.o" \
  "$SLIB/pthread_getname_np.o" \
  "$SLIB/close_range.o" \
  "$SLIB/statx.o" \
  "$SLIB/copy_file_range.o" \
  "$SLIB/getrandom.o" \
  "$SLIB/memfd_create.o" \
  "$SLIB/eventfd.o" \
  "$SLIB/pipe2.o" \
  "$SLIB/dup3.o" \
  "$SLIB/accept4.o"

echo "== rebuild archive from unique staged objects =="
TMP="$RUN/objects"
mkdir -p "$TMP"
cp "$SLIB"/*.o "$TMP"/

cd "$TMP"
llvm-ar rcs "$SLIB/libbraxon_android_libc_extensions.a" ./*.o
llvm-ranlib "$SLIB/libbraxon_android_libc_extensions.a"

cd "$ROOT"

echo "== rebuild shared from unique staged objects =="
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

echo "PASS: libbraxon_android_libc_extensions deduped"
echo "RUN=$RUN"
