#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${ROOT:-$HOME/Braxon}"
OUT="$ROOT/nsq/flag_coverage/reports/flag_coverage_state_$(date +%Y%m%d_%H%M%S).txt"
mkdir -p "$(dirname "$OUT")"

{
  echo "NSQ flag coverage state scan"
  echo "root=$ROOT"
  echo

  echo "== known language/platform families =="
  for p in \
    "$ROOT/nsq/language_capture_tree/nsq" \
    "$ROOT/nsq/language_capture_tree/rust" \
    "$ROOT/nsq/language_capture_tree/python" \
    "$ROOT/nsq/language_capture_tree/c" \
    "$ROOT/nsq/language_capture_tree/sql" \
    "$ROOT/nsq/language_capture_tree/bash" \
    "$ROOT/nsq/language_capture_tree/lua" \
    "$ROOT/nsq/language_capture_tree/ruby" \
    "$ROOT/nsq/language_capture_tree/perl" \
    "$ROOT/nsq/language_capture_tree/guile_lisp" \
    "$ROOT/nsq/language_capture_tree/xml_manifest_style" \
    "$ROOT/nsq/language_capture_tree/toml" \
    "$ROOT/nsq/language_capture_tree/yaml" \
    "$ROOT/nsq/language_capture_tree/json" \
    "$ROOT/nsq/language_capture_tree/java" \
    "$ROOT/nsq/language_capture_tree/kotlin" \
    "$ROOT/nsq/language_capture_tree/html" \
    "$ROOT/nsq/language_capture_tree/css" \
    "$ROOT/nsq/language_capture_tree/javascript" \
    "$ROOT/nsq/language_capture_tree/typescript" \
    "$ROOT/nsq/language_capture_tree/bevy_surface" \
    "$ROOT/nsq/language_capture_tree/wgpu_surface" \
    "$ROOT/nsq/language_capture_tree/egui_surface"
  do
    if [ -d "$p" ]; then
      echo "present $p"
    else
      echo "missing $p"
    fi
  done

  echo
  echo "== runtime/court/proof surfaces =="
  for p in \
    "$ROOT/crates/nsq-runtime" \
    "$ROOT/crates/nsq-decode" \
    "$ROOT/crates/nsq-proof" \
    "$ROOT/crates/nsq-court" \
    "$ROOT/crates/Braxon-court" \
    "$ROOT/crates/nsq-source" \
    "$ROOT/crates/nsq-ir" \
    "$ROOT/crates/nsq-pack"
  do
    if [ -d "$p" ]; then
      echo "present $p"
    else
      echo "missing $p"
    fi
  done

  echo
  echo "== obvious flag/target/projection terms =="
  if command -v rg >/dev/null 2>&1; then
    rg -n \
      -e 'flag' \
      -e 'target' \
      -e 'offline' \
      -e 'mirror' \
      -e 'cache' \
      -e 'projection' \
      -e 'translate' \
      "$ROOT/crates" \
      "$ROOT/config" \
      "$ROOT/nsq" \
      2>/dev/null || true
  fi
} > "$OUT"

echo "$OUT"
