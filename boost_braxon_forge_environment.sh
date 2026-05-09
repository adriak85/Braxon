#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$HOME/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
OUT="$TC/boost_braxon_forge_environment_$(date +%Y%m%d_%H%M%S).log"
JOBS="${JOBS:-7}"

mkdir -p "$TC"/{locks,tmp,reports}

{
  echo "=== Braxon forge environment boost ==="
  date

  cd "$ROOT"

  export LD_LIBRARY_PATH="$TC/install/braxon_android_overlay/lib:/data/data/com.termux/files/usr/lib"
  export PATH="$ROOT:$TC/install/python/bin:/data/data/com.termux/files/usr/opt/rust-nightly/bin:/data/data/com.termux/files/usr/bin:$HOME/.cargo/bin:$HOME/.local/bin:$PATH"

  echo
  echo "=== install robust forge packages ==="
  pkg update -y

  pkg install -y \
    clang lld llvm binutils cmake ninja make pkg-config \
    git gh curl wget rsync rclone jq yq ripgrep fd findutils file tree \
    tar gzip xz-utils zstd unzip zip patch diffutils coreutils moreutils \
    perl ruby nodejs-lts golang lua54 guile \
    openjdk-21 gradle kotlin android-tools aapt apksigner dx zipalign \
    zig zls tree-sitter tree-sitter-parsers \
    harfbuzz harfbuzz-utils fontconfig fontconfig-utils freetype libpng \
    vulkan-loader vulkan-headers vulkan-tools shaderc spirv-tools \
    man mandoc manpages apropos \
    zsh fish bash-completion fzf less bat micro vim nano \
    proot fakeroot tsu \
    htop hyperfine strace ltrace \
    || true

  echo
  echo "=== command proof ==="
  for x in \
    braxon-python braxon-rustc braxon-cargo \
    clang ld.lld llvm-ar llvm-ranlib cmake ninja make pkg-config \
    rustc cargo zig zls tree-sitter \
    python3 perl ruby node npm go lua guile \
    java javac gradle kotlinc kotlin aapt apksigner dx zipalign \
    rg fd jq yq git gh curl wget rsync rclone \
    tar gzip xz zstd unzip zip patch diff file tree \
    hb-shape fc-match glslc spirv-val vulkaninfo \
    man apropos zsh fish fzf bat micro vim \
    proot fakeroot tsu htop hyperfine strace ltrace
  do
    printf "%-16s " "$x"
    command -v "$x" || true
  done

  echo
  echo "=== fast version proof ==="
  "$ROOT/braxon-python" --version || true
  "$ROOT/braxon-rustc" --version || true
  "$ROOT/braxon-cargo" --version || true
  clang --version | head -n 3 || true
  zig version || true
  java -version || true
  gradle --version | head -n 20 || true
  guile --version | head -n 5 || true
  hyperfine --version || true

  echo
  echo "=== write forge env ==="
  cat > "$ROOT/braxon-forge-env" <<EOF
export ROOT="$ROOT"
export TC="$TC"
export JOBS="7"
export LD_LIBRARY_PATH="$TC/install/braxon_android_overlay/lib:/data/data/com.termux/files/usr/lib"
export PATH="$ROOT:$TC/install/python/bin:/data/data/com.termux/files/usr/opt/rust-nightly/bin:/data/data/com.termux/files/usr/bin:\$HOME/.cargo/bin:\$HOME/.local/bin:\$PATH"
export PKG_CONFIG_PATH="/data/data/com.termux/files/usr/lib/pkgconfig:/data/data/com.termux/files/usr/share/pkgconfig:\${PKG_CONFIG_PATH:-}"
export CC="/data/data/com.termux/files/usr/bin/clang"
export CXX="/data/data/com.termux/files/usr/bin/clang++"
export LD="/data/data/com.termux/files/usr/bin/ld.lld"
export AR="/data/data/com.termux/files/usr/bin/llvm-ar"
export RANLIB="/data/data/com.termux/files/usr/bin/llvm-ranlib"
EOF
  chmod +x "$ROOT/braxon-forge-env"

  echo
  echo "=== smoke proofs ==="
  source "$ROOT/braxon-forge-env"
  "$ROOT/fastest_status" || true
  "$ROOT/scripts/verify_braxon_zig_text_stack.sh" || true
  "$ROOT/scripts/verify_braxon_terminal_forge_lane.sh" || true

  echo
  echo "=== lock forge environment ==="
  LOCKDIR="$TC/locks/braxon_forge_environment"
  mkdir -p "$LOCKDIR"

  {
    echo "BRAXON_FORGE_ENVIRONMENT_LOCK=1"
    date
    echo "JOBS=$JOBS"
    "$ROOT/braxon-python" --version || true
    "$ROOT/braxon-rustc" --version --verbose || true
    "$ROOT/braxon-cargo" --version --verbose || true
    clang --version | head -n 3 || true
    zig version || true
    java -version || true
    gradle --version | head -n 20 || true
    guile --version | head -n 5 || true
  } > "$LOCKDIR/LOCKED_BRAXON_FORGE_ENVIRONMENT.txt"

  find "$ROOT/braxon-forge-env" "$ROOT/fastest_status" "$ROOT/scripts" \
    -maxdepth 2 -type f -print0 2>/dev/null \
    | sort -z | xargs -0 sha256sum \
    > "$LOCKDIR/manifest.sha256"

  echo
  echo "DONE"
  echo "env: $ROOT/braxon-forge-env"
  echo "log: $OUT"
  echo "lock: $LOCKDIR/LOCKED_BRAXON_FORGE_ENVIRONMENT.txt"
} 2>&1 | tee "$OUT"

ln -sf "$OUT" "$TC/boost_braxon_forge_environment_latest.log"
