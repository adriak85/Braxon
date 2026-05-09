#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$HOME/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
STAMP="$(date +%Y%m%d_%H%M%S)"
OUTDIR="$HOME/storage/shared/Download/braxon_full_system_state_${STAMP}"
ARCHIVE="$HOME/storage/shared/Download/braxon_full_system_state_${STAMP}.tar.gz"
LOG="$OUTDIR/MASTER_LOG.txt"
JOBS="${JOBS:-7}"

mkdir -p "$OUTDIR"/{system,env,packages,package_files,bin,python,perl,rust,zig,c_cpp,tree_sitter,fonts,vulkan,pkg_config,includes,libs,topology,cargo,repos,locks,tests,hash,manifests,state_registry,tmp}

exec > >(tee -a "$LOG") 2>&1

cap() {
  local out="$1"; shift
  "$@" > "$out" 2>&1 || true
}

section() {
  echo
  echo "=== $* ==="
}

section "BRAXON FULL SYSTEM STATE EXPORT"
date

cd "$ROOT"
source "$ROOT/braxon-rust-env" 2>/dev/null || true
source "$ROOT/braxon-text-env" 2>/dev/null || true

export PATH="$ROOT:$TC/install/python/bin:/data/data/com.termux/files/usr/opt/rust-nightly/bin:/data/data/com.termux/files/usr/bin:$HOME/.cargo/bin:$HOME/.local/bin:$PATH"
export LD_LIBRARY_PATH="$TC/install/braxon_android_overlay/lib:/data/data/com.termux/files/usr/lib"
export PKG_CONFIG_PATH="/data/data/com.termux/files/usr/lib/pkgconfig:/data/data/com.termux/files/usr/share/pkgconfig:${PKG_CONFIG_PATH:-}"

env | sort > "$OUTDIR/env/full_environment.txt"

section "android/system"
cap "$OUTDIR/system/getprop.txt" getprop
cap "$OUTDIR/system/uname.txt" uname -a
cat /proc/version > "$OUTDIR/system/proc_version.txt" 2>/dev/null || uname -v > "$OUTDIR/system/proc_version.txt" 2>/dev/null || echo proc_version_permission_denied > "$OUTDIR/system/proc_version.txt"
cap "$OUTDIR/system/cpuinfo.txt" cat /proc/cpuinfo
cap "$OUTDIR/system/meminfo.txt" cat /proc/meminfo
cap "$OUTDIR/system/df_h.txt" df -h
cap "$OUTDIR/system/termux_info.txt" termux-info

section "package inventories"
cap "$OUTDIR/packages/pkg_list_installed.txt" pkg list-installed
cap "$OUTDIR/packages/apt_list_installed.txt" apt list --installed
pkg list-installed | sed '/^Listing/d' | cut -d/ -f1 | sort > "$OUTDIR/packages/package_names.txt"

section "package file maps, j${JOBS}"
cat "$OUTDIR/packages/package_names.txt" \
  | xargs -r -n 1 -P "$JOBS" sh -c '
      outdir="$1"
      pkgname="$2"
      safe="$(printf "%s" "$pkgname" | tr "/:" "__")"
      pkg files "$pkgname" > "$outdir/package_files/${safe}.files.txt" 2>&1 || true
    ' sh "$OUTDIR"

section "bounded executable inventory"
{
  fd --type executable --max-depth 1 . /data/data/com.termux/files/usr/bin 2>/dev/null
  fd --type executable --max-depth 1 . "$HOME/.cargo/bin" 2>/dev/null
  fd --type executable --max-depth 1 . "$HOME/.local/bin" 2>/dev/null
  fd --type executable --max-depth 1 . "$ROOT" 2>/dev/null
  fd --type executable --max-depth 1 . "$ROOT/scripts" 2>/dev/null
} | sort -u > "$OUTDIR/bin/executable_inventory.txt"

section "command surface"
for x in braxon-python braxon-rustc braxon-cargo fastest_status python3 perl cpan prove rustc cargo clang zig zls tree-sitter hb-shape fc-match glslc spirv-val vulkaninfo rg fd jq bash zsh fish; do
  {
    echo "=== $x ==="
    command -v "$x" || true
    if command -v "$x" >/dev/null 2>&1; then
      file "$(command -v "$x")" 2>/dev/null || true
    fi
    echo
  } >> "$OUTDIR/bin/command_surface.txt"
done

# Safe version probes only. No generic --version/-V loop.
{
  "$ROOT/braxon-python" --version 2>/dev/null || true
  "$ROOT/braxon-rustc" --version 2>/dev/null || true
  "$ROOT/braxon-cargo" --version 2>/dev/null || true
  python3 --version 2>/dev/null || true
  perl -v 2>/dev/null | head -n 2 || true
  rustc --version 2>/dev/null || true
  cargo --version 2>/dev/null || true
  clang --version 2>/dev/null | head -n 3 || true
  zig version 2>/dev/null || true
  tree-sitter --version 2>/dev/null || true
  hb-shape --version 2>/dev/null || true
  fc-match --version 2>/dev/null || true
  glslc --version 2>/dev/null | head -n 3 || true
  spirv-val --version 2>/dev/null | head -n 5 || true
  rg --version 2>/dev/null | head -n 2 || true
  fd --version 2>/dev/null || true
  jq --version 2>/dev/null || true
  bash --version 2>/dev/null | head -n 1 || true
  zsh --version 2>/dev/null || true
  fish --version 2>/dev/null || true
} > "$OUTDIR/bin/safe_versions.txt"

section "python/perl"
cap "$OUTDIR/python/braxon_python_platform.txt" "$ROOT/braxon-python" -c 'import sys,platform; print(sys.version); print(sys.executable); print(platform.platform())'
cap "$OUTDIR/python/system_python_platform.txt" python3 -c 'import sys,platform; print(sys.version); print(sys.executable); print(platform.platform())'
cap "$OUTDIR/python/pip_freeze.txt" python3 -m pip freeze
cap "$OUTDIR/perl/perl_V.txt" perl -V
cap "$OUTDIR/perl/cpan_list.txt" cpan -l

section "rust/cargo"
cap "$OUTDIR/rust/rustc_verbose.txt" "$ROOT/braxon-rustc" --version --verbose
cap "$OUTDIR/rust/cargo_verbose.txt" "$ROOT/braxon-cargo" --version --verbose
cap "$OUTDIR/rust/cargo_metadata.json" "$ROOT/braxon-cargo" metadata --format-version 1
cap "$OUTDIR/rust/cargo_metadata_no_deps.json" "$ROOT/braxon-cargo" metadata --no-deps --format-version 1
cap "$OUTDIR/rust/cargo_tree.txt" "$ROOT/braxon-cargo" tree

cat > "$OUTDIR/tmp/parse_cargo_packages.py" <<'PY'
import json
from pathlib import Path
p = Path(__file__).resolve().parents[1] / "rust" / "cargo_metadata_no_deps.json"
data = p.read_text().strip()
if not data:
    print("cargo_metadata_empty")
else:
    for pkg in json.loads(data).get("packages", []):
        print(pkg.get("name", "UNKNOWN"))
PY
"$ROOT/braxon-python" "$OUTDIR/tmp/parse_cargo_packages.py" > "$OUTDIR/cargo/package_inventory.txt" 2> "$OUTDIR/cargo/package_inventory.stderr" || true

section "zig/c/llvm/text/vulkan"
cap "$OUTDIR/zig/zig_version.txt" zig version
cap "$OUTDIR/zig/zls_version.txt" zls --version
cap "$OUTDIR/c_cpp/clang_version.txt" clang --version
cap "$OUTDIR/c_cpp/clang_dumpmachine.txt" clang -dumpmachine
cap "$OUTDIR/tree_sitter/tree_sitter_version.txt" tree-sitter --version
cap "$OUTDIR/fonts/fc_match_version.txt" fc-match --version
cap "$OUTDIR/fonts/harfbuzz_version.txt" hb-shape --version
cap "$OUTDIR/vulkan/glslc_version.txt" glslc --version
cap "$OUTDIR/vulkan/spirv_val_version.txt" spirv-val --version
cap "$OUTDIR/vulkan/vulkaninfo.txt" vulkaninfo

section "pkg-config/includes/libs"
pkg-config --list-all > "$OUTDIR/pkg_config/list_all.txt" 2>&1 || true
fd --type file --max-depth 4 . /data/data/com.termux/files/usr/include "$TC/adoption/include" 2>/dev/null | sort > "$OUTDIR/includes/header_inventory.txt"
{
  fd --type file --max-depth 2 -e so -e a -e pc . /data/data/com.termux/files/usr/lib 2>/dev/null
  fd --type file --max-depth 3 -e so -e a -e pc . "$TC/install/braxon_android_overlay" 2>/dev/null
  fd --type file --max-depth 3 -e so -e a -e pc . "$TC/install/python" 2>/dev/null
  fd --type file --max-depth 3 -e so -e a -e pc . "$TC/install/rustup-mend" 2>/dev/null
} | sort -u > "$OUTDIR/libs/library_inventory.txt"

section "repo/state topology strains"
rg --files crates tests --hidden -g '!.git' -g '!target' > "$OUTDIR/topology/repo_source_files.txt" 2>/dev/null || true
rg --files state --hidden -g '!.git' -g '!target' > "$OUTDIR/state_registry/state_files.txt" 2>/dev/null || true
rg --files state/full_android_language_toolchain --hidden -g '!.git' -g '!target' > "$OUTDIR/state_registry/toolchain_state_files.txt" 2>/dev/null || true

section "source manifests"
rg --files crates tests --hidden -g 'Cargo.toml' -g '*.rs' -g '*.toml' -g '*.json' -g '*.sh' > "$OUTDIR/manifests/repo_source_manifest.txt" 2>/dev/null || true
rg --files state --hidden -g '*.sh' -g '*.json' -g '*.toml' -g '*.rs' -g '*.dax' > "$OUTDIR/manifests/state_source_manifest.txt" 2>/dev/null || true

section "git/locks"
cap "$OUTDIR/repos/git_status_full.txt" git status
cap "$OUTDIR/repos/git_status_short.txt" git status --short
cap "$OUTDIR/repos/git_branches.txt" git branch -a
cap "$OUTDIR/repos/git_log_200.txt" git log --oneline -n 200
fd --type file . "$TC/locks" 2>/dev/null | sort > "$OUTDIR/locks/lock_inventory.txt"

section "bounded hashes, j${JOBS}"
{
  cat "$OUTDIR/manifests/repo_source_manifest.txt"
  cat "$OUTDIR/manifests/state_source_manifest.txt"
} | sort -u \
  | xargs -r -P "$JOBS" sha256sum \
  > "$OUTDIR/hash/source_hashes.sha256" 2>"$OUTDIR/hash/source_hashes.stderr" || true

section "tests"
cap "$OUTDIR/tests/fastest_status.txt" "$ROOT/fastest_status"
cap "$OUTDIR/tests/verify_zig_text_stack.txt" "$ROOT/scripts/verify_braxon_zig_text_stack.sh"
cap "$OUTDIR/tests/cargo_check_nsq_core.txt" "$ROOT/braxon-cargo" check -p nsq-core
cap "$OUTDIR/tests/cargo_check_braxon_core.txt" "$ROOT/braxon-cargo" check -p Braxon-core
cap "$OUTDIR/tests/cargo_check_braxon_ingest.txt" "$ROOT/braxon-cargo" check -p Braxon-ingest

section "summary"
{
  echo "Braxon full system state"
  echo "date: $(date)"
  echo "outdir: $OUTDIR"
  echo "jobs: $JOBS"
  echo
  echo "installed package count: $(wc -l < "$OUTDIR/packages/package_names.txt")"
  echo "package file map count: $(fd --type file . "$OUTDIR/package_files" | wc -l)"
  echo "executable count: $(wc -l < "$OUTDIR/bin/executable_inventory.txt")"
  echo "pkg-config module count: $(wc -l < "$OUTDIR/pkg_config/list_all.txt")"
  echo "header count: $(wc -l < "$OUTDIR/includes/header_inventory.txt")"
  echo "library/pkgconfig file count: $(wc -l < "$OUTDIR/libs/library_inventory.txt")"
  echo "repo source file count: $(wc -l < "$OUTDIR/topology/repo_source_files.txt")"
  echo "state file count: $(wc -l < "$OUTDIR/state_registry/state_files.txt")"
  echo "toolchain state file count: $(wc -l < "$OUTDIR/state_registry/toolchain_state_files.txt")"
  echo "cargo package count: $(grep -v '^$' "$OUTDIR/cargo/package_inventory.txt" | wc -l)"
} > "$OUTDIR/SUMMARY.txt"

section "compression"
cd "$HOME/storage/shared/Download"
tar -czf "$ARCHIVE" "braxon_full_system_state_${STAMP}"

section "complete"
cat "$OUTDIR/SUMMARY.txt"
echo "archive: $ARCHIVE"
echo "BRAXON FULL SYSTEM EXPORT COMPLETE"
