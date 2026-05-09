#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
OUT="$TC/lock_and_extend_braxon_dev_stack_$(date +%Y%m%d_%H%M%S).log"
LOCKDIR="$TC/locks/braxon_dev_stack"
FAST="$ROOT/fastest_status"

mkdir -p "$LOCKDIR" "$TC/tmp" "$ROOT/scripts"

{
  cd "$ROOT"
  source "$ROOT/braxon-rust-env" 2>/dev/null || true

  export PATH="$ROOT:$TC/install/python/bin:/data/data/com.termux/files/usr/opt/rust-nightly/bin:/data/data/com.termux/files/usr/bin:$PATH"
  export LD_LIBRARY_PATH="$TC/install/braxon_android_overlay/lib:/data/data/com.termux/files/usr/lib:${LD_LIBRARY_PATH:-}"

  echo "=== install nearby useful dev surfaces ==="
  pkg install -y tree-sitter-parsers vulkan-tools shaderc spirv-tools fontconfig-utils harfbuzz-utils zls

  echo
  echo "=== verified command surface ==="
  for x in braxon-python braxon-rustc braxon-cargo clang zig zls tree-sitter hb-shape fc-match glslc spirv-as spirv-val vulkaninfo; do
    printf "%-16s " "$x"
    command -v "$x" || true
  done

  echo
  echo "=== versions ==="
  "$ROOT/braxon-python" -c 'import sys; print(sys.version)'
  "$ROOT/braxon-rustc" --version --verbose
  "$ROOT/braxon-cargo" --version --verbose
  clang --version | head -n 3
  zig version
  zls --version || true
  tree-sitter --version
  hb-shape --version
  fc-match --version
  glslc --version | head -n 3 || true
  spirv-val --version || true

  echo
  echo "=== shader compile smoke ==="
  TMP="$TC/tmp/braxon_shader_probe"
  rm -rf "$TMP"
  mkdir -p "$TMP"

  cat > "$TMP/probe.vert" <<'GLSL'
#version 450
layout(location = 0) in vec2 pos;
void main() {
    gl_Position = vec4(pos, 0.0, 1.0);
}
GLSL

  if command -v glslc >/dev/null 2>&1; then
    glslc "$TMP/probe.vert" -o "$TMP/probe.vert.spv"
    spirv-val "$TMP/probe.vert.spv"
    file "$TMP/probe.vert.spv"
  else
    echo "glslc unavailable"
  fi

  echo
  echo "=== write fastest_status ==="
  cat > "$FAST" <<'EOF'
#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
cd "$ROOT"
source "$ROOT/braxon-rust-env" 2>/dev/null || true

export PATH="$ROOT:$TC/install/python/bin:/data/data/com.termux/files/usr/opt/rust-nightly/bin:/data/data/com.termux/files/usr/bin:$PATH"
export LD_LIBRARY_PATH="$TC/install/braxon_android_overlay/lib:/data/data/com.termux/files/usr/lib:${LD_LIBRARY_PATH:-}"

echo "=== Braxon fastest status ==="
date
echo

echo "python:"
"$ROOT/braxon-python" -c 'import sys, math, _math_integer; print(sys.version.split()[0]); print(math.__file__); print(_math_integer.__file__)'

echo
echo "rust:"
"$ROOT/braxon-rustc" --version
"$ROOT/braxon-cargo" --version

echo
echo "text/dev:"
zig version
tree-sitter --version
hb-shape --version
fc-match --version

echo
echo "cargo packages:"
"$ROOT/braxon-cargo" metadata --no-deps --format-version 1 \
  | "$ROOT/braxon-python" -c 'import json,sys; [print(p["name"]) for p in json.load(sys.stdin)["packages"]]'

echo
echo "core tests:"
"$ROOT/braxon-cargo" test -p nsq-core -- --nocapture
"$ROOT/braxon-cargo" test -p Braxon-core -- --nocapture
"$ROOT/braxon-cargo" test -p Braxon-ingest -- --nocapture

echo
echo "BRAXON FASTEST STATUS OK"
EOF
  chmod +x "$FAST"

  echo
  echo "=== run fastest_status once ==="
  "$FAST"

  echo
  echo "=== lock dev stack ==="
  {
    echo "BRAXON_DEV_STACK_LOCK=1"
    date
    "$ROOT/braxon-python" -c 'import sys; print(sys.version)'
    "$ROOT/braxon-rustc" --version --verbose
    "$ROOT/braxon-cargo" --version --verbose
    clang --version | head -n 3
    zig version
    zls --version || true
    tree-sitter --version
    hb-shape --version
    fc-match --version
    glslc --version | head -n 3 || true
    spirv-val --version || true
  } > "$LOCKDIR/LOCKED_BRAXON_DEV_STACK.txt"

  find "$FAST" "$ROOT/scripts/verify_braxon_zig_text_stack.sh" \
    "$ROOT/braxon-python" "$ROOT/braxon-rustc" "$ROOT/braxon-cargo" \
    "$(command -v zig)" "$(command -v tree-sitter)" "$(command -v hb-shape)" "$(command -v fc-match)" \
    -type f -print0 2>/dev/null | sort -z | xargs -0 sha256sum > "$LOCKDIR/manifest.sha256"

  echo
  echo "DONE"
  echo "fast: $FAST"
  echo "log: $OUT"
  echo "lock: $LOCKDIR/LOCKED_BRAXON_DEV_STACK.txt"
} 2>&1 | tee "$OUT"

ln -sf "$OUT" "$TC/lock_and_extend_braxon_dev_stack_latest.log"
