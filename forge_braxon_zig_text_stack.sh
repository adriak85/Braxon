#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
OUT="$TC/forge_braxon_zig_text_stack_$(date +%Y%m%d_%H%M%S).log"
LOCKDIR="$TC/locks/braxon_zig_text_stack"
ENVFILE="$ROOT/braxon-text-env"
VERIFY="$ROOT/scripts/verify_braxon_zig_text_stack.sh"

mkdir -p "$TC/tmp" "$LOCKDIR" "$ROOT/scripts"

{
  echo "=== Braxon Zig + semantic text stack forge ==="
  date
  echo

  source "$ROOT/braxon-rust-env" 2>/dev/null || true
  source "$TC/terminal/braxon-term-1/braxon-terminal.env" 2>/dev/null || true

  export PATH="$ROOT:$TC/install/python/bin:/data/data/com.termux/files/usr/opt/rust-nightly/bin:/data/data/com.termux/files/usr/bin:$PATH"
  export LD_LIBRARY_PATH="$TC/install/braxon_android_overlay/lib:/data/data/com.termux/files/usr/lib:${LD_LIBRARY_PATH:-}"

  echo "=== proven anchors ==="
  "$ROOT/braxon-python" -c 'import sys, math, _math_integer; print(sys.version); print(math.__file__); print(_math_integer.__file__)'
  "$ROOT/braxon-rustc" --version --verbose
  clang --version | head -n 3
  clang -dumpmachine
  echo

  echo "=== package search ==="
  pkg search zig || true
  pkg search tree-sitter || true
  pkg search harfbuzz || true
  pkg search freetype || true
  pkg search vulkan || true
  pkg search skia || true
  echo

  echo "=== install available forge packages ==="
  pkg install -y \
    zig \
    tree-sitter \
    harfbuzz harfbuzz-utils \
    freetype freetype-dev \
    libpng libpng-dev \
    fontconfig \
    vulkan-loader vulkan-headers \
    pkg-config clang lld cmake ninja make git file binutils \
    || true
  echo

  echo "=== command surface ==="
  for x in zig tree-sitter hb-shape hb-view fc-match pkg-config clang ld.lld cmake ninja git file; do
    printf "%-16s " "$x"
    command -v "$x" || true
  done
  echo

  echo "=== versions ==="
  zig version 2>/dev/null || true
  tree-sitter --version 2>/dev/null || true
  hb-shape --version 2>/dev/null || true
  pkg-config --modversion harfbuzz 2>/dev/null || true
  pkg-config --modversion freetype2 2>/dev/null || true
  echo

  echo "=== write Braxon text env ==="
  cat > "$ENVFILE" <<EOF
export ROOT="$ROOT"
export TC="$TC"
export PATH="$ROOT:$TC/install/python/bin:/data/data/com.termux/files/usr/opt/rust-nightly/bin:/data/data/com.termux/files/usr/bin:\$PATH"
export LD_LIBRARY_PATH="$TC/install/braxon_android_overlay/lib:/data/data/com.termux/files/usr/lib:\${LD_LIBRARY_PATH:-}"
export CC="/data/data/com.termux/files/usr/bin/clang"
export CXX="/data/data/com.termux/files/usr/bin/clang++"
export LD="/data/data/com.termux/files/usr/bin/ld.lld"
export AR="/data/data/com.termux/files/usr/bin/llvm-ar"
export RANLIB="/data/data/com.termux/files/usr/bin/llvm-ranlib"
export PKG_CONFIG_PATH="/data/data/com.termux/files/usr/lib/pkgconfig:/data/data/com.termux/files/usr/share/pkgconfig:\${PKG_CONFIG_PATH:-}"
export CFLAGS="-include $TC/adoption/include/braxon_android_posix_adoption_force.h"
export CXXFLAGS="-include $TC/adoption/include/braxon_android_posix_adoption_force.h"
export LDFLAGS="-L$TC/install/braxon_android_overlay/lib -lbraxon_android_libc_extensions"
EOF
  chmod +x "$ENVFILE"

  echo "=== Zig smoke ==="
  TMP="$TC/tmp/zig_text_probe"
  rm -rf "$TMP"
  mkdir -p "$TMP"
  cat > "$TMP/main.zig" <<'ZIG'
const std = @import("std");

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    try stdout.print("braxon zig ok: semantic text forge\n", .{});
}
ZIG

  if command -v zig >/dev/null 2>&1; then
    zig build-exe "$TMP/main.zig" -O ReleaseFast -femit-bin="$TMP/zig_probe" || true
    [ -x "$TMP/zig_probe" ] && "$TMP/zig_probe" || true
    [ -e "$TMP/zig_probe" ] && file "$TMP/zig_probe" || true
  else
    echo "zig missing"
  fi
  echo

  echo "=== HarfBuzz + FreeType compile smoke ==="
  cat > "$TMP/hb_probe.c" <<'C'
#include <stdio.h>
#include <hb.h>
#include <hb-ft.h>
#include <ft2build.h>
#include FT_FREETYPE_H

int main(void) {
    hb_buffer_t *buf = hb_buffer_create();
    hb_buffer_add_utf8(buf, "Braxon text forge", -1, 0, -1);
    hb_buffer_guess_segment_properties(buf);
    printf("harfbuzz buffer ok, glyphs=%u\n", hb_buffer_get_length(buf));
    hb_buffer_destroy(buf);

    FT_Library ft;
    if (FT_Init_FreeType(&ft) == 0) {
        printf("freetype init ok\n");
        FT_Done_FreeType(ft);
    }
    return 0;
}
C

  if pkg-config --exists harfbuzz freetype2; then
    clang "$TMP/hb_probe.c" \
      $(pkg-config --cflags harfbuzz freetype2) \
      $(pkg-config --libs harfbuzz freetype2) \
      -o "$TMP/hb_probe"
    "$TMP/hb_probe"
    file "$TMP/hb_probe" || true
  else
    echo "harfbuzz/freetype pkg-config surface missing"
  fi
  echo

  echo "=== Tree-sitter grammar probe ==="
  if command -v tree-sitter >/dev/null 2>&1; then
    tree-sitter --version
  else
    echo "tree-sitter missing"
  fi
  echo

  echo "=== create verifier ==="
  cat > "$VERIFY" <<'EOF'
#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
source "$ROOT/braxon-text-env"

echo "=== verify Braxon Zig/text stack ==="
clang -dumpmachine
"$ROOT/braxon-python" -c 'import math, _math_integer; print("python native ok")'
"$ROOT/braxon-rustc" --version --verbose

command -v zig >/dev/null && zig version || echo "zig unavailable"
command -v tree-sitter >/dev/null && tree-sitter --version || echo "tree-sitter unavailable"
pkg-config --exists harfbuzz freetype2
pkg-config --modversion harfbuzz
pkg-config --modversion freetype2

echo "BRAXON ZIG TEXT STACK VERIFY OK"
EOF
  chmod +x "$VERIFY"

  echo "=== verifier run ==="
  "$VERIFY" || true
  echo

  echo "=== lock manifest ==="
  {
    echo "BRAXON_ZIG_TEXT_STACK_LOCK=1"
    date
    command -v zig || true
    zig version 2>/dev/null || true
    command -v tree-sitter || true
    tree-sitter --version 2>/dev/null || true
    pkg-config --modversion harfbuzz 2>/dev/null || true
    pkg-config --modversion freetype2 2>/dev/null || true
    clang -dumpmachine
  } > "$LOCKDIR/LOCKED_ZIG_TEXT_STACK.txt"

  find \
    "$ENVFILE" \
    "$VERIFY" \
    /data/data/com.termux/files/usr/bin/zig \
    /data/data/com.termux/files/usr/bin/tree-sitter \
    /data/data/com.termux/files/usr/bin/hb-shape \
    /data/data/com.termux/files/usr/lib \
    -maxdepth 1 -type f -print0 2>/dev/null \
    | sort -z | xargs -0 sha256sum > "$LOCKDIR/manifest.sha256" || true

  echo "DONE"
  echo "lock: $LOCKDIR/LOCKED_ZIG_TEXT_STACK.txt"
  echo "manifest: $LOCKDIR/manifest.sha256"
  echo "env: $ENVFILE"
  echo "verify: $VERIFY"
  echo "log: $OUT"
} 2>&1 | tee "$OUT"

ln -sf "$OUT" "$TC/forge_braxon_zig_text_stack_latest.log"
