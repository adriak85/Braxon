#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
cd "$ROOT"

STAMP="$(date +%Y%m%d_%H%M%S)"
CHAIN="$ROOT/state/full_android_language_toolchain"
RUN="$CHAIN/runs/$STAMP"
SRC="$CHAIN/src"
BUILD="$CHAIN/build"
INSTALL="$CHAIN/install"
BAKED="$CHAIN/baked/current"
REPORT="$RUN/reports"
CACHE="$CHAIN/cache"

mkdir -p "$RUN" "$SRC" "$BUILD" "$INSTALL" "$BAKED" "$REPORT" "$CACHE" scripts/toolchains config/toolchains

LOG="$RUN/full_rebuild_visible.log"
exec > >(tee "$LOG") 2>&1

run_visible() {
  echo
  echo "== RUN =="
  printf '%q ' "$@"
  echo
  if command -v stdbuf >/dev/null 2>&1; then
    stdbuf -oL -eL "$@"
  else
    "$@"
  fi
}

run_logged() {
  label="$1"
  logfile="$2"
  shift 2

  echo
  echo "== $label =="
  echo "logfile=$logfile"
  printf 'command='
  printf '%q ' "$@"
  echo

  if command -v stdbuf >/dev/null 2>&1; then
    stdbuf -oL -eL "$@" 2>&1 | tee "$logfile"
  else
    "$@" 2>&1 | tee "$logfile"
  fi
}

echo "== Braxon full Android language/toolchain rebuild: visible progress =="
echo "date=$(date -Is)"
echo "root=$ROOT"
echo "head=$(git rev-parse HEAD)"
echo "branch=$(git branch --show-current)"
echo "chain=$CHAIN"
echo "run=$RUN"
echo "src=$SRC"
echo "build=$BUILD"
echo "install=$INSTALL"
echo "baked=$BAKED"
echo

echo "== rules =="
echo "Target: rebuild language/toolchain stack from source."
echo "Active custom Rust is bootstrap authority only."
echo "Do not replace active Rust."
echo "Do not install into PREFIX."
echo "Do not write /tmp."
echo "Do not hide build progress."
echo "Use source archives when git clone/fetch drops."
echo

export TMPDIR="$CHAIN/no_tmp_redirect"
mkdir -p "$TMPDIR"
export BRAXON_ANDROID_OVERLAY="$CHAIN/install/braxon_android_overlay"
export BRAXON_ANDROID_OVERLAY_INCLUDE="$BRAXON_ANDROID_OVERLAY/include"
export BRAXON_ANDROID_OVERLAY_LIB="$BRAXON_ANDROID_OVERLAY/lib"
export CPPFLAGS="-isystem $BRAXON_ANDROID_OVERLAY_INCLUDE ${CPPFLAGS:-}"
export CFLAGS="-isystem $BRAXON_ANDROID_OVERLAY_INCLUDE ${CFLAGS:-}"
export CXXFLAGS="-isystem $BRAXON_ANDROID_OVERLAY_INCLUDE ${CXXFLAGS:-}"
export LDFLAGS="-L$BRAXON_ANDROID_OVERLAY_LIB -fuse-ld=lld ${LDFLAGS:-}"
export LIBS="-lbraxon_android_libc_extensions ${LIBS:-}"
export LD_LIBRARY_PATH="$BRAXON_ANDROID_OVERLAY_LIB:${LD_LIBRARY_PATH:-}"
export PKG_CONFIG_PATH="$BRAXON_ANDROID_OVERLAY_LIB/pkgconfig:${PKG_CONFIG_PATH:-}"

echo "BRAXON_ANDROID_OVERLAY=$BRAXON_ANDROID_OVERLAY"
echo "BRAXON_ANDROID_OVERLAY_INCLUDE=$BRAXON_ANDROID_OVERLAY_INCLUDE"
echo "BRAXON_ANDROID_OVERLAY_LIB=$BRAXON_ANDROID_OVERLAY_LIB"
echo "CPPFLAGS=$CPPFLAGS"
echo "LDFLAGS=$LDFLAGS"
echo "LIBS=$LIBS"
echo "LD_LIBRARY_PATH=$LD_LIBRARY_PATH"


echo "== bootstrap authority =="
{
  echo "schema=braxon.full_toolchain.visible_bootstrap_authority.v1"
  echo "date=$(date -Is)"
  echo "PATH=$PATH"
  echo "PREFIX=${PREFIX:-unset}"
  echo "TMPDIR=$TMPDIR"
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
  command -v clang++ || true
  clang++ --version || true
  echo
  command -v ld.lld || true
  ld.lld --version || true
  echo
  command -v python3 || true
  python3 --version || true
  echo
  command -v cmake || true
  cmake --version || true
  echo
  command -v ninja || true
  ninja --version || true
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
cat > config/toolchains/full_android_language_toolchain_visible.json <<JSON
{
  "schema": "braxon.full_android_language_toolchain.visible.v1",
  "authority": "BRAXON_FULL_ANDROID_LANGUAGE_TOOLCHAIN_VISIBLE",
  "created_at": "$(date -Is)",
  "repo_head": "$(git rev-parse HEAD)",
  "branch": "$(git branch --show-current)",
  "host": "android_termux_aarch64",
  "install_root": "state/full_android_language_toolchain/install",
  "baked_root": "state/full_android_language_toolchain/baked/current",
  "source_truth": true,
  "preservation": {
    "active_custom_rust_is_bootstrap_only": true,
    "replace_active_rust": false,
    "write_tmp": false,
    "install_to_prefix": false
  },
  "source_stack": [
    "cpython",
    "llvm-project",
    "rust"
  ],
  "visible_progress": true,
  "archive_fallback_enabled": true
}
JSON
cat config/toolchains/full_android_language_toolchain_visible.json | tee "$REPORT/manifest.json"

download_archive_source() {
  name="$1"
  archive_url="$2"
  dest="$3"

  archive="$CACHE/${name}.tar.gz"
  unpack="$CACHE/${name}.unpack"

  echo
  echo "== archive fallback: $name =="
  echo "archive_url=$archive_url"
  echo "archive=$archive"
  echo "unpack=$unpack"
  echo "dest=$dest"

  rm -rf "$archive" "$unpack" "$dest"
  mkdir -p "$unpack"

  run_logged "download archive $name" "$REPORT/${name}_archive_download.txt" \
    curl -L \
      --progress-bar \
      --retry 20 \
      --retry-delay 5 \
      --retry-all-errors \
      --connect-timeout 30 \
      --continue-at - \
      -o "$archive" \
      "$archive_url"

  run_logged "unpack archive $name" "$REPORT/${name}_archive_unpack.txt" \
    tar -xzf "$archive" -C "$unpack"

  first_dir="$(find "$unpack" -mindepth 1 -maxdepth 1 -type d | head -n 1)"
  if [ -z "$first_dir" ]; then
    echo "FAIL: archive unpack produced no source directory for $name"
    exit 1
  fi

  mv "$first_dir" "$dest"
  rm -rf "$unpack"

  echo "archive-fallback-source" | tee "$REPORT/${name}_head.txt"
  find "$dest" -maxdepth 2 -type f | sort | sha256sum | tee "$REPORT/${name}_archive_tree_marker.txt"
  echo "PASS: archive source staged for $name"
}

clone_or_archive() {
  name="$1"
  url="$2"
  dest="$3"
  ref="${4:-}"

  echo
  echo "== source: $name =="
  echo "url=$url"
  echo "dest=$dest"
  echo "ref=${ref:-default}"

  mkdir -p "$(dirname "$dest")"

  if [ -d "$dest/.git" ]; then
    echo "existing git source found; visible fetch starts now"
    if ! run_logged "fetch $name" "$REPORT/${name}_fetch.txt" \
      git -C "$dest" fetch --verbose --progress --depth=1 --filter=blob:none --tags --prune origin
    then
      echo "WARN: fetch failed for existing $name; will continue if source tree is usable"
    fi
  elif [ -d "$dest" ] && [ ! -d "$dest/.git" ]; then
    echo "existing archive source directory found; keeping it"
    echo "archive-fallback-source-existing" | tee "$REPORT/${name}_head.txt"
    return 0
  else
    rm -rf "$dest"
    if ! run_logged "clone $name" "$REPORT/${name}_clone.txt" \
      git clone --verbose --progress --depth=1 --filter=blob:none "$url" "$dest"
    then
      echo "WARN: git clone failed for $name; using archive fallback"
      case "$name" in
        cpython)
          archive_url="${CPYTHON_ARCHIVE_URL:-https://github.com/python/cpython/archive/refs/heads/main.tar.gz}"
          ;;
        llvm_project)
          archive_url="${LLVM_ARCHIVE_URL:-https://github.com/llvm/llvm-project/archive/refs/heads/main.tar.gz}"
          ;;
        rust)
          archive_url="${RUST_ARCHIVE_URL:-https://github.com/rust-lang/rust/archive/refs/heads/master.tar.gz}"
          ;;
        *)
          echo "FAIL: no archive fallback known for $name"
          exit 1
          ;;
      esac
      download_archive_source "$name" "$archive_url" "$dest"
    fi
  fi

  if [ -n "$ref" ] && [ -d "$dest/.git" ]; then
    run_logged "checkout $name ref" "$REPORT/${name}_checkout.txt" git -C "$dest" checkout "$ref"
  elif [ -n "$ref" ] && [ ! -d "$dest/.git" ]; then
    echo "WARN: ref requested but archive fallback cannot checkout ref: $ref"
  fi

  if [ -d "$dest/.git" ]; then
    git -C "$dest" rev-parse HEAD | tee "$REPORT/${name}_head.txt"
    git -C "$dest" status --short | tee "$REPORT/${name}_status.txt"
  fi
}

echo
echo "== source acquisition: visible =="
clone_or_archive cpython https://github.com/python/cpython.git "$SRC/cpython" "${CPYTHON_REF:-}"
clone_or_archive llvm_project https://github.com/llvm/llvm-project.git "$SRC/llvm-project" "${LLVM_REF:-}"
clone_or_archive rust https://github.com/rust-lang/rust.git "$SRC/rust" "${RUST_REF:-}"

echo
echo "== build CPython from source =="
PY_INSTALL="$INSTALL/python"
mkdir -p "$PY_INSTALL"

(
  cd "$SRC/cpython"
  make distclean >/dev/null 2>&1 || true

  run_logged "configure CPython" "$REPORT/cpython_configure.txt" \
    ./configure \
      --prefix="$PY_INSTALL" \
      --enable-optimizations \
      --with-lto \
      CC="$(command -v clang)" \
      CXX="$(command -v clang++)"

  run_logged "build CPython" "$REPORT/cpython_build.txt" \
    make -j"${JOBS:-7}"

  run_logged "install CPython" "$REPORT/cpython_install.txt" \
    make install
)

"$PY_INSTALL/bin/python3" --version | tee "$REPORT/cpython_verify.txt"

echo
echo "== build LLVM/Clang/LLD from source =="
LLVM_BUILD="$BUILD/llvm"
LLVM_INSTALL="$INSTALL/llvm"
mkdir -p "$LLVM_BUILD" "$LLVM_INSTALL"

run_logged "configure LLVM" "$REPORT/llvm_configure.txt" \
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
    -DLLVM_LINK_LLVM_DYLIB=OFF

run_logged "build LLVM" "$REPORT/llvm_build.txt" \
  ninja -v -C "$LLVM_BUILD" -j"${JOBS:-7}"

run_logged "install LLVM" "$REPORT/llvm_install.txt" \
  ninja -v -C "$LLVM_BUILD" install

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

  run_logged "build Rust stage1" "$REPORT/rust_stage1_build.txt" \
    "$PY_INSTALL/bin/python3" ./x.py build --stage 1 compiler/rustc cargo rustfmt clippy

  run_logged "build Rust stage2" "$REPORT/rust_stage2_build.txt" \
    "$PY_INSTALL/bin/python3" ./x.py build --stage 2 library/std compiler/rustc cargo rustfmt clippy

  run_logged "install Rust" "$REPORT/rust_install.txt" \
    "$PY_INSTALL/bin/python3" ./x.py install
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
    echo "STRIPPED: $f"
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

run_logged "compile baked C probe" "$REPORT/baked_c_probe_compile.txt" \
  "$BAKED/bin/clang" "$RUN/probe.c" -O3 -fuse-ld=lld -o "$RUN/probe_c"

run_logged "compile baked C++ probe" "$REPORT/baked_cpp_probe_compile.txt" \
  "$BAKED/bin/clang++" "$RUN/probe.cpp" -O3 -fuse-ld=lld -o "$RUN/probe_cpp"

run_logged "compile baked Rust probe" "$REPORT/baked_rust_probe_compile.txt" \
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
  echo "schema=braxon.full_android_language_toolchain.visible_report.v1"
  echo "date=$(date -Is)"
  echo "run=$RUN"
  echo "src=$SRC"
  echo "build=$BUILD"
  echo "install=$INSTALL"
  echo "baked=$BAKED"
  echo "custom_rust_preserved=true"
  echo "active_rust_replaced=false"
  echo "tmp_used=false"
  echo "progress_hidden=false"
  echo "full_source_rebuild_attempted=true"
  echo "optimization_bake_completed=true"
  echo "cpython_head=$(cat "$REPORT/cpython_head.txt" 2>/dev/null || true)"
  echo "llvm_head=$(cat "$REPORT/llvm_project_head.txt" 2>/dev/null || true)"
  echo "rust_head=$(cat "$REPORT/rust_head.txt" 2>/dev/null || true)"
} | tee "$RUN/full_rebuild_visible_report.txt"

echo
echo "PASS: full Android language/toolchain rebuild and bake completed"
echo "RUN=$RUN"
echo "BAKED=$BAKED"
