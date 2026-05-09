#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$HOME/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
SRC="$TC/source_forge"
NSQ="$SRC/nsq_forge"
GUILE="$SRC/guile_nsq_logic"
OUT="$TC/stage_guile_nsq_doc_logic_lane_$(date +%Y%m%d_%H%M%S).log"
JOBS="${JOBS:-7}"

mkdir -p "$GUILE"/{src,build,install,docs,tools,reports,locks,tmp}

{
  cd "$ROOT"
  source "$SRC/source_forge_env" 2>/dev/null || true
  source "$NSQ/config/nsq_source_forge.env" 2>/dev/null || true

  export JOBS="$JOBS"
  export BRAXON_GUILE_NSQ_LOGIC="$GUILE"

  echo "=== Braxon Guile + NSQ doc/logic staging lane ==="
  date
  echo "JOBS=$JOBS"
  echo "GUILE=$GUILE"

  echo
  echo "=== try bootstrap Guile packages ==="
  pkg search '^guile$|guile' > "$GUILE/reports/pkg_search_guile.txt" 2>&1 || true
  pkg install -y guile guile-dev || pkg install -y guile || true

  echo
  echo "=== Guile probe ==="
  {
    command -v guile || true
    guile --version || true
  } | tee "$GUILE/reports/guile_probe.txt"

  echo
  echo "=== write NSQ Guile knowledge base ==="
  cat > "$GUILE/tools/nsq-stamp-logic.scm" <<'EOF'
;; Braxon NSQ Guile logic scaffold.
;; This is advisory documentation/logic only until full stamp completion is proven.

(define nsq-watermark "BRAXON_NSQ_BASE8_LEVER_SCALE_GE220000_PROVEN225370_V1")
(define nsq-active-lever-floor 220000)
(define nsq-proven-effective-positions 225370)
(define nsq-legacy-reference-1126-only #t)
(define nsq-not-u8 #t)
(define nsq-not-bytes #t)

(define core-laws
  '("do no harm"
    "respect user privacy"
    "respect user agency"
    "support user goals"
    "fail closed on false proof"
    "preserve source-first build lanes"
    "do not fake hot-live state"
    "state registry is first-class"
    "NSQ is the bus"
    "court is compositor/internal machine component"))

(define resolver-strategies
  '("current_config_path"
    "tool_config_path"
    "pkg_config_path"
    "overlay_include_path"
    "adoption_include_path"
    "dereferenced_integrated_prefix"
    "copied_native_header_prefix"
    "patched_sysconfig_or_metadata"
    "env_override_flags"
    "generated_config_shim"))

(define (print-list title xs)
  (display title) (newline)
  (for-each
    (lambda (x)
      (display " - ") (display x) (newline))
    xs)
  (newline))

(define (nsq-status)
  (display "NSQ Guile logic scaffold") (newline)
  (display "watermark: ") (display nsq-watermark) (newline)
  (display "active lever floor: ") (display nsq-active-lever-floor) (newline)
  (display "proven effective positions: ") (display nsq-proven-effective-positions) (newline)
  (display "legacy 1126 only: ") (display nsq-legacy-reference-1126-only) (newline)
  (display "not u8: ") (display nsq-not-u8) (newline)
  (display "not bytes: ") (display nsq-not-bytes) (newline)
  (newline)
  (print-list "core laws:" core-laws)
  (print-list "resolver strategies:" resolver-strategies))

(define (suggest-stamp-lane purpose)
  (display "STAMP SUGGESTION SCAFFOLD") (newline)
  (display "purpose: ") (display purpose) (newline)
  (display "status: advisory only until stamp corpus is complete/proven") (newline)
  (display "required proof: watermark + source path + verifier + lock + manifest") (newline)
  (display "default strain: j7") (newline))

(nsq-status)
EOF

  echo
  echo "=== write doc policy: wait for Guile before final NSQ docs ==="
  cat > "$GUILE/docs/NSQ_DOC_STAGING_POLICY.md" <<'EOF'
# NSQ Documentation Staging Policy

Final NSQ documentation should wait until the Guile logic lane is present and verified.

Why:
- Guile can carry symbolic documentation logic.
- Guile can suggest stamp lanes after stamp surfaces are scanned/proven.
- Documentation should describe the proven system, not freeze an incomplete guess.
- Current Guile lane is advisory only until stamp corpus completion is proven.

Current status:
- NSQ watermark is staged.
- Source-first policy is staged.
- Guile logic scaffold is staged.
- Stamp suggestion is scaffolded, not final authority.

Promotion rule:
1. Complete/prove stamp scan.
2. Generate Guile-assisted stamp suggestions.
3. Verify against NSQ laws and repo facts.
4. Write final NSQ docs.
5. Lock docs with manifest.
EOF

  echo
  echo "=== run Guile scaffold if available ==="
  if command -v guile >/dev/null 2>&1; then
    guile "$GUILE/tools/nsq-stamp-logic.scm" \
      > "$GUILE/reports/nsq_guile_logic_run.txt" 2>&1 || true
    cat "$GUILE/reports/nsq_guile_logic_run.txt"
  else
    echo "guile unavailable; source-build lane required next" | tee "$GUILE/reports/guile_missing.txt"
  fi

  echo
  echo "=== write verifier ==="
  cat > "$ROOT/scripts/verify_braxon_guile_nsq_doc_logic_lane.sh" <<'EOF'
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
EOF
  chmod +x "$ROOT/scripts/verify_braxon_guile_nsq_doc_logic_lane.sh"
  "$ROOT/scripts/verify_braxon_guile_nsq_doc_logic_lane.sh"

  echo
  echo "=== lock lane ==="
  {
    echo "BRAXON_GUILE_NSQ_DOC_LOGIC_LANE_LOCK=1"
    date
    echo "JOBS=$JOBS"
    echo "GUILE=$GUILE"
    command -v guile || true
    guile --version || true
    sha256sum "$GUILE/docs/NSQ_DOC_STAGING_POLICY.md" "$GUILE/tools/nsq-stamp-logic.scm"
  } > "$GUILE/locks/LOCKED_GUILE_NSQ_DOC_LOGIC_LANE.txt"

  find "$GUILE" "$ROOT/scripts/verify_braxon_guile_nsq_doc_logic_lane.sh" \
    -type f -print0 2>/dev/null | sort -z | xargs -0 sha256sum \
    > "$GUILE/locks/manifest.sha256"

  echo
  echo "DONE"
  echo "guile lane: $GUILE"
  echo "log: $OUT"
  echo "lock: $GUILE/locks/LOCKED_GUILE_NSQ_DOC_LOGIC_LANE.txt"
} 2>&1 | tee "$OUT"

ln -sf "$OUT" "$TC/stage_guile_nsq_doc_logic_lane_latest.log"
