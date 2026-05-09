#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
BINDING="$ROOT/config/toolchains/current_phone_custom_rust_nightly.json"
RUSTC="/data/data/com.termux/files/usr/opt/rust-nightly/bin/rustc"
CARGO="/data/data/com.termux/files/usr/opt/rust-nightly/bin/cargo"

cd "$ROOT"

echo "== verify custom Rust nightly binding =="
test -f "$BINDING"
grep -q '"termux_pkg_rust_replacement_allowed": false' "$BINDING"
grep -q '"held_rust_package_mutation_allowed": false' "$BINDING"
grep -q '"quality_reduction_allowed_without_boundary_report": false' "$BINDING"
grep -q '"rust_toolchain_payload_inside_braxon": false' "$BINDING"

echo "PASS: Rust nightly binding rejects package replacement and quality reduction"

echo
echo "== verify rustc/cargo paths =="
test -x "$RUSTC"
test -x "$CARGO"
"$RUSTC" --version -v
"$CARGO" --version -v

echo
echo "== verify custom nightly identity =="
"$RUSTC" --version -v | grep -q 'release: 1.96.0-nightly'
"$RUSTC" --version -v | grep -q 'host: aarch64-linux-android'
"$RUSTC" --version -v | grep -q 'LLVM version: 21.1.8'
"$RUSTC" --version | grep -q 'built from a source tarball'

echo "PASS: custom Rust nightly identity matches current-phone lane"

echo
echo "== verify PATH resolves custom nightly first =="
command -v rustc
command -v cargo

if [ "$(command -v rustc)" != "$RUSTC" ]; then
  echo "FAIL: PATH rustc is not custom nightly rustc"
  exit 1
fi

if [ "$(command -v cargo)" != "$CARGO" ]; then
  echo "FAIL: PATH cargo is not custom nightly cargo"
  exit 1
fi

echo "PASS: PATH resolves custom nightly first"

echo
echo "== write report =="
REPORT="$ROOT/state/toolchains/current_phone_custom_llvm/custom_rust_nightly_$(date +%Y%m%d_%H%M%S).txt"
{
  echo "date=$(date -Is)"
  echo "rustc=$RUSTC"
  "$RUSTC" --version -v
  echo
  echo "cargo=$CARGO"
  "$CARGO" --version -v
} | tee "$REPORT"

echo
echo "Report: $REPORT"
