#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
CHAIN="$ROOT/state/full_android_language_toolchain"

STAGE="$CHAIN/install/braxon_android_builtin_stage"
OVERLAY="$CHAIN/install/braxon_android_overlay"
BIN="$CHAIN/install/braxon_private_bin"

mkdir -p "$STAGE/include" "$STAGE/lib" "$OVERLAY/include" "$OVERLAY/lib" "$BIN"

# Symlink layer: overlay points to staged files.
# Android/Termux does not see this unless the wrapper adds -isystem/-L.
ln -sfn "$STAGE/include" "$OVERLAY/include"
ln -sfn "$STAGE/lib" "$OVERLAY/lib"

cat > "$BIN/braxon-clang" <<'SH'
#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

OVERLAY="${BRAXON_ANDROID_OVERLAY:?BRAXON_ANDROID_OVERLAY not set}"

exec clang \
  -isystem "$OVERLAY/include" \
  -L"$OVERLAY/lib" \
  "$@"
SH

cat > "$BIN/braxon-clang++" <<'SH'
#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

OVERLAY="${BRAXON_ANDROID_OVERLAY:?BRAXON_ANDROID_OVERLAY not set}"

exec clang++ \
  -isystem "$OVERLAY/include" \
  -L"$OVERLAY/lib" \
  "$@"
SH

chmod 755 "$BIN/braxon-clang" "$BIN/braxon-clang++"

# Permission hardening: readable/executable, not writable by accident.
chmod -R u=rwX,go= "$STAGE" "$OVERLAY"
find "$STAGE" "$OVERLAY" -type f -exec chmod 444 {} +
find "$STAGE" "$OVERLAY" -type d -exec chmod 555 {} +

cat > "$CHAIN/USE_BRAXON_PRIVATE_CC.env" <<ENV
export BRAXON_ANDROID_OVERLAY="$OVERLAY"
export PATH="$BIN:\$PATH"
export CC="braxon-clang"
export CXX="braxon-clang++"
export CPPFLAGS="-isystem $OVERLAY/include \${CPPFLAGS:-}"
export CFLAGS="-isystem $OVERLAY/include \${CFLAGS:-}"
export CFLAGS_NODIST="-isystem $OVERLAY/include \${CFLAGS_NODIST:-}"
export LDFLAGS="-L$OVERLAY/lib \${LDFLAGS:-}"
export LDFLAGS_NODIST="-L$OVERLAY/lib \${LDFLAGS_NODIST:-}"
export LIBS="-ldl $OVERLAY/lib/libbraxon_android_libc_extensions.a -llog"
export LD_LIBRARY_PATH="$OVERLAY/lib:\${LD_LIBRARY_PATH:-}"
ENV

echo "PASS: Braxon private compiler overlay installed"
echo "source $CHAIN/USE_BRAXON_PRIVATE_CC.env"
