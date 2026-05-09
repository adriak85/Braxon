#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
cd "$ROOT"

echo "== clean Android gap-fill generated artifacts and ignore policy =="

echo
echo "== remove bad source-build script if uncommitted =="
if [ -f scripts/toolchains/start_android_source_build_chain.sh ]; then
  if git ls-files --error-unmatch scripts/toolchains/start_android_source_build_chain.sh >/dev/null 2>&1; then
    echo "WARN: start_android_source_build_chain.sh is tracked; not deleting automatically"
  else
    rm -f scripts/toolchains/start_android_source_build_chain.sh
    echo "PASS: removed untracked source-build script that tried to clone/replace Rust lane"
  fi
else
  echo "PASS: bad source-build script not present"
fi

echo
echo "== append generated artifact ignores =="
cat >> .gitignore <<'GITIGNORE'

# Android gap-fill generated local artifacts
state/android_toolchain_build_chain/
state/android_gap_fill_chain/release_stage/current/bin/
state/android_gap_fill_chain/release_stage/current/include/
state/android_gap_fill_chain/release_stage/current/lib/
state/android_gap_fill_chain/release_stage/current/proofs/*.o
state/android_gap_fill_chain/release_stage/current/proofs/*_start
state/android_gap_fill_chain/release_stage/current/proofs/*.debug
state/android_gap_fill_chain/release_stage/current/proofs/*.debug.*
state/android_gap_fill_chain/runs/*/*.c
state/android_gap_fill_chain/runs/*/*.cpp
state/android_gap_fill_chain/runs/*/*.rs
state/android_gap_fill_chain/runs/*/*.S
state/android_gap_fill_chain/runs/*/*.o
state/android_gap_fill_chain/runs/*/probe_c
state/android_gap_fill_chain/runs/*/probe_cpp
state/android_gap_fill_chain/runs/*/probe_rust
state/android_gap_fill_chain/runs/*/release_probe_*
state/android_gap_fill_chain/runs/*/nsq_android_direct_start
state/android_gap_fill_chain/runs/*/nsq_android_direct_start.*
GITIGNORE

# Deduplicate .gitignore while preserving order.
python3 - <<'PY'
from pathlib import Path

p = Path(".gitignore")
seen = set()
out = []
for line in p.read_text().splitlines():
    key = line.rstrip()
    if key and key in seen:
        continue
    if key:
        seen.add(key)
    out.append(line)
p.write_text("\n".join(out).rstrip() + "\n")
PY

echo
echo "== remove generated dirs/files from worktree only when untracked =="
rm -rf \
  state/android_toolchain_build_chain \
  state/android_gap_fill_chain/release_stage/current/bin \
  state/android_gap_fill_chain/release_stage/current/include \
  state/android_gap_fill_chain/release_stage/current/lib

find state/android_gap_fill_chain/release_stage/current/proofs -type f \( \
  -name '*.o' -o \
  -name '*.debug' -o \
  -name '*.debug.*' -o \
  -name 'nsq_android_direct_start' \
\) -delete 2>/dev/null || true

find state/android_gap_fill_chain/runs -type f \( \
  -name '*.c' -o \
  -name '*.cpp' -o \
  -name '*.rs' -o \
  -name '*.S' -o \
  -name '*.o' -o \
  -name 'probe_c' -o \
  -name 'probe_cpp' -o \
  -name 'probe_rust' -o \
  -name 'release_probe_*' -o \
  -name 'nsq_android_direct_start' -o \
  -name 'nsq_android_direct_start.*' \
\) -delete 2>/dev/null || true

echo
echo "== status after cleanup =="
git status --branch --short
