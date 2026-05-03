#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-${ROOT:-$HOME/Braxon}}"
OUT="$ROOT/nsq/runtime_native/reports/runtime_native_state_$(date +%Y%m%d_%H%M%S).txt"
mkdir -p "$(dirname "$OUT")"

{
  echo "NSQ runtime-native state scan v3"
  echo "root=$ROOT"
  echo

  echo "== required runtime domains =="
  for p in \
    "$ROOT/nsq/runtime_native/databases/runtime_domain_registry.db" \
    "$ROOT/nsq/runtime_native/databases/graded_selector_registry.db" \
    "$ROOT/nsq/runtime_native/databases/local_package_repo_registry.db" \
    "$ROOT/nsq/runtime_native/databases/package_db_multiport_registry.db" \
    "$ROOT/nsq/runtime_native/databases/human_machine_doc_registry.db" \
    "$ROOT/nsq/runtime_native/databases/tokenizer_bridge_registry.db"
  do
    if [ -f "$p" ]; then
      echo "present $p"
    else
      echo "missing $p"
    fi
  done

  echo
  echo "== multiport package-db config =="
  if [ -f "$ROOT/config/nsq/runtime_native/package_db/multiport_package_db.json" ]; then
    cat "$ROOT/config/nsq/runtime_native/package_db/multiport_package_db.json"
  else
    echo "missing multiport_package_db.json"
  fi
} > "$OUT"

echo "$OUT"
