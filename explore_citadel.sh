#!/bin/bash
set -euo pipefail

echo "=== Citadel 699 Exploratory Scan ==="

cd crates

echo "1. NSQ-UNIVERSAL-FETCH structure:"
find nsq-universal-fetch -name "*.rs" -o -name "Cargo.toml" | xargs ls -la
echo ""

echo "2. Current main.rs CLI surface:"
grep -n "match|if.*args|subcommand" nsq-universal-fetch/src/main.rs || echo "No CLI pattern found"
echo ""

echo "3. NSQ-RUNTIME bit circulation:"
grep -n "nu336|circulat|reconstruct|stamp" nsq-runtime/src/ || echo "No nu336 pattern"
echo ""

echo "4. NSQ-COMPRESS NSQ encoding:"
grep -n "nsq|macro|stamp" nsq-compress/src/ || echo "No NSQ pattern"
echo ""

echo "5. Port 699 binding capability:"
grep -n "TcpListener|port|699|bind" */src/ || echo "No networking found"
echo ""

echo "6. Citadel handshake signatures:"
grep -n "NSQ_V2|CITADEL|magic|handshake" */src/ || echo "No Citadel pattern"

echo "=== Scan complete ==="
