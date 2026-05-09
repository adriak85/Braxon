#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
PY="$ROOT/braxon-python"
RUSTHOME="$TC/install/rustup-nightly3"
CARGO_HOME="$RUSTHOME/cargo"
RUSTUP_HOME="$RUSTHOME/rustup"
LOCKDIR="$TC/locks/braxon_rust_nightly3_candidate"
LOG="$TC/unlock_braxon_rustup_nightly3_$(date +%Y%m%d_%H%M%S).log"

mkdir -p "$TC" "$CARGO_HOME" "$RUSTUP_HOME" "$LOCKDIR"

export PATH="/data/data/com.termux/files/usr/bin:$CARGO_HOME/bin:$PATH"
export CARGO_HOME
export RUSTUP_HOME
export LD_LIBRARY_PATH="$TC/install/braxon_android_overlay/lib:/data/data/com.termux/files/usr/lib:${LD_LIBRARY_PATH:-}"

{
  echo "=== Braxon Rustup Nightly 3 unlock ==="
  date
  echo "ROOT=$ROOT"
  echo "TC=$TC"
  echo "PY=$PY"
  echo "CARGO_HOME=$CARGO_HOME"
  echo "RUSTUP_HOME=$RUSTUP_HOME"
  echo

  echo "=== install host deps ==="
  pkg install -y git curl clang lld make cmake ninja pkg-config openssl zlib python rust rustup binutils

  echo
  echo "=== prove Braxon Python host ==="
  "$PY" - <<'PY'
import sys, math, _math_integer, cmath, decimal, _decimal, tkinter, _tkinter
print("braxon-python:", sys.version)
print("math:", math.__file__)
print("_math_integer:", _math_integer.__file__)
print("cmath:", cmath.__file__)
print("_decimal:", _decimal.__file__)
print("_tkinter:", _tkinter.__file__)
print("Braxon Python host OK")
PY

  echo
  echo "=== current Rust surface before unlock ==="
  command -v rustc || true
  rustc --version --verbose || true
  command -v cargo || true
  cargo --version --verbose || true
  command -v rustup || true
  rustup --version || true

  echo
  echo "=== rustup env ==="
  echo "CARGO_HOME=$CARGO_HOME"
  echo "RUSTUP_HOME=$RUSTUP_HOME"

  echo
  echo "=== try rustup nightly install ==="
  rustup toolchain install nightly --profile complete || rustup toolchain install nightly --profile default

  echo
  echo "=== set nightly default inside Braxon rustup home ==="
  rustup default nightly

  echo
  echo "=== add important targets if available ==="
  rustup target add aarch64-linux-android || true
  rustup target add aarch64-unknown-linux-gnu || true

  echo
  echo "=== verify unlocked nightly ==="
  rustup show
  rustc --version --verbose
  cargo --version --verbose
  rustfmt --version || true
  clippy-driver --version || true

  echo
  echo "=== create Braxon Rust wrappers ==="
  cat > "$ROOT/braxon-rust-env" <<EOF
export CARGO_HOME="$CARGO_HOME"
export RUSTUP_HOME="$RUSTUP_HOME"
export PATH="$CARGO_HOME/bin:/data/data/com.termux/files/usr/bin:\$PATH"
export LD_LIBRARY_PATH="$TC/install/braxon_android_overlay/lib:/data/data/com.termux/files/usr/lib:\${LD_LIBRARY_PATH:-}"
EOF

  cat > "$ROOT/braxon-rustc" <<EOF
#!/data/data/com.termux/files/usr/bin/bash
source "$ROOT/braxon-rust-env"
exec rustc "\$@"
EOF

  cat > "$ROOT/braxon-cargo" <<EOF
#!/data/data/com.termux/files/usr/bin/bash
source "$ROOT/braxon-rust-env"
exec cargo "\$@"
EOF

  chmod +x "$ROOT/braxon-rust-env" "$ROOT/braxon-rustc" "$ROOT/braxon-cargo"

  echo
  echo "=== wrapper verification ==="
  "$ROOT/braxon-rustc" --version --verbose
  "$ROOT/braxon-cargo" --version --verbose

  echo
  echo "=== Braxon workspace smoke if Cargo.toml exists ==="
  cd "$ROOT"
  if [ -f Cargo.toml ]; then
    "$ROOT/braxon-cargo" fmt --all --check || true
    "$ROOT/braxon-cargo" test -p nsq-core -- --nocapture || true
    "$ROOT/braxon-cargo" test -p nsq-runtime -- --nocapture || true
    "$ROOT/braxon-cargo" test -p braxon-core -- --nocapture || true
    "$ROOT/braxon-cargo" test -p braxon-ingest -- --nocapture || true
  else
    echo "No Cargo.toml at $ROOT; skipped workspace tests."
  fi

  echo
  echo "=== lock candidate manifest ==="
  {
    echo "Braxon Rust Nightly 3 Candidate"
    date
    echo "CARGO_HOME=$CARGO_HOME"
    echo "RUSTUP_HOME=$RUSTUP_HOME"
    "$ROOT/braxon-rustc" --version --verbose
    "$ROOT/braxon-cargo" --version --verbose
  } > "$LOCKDIR/LOCKED_RUST_NIGHTLY3_CANDIDATE.txt"

  find "$RUSTHOME" "$ROOT/braxon-rust-env" "$ROOT/braxon-rustc" "$ROOT/braxon-cargo" \
    -type f -exec sha256sum {} + > "$LOCKDIR/manifest.sha256"

  echo
  echo "LOCKED:"
  echo "$LOCKDIR"
  echo "$LOCKDIR/LOCKED_RUST_NIGHTLY3_CANDIDATE.txt"
  echo "$LOCKDIR/manifest.sha256"
  echo "$ROOT/braxon-rustc"
  echo "$ROOT/braxon-cargo"
} 2>&1 | tee "$LOG"

ln -sf "$LOG" "$TC/unlock_braxon_rustup_nightly3_latest.log"
echo "log: $LOG"
