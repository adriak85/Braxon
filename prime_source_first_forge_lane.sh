#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$HOME/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
SRC="$TC/source_forge"
OUT="$TC/prime_source_first_forge_lane_$(date +%Y%m%d_%H%M%S).log"
JOBS="${JOBS:-7}"

mkdir -p "$SRC"/{downloads,build,install,logs,locks,manifests,tmp}

{
  cd "$ROOT"
  source "$ROOT/braxon-forge-env" 2>/dev/null || true

  echo "=== Braxon source-first forge lane ==="
  date
  echo "JOBS=$JOBS"

  cat > "$SRC/SOURCE_FIRST_POLICY.md" <<'EOF'
# Braxon Source-First Forge Policy

Use package-manager tools as bootstrap only.

Preferred order:
1. Build from source into the Braxon state registry.
2. Verify with command, version, compile smoke, and hash manifest.
3. Promote into Braxon env path only after proof.
4. Keep Termux package binaries as fallback.
5. Do not replace working proven host tools blindly.
6. Use j7 as the default phone-local concurrency.
7. Build small/core dependencies first, then larger stacks.
8. Keep every source-built lane isolated, lockable, and reproducible.

Target source lanes:
- shell/help: zsh, fish, guile, mandoc/man helpers
- build tools: cmake, ninja, pkgconf where practical
- language tools: Zig, Tree-sitter, Lua, Perl modules, Ruby gems where practical
- Android lane: Gradle/AGP/cargo-apk bridge later
- graphics/text: HarfBuzz, FreeType, Fontconfig later if needed
EOF

  cat > "$SRC/source_forge_env" <<EOF
export BRAXON_SOURCE_FIRST=1
export ROOT="$ROOT"
export TC="$TC"
export BRAXON_SOURCE_FORGE="$SRC"
export JOBS="7"
export PREFIX="$SRC/install"
export PATH="$SRC/install/bin:$ROOT:$TC/install/python/bin:/data/data/com.termux/files/usr/opt/rust-nightly/bin:/data/data/com.termux/files/usr/bin:\$HOME/.cargo/bin:\$HOME/.local/bin:\$PATH"
export LD_LIBRARY_PATH="$SRC/install/lib:$TC/install/braxon_android_overlay/lib:/data/data/com.termux/files/usr/lib"
export PKG_CONFIG_PATH="$SRC/install/lib/pkgconfig:$SRC/install/share/pkgconfig:/data/data/com.termux/files/usr/lib/pkgconfig:/data/data/com.termux/files/usr/share/pkgconfig"
export CC="/data/data/com.termux/files/usr/bin/clang"
export CXX="/data/data/com.termux/files/usr/bin/clang++"
export LD="/data/data/com.termux/files/usr/bin/ld.lld"
export AR="/data/data/com.termux/files/usr/bin/llvm-ar"
export RANLIB="/data/data/com.termux/files/usr/bin/llvm-ranlib"
EOF

  chmod +x "$SRC/source_forge_env"

  echo
  echo "=== source-first inventory ==="
  {
    echo "bootstrap tools:"
    for x in git curl wget tar gzip xz unzip patch clang clang++ ld.lld cmake ninja make pkg-config rustc cargo zig guile perl ruby node npm go lua; do
      printf "%-14s " "$x"
      command -v "$x" || true
    done
  } | tee "$SRC/manifests/bootstrap_tools.txt"

  echo
  echo "=== write source lane verifier ==="
  cat > "$ROOT/scripts/verify_braxon_source_first_forge_lane.sh" <<'EOF'
#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$HOME/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
SRC="$TC/source_forge"

test -f "$SRC/SOURCE_FIRST_POLICY.md"
test -x "$SRC/source_forge_env"

source "$SRC/source_forge_env"

echo "=== verify Braxon source-first forge lane ==="
echo "BRAXON_SOURCE_FIRST=$BRAXON_SOURCE_FIRST"
echo "BRAXON_SOURCE_FORGE=$BRAXON_SOURCE_FORGE"
echo "PREFIX=$PREFIX"
echo "JOBS=$JOBS"

command -v clang
command -v git
command -v curl
command -v make
command -v cmake
command -v ninja
command -v pkg-config

echo "BRAXON SOURCE-FIRST FORGE LANE VERIFY OK"
EOF

  chmod +x "$ROOT/scripts/verify_braxon_source_first_forge_lane.sh"
  "$ROOT/scripts/verify_braxon_source_first_forge_lane.sh"

  echo
  echo "=== lock source-first lane ==="
  {
    echo "BRAXON_SOURCE_FIRST_FORGE_LANE_LOCK=1"
    date
    echo "SRC=$SRC"
    echo "JOBS=$JOBS"
    sha256sum "$SRC/SOURCE_FIRST_POLICY.md" "$SRC/source_forge_env"
  } > "$SRC/locks/LOCKED_SOURCE_FIRST_FORGE_LANE.txt"

  find "$SRC" "$ROOT/scripts/verify_braxon_source_first_forge_lane.sh" \
    -type f -print0 2>/dev/null | sort -z | xargs -0 sha256sum \
    > "$SRC/locks/manifest.sha256"

  echo
  echo "DONE"
  echo "source forge: $SRC"
  echo "env: $SRC/source_forge_env"
  echo "log: $OUT"
} 2>&1 | tee "$OUT"

ln -sf "$OUT" "$TC/prime_source_first_forge_lane_latest.log"
