#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
cd "$ROOT"

OUT_DIR="$ROOT/state/substrate/nsq_court_start"
BIN="$OUT_DIR/nsq_court_start"
REPORT="$OUT_DIR/nsq_court_start_verify_$(date +%Y%m%d_%H%M%S).txt"
DYNAMIC_CHECK="$OUT_DIR/nsq_court_start_dynamic_check_$(date +%Y%m%d_%H%M%S).txt"

echo "== verify NSQ Court start proof =="

test -x "$BIN"

set +e
OUTPUT="$("$BIN" 2>&1)"
STATUS="$?"
set -e

{
  echo "date=$(date -Is)"
  echo "binary=$BIN"
  echo "exit_status=$STATUS"
  echo "output=$OUTPUT"
  echo

  echo "== readelf header =="
  readelf -h "$BIN"
  echo

  echo "== readelf sections =="
  readelf -S "$BIN"
  echo

  echo "== readelf program headers =="
  readelf -l "$BIN"
  echo

  echo "== readelf dynamic check =="
  set +e
  readelf -d "$BIN" > "$DYNAMIC_CHECK" 2>&1
  READelf_DYNAMIC_STATUS="$?"
  set -e
  cat "$DYNAMIC_CHECK"
  echo "readelf_dynamic_status=$READelf_DYNAMIC_STATUS"
  echo

  echo "== symbols =="
  llvm-nm "$BIN" || true
  echo

  echo "== disassembly =="
  llvm-objdump -d "$BIN"
} | tee "$REPORT"

echo
echo "== assertions =="

if [ "$OUTPUT" != "NSQ_COURT_START_PROOF_OK" ]; then
  echo "FAIL: unexpected output: $OUTPUT"
  exit 1
fi

if [ "$STATUS" != "37" ]; then
  echo "FAIL: unexpected exit status: $STATUS"
  exit 1
fi

if grep -q 'Dynamic section at offset' "$DYNAMIC_CHECK"; then
  echo "FAIL: binary has dynamic section; this proof must stay no-libc/no-dynamic"
  exit 1
fi

if readelf -l "$BIN" | grep -q 'INTERP'; then
  echo "FAIL: binary has interpreter segment; this proof must stay no-libc/no-dynamic"
  exit 1
fi

grep -q '_start' "$REPORT"
grep -q 'svc' "$REPORT"

echo "PASS: NSQ Court no-libc assembly start proof verified"
echo "Report: $REPORT"
