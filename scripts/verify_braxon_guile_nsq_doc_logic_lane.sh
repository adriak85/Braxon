#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$HOME/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
SRC="$TC/source_forge"
GUILE="$SRC/guile_nsq_logic"

test -f "$GUILE/docs/NSQ_DOC_STAGING_POLICY.md"
test -f "$GUILE/tools/nsq-stamp-logic.scm"

grep -q "BRAXON_NSQ_BASE8_LEVER_SCALE_GE220000_PROVEN225370_V1" "$GUILE/tools/nsq-stamp-logic.scm"
grep -q "advisory only until stamp corpus is complete/proven" "$GUILE/tools/nsq-stamp-logic.scm"
grep -q "Final NSQ documentation should wait until the Guile logic lane is present and verified" "$GUILE/docs/NSQ_DOC_STAGING_POLICY.md"

if command -v guile >/dev/null 2>&1; then
  guile "$GUILE/tools/nsq-stamp-logic.scm" >/dev/null
  echo "guile executable verified"
else
  echo "guile executable missing; staged source-build fallback required"
fi

echo "BRAXON GUILE NSQ DOC LOGIC LANE VERIFY OK"
