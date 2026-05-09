#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$HOME/Braxon"
REPORT="$ROOT/export_full_braxon_system_state.sh"

cat > "$REPORT" <<'REPORTSH'
#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$HOME/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
STAMP="$(date +%Y%m%d_%H%M%S)"
OUTDIR="$HOME/storage/shared/Download/braxon_full_system_state_${STAMP}"
ARCHIVE="$HOME/storage/shared/Download/braxon_full_system_state_${STAMP}.tar.gz"
LOG="$OUTDIR/MASTER_LOG.txt"

mkdir -p "$OUTDIR"/{system,env,packages,package_files,bin,python,perl,ruby,node,go,lua,java,rust,zig,c_cpp,tree_sitter,fonts,vulkan,pkg_config,includes,libs,topology,cargo,repos,locks,logs,tests,hash,manifests,language_surface,tmp}

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
export LD_LIBRARY_PATH="$TC/install/braxon_android_overlay/lib:/data/data/com.termux/files/usr/lib:${LD_LIBRARY_PATH:-}"
export PKG_CONFIG_PATH="/data/data/com.termux/files/usr/lib/pkgconfig:/data/data/com.termux/files/usr/share/pkgconfig:${PKG_CONFIG_PATH:-}"

env | sort > "$OUTDIR/env/full_environment.txt"
printf '%s\n' "$PATH" > "$OUTDIR/env/PATH.txt"
printf '%s\n' "$LD_LIBRARY_PATH" > "$OUTDIR/env/LD_LIBRARY_PATH.txt"
printf '%s\n' "$PKG_CONFIG_PATH" > "$OUTDIR/env/PKG_CONFIG_PATH.txt"

section "android/system"
cap "$OUTDIR/system/getprop.txt" getprop
cap "$OUTDIR/system/uname.txt" uname -a
cat /proc/version > "$OUTDIR/system/proc_version.txt" 2>/dev/null || uname -v > "$OUTDIR/system/proc_version.txt" 2>/dev/null || echo proc_version_permission_denied > "$OUTDIR/system/proc_version.txt"
cap "$OUTDIR/system/cpuinfo.txt" cat /proc/cpuinfo
cap "$OUTDIR/system/meminfo.txt" cat /proc/meminfo
cap "$OUTDIR/system/mounts.txt" mount
cap "$OUTDIR/system/df_h.txt" df -h
cap "$OUTDIR/system/processes.txt" ps -A
cap "$OUTDIR/system/termux_info.txt" termux-info

section "package inventories"
cap "$OUTDIR/packages/pkg_list_installed.txt" pkg list-installed
cap "$OUTDIR/packages/apt_list_installed.txt" apt list --installed
cap "$OUTDIR/packages/pkg_upgradable.txt" apt list --upgradable
cap "$OUTDIR/packages/dpkg_list.txt" dpkg -l
cap "$OUTDIR/packages/dpkg_selections.txt" dpkg --get-selections

section "full package file maps"
pkg list-installed | cut -d/ -f1 | sort > "$OUTDIR/packages/package_names.txt"
while read -r pkgname; do
  [ -n "$pkgname" ] || continue
  safe="$(printf '%s' "$pkgname" | tr '/:' '__')"
  pkg files "$pkgname" > "$OUTDIR/package_files/${safe}.files.txt" 2>&1 || true
done < "$OUTDIR/packages/package_names.txt"

section "executable inventories"
find /data/data/com.termux/files/usr/bin "$HOME/.cargo/bin" "$HOME/.local/bin" "$ROOT" \
  -maxdepth 1 -type f -perm -111 -print 2>/dev/null | sort > "$OUTDIR/bin/executable_inventory.txt"

COMMANDS=(
  braxon-python braxon-rustc braxon-cargo fastest_status
  python python3 pip pip3 python-config python3-config
  perl cpan prove perldoc
  ruby gem bundle bundler rake irb
  node npm npx yarn pnpm bun deno
  go gopls
  lua luajit luarocks
  java javac jar kotlinc kotlin gradle
  rustc cargo rustdoc rustfmt clippy-driver rustup
  clang clang++ cc c++ gcc g++ cpp ld ld.lld lld llvm-ar llvm-ranlib llvm-config llvm-nm llvm-objdump llvm-readelf llvm-strip ar ranlib nm objdump readelf strip
  zig zls
  cmake ctest ninja make meson pkg-config autoconf automake libtool m4
  git gh curl wget openssl ssh scp rsync tar xz gzip bzip2 zstd unzip zip patch diff sed awk gawk grep rg fd find file sha256sum md5sum
  tree-sitter
  hb-shape hb-view fc-match fc-list
  glslc spirv-as spirv-val spirv-dis spirv-opt vulkaninfo
  sqlite3 jq yq
  bash sh zsh fish dash
)

: > "$OUTDIR/bin/command_surface.txt"
for x in "${COMMANDS[@]}"; do
  {
    echo "=== $x ==="
    command -v "$x" || true
    if command -v "$x" >/dev/null 2>&1; then
      file "$(command -v "$x")" 2>/dev/null || true
      "$x" --version 2>/dev/null | head -n 20 || true
      "$x" -version 2>/dev/null | head -n 20 || true
      "$x" -V 2>/dev/null | head -n 20 || true
    fi
    echo
  } >> "$OUTDIR/bin/command_surface.txt"
done

section "python"
cap "$OUTDIR/python/braxon_python_platform.txt" "$ROOT/braxon-python" -c 'import sys,platform,sysconfig,json; print(sys.version); print(sys.executable); print(platform.platform()); print(json.dumps(sysconfig.get_paths(), indent=2))'
cap "$OUTDIR/python/system_python_platform.txt" python3 -c 'import sys,platform,sysconfig,json; print(sys.version); print(sys.executable); print(platform.platform()); print(json.dumps(sysconfig.get_paths(), indent=2))'
cap "$OUTDIR/python/pip_freeze.txt" python3 -m pip freeze
find "$TC/install/python" /data/data/com.termux/files/usr/lib/python* -maxdepth 5 -type f 2>/dev/null | sort > "$OUTDIR/python/python_file_inventory.txt"

section "perl"
cap "$OUTDIR/perl/perl_V.txt" perl -V
cap "$OUTDIR/perl/perl_INC.txt" perl -e 'print join("\n", @INC), "\n"'
cap "$OUTDIR/perl/cpan_list.txt" cpan -l
cap "$OUTDIR/perl/prove_version.txt" prove --version
find /data/data/com.termux/files/usr/lib/perl5 "$HOME/perl5" -maxdepth 5 -type f 2>/dev/null | sort > "$OUTDIR/perl/perl_file_inventory.txt"

section "ruby/node/go/lua/java"
cap "$OUTDIR/ruby/gem_env.txt" gem env
cap "$OUTDIR/ruby/gem_list.txt" gem list
cap "$OUTDIR/node/npm_global_list.txt" npm list -g --depth=0
cap "$OUTDIR/go/go_env.txt" go env
cap "$OUTDIR/lua/luarocks_list.txt" luarocks list
cap "$OUTDIR/java/java_version.txt" java -version
cap "$OUTDIR/java/javac_version.txt" javac -version

section "rust/cargo"
cap "$OUTDIR/rust/rustc_verbose.txt" "$ROOT/braxon-rustc" --version --verbose
cap "$OUTDIR/rust/cargo_verbose.txt" "$ROOT/braxon-cargo" --version --verbose
cap "$OUTDIR/rust/cargo_metadata.json" "$ROOT/braxon-cargo" metadata --format-version 1
cap "$OUTDIR/rust/cargo_metadata_no_deps.json" "$ROOT/braxon-cargo" metadata --no-deps --format-version 1
cap "$OUTDIR/rust/cargo_tree.txt" "$ROOT/braxon-cargo" tree
cap "$OUTDIR/rust/cargo_tree_all_features.txt" "$ROOT/braxon-cargo" tree --all-features
cap "$OUTDIR/rust/cargo_installed_bins.txt" cargo install --list

cat > "$OUTDIR/tmp/parse_cargo_packages.py" <<'PY'
import json
from pathlib import Path
p = Path(__file__).resolve().parents[1] / "rust" / "cargo_metadata_no_deps.json"
data = p.read_text().strip()
if not data:
    print("cargo_metadata_empty")
else:
    j = json.loads(data)
    for pkg in j.get("packages", []):
        print(pkg.get("name", "UNKNOWN"))
PY
"$ROOT/braxon-python" "$OUTDIR/tmp/parse_cargo_packages.py" > "$OUTDIR/cargo/package_inventory.txt" 2> "$OUTDIR/cargo/package_inventory.stderr" || true

section "zig/c_cpp/build systems"
cap "$OUTDIR/zig/zig_env.txt" zig env
cap "$OUTDIR/zig/zig_version.txt" zig version
cap "$OUTDIR/zig/zls_version.txt" zls --version
cap "$OUTDIR/c_cpp/clang_version.txt" clang --version
cap "$OUTDIR/c_cpp/clang_dumpmachine.txt" clang -dumpmachine
cap "$OUTDIR/c_cpp/clang_resource_dir.txt" clang -print-resource-dir
cap "$OUTDIR/c_cpp/llvm_config_all.txt" llvm-config --all
cap "$OUTDIR/c_cpp/linker_version.txt" ld.lld --version
cap "$OUTDIR/c_cpp/cmake_version.txt" cmake --version
cap "$OUTDIR/c_cpp/ninja_version.txt" ninja --version
cap "$OUTDIR/c_cpp/make_version.txt" make --version

section "headers/libs/pkg-config"
find /data/data/com.termux/files/usr/include "$TC/install" "$TC/adoption/include" -maxdepth 4 -type f 2>/dev/null | sort > "$OUTDIR/includes/header_inventory.txt"
find /data/data/com.termux/files/usr/lib "$TC/install" -maxdepth 4 -type f \( -name '*.so' -o -name '*.a' -o -name '*.pc' \) 2>/dev/null | sort > "$OUTDIR/libs/library_inventory.txt"
pkg-config --list-all > "$OUTDIR/pkg_config/list_all.txt" 2>&1 || true
while read -r mod rest; do
  [ -n "${mod:-}" ] || continue
  {
    echo "=== $mod ==="
    pkg-config --modversion "$mod" 2>/dev/null || true
    pkg-config --cflags "$mod" 2>/dev/null || true
    pkg-config --libs "$mod" 2>/dev/null || true
    echo
  } >> "$OUTDIR/pkg_config/module_details.txt"
done < "$OUTDIR/pkg_config/list_all.txt"

section "tree-sitter/text/vulkan"
cap "$OUTDIR/tree_sitter/tree_sitter_version.txt" tree-sitter --version
cap "$OUTDIR/tree_sitter/parser_package_files.txt" bash -lc 'for p in $(pkg list-installed | cut -d/ -f1 | grep ^tree-sitter); do echo ===$p===; pkg files $p; done'
cap "$OUTDIR/fonts/fc_match_version.txt" fc-match --version
cap "$OUTDIR/fonts/fc_list.txt" fc-list
cap "$OUTDIR/fonts/harfbuzz_version.txt" hb-shape --version
cap "$OUTDIR/vulkan/vulkaninfo.txt" vulkaninfo
cap "$OUTDIR/vulkan/glslc_version.txt" glslc --version
cap "$OUTDIR/vulkan/spirv_val_version.txt" spirv-val --version

section "repo topology/git/locks"
find "$ROOT" -maxdepth 5 -type d 2>/dev/null | sort > "$OUTDIR/topology/directory_tree_depth5.txt"
find "$ROOT" -maxdepth 5 -type f 2>/dev/null | sort > "$OUTDIR/topology/file_tree_depth5.txt"
cap "$OUTDIR/repos/git_status_full.txt" git status
cap "$OUTDIR/repos/git_status_short.txt" git status --short
cap "$OUTDIR/repos/git_branches.txt" git branch -a
cap "$OUTDIR/repos/git_remotes.txt" git remote -v
cap "$OUTDIR/repos/git_log_200.txt" git log --oneline -n 200
find "$TC/locks" -type f 2>/dev/null | sort > "$OUTDIR/locks/lock_inventory.txt"
cp -r "$TC/locks" "$OUTDIR/locks/full_locks" 2>/dev/null || true

section "source manifests/hashes"
find "$ROOT" -name Cargo.toml 2>/dev/null | sort > "$OUTDIR/manifests/cargo_toml_inventory.txt"
find "$ROOT" -type f \( -name '*.rs' -o -name '*.toml' -o -name '*.py' -o -name '*.pl' -o -name '*.pm' -o -name '*.zig' -o -name '*.sh' -o -name '*.json' -o -name '*.dax' \) \
  -print0 2>/dev/null | sort -z | xargs -0 sha256sum > "$OUTDIR/hash/source_hashes.sha256"

section "tests"
cap "$OUTDIR/tests/fastest_status.txt" "$ROOT/fastest_status"
cap "$OUTDIR/tests/verify_zig_text_stack.txt" "$ROOT/scripts/verify_braxon_zig_text_stack.sh"
cap "$OUTDIR/tests/cargo_check_nsq_core.txt" "$ROOT/braxon-cargo" check -p nsq-core
cap "$OUTDIR/tests/cargo_check_braxon_core.txt" "$ROOT/braxon-cargo" check -p Braxon-core
cap "$OUTDIR/tests/cargo_check_braxon_ingest.txt" "$ROOT/braxon-cargo" check -p Braxon-ingest

section "final summary"
{
  echo "Braxon full system state"
  echo "date: $(date)"
  echo "outdir: $OUTDIR"
  echo
  echo "installed package count: $(wc -l < "$OUTDIR/packages/package_names.txt")"
  echo "executable count: $(wc -l < "$OUTDIR/bin/executable_inventory.txt")"
  echo "pkg-config module count: $(wc -l < "$OUTDIR/pkg_config/list_all.txt")"
  echo "header count: $(wc -l < "$OUTDIR/includes/header_inventory.txt")"
  echo "library/pkgconfig file count: $(wc -l < "$OUTDIR/libs/library_inventory.txt")"
  echo "cargo package count: $(grep -v '^$' "$OUTDIR/cargo/package_inventory.txt" | wc -l)"
} > "$OUTDIR/SUMMARY.txt"

section "compression"
cd "$HOME/storage/shared/Download"
tar -czf "$ARCHIVE" "braxon_full_system_state_${STAMP}"

section "complete"
cat "$OUTDIR/SUMMARY.txt"
echo "archive: $ARCHIVE"
echo "BRAXON FULL SYSTEM EXPORT COMPLETE"
REPORTSH

chmod +x "$REPORT"
bash -n "$REPORT"

echo "Repaired deep report:"
echo "$REPORT"
