#!/usr/bin/env bash
set -euo pipefail

cd /data/data/com.termux/files/home/Braxon

OUT="state/reports/root_entry_review_$(date +%Y%m%d_%H%M%S).txt"
mkdir -p "$(dirname "$OUT")"

{
  echo "=== Braxon root entry review ==="
  date
  echo

  for p in "./--help" "./status" "./criots" "./crisp, exact, integrity-bound,"; do
    echo "### $p"
    if [ -e "$p" ]; then
      ls -la "$p"
      file "$p" 2>/dev/null || true
      sha256sum "$p" || true
      echo "--- full content ---"
      sed -n '1,240p' "$p" 2>/dev/null || true
      echo "--- references in repo by exact path ---"
      grep -RInF --exclude-dir=.git --exclude-dir=target -- "$p" . 2>/dev/null | head -50 || true
      echo "--- references in repo by basename ---"
      bn="${p#./}"
      grep -RInF --exclude-dir=.git --exclude-dir=target -- "$bn" . 2>/dev/null | head -50 || true
    else
      echo "missing"
    fi
    echo
  done

  echo "=== NSQ triple reference scan ==="
  grep -RInE --exclude-dir=.git --exclude-dir=target \
    'triple repo\.core -> has -> nsq\.(source|compile|inspect)' . 2>/dev/null | head -100 || true

} | tee "$OUT"

echo
echo "review=$OUT"
