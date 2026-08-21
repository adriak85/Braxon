#!/data/data/com.termux/files/usr/bin/bash
# SPDX-License-Identifier: LicenseRef-Braxon-Private
# Copyright (c) 2026 Michael David Norris. All rights reserved.
#
# Emit the sole normal-operation tool authority for Braxon. This script never
# discovers or records an ambient Termux binary as a normal tool route.
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
ROOT="$(cd "$ROOT" && pwd)"
CHAIN="$ROOT/state/full_android_language_toolchain"
INSTALL="$CHAIN/install"
OUTPUT="$INSTALL/braxon_repository_tool_dispatch.json"
TMP="$OUTPUT.tmp.$$"

fail() { printf '%s\n' "braxon-repository-tool-dispatch: $*" >&2; exit 1; }
mkdir -p "$INSTALL"

rust_root="$INSTALL/rust"
if [ -L "$INSTALL/rust-edge-active" ] && [ -d "$INSTALL/rust-edge-active" ]; then
  rust_root="$INSTALL/rust-edge-active"
fi

entry() {
  local name="$1"
  local path="$2"
  local comma="$3"
  local status="target_build_pending"
  local sha=""
  if [ -x "$path" ]; then
    status="verified_repository_built"
    sha="$(sha256sum "$path" | awk '{print $1}')"
  fi
  case "$path" in
    "$ROOT"/*) ;;
    *) fail "declared path for $name escapes the repository: $path" ;;
  esac
  printf '    {"name":"%s","path":"%s","status":"%s","sha256":"%s"}%s\n' \
    "$name" "$path" "$status" "$sha" "$comma" >> "$TMP"
}

cat > "$TMP" <<EOF
{
  "schema": "braxon.repository_tool_dispatch.v1",
  "authority": "BRAXON_REPOSITORY_BUILT_TOOL_AUTHORITY",
  "owner": "Michael David Norris",
  "target_environment": "aarch64-linux-android",
  "normal_operation_ambient_termux_fallback": false,
  "bootstrap_termux_tools_may_appear_only_in_source_build_receipts": true,
  "rust_selection": "${rust_root#$ROOT/}",
  "tools": [
EOF

entry rustc "$rust_root/bin/rustc" ,
entry cargo "$rust_root/bin/cargo" ,
entry rustdoc "$rust_root/bin/rustdoc" ,
entry rustfmt "$rust_root/bin/rustfmt" ,
entry clippy-driver "$rust_root/bin/clippy-driver" ,
entry clang "$INSTALL/llvm/bin/clang" ,
entry clang++ "$INSTALL/llvm/bin/clang++" ,
entry ld.lld "$INSTALL/llvm/bin/ld.lld" ,
entry llvm-ar "$INSTALL/llvm/bin/llvm-ar" ,
entry llvm-ranlib "$INSTALL/llvm/bin/llvm-ranlib" ,
entry llvm-nm "$INSTALL/llvm/bin/llvm-nm" ,
entry llvm-objdump "$INSTALL/llvm/bin/llvm-objdump" ,
entry llvm-readelf "$INSTALL/llvm/bin/llvm-readelf" ,
entry llvm-strip "$INSTALL/llvm/bin/llvm-strip" ,
entry python3 "$INSTALL/python/bin/python3" ,
entry guile "$INSTALL/guile/bin/guile" ,
entry zig "$INSTALL/zig/zig" ""

cat >> "$TMP" <<EOF
  ]
}
EOF
mv "$TMP" "$OUTPUT"
printf '%s\n' "repository_tool_dispatch=$OUTPUT"
