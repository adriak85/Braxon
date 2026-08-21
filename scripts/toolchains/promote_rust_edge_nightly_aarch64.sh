#!/data/data/com.termux/files/usr/bin/bash
# Native Android AArch64 promotion only. This script never downloads a compiler,
# invokes an external toolchain manager, overwrites the stage0 bootstrap, or promotes an unverified build.
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
ROOT="$(cd "$ROOT" && pwd)"
CHAIN="$ROOT/state/full_android_language_toolchain"
EDGE_COMMIT="f7d782a3be46d6bb4b9792fe69a61db389ba1769"
EDGE_RELEASE="1.100.0-nightly"
TARGET="aarch64-linux-android"
ANDROID_API="${BRAXON_ANDROID_API:-24}"
JOBS="${JOBS:-1}"
# Bootstrap is a strictly limited stage0 authority. It is never resolved through
# command -v, and it cannot become the normal dispatch target after promotion.
BOOTSTRAP_RUSTC="${BRAXON_BOOTSTRAP_RUSTC:-${PREFIX:+$PREFIX/bin/rustc}}"
BOOTSTRAP_CARGO="${BRAXON_BOOTSTRAP_CARGO:-${PREFIX:+$PREFIX/bin/cargo}}"
PYTHON="${BRAXON_SOURCE_PYTHON:-$CHAIN/install/python/bin/python3}"
LLVM_INSTALL="${BRAXON_SOURCE_LLVM:-$CHAIN/install/llvm}"
BIONIC_OVERLAY="$CHAIN/install/braxon_android_overlay"
BIONIC_OVERLAY_PROOF="$CHAIN/native/android_libc_extensions/UNIFIED_ANDROID_LIBC_CONTRACTS.json"
ARCHIVE="$CHAIN/source_archives/rust-${EDGE_COMMIT}.tar.gz"
PROVENANCE="$CHAIN/source_archives/rust-${EDGE_COMMIT}.provenance.json"
SRC_ROOT="$CHAIN/src"
EDGE_SOURCE="$SRC_ROOT/rust-${EDGE_COMMIT}"
BUILD_ROOT="$CHAIN/build/rust-edge-${EDGE_COMMIT}"
INSTALL_ROOT="$CHAIN/install/rust-${EDGE_RELEASE}-${EDGE_COMMIT:0:12}"
ACTIVE_LINK="$CHAIN/install/rust-edge-active"
RUN_ID="$(date +%Y%m%d_%H%M%S)"
RUN="$CHAIN/runs/$RUN_ID-rust-edge-${EDGE_COMMIT:0:12}"
REPORT="$RUN/reports"

fail() { printf 'FAIL: %s\n' "$*" >&2; exit 1; }
notice() { printf 'rust-edge-promotion: %s\n' "$*"; }
require_file() { [ -f "$1" ] || fail "required file is absent: $1"; }
require_executable() { [ -x "$1" ] || fail "required executable is absent: $1"; }

case "$JOBS" in ''|*[!0-9]*|0) fail "JOBS must be a positive integer" ;; esac

machine="$(uname -m 2>/dev/null || true)"
is_android=0
command -v getprop >/dev/null 2>&1 && is_android=1
case "${PREFIX:-}" in *com.termux*) is_android=1 ;; esac
[ "$machine" = "aarch64" ] || fail "native AArch64 is required; observed $machine"
[ "$is_android" = "1" ] || fail "native Android Termux is required"
[ "${BRAXON_SOURCE_BUILD_APPROVED:-0}" = "1" ] || fail "set BRAXON_SOURCE_BUILD_APPROVED=1 only after reviewing capacity, source provenance, and licenses"

mkdir -p "$RUN" "$REPORT" "$BUILD_ROOT" "$INSTALL_ROOT"
exec > >(tee "$RUN/rust_edge_promotion.log") 2>&1

notice "root=$ROOT"
notice "edge_commit=$EDGE_COMMIT"
notice "edge_release=$EDGE_RELEASE"
notice "target=$TARGET"
notice "android_api=$ANDROID_API"
notice "jobs=$JOBS"

available_kib="$(df -Pk "$ROOT" | awk 'NR==2 {print $4}')"
required_kib="${BRAXON_MIN_BUILD_FREE_KIB:-33554432}"
case "$available_kib" in ''|*[!0-9]*) fail "unable to determine executable workspace free KiB" ;; esac
[ "$available_kib" -ge "$required_kib" ] || fail "insufficient executable-workspace capacity: available_kib=$available_kib required_kib=$required_kib"

for tool in tar sha256sum file readelf awk grep sed find; do command -v "$tool" >/dev/null 2>&1 || fail "required native tool is absent: $tool"; done
require_file "$ARCHIVE"
require_file "$PROVENANCE"
require_executable "$PYTHON"
require_executable "$LLVM_INSTALL/bin/llvm-config"
require_executable "$LLVM_INSTALL/bin/clang"
require_executable "$LLVM_INSTALL/bin/clang++"
require_executable "$LLVM_INSTALL/bin/llvm-ar"
require_executable "$LLVM_INSTALL/bin/llvm-readelf"
require_file "$BIONIC_OVERLAY_PROOF"
require_file "$BIONIC_OVERLAY/lib/libbraxon_android_libc_extensions.so"
grep -q '"probe_passed": true' "$BIONIC_OVERLAY_PROOF" || fail "Bionic/GNU compatibility overlay lacks a successful target probe"
[ -n "$BOOTSTRAP_RUSTC" ] || fail "set BRAXON_BOOTSTRAP_RUSTC to the preserved source-built Rust 1.97.1 bootstrap compiler"
[ -n "$BOOTSTRAP_CARGO" ] || fail "set BRAXON_BOOTSTRAP_CARGO to the preserved source-built Rust 1.97.1 bootstrap cargo"
require_executable "$BOOTSTRAP_RUSTC"
require_executable "$BOOTSTRAP_CARGO"

{
  echo "bionic_overlay=$BIONIC_OVERLAY"
  echo "bionic_overlay_proof=$BIONIC_OVERLAY_PROOF"
  cat "$BIONIC_OVERLAY_PROOF"
} | tee "$REPORT/bionic_gnu_contract_proof.txt"

bootstrap_version="$($BOOTSTRAP_RUSTC --version --verbose)"
printf '%s\n' "$bootstrap_version" | tee "$REPORT/bootstrap_rustc_version.txt"
printf '%s\n' "$bootstrap_version" | grep -q '^release: 1\.97\.1$' || fail "bootstrap rustc must remain release 1.97.1"
printf '%s\n' "$bootstrap_version" | grep -q "^host: $TARGET$" || fail "bootstrap rustc host must be $TARGET"
"$BOOTSTRAP_CARGO" --version --verbose | tee "$REPORT/bootstrap_cargo_version.txt"

expected_sha="$(awk -F '"' '/"sha256"/ {print $4; exit}' "$PROVENANCE")"
[ "$expected_sha" = "50e6078f413d40a1991b8f7ee0b19c9ec28f93bfbc5f5e7cb22575a610e56cb0" ] || fail "unexpected edge source provenance identity"
actual_sha="$(sha256sum "$ARCHIVE" | awk '{print $1}')"
[ "$actual_sha" = "$expected_sha" ] || fail "edge source archive SHA-256 mismatch"

if [ ! -f "$EDGE_SOURCE/x.py" ]; then
  notice "extracting repository-contained pinned edge source"
  rm -rf "$EDGE_SOURCE"
  extraction_tmp="$RUN/extracted-source"
  mkdir -p "$extraction_tmp"
  tar -xzf "$ARCHIVE" -C "$extraction_tmp"
  extracted="$(find "$extraction_tmp" -mindepth 1 -maxdepth 1 -type d -name "rust-${EDGE_COMMIT}" -print -quit)"
  [ -n "$extracted" ] || fail "edge archive did not contain the expected Rust source root"
  mv "$extracted" "$EDGE_SOURCE"
fi
require_file "$EDGE_SOURCE/x.py"
require_file "$EDGE_SOURCE/compiler/rustc/Cargo.toml"

cat > "$REPORT/source_identity.txt" <<EOF
schema=braxon.rust.edge_nightly.source_identity.v1
edge_commit=$EDGE_COMMIT
edge_release=$EDGE_RELEASE
archive=$ARCHIVE
archive_sha256=$actual_sha
source=$EDGE_SOURCE
bootstrap_rustc=$BOOTSTRAP_RUSTC
bootstrap_cargo=$BOOTSTRAP_CARGO
bootstrap_release=1.97.1
EOF

cat > "$EDGE_SOURCE/config.toml" <<EOF
profile = "compiler"
change-id = 0

[llvm]
download-ci-llvm = false
ninja = true
llvm-config = "$LLVM_INSTALL/bin/llvm-config"
targets = "AArch64;ARM;X86"

[build]
build = "$TARGET"
host = ["$TARGET"]
target = ["$TARGET"]
cargo = "$BOOTSTRAP_CARGO"
rustc = "$BOOTSTRAP_RUSTC"
python = "$PYTHON"
extended = true
tools = ["cargo", "rustfmt", "clippy"]
verbose = 1
build-dir = "$BUILD_ROOT"

[install]
prefix = "$INSTALL_ROOT"

[target.$TARGET]
cc = "$LLVM_INSTALL/bin/clang"
cxx = "$LLVM_INSTALL/bin/clang++"
ar = "$LLVM_INSTALL/bin/llvm-ar"
linker = "$LLVM_INSTALL/bin/clang"
EOF
cp "$EDGE_SOURCE/config.toml" "$REPORT/edge_config.toml"

cd "$EDGE_SOURCE"
"$PYTHON" ./x.py build --stage 1 library/std compiler/rustc src/tools/cargo src/tools/rustfmt src/tools/clippy | tee "$REPORT/stage1_build.txt"
"$PYTHON" ./x.py build --stage 2 library/std compiler/rustc src/tools/cargo src/tools/rustfmt src/tools/clippy | tee "$REPORT/stage2_build.txt"

STAGE1_RUSTC="$BUILD_ROOT/$TARGET/stage1/bin/rustc"
STAGE2_RUSTC="$BUILD_ROOT/$TARGET/stage2/bin/rustc"
require_executable "$STAGE1_RUSTC"
require_executable "$STAGE2_RUSTC"

"$STAGE1_RUSTC" --version --verbose | tee "$REPORT/stage1_rustc_version.txt"
"$STAGE2_RUSTC" --version --verbose | tee "$REPORT/stage2_rustc_version.txt"
for version_file in "$REPORT/stage1_rustc_version.txt" "$REPORT/stage2_rustc_version.txt"; do
  grep -q "^release: $EDGE_RELEASE$" "$version_file" || fail "expected $EDGE_RELEASE in $version_file"
  grep -q "^commit-hash: $EDGE_COMMIT$" "$version_file" || fail "expected pinned edge commit in $version_file"
  grep -q "^host: $TARGET$" "$version_file" || fail "expected $TARGET host in $version_file"
done

cat > "$RUN/edge_equivalence_probe.rs" <<'RS'
fn main() {
    let pointer_bytes = std::mem::size_of::<usize>();
    println!("BRAXON_RUST_EDGE_EQUIVALENCE_OK");
    println!("target={}", std::env::consts::ARCH);
    println!("pointer_bytes={pointer_bytes}");
}
RS

"$STAGE1_RUSTC" "$RUN/edge_equivalence_probe.rs" -C opt-level=3 -C codegen-units=1 -o "$RUN/stage1_probe"
"$STAGE2_RUSTC" "$RUN/edge_equivalence_probe.rs" -C opt-level=3 -C codegen-units=1 -o "$RUN/stage2_probe"

for binary in "$RUN/stage1_probe" "$RUN/stage2_probe"; do
  file "$binary" | tee -a "$REPORT/elf_identity.txt"
  "$LLVM_INSTALL/bin/llvm-readelf" -h "$binary" | tee -a "$REPORT/elf_identity.txt"
  "$LLVM_INSTALL/bin/llvm-readelf" -d "$binary" | tee -a "$REPORT/elf_dynamic.txt"
  file "$binary" | grep -Eiq 'ELF 64-bit.*aarch64|ELF 64-bit.*ARM aarch64' || fail "binary is not an AArch64 ELF: $binary"
  "$LLVM_INSTALL/bin/llvm-readelf" -h "$binary" | grep -q 'Machine:.*AArch64' || fail "ELF machine is not AArch64: $binary"
  "$LLVM_INSTALL/bin/llvm-readelf" -d "$binary" | grep -q 'Shared library: \[libc.so\]' || fail "Android Bionic libc dependency is absent: $binary"
done

"$RUN/stage1_probe" | tee "$REPORT/stage1_probe_output.txt"
"$RUN/stage2_probe" | tee "$REPORT/stage2_probe_output.txt"
for output in "$REPORT/stage1_probe_output.txt" "$REPORT/stage2_probe_output.txt"; do
  grep -q '^BRAXON_RUST_EDGE_EQUIVALENCE_OK$' "$output" || fail "nightly target probe did not execute: $output"
  grep -q '^target=aarch64$' "$output" || fail "nightly target probe architecture mismatch: $output"
  grep -q '^pointer_bytes=8$' "$output" || fail "nightly target probe ABI width mismatch: $output"
done
cmp "$REPORT/stage1_probe_output.txt" "$REPORT/stage2_probe_output.txt" || fail "stage1 and stage2 target behavior differs"

"$PYTHON" ./x.py install | tee "$REPORT/edge_install.txt"
require_executable "$INSTALL_ROOT/bin/rustc"
require_executable "$INSTALL_ROOT/bin/cargo"
require_executable "$INSTALL_ROOT/bin/rustdoc"
require_executable "$INSTALL_ROOT/bin/rustfmt"
require_executable "$INSTALL_ROOT/bin/clippy-driver"
"$INSTALL_ROOT/bin/rustc" --version --verbose | tee "$REPORT/promoted_rustc_version.txt"
grep -q "^release: $EDGE_RELEASE$" "$REPORT/promoted_rustc_version.txt" || fail "installed nightly release mismatch"
grep -q "^commit-hash: $EDGE_COMMIT$" "$REPORT/promoted_rustc_version.txt" || fail "installed nightly commit mismatch"
grep -q "^host: $TARGET$" "$REPORT/promoted_rustc_version.txt" || fail "installed nightly target mismatch"
"$INSTALL_ROOT/bin/cargo" --version --verbose | tee "$REPORT/promoted_cargo_version.txt"
"$INSTALL_ROOT/bin/rustdoc" --version --verbose | tee "$REPORT/promoted_rustdoc_version.txt"
"$INSTALL_ROOT/bin/rustfmt" --version | tee "$REPORT/promoted_rustfmt_version.txt"
"$INSTALL_ROOT/bin/clippy-driver" --version | tee "$REPORT/promoted_clippy_version.txt"
grep -q "$EDGE_RELEASE" "$REPORT/promoted_rustdoc_version.txt" || fail "installed rustdoc release mismatch"
grep -q "$EDGE_RELEASE" "$REPORT/promoted_rustfmt_version.txt" || fail "installed rustfmt release mismatch"
grep -q "$EDGE_RELEASE" "$REPORT/promoted_clippy_version.txt" || fail "installed Clippy release mismatch"

cat > "$RUN/edge_proc_macro.rs" <<'RS'
extern crate proc_macro;
use proc_macro::TokenStream;
#[proc_macro]
pub fn braxon_edge_identity(input: TokenStream) -> TokenStream { input }
RS
"$INSTALL_ROOT/bin/rustc" --crate-name braxon_edge_proc_macro --crate-type proc-macro "$RUN/edge_proc_macro.rs" -C opt-level=3 -o "$RUN/libbraxon_edge_proc_macro.so"
file "$RUN/libbraxon_edge_proc_macro.so" | tee "$REPORT/proc_macro_elf.txt"
"$LLVM_INSTALL/bin/llvm-readelf" -h "$RUN/libbraxon_edge_proc_macro.so" | tee -a "$REPORT/proc_macro_elf.txt"
file "$RUN/libbraxon_edge_proc_macro.so" | grep -Eiq 'ELF 64-bit.*aarch64|ELF 64-bit.*ARM aarch64' || fail "proc-macro artifact is not an AArch64 ELF"
"$LLVM_INSTALL/bin/llvm-readelf" -h "$RUN/libbraxon_edge_proc_macro.so" | grep -q 'Machine:.*AArch64' || fail "proc-macro ELF machine is not AArch64"

ln -sfn "$(basename "$INSTALL_ROOT")" "$ACTIVE_LINK"
[ "$(readlink "$ACTIVE_LINK")" = "$(basename "$INSTALL_ROOT")" ] || fail "edge active link did not resolve to verified nightly install"

cat > "$RUN/rust_edge_promotion_receipt.json" <<EOF
{
  "schema": "braxon.rust.edge_nightly_promotion_receipt.v1",
  "status": "verified_target_build_ready_for_activation",
  "edge_release": "$EDGE_RELEASE",
  "edge_commit": "$EDGE_COMMIT",
  "target": "$TARGET",
  "android_api_floor": $ANDROID_API,
  "bootstrap": {
    "release": "1.97.1",
    "rustc": "$BOOTSTRAP_RUSTC",
    "cargo": "$BOOTSTRAP_CARGO",
    "preserved": true,
    "overwritten": false
  },
  "source": {
    "archive": "$ARCHIVE",
    "sha256": "$actual_sha",
    "repository_contained": true,
    "network_used": false,
    "rustup_used": false
  },
  "verification": {
    "stage1_release_commit_host": true,
    "stage2_release_commit_host": true,
    "aarch64_elf": true,
    "bionic_libc_dependency": true,
    "bionic_gnu_contract_overlay_target_proved": true,
    "stage1_target_execution": true,
    "stage2_target_execution": true,
    "stage_equivalent_output": true,
    "cargo_verified": true,
    "rustdoc_verified": true,
    "rustfmt_verified": true,
    "clippy_verified": true,
    "proc_macro_aarch64_elf_verified": true
  },
  "active_install": "$ACTIVE_LINK",
  "receipt": "$RUN/rust_edge_promotion_receipt.json"
}
EOF
cp "$RUN/rust_edge_promotion_receipt.json" "$CHAIN/rust_edge_nightly_promotion_latest.json"
notice "PASS: Rust $EDGE_RELEASE $EDGE_COMMIT is target-built, verified, and selected at $ACTIVE_LINK; bootstrap 1.97.1 remains unchanged"
