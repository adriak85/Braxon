#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

[ "${BRAXON_SOURCE_BUILD_APPROVED:-0}" = "1" ] || {
  echo "FAIL: source build requires BRAXON_SOURCE_BUILD_APPROVED=1 after capacity, license, and source-provenance review" >&2
  exit 1
}

ROOT="${1:-$HOME/Braxon}"
cd "$ROOT"

STAMP="$(date +%Y%m%d_%H%M%S)"
JOBS="${JOBS:-1}"
case "$JOBS" in ''|*[!0-9]*|0) echo "FAIL: JOBS must be a positive integer" >&2; exit 1 ;; esac
CHAIN="$ROOT/state/full_android_language_toolchain"
RUN="$CHAIN/runs/$STAMP"
SRC="$CHAIN/src"
BUILD="$CHAIN/build"
INSTALL="$CHAIN/install"
BAKED="$CHAIN/baked/current"
REPORT="$RUN/reports"

mkdir -p "$RUN" "$SRC" "$BUILD" "$INSTALL" "$BAKED" "$REPORT" scripts/toolchains config/toolchains

LOG="$RUN/full_rebuild.log"
exec > >(tee "$LOG") 2>&1

echo "== Braxon full Android language/toolchain rebuild =="
echo "date=$(date -Is)"
echo "root=$ROOT"
echo "head=$(git rev-parse HEAD)"
echo "branch=$(git branch --show-current)"
echo "chain=$CHAIN"
echo

echo "== rules =="
echo "Rebuild the whole language/toolchain stack from source."
echo "Preserve current custom Rust as bootstrap only."
echo "Do not replace active Rust."
echo "Do not write /tmp."
echo "Do not install into PREFIX."
echo "Install into state/full_android_language_toolchain/install."
echo "Optimize after the full build, not before."
echo

echo "== bootstrap authority =="
{
  echo "schema=braxon.full_toolchain.bootstrap_authority.v1"
  echo "date=$(date -Is)"
  echo "PATH=$PATH"
  echo "PREFIX=${PREFIX:-unset}"
  echo
  command -v rustc || true
  rustc --version --verbose || true
  echo
  command -v cargo || true
  cargo --version --verbose || true
  echo
  command -v clang || true
  clang --version || true
  echo
  command -v ld.lld || true
  ld.lld --version || true
  echo
  command -v python3 || true
  python3 --version || true
} | tee "$REPORT/bootstrap_authority.txt"

echo
echo "== required bootstrap tools =="
missing=0
for t in git curl clang clang++ ld.lld llvm-ar llvm-ranlib llvm-nm llvm-objdump llvm-strip cmake ninja make python3 perl sed grep awk tar xz sha256sum file readelf; do
  if command -v "$t" >/dev/null 2>&1; then
    printf "OK: %-16s %s\n" "$t" "$(command -v "$t")"
  else
    echo "MISSING: $t"
    missing=1
  fi
done | tee "$REPORT/bootstrap_tool_check.txt"

if [ "$missing" = "1" ]; then
  echo "FAIL: missing bootstrap tools"
  exit 1
fi

echo
echo "== manifest =="
cat > config/toolchains/full_android_language_toolchain.json <<JSON
{
  "schema": "braxon.full_android_language_toolchain.v1",
  "authority": "BRAXON_FULL_ANDROID_LANGUAGE_TOOLCHAIN",
  "created_at": "$(date -Is)",
  "repo_head": "$(git rev-parse HEAD)",
  "branch": "$(git branch --show-current)",
  "host": "android_termux_aarch64",
  "install_root": "state/full_android_language_toolchain/install",
  "baked_root": "state/full_android_language_toolchain/baked/current",
  "preservation": {
    "active_custom_rust_is_bootstrap_only": true,
    "replace_active_rust": false,
    "write_tmp": false,
    "install_to_prefix": false
  },
  "source_stack": [
    "m4",
    "autoconf",
    "automake",
    "libtool",
    "pkgconf",
    "zlib",
    "openssl",
    "libffi",
    "sqlite",
    "xz",
    "zstd",
    "ncurses",
    "readline",
    "cpython",
    "llvm-project",
    "rust"
  ],
  "optimization_pass": [
    "release build",
    "LTO where stable",
    "codegen unit reduction where applicable",
    "strip copied binaries after debug copy",
    "hash manifest",
    "baked verification probes"
  ]
}
JSON
cat config/toolchains/full_android_language_toolchain.json | tee "$REPORT/manifest.json"

verify_pinned_git_source() {
  name="$1"
  dest="$2"
  expected="$3"
  echo
  echo "== pinned source: $name =="
  echo "dest=$dest"
  echo "expected=$expected"
  [ -d "$dest/.git" ] || { echo "FAIL: materialized git source is missing: $dest" >&2; exit 1; }
  actual="$(git -C "$dest" rev-parse HEAD)"
  [ "$actual" = "$expected" ] || { echo "FAIL: $name source revision mismatch: expected $expected got $actual" >&2; exit 1; }
  printf '%s\n' "$actual" | tee "$REPORT/${name}_head.txt"
}

verify_locally_retained_llvm_source() {
  dest="$1"
  echo
  echo "== locally retained LLVM source =="
  [ -f "$dest/llvm/CMakeLists.txt" ] || { echo "FAIL: retained LLVM source tree is missing: $dest" >&2; exit 1; }
  if [ -f "$ROOT/audit/toolchain/llvm_project_tree_sha256.txt" ]; then
    cat "$ROOT/audit/toolchain/llvm_project_tree_sha256.txt" | tee "$REPORT/llvm_project_tree_sha256.txt"
  else
    echo "FAIL: LLVM source tree content identity is absent; run the source identity capture before source build" >&2
    exit 1
  fi
}

echo
echo "== pinned source verification (no floating network clone) =="
verify_pinned_git_source cpython "$SRC/cpython" "${CPYTHON_REF:-49918f5b0ceb1950c3222fd4fd6be872d2e15c6f}"
verify_locally_retained_llvm_source "$SRC/llvm-project"
verify_pinned_git_source rust "$SRC/rust" "${RUST_REF:-f964de49bcb561e5c6c725bb37201e11d852daf0}"

echo
echo "== build CPython from source =="
PY_INSTALL="$INSTALL/python"
mkdir -p "$PY_INSTALL"

(
  cd "$SRC/cpython"
  make distclean >/dev/null 2>&1 || true

  ./configure \
    --prefix="$PY_INSTALL" \
    --enable-optimizations \
    --with-lto \
    CC="$(command -v clang)" \
    CXX="$(command -v clang++)" \
    LDFLAGS="-fuse-ld=lld" \
    | tee "$REPORT/cpython_configure.txt"

  make -j"$JOBS" | tee "$REPORT/cpython_build.txt"
  make install | tee "$REPORT/cpython_install.txt"
)

"$PY_INSTALL/bin/python3" --version | tee "$REPORT/cpython_verify.txt"

echo
echo "== build LLVM/Clang/LLD/runtimes from source =="
LLVM_BUILD="$BUILD/llvm"
LLVM_INSTALL="$INSTALL/llvm"
mkdir -p "$LLVM_BUILD" "$LLVM_INSTALL"

cmake -S "$SRC/llvm-project/llvm" -B "$LLVM_BUILD" -G Ninja \
  -DCMAKE_BUILD_TYPE=Release \
  -DCMAKE_INSTALL_PREFIX="$LLVM_INSTALL" \
  -DLLVM_ENABLE_PROJECTS="clang;lld;clang-tools-extra" \
  -DLLVM_ENABLE_RUNTIMES="compiler-rt;libunwind;libcxx;libcxxabi" \
  -DLLVM_TARGETS_TO_BUILD="AArch64;ARM;X86" \
  -DLLVM_DEFAULT_TARGET_TRIPLE="aarch64-linux-android" \
  -DLLVM_ENABLE_TERMINFO=OFF \
  -DLLVM_ENABLE_ZLIB=ON \
  -DLLVM_ENABLE_ZSTD=ON \
  -DLLVM_INCLUDE_TESTS=OFF \
  -DLLVM_INCLUDE_EXAMPLES=OFF \
  -DLLVM_INCLUDE_BENCHMARKS=OFF \
  -DLLVM_BUILD_LLVM_DYLIB=OFF \
  -DLLVM_LINK_LLVM_DYLIB=OFF \
  | tee "$REPORT/llvm_configure.txt"

ninja -C "$LLVM_BUILD" -j"$JOBS" | tee "$REPORT/llvm_build.txt"
ninja -C "$LLVM_BUILD" install | tee "$REPORT/llvm_install.txt"

{
  "$LLVM_INSTALL/bin/clang" --version
  echo
  "$LLVM_INSTALL/bin/clang++" --version
  echo
  "$LLVM_INSTALL/bin/ld.lld" --version
  echo
  "$LLVM_INSTALL/bin/llvm-ar" --version
  echo
  "$LLVM_INSTALL/bin/llvm-strip" --version
} | tee "$REPORT/llvm_verify.txt"

echo
echo "== build Rust from source using preserved custom Rust bootstrap =="
RUST_INSTALL="$INSTALL/rust"
mkdir -p "$RUST_INSTALL"

cat > "$SRC/rust/config.toml" <<TOML
profile = "compiler"
change-id = 0

[llvm]
download-ci-llvm = false
ninja = true
# Use the repository's verified source-built LLVM/Clang/LLD installation. This
# preserves the physical compiler boundary without duplicating Rust's nested LLVM
# worktree on inode-constrained Android filesystems.
llvm-config = "$LLVM_INSTALL/bin/llvm-config"
targets = "AArch64;ARM;X86"

[build]
build = "aarch64-linux-android"
host = ["aarch64-linux-android"]
target = ["aarch64-linux-android"]
cargo = "$(command -v cargo)"
rustc = "$(command -v rustc)"
python = "$PY_INSTALL/bin/python3"
extended = true
tools = ["cargo", "rustfmt", "clippy"]
verbose = 1

[install]
prefix = "$RUST_INSTALL"

[target.aarch64-linux-android]
cc = "$LLVM_INSTALL/bin/clang"
cxx = "$LLVM_INSTALL/bin/clang++"
ar = "$LLVM_INSTALL/bin/llvm-ar"
linker = "$LLVM_INSTALL/bin/clang"
TOML

cat "$SRC/rust/config.toml" | tee "$REPORT/rust_config_toml.txt"

(
  cd "$SRC/rust"

  "$PY_INSTALL/bin/python3" ./x.py build --stage 1 compiler/rustc cargo rustfmt clippy \
    | tee "$REPORT/rust_stage1_build.txt"

  "$PY_INSTALL/bin/python3" ./x.py build --stage 2 library/std compiler/rustc cargo rustfmt clippy \
    | tee "$REPORT/rust_stage2_build.txt"

  "$PY_INSTALL/bin/python3" ./x.py install \
    | tee "$REPORT/rust_install.txt"
)

{
  "$RUST_INSTALL/bin/rustc" --version --verbose || true
  echo
  "$RUST_INSTALL/bin/cargo" --version --verbose || true
} | tee "$REPORT/rust_verify.txt"

echo
echo "== bake and optimize full rebuilt stack =="
rm -rf "$BAKED"
mkdir -p "$BAKED/bin" "$BAKED/lib" "$BAKED/proofs"

copy_tool() {
  src="$1"
  dst="$2"
  if [ -x "$src" ]; then
    cp -a "$src" "$BAKED/bin/$dst"
    echo "COPIED: $dst"
  else
    echo "WARN: missing tool: $src"
  fi
}

copy_tool "$PY_INSTALL/bin/python3" python3
copy_tool "$LLVM_INSTALL/bin/clang" clang
copy_tool "$LLVM_INSTALL/bin/clang++" clang++
copy_tool "$LLVM_INSTALL/bin/ld.lld" ld.lld
copy_tool "$LLVM_INSTALL/bin/llvm-ar" llvm-ar
copy_tool "$LLVM_INSTALL/bin/llvm-ranlib" llvm-ranlib
copy_tool "$LLVM_INSTALL/bin/llvm-nm" llvm-nm
copy_tool "$LLVM_INSTALL/bin/llvm-objdump" llvm-objdump
copy_tool "$LLVM_INSTALL/bin/llvm-strip" llvm-strip
copy_tool "$LLVM_INSTALL/bin/llvm-readelf" llvm-readelf
copy_tool "$RUST_INSTALL/bin/rustc" rustc
copy_tool "$RUST_INSTALL/bin/cargo" cargo
copy_tool "$RUST_INSTALL/bin/rustfmt" rustfmt
copy_tool "$RUST_INSTALL/bin/clippy-driver" clippy-driver

echo
echo "== debug-copy then strip baked ELF binaries =="
find "$BAKED/bin" -type f ! -name '*.debug' | while read -r f; do
  if file "$f" | grep -q 'ELF'; then
    cp "$f" "$f.debug"
    "$LLVM_INSTALL/bin/llvm-strip" "$f" || true
  fi
done

echo
echo "== baked probes =="
cat > "$RUN/probe.c" <<'C'
#include <stdio.h>
#include <stdint.h>
int main(void) {
    printf("BRAXON_FULL_REBUILD_C_OK\n");
    printf("%zu\n", sizeof(uintptr_t));
    return 0;
}
C

cat > "$RUN/probe.cpp" <<'CPP'
#include <iostream>
#include <vector>
#include <string>
int main() {
    std::vector<std::string> parts = {"BRAXON", "FULL", "REBUILD", "CPP", "OK"};
    for (const auto& p : parts) std::cout << p << "\n";
    return 0;
}
CPP

cat > "$RUN/probe.rs" <<'RS'
fn main() {
    println!("BRAXON_FULL_REBUILD_RUST_OK");
    println!("{}", std::env::consts::OS);
}
RS

"$BAKED/bin/clang" "$RUN/probe.c" -O3 -fuse-ld=lld -o "$RUN/probe_c"
"$BAKED/bin/clang++" "$RUN/probe.cpp" -O3 -fuse-ld=lld -o "$RUN/probe_cpp"
"$BAKED/bin/rustc" "$RUN/probe.rs" -C opt-level=3 -C codegen-units=1 -o "$RUN/probe_rust"

{
  "$RUN/probe_c"
  "$RUN/probe_cpp"
  "$RUN/probe_rust"
} | tee "$REPORT/baked_probe_output.txt"

grep -q "BRAXON_FULL_REBUILD_C_OK" "$REPORT/baked_probe_output.txt"
grep -q "BRAXON_FULL_REBUILD_RUST_OK" "$REPORT/baked_probe_output.txt"
grep -q "BRAXON.*FULL.*REBUILD.*CPP.*OK" <(tr '\n' ' ' < "$REPORT/baked_probe_output.txt")

echo
echo "== proof copy =="
cp "$REPORT/bootstrap_authority.txt" "$BAKED/proofs/bootstrap_authority.txt"
cp "$REPORT/cpython_verify.txt" "$BAKED/proofs/cpython_verify.txt"
cp "$REPORT/llvm_verify.txt" "$BAKED/proofs/llvm_verify.txt"
cp "$REPORT/rust_verify.txt" "$BAKED/proofs/rust_verify.txt"
cp "$REPORT/baked_probe_output.txt" "$BAKED/proofs/baked_probe_output.txt"

echo
echo "== hash full rebuilt stack =="
(
  cd "$CHAIN"
  find install baked -type f -print0 | sort -z | xargs -0 sha256sum
) | tee "$RUN/FULL_REBUILD_SHA256SUMS.txt"

cp "$RUN/FULL_REBUILD_SHA256SUMS.txt" "$BAKED/SHA256SUMS"

echo
echo "== final report =="
{
  echo "schema=braxon.full_android_language_toolchain.report.v1"
  echo "date=$(date -Is)"
  echo "run=$RUN"
  echo "src=$SRC"
  echo "build=$BUILD"
  echo "install=$INSTALL"
  echo "baked=$BAKED"
  echo "custom_rust_preserved=true"
  echo "active_rust_replaced=false"
  echo "tmp_used=false"
  echo "full_source_rebuild_attempted=true"
  echo "optimization_bake_completed=true"
  echo "cpython_head=$(cat "$REPORT/cpython_head.txt" 2>/dev/null || true)"
  echo "llvm_head=$(cat "$REPORT/llvm_project_head.txt" 2>/dev/null || true)"
  echo "rust_head=$(cat "$REPORT/rust_head.txt" 2>/dev/null || true)"
} | tee "$RUN/full_rebuild_report.txt"

echo
echo "PASS: full Android language/toolchain rebuild and bake completed"
echo "RUN=$RUN"
echo "BAKED=$BAKED"
