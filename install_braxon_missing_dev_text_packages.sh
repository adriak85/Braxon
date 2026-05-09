#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
OUT="$TC/install_braxon_missing_dev_text_packages_$(date +%Y%m%d_%H%M%S).log"
VERIFY="$ROOT/scripts/verify_braxon_zig_text_stack.sh"
LOCKDIR="$TC/locks/braxon_zig_text_stack"

mkdir -p "$TC/tmp" "$ROOT/scripts" "$LOCKDIR"

{
  cd "$ROOT"
  source "$ROOT/braxon-rust-env" 2>/dev/null || true

  export PATH="$ROOT:$TC/install/python/bin:/data/data/com.termux/files/usr/opt/rust-nightly/bin:/data/data/com.termux/files/usr/bin:$PATH"
  export LD_LIBRARY_PATH="$TC/install/braxon_android_overlay/lib:/data/data/com.termux/files/usr/lib:${LD_LIBRARY_PATH:-}"

  echo "=== install Termux dev/runtime packages ==="
  pkg update -y
  pkg install -y zig zls tree-sitter tree-sitter-rust tree-sitter-c tree-sitter-json tree-sitter-toml tree-sitter-yaml tree-sitter-python tree-sitter-bash tree-sitter-markdown
  pkg install -y harfbuzz harfbuzz-utils freetype libpng fontconfig vulkan-loader vulkan-headers vulkan-tools shaderc
  pkg install -y clang lld cmake ninja make pkg-config git file binutils

  echo
  echo "=== command proof ==="
  for x in zig zls tree-sitter hb-shape hb-view fc-match pkg-config clang ld.lld cmake ninja git file; do
    printf "%-16s " "$x"
    command -v "$x"
  done

  echo
  echo "=== version proof ==="
  zig version
  zls --version || true
  tree-sitter --version
  hb-shape --version
  pkg-config --modversion harfbuzz
  pkg-config --modversion freetype2
  clang -dumpmachine

  echo
  echo "=== strict verifier rewrite ==="
  cat > "$VERIFY" <<'EOF'
#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
source "$ROOT/braxon-text-env" 2>/dev/null || true
source "$ROOT/braxon-rust-env" 2>/dev/null || true

export PATH="$ROOT:$TC/install/python/bin:/data/data/com.termux/files/usr/opt/rust-nightly/bin:/data/data/com.termux/files/usr/bin:$PATH"
export LD_LIBRARY_PATH="$TC/install/braxon_android_overlay/lib:/data/data/com.termux/files/usr/lib:${LD_LIBRARY_PATH:-}"

echo "=== verify Braxon Zig/text stack ==="
clang -dumpmachine
"$ROOT/braxon-python" -c 'import math, _math_integer; print("python native ok")'
"$ROOT/braxon-rustc" --version --verbose

command -v zig
zig version

command -v tree-sitter
tree-sitter --version

command -v hb-shape
pkg-config --exists harfbuzz freetype2
pkg-config --modversion harfbuzz
pkg-config --modversion freetype2

echo "BRAXON ZIG TEXT STACK VERIFY OK"
EOF
  chmod +x "$VERIFY"

  echo
  echo "=== smoke compile Zig ==="
  TMP="$TC/tmp/zig_text_probe_strict"
  rm -rf "$TMP"
  mkdir -p "$TMP"
  cat > "$TMP/main.zig" <<'ZIG'
const std = @import("std");

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    try stdout.print("braxon zig ok: strict text stack\n", .{});
}
ZIG

  zig build-exe "$TMP/main.zig" -O ReleaseFast -femit-bin="$TMP/zig_probe"
  "$TMP/zig_probe"
  file "$TMP/zig_probe"

  echo
  echo "=== smoke compile HarfBuzz + FreeType ==="
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
    if (FT_Init_FreeType(&ft) != 0) return 2;
    printf("freetype init ok\n");
    FT_Done_FreeType(ft);
    return 0;
}
C

  clang "$TMP/hb_probe.c" \
    $(pkg-config --cflags harfbuzz freetype2) \
    $(pkg-config --libs harfbuzz freetype2) \
    -o "$TMP/hb_probe"

  "$TMP/hb_probe"
  file "$TMP/hb_probe"

  echo
  echo "=== run strict verifier ==="
  "$VERIFY"

  echo
  echo "=== refresh lock ==="
  {
    echo "BRAXON_ZIG_TEXT_STACK_LOCK=1"
    date
    command -v zig
    zig version
    command -v tree-sitter
    tree-sitter --version
    command -v hb-shape
    hb-shape --version
    pkg-config --modversion harfbuzz
    pkg-config --modversion freetype2
    clang -dumpmachine
  } > "$LOCKDIR/LOCKED_ZIG_TEXT_STACK.txt"

  find "$VERIFY" "$ROOT/braxon-text-env" \
    "$(command -v zig)" "$(command -v tree-sitter)" "$(command -v hb-shape)" \
    -type f -print0 | sort -z | xargs -0 sha256sum > "$LOCKDIR/manifest.sha256"

  echo
  echo "DONE"
  echo "log: $OUT"
  echo "verify: $VERIFY"
  echo "lock: $LOCKDIR/LOCKED_ZIG_TEXT_STACK.txt"
} 2>&1 | tee "$OUT"

ln -sf "$OUT" "$TC/install_braxon_missing_dev_text_packages_latest.log"
