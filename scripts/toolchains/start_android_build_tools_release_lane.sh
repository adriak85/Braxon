#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
cd "$ROOT"

STAMP="$(date +%Y%m%d_%H%M%S)"
LANE_ROOT="$ROOT/state/android_build_tools_release_lane"
RUN_DIR="$LANE_ROOT/runs/$STAMP"
REPORT_DIR="$RUN_DIR/reports"
STAGE_DIR="$LANE_ROOT/stage/current"
TOOL_DIR="$STAGE_DIR/bin"
PROOF_DIR="$STAGE_DIR/proofs"

mkdir -p "$REPORT_DIR" "$TOOL_DIR" "$PROOF_DIR" scripts/toolchains config/toolchains

LOG="$RUN_DIR/android_build_tools_release_lane.log"
exec > >(tee "$LOG") 2>&1

echo "== Braxon Android build-tools release lane =="
echo "date=$(date -Is)"
echo "root=$ROOT"
echo "head=$(git rev-parse HEAD)"
echo "branch=$(git branch --show-current)"
echo "lane_root=$LANE_ROOT"
echo "run_dir=$RUN_DIR"
echo "stage_dir=$STAGE_DIR"
echo

echo "== hard target =="
echo "This lane is for BUILD TOOLS, not runtime packaging."
echo "Custom Rust is preserved and treated as active compiler authority."
echo "Runtime objects/libs are dependency observations only."
echo "Do not replace Rust."
echo "Do not write /tmp."
echo "Do not commit generated binaries by default."
echo

echo "== quarantine runtime-focused direction marker =="
cat > "$REPORT_DIR/runtime_lane_quarantine_note.txt" <<'NOTE'
The previous runtime gap surface staging is preserved as evidence, but it is not the current primary target.

Current primary target:
- build tools
- compiler/linker/archive/object/disassembly surfaces
- source-built or locally verified toolchain authority
- Android/Termux release packaging for build tools

Runtime libraries are only checked when needed to prove build tools can produce valid Android binaries.
NOTE
cat "$REPORT_DIR/runtime_lane_quarantine_note.txt"
echo

echo "== write build-tools manifest =="
cat > config/toolchains/android_build_tools_release_lane.json <<JSON
{
  "schema": "braxon.android_build_tools_release_lane.v1",
  "authority": "BRAXON_ANDROID_BUILD_TOOLS",
  "created_at": "$(date -Is)",
  "repo_head": "$(git rev-parse HEAD)",
  "branch": "$(git branch --show-current)",
  "purpose": "Prepare a releasable Android/Termux build-tools gap-fill package.",
  "primary_scope": [
    "clang",
    "clang++",
    "ld.lld",
    "lld",
    "llvm-ar",
    "llvm-ranlib",
    "llvm-nm",
    "llvm-objdump",
    "llvm-readelf",
    "llvm-strip",
    "llvm-strings",
    "llvm-size",
    "rustc",
    "cargo",
    "rustfmt",
    "clippy-driver",
    "python3",
    "cmake",
    "ninja",
    "pkg-config",
    "make"
  ],
  "non_goals": [
    "runtime-library packaging as the main release target",
    "model weight packaging",
    "replacing custom Rust",
    "claiming complete Android NDK replacement without separate proof"
  ],
  "rules": {
    "custom_rust_preserved": true,
    "runtime_surfaces_are_dependency_observations": true,
    "source_build_preferred": true,
    "active_verified_tools_allowed_as_current_release_lane": true,
    "tmp_usage_allowed": false,
    "generated_binaries_committed_by_default": false
  }
}
JSON

cat config/toolchains/android_build_tools_release_lane.json | tee "$REPORT_DIR/build_tools_manifest.json"
echo

echo "== active build-tool authority =="
{
  echo "schema=braxon.android_build_tools.authority_report.v1"
  echo "date=$(date -Is)"
  echo "PATH=$PATH"
  echo "PREFIX=${PREFIX:-unset}"
  echo
  for tool in \
    clang clang++ ld.lld lld llvm-ar llvm-ranlib llvm-nm llvm-objdump llvm-readelf llvm-strip llvm-strings llvm-size \
    rustc cargo rustfmt clippy-driver python3 cmake ninja pkg-config make git file readelf sha256sum
  do
    printf '%-18s ' "$tool"
    command -v "$tool" || true
  done
  echo
  echo "== versions =="
  clang --version || true
  echo
  ld.lld --version || true
  echo
  llvm-ar --version || true
  echo
  llvm-objdump --version | head -n 20 || true
  echo
  rustc --version --verbose || true
  echo
  cargo --version --verbose || true
  echo
  python3 --version || true
  echo
  cmake --version || true
  echo
  ninja --version || true
} | tee "$REPORT_DIR/build_tool_authority.txt"
echo

echo "== build-tool inventory =="
python3 - "$REPORT_DIR/build_tool_inventory.json" <<'PY'
import json
import os
import shutil
import subprocess
from pathlib import Path

tools = [
    "clang", "clang++", "ld.lld", "lld",
    "llvm-ar", "llvm-ranlib", "llvm-nm", "llvm-objdump", "llvm-readelf",
    "llvm-strip", "llvm-strings", "llvm-size",
    "rustc", "cargo", "rustfmt", "clippy-driver",
    "python3", "cmake", "ninja", "pkg-config", "make", "git", "file", "readelf", "sha256sum",
]

records = []
for tool in tools:
    path = shutil.which(tool)
    rec = {
        "tool": tool,
        "path": path,
        "present": bool(path),
        "sha256": None,
        "file": None,
    }
    if path and Path(path).is_file():
        try:
            rec["sha256"] = subprocess.check_output(["sha256sum", path], text=True).split()[0]
        except Exception as e:
            rec["sha256_error"] = str(e)
        try:
            rec["file"] = subprocess.check_output(["file", path], text=True).strip()
        except Exception as e:
            rec["file_error"] = str(e)
    records.append(rec)

missing = [r["tool"] for r in records if not r["present"]]

out = {
    "schema": "braxon.android_build_tools.inventory.v1",
    "records": records,
    "missing": missing,
    "complete": len(missing) == 0,
}
Path(os.sys.argv[1]).write_text(json.dumps(out, indent=2), encoding="utf-8")
print(json.dumps(out, indent=2))
if missing:
    print("WARN: missing build tools:", ", ".join(missing))
PY
echo

echo "== build-tool functional proof =="
cat > "$RUN_DIR/build_tool_probe.c" <<'C'
#include <stdint.h>

uint64_t braxon_build_tool_probe(uint64_t x) {
    return (x * 37u) ^ 0xBADC0FFEEu;
}

int main(void) {
    return (int)(braxon_build_tool_probe(7) & 0);
}
C

cat > "$RUN_DIR/build_tool_probe.cpp" <<'CPP'
#include <string>
#include <vector>

int main() {
    std::vector<std::string> parts = {"BRAXON", "BUILD", "TOOLS"};
    return parts.size() == 3 ? 0 : 1;
}
CPP

cat > "$RUN_DIR/build_tool_probe.rs" <<'RS'
pub fn braxon_build_tool_probe(x: u64) -> u64 {
    (x * 37) ^ 0xBADC0FFEE
}

fn main() {
    let _ = braxon_build_tool_probe(7);
}
RS

{
  echo "== C object build =="
  clang -target aarch64-linux-android24 -O3 -c "$RUN_DIR/build_tool_probe.c" -o "$RUN_DIR/build_tool_probe_c.o"
  file "$RUN_DIR/build_tool_probe_c.o"
  llvm-nm "$RUN_DIR/build_tool_probe_c.o" | grep braxon_build_tool_probe
  llvm-objdump -d "$RUN_DIR/build_tool_probe_c.o" | sed -n '1,120p'

  echo
  echo "== archive build =="
  llvm-ar rcs "$RUN_DIR/libbraxon_build_tool_probe.a" "$RUN_DIR/build_tool_probe_c.o"
  llvm-ranlib "$RUN_DIR/libbraxon_build_tool_probe.a"
  llvm-ar t "$RUN_DIR/libbraxon_build_tool_probe.a"
  llvm-nm "$RUN_DIR/libbraxon_build_tool_probe.a" | grep braxon_build_tool_probe

  echo
  echo "== C executable link =="
  clang -target aarch64-linux-android24 -O3 "$RUN_DIR/build_tool_probe.c" -fuse-ld=lld -o "$RUN_DIR/build_tool_probe_c"
  "$RUN_DIR/build_tool_probe_c"
  readelf -h "$RUN_DIR/build_tool_probe_c" | sed -n '1,80p'

  echo
  echo "== C++ executable link =="
  clang++ -target aarch64-linux-android24 -O3 "$RUN_DIR/build_tool_probe.cpp" -fuse-ld=lld -o "$RUN_DIR/build_tool_probe_cpp"
  "$RUN_DIR/build_tool_probe_cpp"
  readelf -h "$RUN_DIR/build_tool_probe_cpp" | sed -n '1,80p'

  echo
  echo "== Rust executable build with preserved custom Rust =="
  rustc "$RUN_DIR/build_tool_probe.rs" -C opt-level=3 -C codegen-units=1 -o "$RUN_DIR/build_tool_probe_rust"
  "$RUN_DIR/build_tool_probe_rust"
  readelf -h "$RUN_DIR/build_tool_probe_rust" | sed -n '1,80p'
} | tee "$REPORT_DIR/build_tool_functional_proof.txt"
echo

echo "== stage build tools as references, not source truth =="
rm -rf "$STAGE_DIR"
mkdir -p "$TOOL_DIR" "$PROOF_DIR"

python3 - "$REPORT_DIR/build_tool_inventory.json" "$TOOL_DIR" "$PROOF_DIR/staged_build_tools.json" <<'PY'
import json
import shutil
import sys
from pathlib import Path

inventory = json.loads(Path(sys.argv[1]).read_text())
tool_dir = Path(sys.argv[2])
proof_path = Path(sys.argv[3])
tool_dir.mkdir(parents=True, exist_ok=True)

copied = []
for rec in inventory["records"]:
    path = rec.get("path")
    if not path:
        continue
    src = Path(path)
    if not src.is_file():
        continue
    dst = tool_dir / rec["tool"]
    shutil.copy2(src, dst)
    copied.append({
        "tool": rec["tool"],
        "src": str(src),
        "dst": str(dst),
        "sha256": rec.get("sha256"),
        "file": rec.get("file"),
    })

proof = {
    "schema": "braxon.android_build_tools.staged_build_tools.v1",
    "note": "These are staged active build-tool binaries from the current verified Android/Termux lane. They are not claimed as source truth unless separately tied to source build proof.",
    "copied_count": len(copied),
    "copied": copied,
}
proof_path.write_text(json.dumps(proof, indent=2), encoding="utf-8")
print(json.dumps({"copied_count": len(copied), "proof": str(proof_path)}, indent=2))
PY

cp "$REPORT_DIR/build_tool_authority.txt" "$PROOF_DIR/build_tool_authority.txt"
cp "$REPORT_DIR/build_tool_inventory.json" "$PROOF_DIR/build_tool_inventory.json"
cp "$REPORT_DIR/build_tool_functional_proof.txt" "$PROOF_DIR/build_tool_functional_proof.txt"
cp "$REPORT_DIR/runtime_lane_quarantine_note.txt" "$PROOF_DIR/runtime_lane_quarantine_note.txt"

echo
echo "== write build-tools release stage manifest =="
cat > "$STAGE_DIR/ANDROID_BUILD_TOOLS_RELEASE_STAGE.json" <<JSON
{
  "schema": "braxon.android_build_tools.release_stage.v1",
  "created_at": "$(date -Is)",
  "repo_head": "$(git rev-parse HEAD)",
  "branch": "$(git branch --show-current)",
  "stage_dir": "state/android_build_tools_release_lane/stage/current",
  "custom_rust_preserved": true,
  "rustc": "$(rustc --version | sed 's/"/\\"/g')",
  "cargo": "$(cargo --version | sed 's/"/\\"/g')",
  "clang": "$(clang --version | head -n 1 | sed 's/"/\\"/g')",
  "target": "aarch64-linux-android24",
  "primary_claim": "Android/Termux build-tools gap-fill staging lane",
  "runtime_scope": "dependency observation only",
  "contains": [
    "build tool authority report",
    "build tool inventory",
    "functional compile/link/archive/disassemble proof",
    "staged active build-tool references",
    "hash manifest"
  ],
  "non_claims": [
    "does not replace custom Rust",
    "does not claim runtime library package as primary target",
    "does not claim complete Android NDK replacement",
    "does not package model weights"
  ]
}
JSON

echo
echo "== hash staged build-tools lane =="
(
  cd "$STAGE_DIR"
  rm -f SHA256SUMS SHA256SUMS.tmp
  find . -type f ! -name 'SHA256SUMS' ! -name 'SHA256SUMS.tmp' -print0 | sort -z | xargs -0 sha256sum
) | tee "$STAGE_DIR/SHA256SUMS"

echo
echo "== final build-tools release lane report =="
{
  echo "schema=braxon.android_build_tools.run_report.v1"
  echo "date=$(date -Is)"
  echo "run_dir=$RUN_DIR"
  echo "stage_dir=$STAGE_DIR"
  echo "custom_rust_preserved=true"
  echo "runtime_lane_primary=false"
  echo "build_tools_primary=true"
  echo "functional_proof=$REPORT_DIR/build_tool_functional_proof.txt"
  echo "hash_manifest=$STAGE_DIR/SHA256SUMS"
} | tee "$RUN_DIR/android_build_tools_run_report.txt"

echo
echo "PASS: Android build-tools release lane staged"
echo "RUN_DIR=$RUN_DIR"
echo "STAGE_DIR=$STAGE_DIR"
