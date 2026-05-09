#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
cd "$ROOT"

echo "== verify NSQ Court start proof manifest =="
test -f config/substrate/nsq_court_start_proof.json
grep -q '"authority": "NSQ_COURT"' config/substrate/nsq_court_start_proof.json
grep -q '"requires_no_libc": true' config/substrate/nsq_court_start_proof.json
grep -q '"requires_no_dynamic_section": true' config/substrate/nsq_court_start_proof.json
grep -q '"runtime_claim": "substrate_start_proof_only_not_full_runtime"' config/substrate/nsq_court_start_proof.json

echo "PASS: manifest is present and fail-closed"

echo
asm/nsq_court_start/build.sh "$ROOT"

echo
asm/nsq_court_start/verify.sh "$ROOT"

echo
echo "PASS: NSQ Court substrate start proof complete"
