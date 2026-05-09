#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
TERM_ENV="$TC/terminal/braxon-term-1/braxon-terminal.env"
OUT="$TC/audit_reseat_braxon_rust_nightly_$(date +%Y%m%d_%H%M%S).log"
LOCKDIR="$TC/locks/braxon_rust_nightly_native"
mkdir -p "$LOCKDIR" "$ROOT/scripts"

{
  echo "=== Braxon Rust nightly audit / reseat ==="
  date
  echo

  [ -f "$TERM_ENV" ] && source "$TERM_ENV" || true

  export LD_LIBRARY_PATH="$TC/install/braxon_android_overlay/lib:/data/data/com.termux/files/usr/lib"
  export PATH="$ROOT:$TC/terminal/braxon-term-1/bin:$TC/install/python/bin:/data/data/com.termux/files/usr/opt/rust-nightly/bin:/data/data/com.termux/files/home/.cargo/bin:/data/data/com.termux/files/usr/bin:$PATH"

  echo "=== candidates ==="
  for x in rustc cargo rustdoc rustfmt clippy-driver llvm-ar llvm-ranlib clang ld.lld; do
    printf "%-16s " "$x"
    command -v "$x" || true
  done
  echo

  echo "=== rustc proof ==="
  rustc --version --verbose
  echo

  echo "=== cargo proof ==="
  cargo --version --verbose
  echo

  echo "=== component proof ==="
  rustdoc --version || true
  rustfmt --version || true
  clippy-driver --version || true
  echo

  echo "=== target proof ==="
  rustc --print target-list | grep -E '^aarch64-linux-android$|^aarch64-unknown-linux-gnu$|^wasm32' || true
  echo

  echo "=== sysroot proof ==="
  SYSROOT="$(rustc --print sysroot)"
  echo "SYSROOT=$SYSROOT"
  find "$SYSROOT" -maxdepth 4 \( -name 'libstd-*.rlib' -o -name 'libcore-*.rlib' -o -name 'liballoc-*.rlib' \) | sort | head -80
  echo

  echo "=== wrapper install ==="
  cat > "$ROOT/braxon-rust-env" <<EOF
export ROOT="$ROOT"
export TC="$TC"
export PATH="$ROOT:$TC/terminal/braxon-term-1/bin:$TC/install/python/bin:/data/data/com.termux/files/usr/opt/rust-nightly/bin:/data/data/com.termux/files/home/.cargo/bin:/data/data/com.termux/files/usr/bin:\$PATH"
export LD_LIBRARY_PATH="$TC/install/braxon_android_overlay/lib:/data/data/com.termux/files/usr/lib"
export CC="/data/data/com.termux/files/usr/bin/clang"
export CXX="/data/data/com.termux/files/usr/bin/clang++"
export AR="/data/data/com.termux/files/usr/bin/llvm-ar"
export RANLIB="/data/data/com.termux/files/usr/bin/llvm-ranlib"
export LD="/data/data/com.termux/files/usr/bin/ld.lld"
export CFLAGS="-include $TC/adoption/include/braxon_android_posix_adoption_force.h"
export CXXFLAGS="-include $TC/adoption/include/braxon_android_posix_adoption_force.h"
export LDFLAGS="-L$TC/install/braxon_android_overlay/lib -lbraxon_android_libc_extensions"
EOF

  cat > "$ROOT/braxon-rustc" <<'EOF'
#!/data/data/com.termux/files/usr/bin/bash
source /data/data/com.termux/files/home/Braxon/braxon-rust-env
exec rustc "$@"
EOF

  cat > "$ROOT/braxon-cargo" <<'EOF'
#!/data/data/com.termux/files/usr/bin/bash
source /data/data/com.termux/files/home/Braxon/braxon-rust-env
exec cargo "$@"
EOF

  chmod +x "$ROOT/braxon-rust-env" "$ROOT/braxon-rustc" "$ROOT/braxon-cargo"

  echo "=== wrapper proof ==="
  "$ROOT/braxon-rustc" --version --verbose
  "$ROOT/braxon-cargo" --version --verbose
  echo

  echo "=== compile smoke ==="
  TMP="$TC/tmp/rust_nightly_probe"
  rm -rf "$TMP"
  mkdir -p "$TMP"
  cat > "$TMP/main.rs" <<'EOF'
fn main() {
    let x: u128 = 225370;
    println!("braxon rust nightly ok: {}", x);
}
EOF
  "$ROOT/braxon-rustc" "$TMP/main.rs" -O -o "$TMP/probe"
  "$TMP/probe"
  file "$TMP/probe" || true
  echo

  echo "=== workspace smoke ==="
  cd "$ROOT"
  if [ -f Cargo.toml ]; then
    "$ROOT/braxon-cargo" fmt --all --check || true
    "$ROOT/braxon-cargo" test -p nsq-core -- --nocapture || true
    "$ROOT/braxon-cargo" test -p nsq-runtime -- --nocapture || true
    "$ROOT/braxon-cargo" test -p braxon-core -- --nocapture || true
    "$ROOT/braxon-cargo" test -p braxon-ingest -- --nocapture || true
  else
    echo "No Cargo.toml at $ROOT"
  fi
  echo

  echo "=== lock manifest ==="
  {
    echo "BRAXON_RUST_NIGHTLY_NATIVE_LOCK=1"
    date
    echo "RUSTC=$(command -v rustc)"
    echo "CARGO=$(command -v cargo)"
    echo "SYSROOT=$SYSROOT"
    "$ROOT/braxon-rustc" --version --verbose
    "$ROOT/braxon-cargo" --version --verbose
  } > "$LOCKDIR/LOCKED_RUST_NIGHTLY_NATIVE.txt"

  find "$SYSROOT" "$ROOT/braxon-rust-env" "$ROOT/braxon-rustc" "$ROOT/braxon-cargo" \
    -type f -print0 | sort -z | xargs -0 sha256sum > "$LOCKDIR/manifest.sha256"

  echo "LOCKED:"
  echo "$LOCKDIR"
  echo "$LOCKDIR/LOCKED_RUST_NIGHTLY_NATIVE.txt"
  echo "$LOCKDIR/manifest.sha256"
  echo "$ROOT/braxon-rustc"
  echo "$ROOT/braxon-cargo"
} 2>&1 | tee "$OUT"

ln -sf "$OUT" "$TC/audit_reseat_braxon_rust_nightly_latest.log"
echo "log: $OUT"
