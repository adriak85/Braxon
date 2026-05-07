#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
OUT_DIR="$ROOT/state/toolchains/current_phone_custom_llvm"
STAMP="$(date +%Y%m%d_%H%M%S)"
REPORT="$OUT_DIR/custom_rust_nightly_depth_feature_audit_$STAMP.txt"

RUSTC="/data/data/com.termux/files/usr/opt/rust-nightly/bin/rustc"
CARGO="/data/data/com.termux/files/usr/opt/rust-nightly/bin/cargo"
RUST_ROOT="/data/data/com.termux/files/usr/opt/rust-nightly"

mkdir -p "$OUT_DIR"
cd "$ROOT"

{
  echo "== Braxon custom Rust nightly depth / feature audit =="
  echo "date=$(date -Is)"
  echo "root=$ROOT"
  echo "rust_root=$RUST_ROOT"
  echo

  echo "== active toolchain paths =="
  echo "command_rustc=$(command -v rustc || true)"
  echo "command_cargo=$(command -v cargo || true)"
  echo "expected_rustc=$RUSTC"
  echo "expected_cargo=$CARGO"
  echo

  echo "== rustc identity =="
  "$RUSTC" --version || true
  "$RUSTC" --version -v || true
  echo

  echo "== cargo identity =="
  "$CARGO" --version || true
  "$CARGO" --version -v || true
  echo

  echo "== PATH verification =="
  if [ "$(command -v rustc)" = "$RUSTC" ]; then
    echo "PASS: PATH rustc resolves to custom nightly"
  else
    echo "FAIL: PATH rustc does not resolve to custom nightly"
  fi

  if [ "$(command -v cargo)" = "$CARGO" ]; then
    echo "PASS: PATH cargo resolves to custom nightly"
  else
    echo "FAIL: PATH cargo does not resolve to custom nightly"
  fi
  echo

  echo "== source-tarball marker =="
  if "$RUSTC" --version | grep -q 'built from a source tarball'; then
    echo "PASS: rustc reports built from a source tarball"
  else
    echo "WARN: rustc did not report source-tarball build in plain --version"
  fi
  echo

  echo "== search local rust build-stage evidence =="
  find "$HOME" \
    -path "$HOME/Braxon/target" -prune -o \
    -path "$HOME/.cargo/registry" -prune -o \
    -path "$HOME/.cargo/git" -prune -o \
    -type f \
    \( -name 'config.toml' -o -name 'config.example.toml' -o -name 'bootstrap.toml' -o -name 'build-manifest.toml' -o -name '*stage*' -o -name '*rust*build*.log' -o -name '*tool_versions*' -o -name '*resume*' \) \
    -print 2>/dev/null | sort | sed -n '1,300p'
  echo

  echo "== grep stage markers near custom toolchains and rust root =="
  grep -RInE 'stage[[:space:]]*=?[[:space:]]*3|stage3|full-tools|extended[[:space:]]*=?[[:space:]]*true|tools[[:space:]]*=|profiler|llvm-tools|rust-src|rustc-dev|clippy|rustfmt|miri|rust-analyzer' \
    "$HOME/custom_toolchains" "$RUST_ROOT" "$HOME/.rustup" "$HOME" 2>/dev/null \
    | grep -v '/target/' \
    | grep -v '/.cargo/registry/' \
    | grep -v '/.cargo/git/' \
    | sed -n '1,300p' || true
  echo

  echo "== installed rust component-like binaries =="
  for b in rustc cargo rustdoc rustfmt clippy-driver rust-lldb rust-gdb llvm-config clang lld ld.lld ar llvm-ar llvm-nm llvm-objdump llvm-readelf; do
    printf '%-20s ' "$b"
    command -v "$b" || true
  done
  echo

  echo "== repo nightly feature-gate scan =="
  grep -RIn --exclude-dir=.git --exclude-dir=target --exclude='*.before_*' --exclude='*.bak*' '#!\[feature(' "$ROOT" 2>/dev/null || true
  echo

  echo "== repo toolchain binding files =="
  find "$ROOT/config/toolchains" "$ROOT/scripts/toolchains" "$ROOT/state/toolchains" -maxdepth 5 -type f 2>/dev/null | sort
  echo

  echo "== package hold status for rust =="
  apt-mark showhold 2>/dev/null | grep '^rust$' && echo "PASS: rust package is held" || echo "WARN: rust package is not shown as held"
  echo

  echo "== glibc exposure check =="
  pkg list-installed 2>/dev/null | grep -Ei 'glibc|libc' || true
  echo
  echo "NOTE: Presence of a Termux glibc repo/package is not by itself proof Braxon uses glibc. This only reports exposure."

} | tee "$REPORT"

echo
echo "Report written: $REPORT"
