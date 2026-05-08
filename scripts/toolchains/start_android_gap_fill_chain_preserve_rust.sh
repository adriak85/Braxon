#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
cd "$ROOT"

STAMP="$(date +%Y%m%d_%H%M%S)"
CHAIN_ROOT="$ROOT/state/android_gap_fill_chain"
RUN_DIR="$CHAIN_ROOT/runs/$STAMP"
REPORT_DIR="$RUN_DIR/reports"
SRC_DIR="$CHAIN_ROOT/src"
BUILD_DIR="$CHAIN_ROOT/build"
INSTALL_DIR="$CHAIN_ROOT/install"

mkdir -p "$REPORT_DIR" "$SRC_DIR" "$BUILD_DIR" "$INSTALL_DIR" scripts/toolchains config/toolchains

LOG="$RUN_DIR/android_gap_fill_chain_preserve_rust.log"
exec > >(tee "$LOG") 2>&1

echo "== Braxon Android gap-fill chain: preserve custom Rust =="
echo "date=$(date -Is)"
echo "root=$ROOT"
echo "head=$(git rev-parse HEAD)"
echo "branch=$(git branch --show-current)"
echo

echo "== hard rule =="
echo "Do not replace custom Rust."
echo "Do not clone upstream Rust over the active Rust lane."
echo "Do not write into /tmp."
echo "Detect active compiler authority first."
echo "Build only missing Android gaps from source."
echo

echo "== current git state =="
git status --branch --short | tee "$REPORT_DIR/git_status_start.txt"
echo

echo "== repair known failed local resolver edit if present =="
if git diff -- crates/nsqasm-stamp-db/src/main.rs | grep -q 'resolve_stamp'; then
  echo "Detected uncommitted failed resolver patch in crates/nsqasm-stamp-db/src/main.rs"
  git restore -- crates/nsqasm-stamp-db/src/main.rs
  echo "PASS: restored nsqasm-stamp-db/src/main.rs to committed good state"
else
  echo "PASS: no failed resolver patch detected in nsqasm-stamp-db/src/main.rs"
fi
echo

echo "== record active Rust authority =="
{
  echo "schema=braxon.android_gap_fill.active_rust_authority.v1"
  echo "date=$(date -Is)"
  echo "which_rustc=$(command -v rustc || true)"
  echo "which_cargo=$(command -v cargo || true)"
  echo "which_rustup=$(command -v rustup || true)"
  echo
  rustc --version --verbose || true
  echo
  cargo --version --verbose || true
  echo
  echo "RUSTUP_HOME=${RUSTUP_HOME:-unset}"
  echo "CARGO_HOME=${CARGO_HOME:-unset}"
  echo "PATH=$PATH"
} | tee "$REPORT_DIR/active_rust_authority.txt"
echo

echo "== record active LLVM/Clang/linker/libc authority =="
{
  echo "schema=braxon.android_gap_fill.active_c_toolchain_authority.v1"
  echo "date=$(date -Is)"
  echo "which_clang=$(command -v clang || true)"
  echo "which_clangxx=$(command -v clang++ || true)"
  echo "which_ld_lld=$(command -v ld.lld || true)"
  echo "which_llvm_ar=$(command -v llvm-ar || true)"
  echo "which_llvm_nm=$(command -v llvm-nm || true)"
  echo "which_readelf=$(command -v readelf || true)"
  echo
  clang --version || true
  echo
  ld.lld --version || true
  echo
  llvm-ar --version || true
  echo
  getconf GNU_LIBC_VERSION 2>/dev/null || true
  echo "PREFIX=${PREFIX:-unset}"
} | tee "$REPORT_DIR/active_c_toolchain_authority.txt"
echo

echo "== write gap-fill manifest =="
cat > config/toolchains/android_gap_fill_chain.json <<JSON
{
  "schema": "braxon.android_gap_fill_chain.v1",
  "authority": "BRAXON_ANDROID_GAP_FILL",
  "created_at": "$(date -Is)",
  "repo_head": "$(git rev-parse HEAD)",
  "branch": "$(git branch --show-current)",
  "paths": {
    "chain_root": "state/android_gap_fill_chain",
    "runs": "state/android_gap_fill_chain/runs",
    "src": "state/android_gap_fill_chain/src",
    "build": "state/android_gap_fill_chain/build",
    "install": "state/android_gap_fill_chain/install"
  },
  "preservation_rules": {
    "custom_rust_is_preserved": true,
    "active_rust_is_input_authority": true,
    "do_not_clone_or_replace_rust": true,
    "do_not_write_tmp": true,
    "build_missing_gaps_from_source": true,
    "nsqasm_allowed_as_dispatch_prebake_not_source_replacement": true
  },
  "gap_targets": [
    "android_no_libc_assembly_start",
    "linker_surface",
    "crt_start_objects",
    "libunwind_surface",
    "compiler_rt_builtins_surface",
    "libcxx_surface",
    "pkg_config_surface",
    "headers_surface",
    "sysroot_probe",
    "source_built_release_bake"
  ]
}
JSON

cat config/toolchains/android_gap_fill_chain.json | tee "$REPORT_DIR/android_gap_fill_chain_manifest.json"
echo

echo "== probe Android toolchain gaps =="
cat > "$RUN_DIR/probe_android_gaps.c" <<'C'
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

int main(void) {
    printf("BRAXON_ANDROID_GAP_PROBE_OK\n");
    printf("sizeof(void*)=%zu\n", sizeof(void*));
    printf("sizeof(uintptr_t)=%zu\n", sizeof(uintptr_t));
    return 0;
}
C

cat > "$RUN_DIR/probe_android_gaps.cpp" <<'CPP'
#include <iostream>
#include <vector>
#include <string>

int main() {
    std::vector<std::string> v = {"BRAXON", "ANDROID", "GAP", "PROBE", "OK"};
    for (const auto& s : v) std::cout << s << '\n';
    return 0;
}
CPP

cat > "$RUN_DIR/probe_android_gaps.rs" <<'RS'
fn main() {
    println!("BRAXON_RUST_CUSTOM_AUTHORITY_PROBE_OK");
    println!("target_arch={}", std::env::consts::ARCH);
    println!("target_os={}", std::env::consts::OS);
}
RS

{
  echo "== C compile/run =="
  clang "$RUN_DIR/probe_android_gaps.c" -o "$RUN_DIR/probe_c" -fuse-ld=lld
  "$RUN_DIR/probe_c"

  echo
  echo "== C++ compile/run =="
  clang++ "$RUN_DIR/probe_android_gaps.cpp" -o "$RUN_DIR/probe_cpp" -fuse-ld=lld
  "$RUN_DIR/probe_cpp"

  echo
  echo "== Rust compile/run using active custom Rust =="
  rustc "$RUN_DIR/probe_android_gaps.rs" -o "$RUN_DIR/probe_rust"
  "$RUN_DIR/probe_rust"
} | tee "$REPORT_DIR/language_probe.txt"
echo

echo "== inspect generated probe binaries =="
{
  for bin in "$RUN_DIR/probe_c" "$RUN_DIR/probe_cpp" "$RUN_DIR/probe_rust"; do
    echo "---- $bin ----"
    file "$bin" || true
    readelf -h "$bin" || true
    echo
    echo "dynamic:"
    readelf -d "$bin" || true
    echo
    echo "needed:"
    readelf -d "$bin" 2>/dev/null | grep NEEDED || true
    echo
  done
} | tee "$REPORT_DIR/probe_binary_inspection.txt"
echo

echo "== classify initial gaps =="
python3 - "$REPORT_DIR" <<'PY'
import json
import sys
from pathlib import Path

report_dir = Path(sys.argv[1])
inspection = (report_dir / "probe_binary_inspection.txt").read_text(errors="replace")
language = (report_dir / "language_probe.txt").read_text(errors="replace")

gaps = []

def add(name, status, detail):
    gaps.append({"name": name, "status": status, "detail": detail})

add("custom_rust_active", "pass" if "BRAXON_RUST_CUSTOM_AUTHORITY_PROBE_OK" in language else "fail", "active rustc compiled and ran probe")
add("clang_c_active", "pass" if "BRAXON_ANDROID_GAP_PROBE_OK" in language else "fail", "clang compiled and ran C probe")
add("clang_cpp_active", "pass" if "BRAXON\nANDROID\nGAP\nPROBE\nOK" in language else "fail", "clang++ compiled and ran C++ STL probe")
add("dynamic_link_surface", "observed" if "NEEDED" in inspection else "static_or_not_reported", "readelf dynamic dependency surface")
add("elf_surface", "pass" if "ELF" in inspection else "fail", "probe binaries are inspectable ELF artifacts")

out = {
    "schema": "braxon.android_gap_fill.initial_gap_report.v1",
    "gaps": gaps,
    "next_build_targets": [
        "build or verify crt objects",
        "build or verify compiler-rt builtins",
        "build or verify libunwind",
        "build or verify libc++/libc++abi",
        "bake source-built or verified-active toolchain into release lane"
    ]
}

path = report_dir / "initial_gap_report.json"
path.write_text(json.dumps(out, indent=2), encoding="utf-8")
print(json.dumps(out, indent=2))
PY
echo

echo "== run existing repo gates without replacing Rust =="
cargo test -p nsqasm-stamp-db -- --nocapture | tee "$REPORT_DIR/nsqasm_stamp_db_tests.txt"
cargo nextest run --workspace --bins --lib --all-targets --all-features --all --release --no-fail-fast -j7 | tee "$REPORT_DIR/workspace_nextest_release.txt"
echo

echo "== final report =="
{
  echo "schema=braxon.android_gap_fill.run_report.v1"
  echo "date=$(date -Is)"
  echo "run_dir=$RUN_DIR"
  echo "custom_rust_preserved=true"
  echo "upstream_rust_clone_attempted=false"
  echo "tmp_used=false"
  echo "manifest=config/toolchains/android_gap_fill_chain.json"
  echo "initial_gap_report=$REPORT_DIR/initial_gap_report.json"
} | tee "$RUN_DIR/android_gap_fill_run_report.txt"

echo
echo "PASS: Android gap-fill chain started without replacing custom Rust"
echo "Run dir: $RUN_DIR"
