#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$HOME/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
SRC="$TC/source_forge"
NSQ="$SRC/nsq_forge"

test -f "$SRC/SOURCE_FIRST_POLICY.md"
test -x "$SRC/source_forge_env"
test -f "$NSQ/config/NSQ_SOURCE_FORGE_POLICY.md"
test -x "$NSQ/config/nsq_source_forge.env"
test -x "$NSQ/tools/nsq_forge_status.sh"
test -x "$NSQ/tools/nsq_forge_scan.sh"

source "$SRC/source_forge_env"
source "$NSQ/config/nsq_source_forge.env"

test "$BRAXON_NSQ_WATERMARK" = "BRAXON_NSQ_BASE8_LEVER_SCALE_GE220000_PROVEN225370_V1"
test "$BRAXON_NSQ_ACTIVE_LEVER_FLOOR" = "220000"
test "$BRAXON_NSQ_PROVEN_EFFECTIVE_POSITIONS" = "225370"
test "$BRAXON_NSQ_LEGACY_REFERENCE_1126_ONLY" = "1"
test "$BRAXON_NSQ_NOT_U8" = "1"
test "$BRAXON_NSQ_NOT_BYTES" = "1"

"$NSQ/tools/nsq_forge_status.sh" > "$NSQ/proofs/status_verify.txt"
"$NSQ/tools/nsq_forge_scan.sh" > "$NSQ/proofs/scan_verify.txt"

"$ROOT/braxon-cargo" test -p nsq-core -- --nocapture
"$ROOT/braxon-cargo" test -p Braxon-core -- --nocapture
"$ROOT/braxon-cargo" test -p Braxon-ingest -- --nocapture

echo "BRAXON SOURCE-FIRST NSQ FORGE LANE VERIFY OK"
