#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
cd "$ROOT"

echo "== build and run NSQASM stamp database scanner =="
cargo run -p nsqasm-stamp-db --release -- "$ROOT"

echo
echo "== verify accepted stamp database exists =="
test -s state/nsq/stamp_build_chain/candidates.jsonl
test -s state/nsq/stamp_build_chain/accepted.jsonl
test -s state/nsq/stamp_build_chain/scanner_report.txt

echo "PASS: stamp database files exist"

echo
echo "== accepted stamp count =="
wc -l state/nsq/stamp_build_chain/accepted.jsonl

echo
echo "== sample accepted stamps =="
sed -n '1,5p' state/nsq/stamp_build_chain/accepted.jsonl
