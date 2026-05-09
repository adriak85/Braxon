#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
OUT="$TC/mend_rustup_toolchain_lane_$(date +%Y%m%d_%H%M%S).log"
mkdir -p "$TC/tmp" "$TC/locks" "$ROOT/scripts"

{
  echo "=== Braxon rustup/toolchain lane mend ==="
  date
  echo

  source "$ROOT/braxon-rust-env" 2>/dev/null || true
  source "$TC/terminal/braxon-term-1/braxon-terminal.env" 2>/dev/null || true

  export PATH="$ROOT:$TC/install/python/bin:/data/data/com.termux/files/usr/opt/rust-nightly/bin:/data/data/com.termux/files/home/.cargo/bin:/data/data/com.termux/files/usr/bin:$PATH"
  export LD_LIBRARY_PATH="$TC/install/braxon_android_overlay/lib:/data/data/com.termux/files/usr/lib"

  echo "=== current proven anchors ==="
  "$ROOT/braxon-python" -c 'import sys, math, _math_integer; print(sys.version); print(math.__file__); print(_math_integer.__file__)'
  rustc --version --verbose
  cargo --version --verbose
  clang --version | head -n 3
  clang -dumpmachine
  echo

  echo "=== package availability check ==="
  pkg search rustup || true
  apt-cache search rustup || true
  echo

  echo "=== installed/missing build tools ==="
  for x in rustup rustc cargo rustdoc rustfmt clippy-driver clang clang++ ld.lld lld llvm-ar llvm-ranlib cmake ninja make pkg-config git curl perl cpan prove python python3 openssl zlib tar xz unzip patch gawk sed grep file sha256sum; do
    printf "%-18s " "$x"
    command -v "$x" || true
  done
  echo

  echo "=== rustup direct probe ==="
  if command -v rustup >/dev/null 2>&1; then
    rustup --version || true
    rustup show || true
  else
    echo "rustup command missing"
  fi
  echo

  echo "=== cargo-installed rustup probe ==="
  if command -v cargo >/dev/null 2>&1; then
    cargo install --list | grep -E '^rustup ' || true
  fi
  echo

  echo "=== mending missing general deps only ==="
  pkg install -y git curl clang lld make cmake ninja pkg-config openssl zlib tar xz-utils unzip patch gawk sed grep file binutils rust perl python || true
  echo

  echo "=== try installing rustup from cargo into isolated Braxon cargo home ==="
  export CARGO_HOME="$TC/install/rustup-mend/cargo"
  export RUSTUP_HOME="$TC/install/rustup-mend/rustup"
  mkdir -p "$CARGO_HOME" "$RUSTUP_HOME"
  export PATH="$CARGO_HOME/bin:$ROOT:$TC/install/python/bin:/data/data/com.termux/files/usr/opt/rust-nightly/bin:/data/data/com.termux/files/usr/bin:$PATH"

  cargo install rustup-init --locked || cargo install rustup-init || true

  echo
  echo "=== rustup-init probe ==="
  command -v rustup-init || true
  rustup-init --version || true

  echo
  echo "=== if rustup-init exists, attempt no-modify bootstrap into isolated home ==="
  if command -v rustup-init >/dev/null 2>&1; then
    rustup-init -y --no-modify-path --profile minimal --default-toolchain none || true
  fi

  echo
  echo "=== post-mend rustup probe ==="
  command -v rustup || true
  rustup --version || true
  rustup show || true

  echo
  echo "=== preserve existing proven native nightly wrappers ==="
  "$ROOT/braxon-rustc" --version --verbose
  "$ROOT/braxon-cargo" --version --verbose

  echo
  echo "=== workspace proof after mend ==="
  cd "$ROOT"
  "$ROOT/braxon-cargo" test -p nsq-core -- --nocapture
  "$ROOT/braxon-cargo" test -p Braxon-core -- --nocapture
  "$ROOT/braxon-cargo" test -p Braxon-ingest -- --nocapture

  echo
  echo "=== lock rustup mend report ==="
  LOCKDIR="$TC/locks/braxon_rustup_mend_lane"
  mkdir -p "$LOCKDIR"
  {
    echo "BRAXON_RUSTUP_MEND_LANE=1"
    date
    echo "CARGO_HOME=$CARGO_HOME"
    echo "RUSTUP_HOME=$RUSTUP_HOME"
    command -v rustup || true
    rustup --version || true
    "$ROOT/braxon-rustc" --version --verbose
    "$ROOT/braxon-cargo" --version --verbose
  } > "$LOCKDIR/LOCKED_RUSTUP_MEND_LANE.txt"

  find "$ROOT/braxon-rust-env" "$ROOT/braxon-rustc" "$ROOT/braxon-cargo" "$CARGO_HOME" "$RUSTUP_HOME" \
    -type f -print0 2>/dev/null | sort -z | xargs -0 sha256sum > "$LOCKDIR/manifest.sha256" || true

  echo "DONE"
  echo "log: $OUT"
  echo "lock: $LOCKDIR/LOCKED_RUSTUP_MEND_LANE.txt"
} 2>&1 | tee "$OUT"

ln -sf "$OUT" "$TC/mend_rustup_toolchain_lane_latest.log"
