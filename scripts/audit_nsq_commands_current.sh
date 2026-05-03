#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail
ROOT="${1:-$HOME/Braxon}"
OUT="$HOME/storage/shared/Download/nsq_command_audit_$(date +%Y%m%d_%H%M%S).txt"

{
  echo "NSQ COMMAND AUDIT"
  echo "root=$ROOT"
  echo

  echo "== visible nsq commands =="
  compgen -c | rg '^nsq([-/].*|.*)$' | sort -u || true
  echo

  echo "== resolution + help =="
  while read -r cmd; do
    [ -n "$cmd" ] || continue
    echo "--- $cmd ---"
    type -a "$cmd" || true
    resolved="$(command -v "$cmd" 2>/dev/null || true)"
    [ -n "$resolved" ] && printf 'path=%s\n' "$resolved"
    "$cmd" --help 2>&1 | sed -n '1,24p' || true
    echo
  done < <(compgen -c | rg '^nsq([-/].*|.*)$' | sort -u)
  echo

  echo "== repo safety markers =="
  rg -n -S \
    -e 'legacy compile path' \
    -e 'disabled:' \
    -e 'lowers canonical NSQ' \
    -e 'derived machine forms' \
    -e 'quarantine' \
    -e 'nsq-proof-run' \
    -e 'nsq-pressure-bench' \
    -e 'nsq-native-bench' \
    -e 'nsq-real-bench' \
    -e 'nsq-bench' \
    "$ROOT" || true
} > "$OUT"

echo "wrote=$OUT"
sed -n '1,260p' "$OUT"
