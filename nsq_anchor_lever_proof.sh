#!/usr/bin/env bash

BIN=./target/release/Braxon
STAMP=$(date +%Y%m%d_%H%M%S)
OUT=state/reports/nsq_anchor_lever_proof_${STAMP}.txt

mkdir -p state/reports

{
  echo "=== NSQ ANCHOR / LEVER SUBSTRATE PROOF ==="
  echo "STAMP: $STAMP"
  echo

  echo "--- 1. RUNTIME IDENTITY ---"
  $BIN --version 2>&1 || true
  echo

  echo "--- 2. NSQ TRACE MODE ---"
  export NSQ_TRACE=1
  echo "NSQ_TRACE=1 enabled"
  echo

  echo "--- 3. ANCHOR / LEVER STREAM TEST ---"
  echo "INPUT STREAM: '+1101 -1101 +1001 -1001'"
  $BIN nsq decode '+1101 -1101 +1001 -1001' 2>&1 || true
  echo

  echo "--- 4. SWITCH POSITION PROGRESSION TEST ---"
  for pos in 1 537 1126; do
    echo "---- POSITION TEST: $pos ----"
    $BIN nsq set-lever "$pos" 2>&1 || true
    $BIN nsq read-lever 2>&1 || true
    echo
  done

  echo "--- 5. ANCHOR/LEVER SEQUENCING ASSERTION ---"
  echo "EXPECTED PATTERN: A L A L A L (anchor then lever alternating)"
  echo "VERIFY TRACE OUTPUT ABOVE FOR STRICT INTERLEAVING"
  echo

  echo "--- 6. SUBSTRATE INTEGRITY CHECK ---"
  grep -RniE "bit|byte|u8|u16|u32|u64|pack|unpack|to_bits|from_bits" \
    crates src 2>/dev/null | head -50 || true
  echo

} | tee "$OUT"

cp "$OUT" ~/storage/shared/Download/ 2>/dev/null || true

echo
echo "REPORT: $OUT"
echo "DOWNLOAD COPY: ~/storage/shared/Download/$(basename "$OUT")"
