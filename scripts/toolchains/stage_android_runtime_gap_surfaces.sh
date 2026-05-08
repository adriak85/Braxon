#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
cd "$ROOT"

STAMP="$(date +%Y%m%d_%H%M%S)"
CHAIN_ROOT="$ROOT/state/android_gap_fill_chain"
RUN_DIR="$CHAIN_ROOT/runs/$STAMP"
REPORT_DIR="$RUN_DIR/reports"
STAGE_DIR="$CHAIN_ROOT/release_stage/current"
BIN_DIR="$STAGE_DIR/bin"
LIB_DIR="$STAGE_DIR/lib"
INCLUDE_DIR="$STAGE_DIR/include"
PROOF_DIR="$STAGE_DIR/proofs"

mkdir -p "$REPORT_DIR" "$BIN_DIR" "$LIB_DIR" "$INCLUDE_DIR" "$PROOF_DIR" scripts/toolchains config/toolchains

LOG="$RUN_DIR/stage_android_runtime_gap_surfaces.log"
exec > >(tee "$LOG") 2>&1

echo "== Braxon Android runtime gap surface staging =="
echo "date=$(date -Is)"
echo "root=$ROOT"
echo "head=$(git rev-parse HEAD)"
echo "branch=$(git branch --show-current)"
echo "stage_dir=$STAGE_DIR"
echo

echo "== hard rules =="
echo "custom Rust is input authority; do not replace it"
echo "Termux tools are bootstrap or currently verified surfaces"
echo "stage runtime gap surfaces before claiming release"
echo "do not write /tmp"
echo "do not commit generated binaries by default"
echo

echo "== required commands =="
for tool in clang clang++ ld.lld llvm-ar llvm-nm llvm-objdump llvm-strip readelf rustc cargo python3; do
  printf '%-16s ' "$tool"
  command -v "$tool" || {
    echo "FAIL: missing required tool: $tool"
    exit 1
  }
done | tee "$REPORT_DIR/required_commands.txt"
echo

echo "== active compiler authority =="
{
  clang --version
  echo
  ld.lld --version
  echo
  rustc --version --verbose
  echo
  cargo --version --verbose
} | tee "$REPORT_DIR/compiler_authority.txt"
echo

echo "== locate Android runtime gap surfaces =="
python3 - "$REPORT_DIR/runtime_surface_inventory.json" <<'PY'
import json
import os
import subprocess
from pathlib import Path

prefix = Path(os.environ.get("PREFIX", "/data/data/com.termux/files/usr"))

names = {
    "crt_objects": [
        "crtbegin_dynamic.o",
        "crtbegin_static.o",
        "crtbegin_so.o",
        "crtend_android.o",
        "crtend_so.o",
    ],
    "compiler_rt": [
        "libclang_rt.builtins-aarch64-android.a",
        "libclang_rt.builtins.a",
    ],
    "unwind": [
        "libunwind.a",
        "libunwind.so",
        "libgcc.a",
    ],
    "libcxx": [
        "libc++_shared.so",
        "libc++_static.a",
        "libc++.so",
        "libc++.a",
    ],
    "libcxxabi": [
        "libc++abi.a",
        "libc++abi.so",
    ],
    "system_libs": [
        "libc.so",
        "libm.so",
        "libdl.so",
    ],
}

search_roots = [
    prefix,
    prefix / "lib",
    prefix / "lib" / "clang",
    prefix / "include",
]

def find_name(name):
    hits = []
    for root in search_roots:
        if not root.exists():
            continue
        try:
            for p in root.rglob(name):
                if p.is_file():
                    hits.append(str(p))
        except PermissionError:
            pass
    return sorted(set(hits))

inventory = {
    "schema": "braxon.android_gap_fill.runtime_surface_inventory.v1",
    "prefix": str(prefix),
    "groups": {},
}

for group, group_names in names.items():
    inventory["groups"][group] = {}
    for name in group_names:
        inventory["groups"][group][name] = find_name(name)

# Headers
headers = {
    "stdint.h": find_name("stdint.h"),
    "stdio.h": find_name("stdio.h"),
    "iostream": find_name("iostream"),
    "vector": find_name("vector"),
    "string": find_name("string"),
    "unwind.h": find_name("unwind.h"),
}
inventory["headers"] = headers

Path(os.sys.argv[1]).write_text(json.dumps(inventory, indent=2), encoding="utf-8")
print(json.dumps(inventory, indent=2))
PY
echo

echo "== classify missing runtime surfaces =="
python3 - "$REPORT_DIR/runtime_surface_inventory.json" "$REPORT_DIR/runtime_gap_classification.json" <<'PY'
import json
import sys
from pathlib import Path

inventory = json.loads(Path(sys.argv[1]).read_text())
groups = inventory["groups"]

def any_present(group):
    return any(bool(paths) for paths in groups.get(group, {}).values())

classification = {
    "schema": "braxon.android_gap_fill.runtime_gap_classification.v1",
    "surfaces": {
        "crt_objects": "present" if any_present("crt_objects") else "missing",
        "compiler_rt": "present" if any_present("compiler_rt") else "missing",
        "libunwind": "present" if any_present("unwind") else "missing",
        "libcxx": "present" if any_present("libcxx") else "missing",
        "libcxxabi": "present" if any_present("libcxxabi") else "missing",
        "system_libs": "present" if any_present("system_libs") else "missing",
    },
    "next_action": [],
}

for name, status in classification["surfaces"].items():
    if status == "missing":
        classification["next_action"].append(f"build_or_vendor_from_source:{name}")
    else:
        classification["next_action"].append(f"stage_and_hash:{name}")

Path(sys.argv[2]).write_text(json.dumps(classification, indent=2), encoding="utf-8")
print(json.dumps(classification, indent=2))

if any(v == "missing" for v in classification["surfaces"].values()):
    print("WARN: some runtime surfaces are missing; release remains gap-fill staging, not complete replacement")
else:
    print("PASS: all primary runtime surfaces located")
PY
echo

echo "== stage selected runtime files as references, not source truth =="
python3 - "$REPORT_DIR/runtime_surface_inventory.json" "$STAGE_DIR" <<'PY'
import json
import shutil
import sys
from pathlib import Path

inventory = json.loads(Path(sys.argv[1]).read_text())
stage = Path(sys.argv[2])
lib = stage / "lib"
include = stage / "include"
proofs = stage / "proofs"
lib.mkdir(parents=True, exist_ok=True)
include.mkdir(parents=True, exist_ok=True)
proofs.mkdir(parents=True, exist_ok=True)

copied = []

for group_name, entries in inventory.get("groups", {}).items():
    for name, paths in entries.items():
        if not paths:
            continue
        src = Path(paths[0])
        dst = lib / name
        shutil.copy2(src, dst)
        copied.append({"group": group_name, "name": name, "src": str(src), "dst": str(dst)})

header_allow = {"stdint.h", "stdio.h", "unwind.h"}
for name, paths in inventory.get("headers", {}).items():
    if name not in header_allow or not paths:
        continue
    src = Path(paths[0])
    dst = include / name
    shutil.copy2(src, dst)
    copied.append({"group": "headers", "name": name, "src": str(src), "dst": str(dst)})

(proofs / "staged_runtime_files.json").write_text(json.dumps({
    "schema": "braxon.android_gap_fill.staged_runtime_files.v1",
    "note": "These are staged verified runtime surfaces from the active Android/Termux toolchain. They are not claimed as source truth.",
    "copied": copied,
}, indent=2), encoding="utf-8")

print(json.dumps({"copied_count": len(copied), "copied": copied[:25]}, indent=2))
PY
echo

echo "== create no-libc direct assembly proof in this chain =="
ASM="$RUN_DIR/nsq_android_direct_start.S"
OBJ="$RUN_DIR/nsq_android_direct_start.o"
BIN="$RUN_DIR/nsq_android_direct_start"

cat > "$ASM" <<'ASM'
.section .rodata
message:
    .ascii "BRAXON_ANDROID_DIRECT_START_OK\n"
message_len = . - message

.section .text
.global _start
.type _start, %function
_start:
    mov x0, #1
    adr x1, message
    mov x2, #31
    mov x8, #64
    svc #0

    mov x0, #37
    mov x8, #93
    svc #0
ASM

clang -target aarch64-linux-android24 -c "$ASM" -o "$OBJ"
ld.lld -o "$BIN" "$OBJ" -nostdlib -static --entry=_start

set +e
DIRECT_OUTPUT="$("$BIN" 2>&1)"
DIRECT_STATUS="$?"
set -e

{
  echo "schema=braxon.android_gap_fill.no_libc_direct_start_proof.v1"
  echo "date=$(date -Is)"
  echo "asm=$ASM"
  echo "object=$OBJ"
  echo "binary=$BIN"
  echo "exit_status=$DIRECT_STATUS"
  echo "output=$DIRECT_OUTPUT"
  echo
  readelf -h "$BIN"
  echo
  readelf -d "$BIN" || true
  echo
  llvm-objdump -d "$BIN"
} | tee "$REPORT_DIR/no_libc_direct_start_proof.txt"

if [ "$DIRECT_STATUS" != "37" ]; then
  echo "FAIL: no-libc direct start exit status was $DIRECT_STATUS, expected 37"
  exit 1
fi

printf '%s' "$DIRECT_OUTPUT" | grep -q "BRAXON_ANDROID_DIRECT_START_OK"

cp "$ASM" "$PROOF_DIR/nsq_android_direct_start.S"
cp "$OBJ" "$PROOF_DIR/nsq_android_direct_start.o"
cp "$BIN" "$PROOF_DIR/nsq_android_direct_start"
cp "$REPORT_DIR/no_libc_direct_start_proof.txt" "$PROOF_DIR/no_libc_direct_start_proof.txt"
echo "PASS: no-libc direct Android start proof staged"
echo

echo "== compile staged C/C++/Rust release probes =="
cat > "$RUN_DIR/release_probe.c" <<'C'
#include <stdio.h>
#include <stdint.h>

int main(void) {
    printf("BRAXON_RELEASE_C_PROBE_OK\n");
    printf("%zu\n", sizeof(uintptr_t));
    return 0;
}
C

cat > "$RUN_DIR/release_probe.cpp" <<'CPP'
#include <iostream>
#include <string>
#include <vector>

int main() {
    std::vector<std::string> parts = {"BRAXON", "RELEASE", "CPP", "PROBE", "OK"};
    for (const auto& p : parts) std::cout << p << "\n";
    return 0;
}
CPP

cat > "$RUN_DIR/release_probe.rs" <<'RS'
fn main() {
    println!("BRAXON_RELEASE_RUST_PROBE_OK");
    println!("{}", std::env::consts::OS);
}
RS

clang "$RUN_DIR/release_probe.c" -O3 -fuse-ld=lld -o "$BIN_DIR/release_probe_c"
clang++ "$RUN_DIR/release_probe.cpp" -O3 -fuse-ld=lld -o "$BIN_DIR/release_probe_cpp"
rustc "$RUN_DIR/release_probe.rs" -C opt-level=3 -C codegen-units=1 -o "$BIN_DIR/release_probe_rust"

{
  "$BIN_DIR/release_probe_c"
  "$BIN_DIR/release_probe_cpp"
  "$BIN_DIR/release_probe_rust"
} | tee "$REPORT_DIR/release_probe_output.txt"

grep -q "BRAXON_RELEASE_C_PROBE_OK" "$REPORT_DIR/release_probe_output.txt"
grep -q "BRAXON_RELEASE_RUST_PROBE_OK" "$REPORT_DIR/release_probe_output.txt"
grep -q "BRAXON.*RELEASE.*CPP.*PROBE.*OK" <(tr '\n' ' ' < "$REPORT_DIR/release_probe_output.txt")
echo "PASS: release probes compiled and ran"
echo

echo "== strip staged ELF binaries after debug copies =="
find "$BIN_DIR" "$PROOF_DIR" -type f | while read -r f; do
  if file "$f" | grep -q 'ELF'; then
    cp "$f" "$f.debug"
    llvm-strip "$f" || true
  fi
done
echo

echo "== write release-stage manifest =="
cat > "$STAGE_DIR/ANDROID_GAP_FILL_RELEASE_STAGE.json" <<JSON
{
  "schema": "braxon.android_gap_fill.release_stage.v1",
  "created_at": "$(date -Is)",
  "repo_head": "$(git rev-parse HEAD)",
  "branch": "$(git branch --show-current)",
  "custom_rust_preserved": true,
  "custom_rust": "$(rustc --version | sed 's/"/\\"/g')",
  "clang": "$(clang --version | head -n 1 | sed 's/"/\\"/g')",
  "target": "aarch64-linux-android",
  "android_api_observed": "24",
  "stage_dir": "state/android_gap_fill_chain/release_stage/current",
  "contains": [
    "runtime surface references",
    "headers references",
    "no-libc direct start proof",
    "C release probe",
    "C++ release probe",
    "Rust release probe",
    "hash manifest"
  ],
  "non_claims": [
    "does not replace custom Rust",
    "does not claim complete Android NDK replacement",
    "does not claim generated binaries are source truth",
    "does not package model weights"
  ]
}
JSON

echo "== hash release stage =="
(
  cd "$STAGE_DIR"
  find . -type f -print0 | sort -z | xargs -0 sha256sum
) | tee "$STAGE_DIR/SHA256SUMS"

cp "$REPORT_DIR/runtime_surface_inventory.json" "$PROOF_DIR/runtime_surface_inventory.json"
cp "$REPORT_DIR/runtime_gap_classification.json" "$PROOF_DIR/runtime_gap_classification.json"
cp "$REPORT_DIR/release_probe_output.txt" "$PROOF_DIR/release_probe_output.txt"
cp "$REPORT_DIR/compiler_authority.txt" "$PROOF_DIR/compiler_authority.txt"

echo
echo "== final runtime gap stage report =="
{
  echo "schema=braxon.android_gap_fill.runtime_gap_stage_report.v1"
  echo "date=$(date -Is)"
  echo "run_dir=$RUN_DIR"
  echo "stage_dir=$STAGE_DIR"
  echo "custom_rust_preserved=true"
  echo "no_libc_direct_start=pass"
  echo "release_probes=pass"
  echo "hash_manifest=$STAGE_DIR/SHA256SUMS"
} | tee "$RUN_DIR/runtime_gap_stage_report.txt"

echo
echo "PASS: Android runtime gap surfaces staged"
echo "RUN_DIR=$RUN_DIR"
echo "STAGE_DIR=$STAGE_DIR"
