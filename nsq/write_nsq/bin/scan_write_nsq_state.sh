#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-${ROOT:-$HOME/Braxon}}"
OUT="$ROOT/nsq/write_nsq/reports/write_nsq_state_$(date +%Y%m%d_%H%M%S).txt"
mkdir -p "$(dirname "$OUT")"

{
  echo "NSQ write_nsq/docs/tokenizer state scan v3"
  echo "root=$ROOT"
  echo

  echo "== required registries =="
  for p in \
    "$ROOT/nsq/write_nsq/databases/authoring_registry.db" \
    "$ROOT/nsq/write_nsq/databases/doc_emit_registry.db" \
    "$ROOT/nsq/write_nsq/databases/tokenizer_emit_registry.db" \
    "$ROOT/nsq/write_nsq/databases/package_db_binding_registry.db"
  do
    if [ -f "$p" ]; then
      echo "present $p"
    else
      echo "missing $p"
    fi
  done

  echo
  echo "== package-db binding =="
  if [ -f "$ROOT/nsq/write_nsq/databases/package_db_binding_registry.db" ]; then
    cat "$ROOT/nsq/write_nsq/databases/package_db_binding_registry.db"
  fi
} > "$OUT"

echo "$OUT"
