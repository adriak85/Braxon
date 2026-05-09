#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$HOME/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
SRC="$TC/source_forge"
LANE="$SRC/gpm_guile_extension_probe"
OUT="$TC/probe_gpm_guile_extension_lane_$(date +%Y%m%d_%H%M%S).log"
JOBS="${JOBS:-7}"

mkdir -p "$LANE"/{reports,docs,locks,tmp}

{
  cd "$ROOT"
  source "$SRC/source_forge_env" 2>/dev/null || true
  source "$SRC/guile_nsq_logic/guile_nsq_env" 2>/dev/null || true
  source "$SRC/mandoc_apropos_logic/mandoc_apropos_env" 2>/dev/null || true

  echo "=== Braxon GPM / Guile extension probe ==="
  date
  echo "JOBS=$JOBS"

  echo
  echo "=== package search ==="
  pkg search '^gpm$|gpm|guile|g-golf|guix|guild|gperf|gmp|gcrypt|gnutls|readline|sqlite|json|fibers' \
    > "$LANE/reports/pkg_search_gpm_guile.txt" 2>&1 || true

  echo
  echo "=== command probes ==="
  for x in gpm gpm-root guile guild guile-config guix gperf pkg-config apropos man makewhatis; do
    printf "%-16s " "$x"
    command -v "$x" || true
  done | tee "$LANE/reports/command_probe.txt"

  echo
  echo "=== guile module probe ==="
  if command -v guile >/dev/null 2>&1; then
    guile -c '
      (display "guile ok")(newline)
      (for-each
        (lambda (m)
          (display "probe module ")
          (write m)
          (display ": ")
          (catch #t
            (lambda () (resolve-module m) (display "yes"))
            (lambda args (display "no")))
          (newline))
        (quote ((ice-9 readline)
                (json)
                (sqlite3)
                (git)
                (gcrypt)
                (gnutls)
                (fibers)
                (g-golf)
                (gpm))))
    ' > "$LANE/reports/guile_module_probe.txt" 2>&1 || true
  else
    echo "guile missing" > "$LANE/reports/guile_module_probe.txt"
  fi

  echo
  echo "=== docs/apropos probe ==="
  {
    apropos gpm || true
    apropos guile || true
    apropos guild || true
    apropos gperf || true
  } > "$LANE/reports/apropos_probe.txt" 2>&1 || true

  echo
  echo "=== write interpretation note ==="
  cat > "$LANE/docs/GPM_GUILE_INTERPRETATION.md" <<'EOF'
# GPM / Guile Extension Interpretation

Do not assume what GPM means until the probe proves it.

Possible meanings:
- gpm: Linux console mouse daemon. Useful only if Android/Termux exposes the required console/input control. It should not be treated as guaranteed app-level control.
- gmp: GNU multiple precision math library. Already relevant to Guile and build math.
- gperf: perfect hash generator. Useful for parser/table/compiler support.
- guile module/plugin path: useful for NSQ symbolic logic, stamp suggestion, docs, and resolver intelligence.

Preferred Braxon path:
1. Probe package and command existence.
2. Probe Guile modules.
3. Stage source build only for proven-useful missing pieces.
4. Add NSQ stamp-suggestion logic after stamp corpus scan/proof.
5. Do not claim Android control changes unless verified by a real permission/control proof.
EOF

  echo
  echo "=== write verifier ==="
  cat > "$ROOT/scripts/verify_gpm_guile_extension_probe.sh" <<'EOF'
#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$HOME/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
LANE="$TC/source_forge/gpm_guile_extension_probe"

test -f "$LANE/reports/pkg_search_gpm_guile.txt"
test -f "$LANE/reports/command_probe.txt"
test -f "$LANE/reports/guile_module_probe.txt"
test -f "$LANE/docs/GPM_GUILE_INTERPRETATION.md"

grep -q "Do not assume what GPM means" "$LANE/docs/GPM_GUILE_INTERPRETATION.md"

echo "BRAXON GPM GUILE EXTENSION PROBE VERIFY OK"
EOF
  chmod +x "$ROOT/scripts/verify_gpm_guile_extension_probe.sh"
  "$ROOT/scripts/verify_gpm_guile_extension_probe.sh"

  echo
  echo "=== lock ==="
  {
    echo "BRAXON_GPM_GUILE_EXTENSION_PROBE_LOCK=1"
    date
    echo "JOBS=$JOBS"
    command -v guile || true
    guile --version || true
    command -v gpm || true
    command -v gperf || true
  } > "$LANE/locks/LOCKED_GPM_GUILE_EXTENSION_PROBE.txt"

  find "$LANE" "$ROOT/scripts/verify_gpm_guile_extension_probe.sh" \
    -type f -print0 | sort -z | xargs -0 sha256sum \
    > "$LANE/locks/manifest.sha256"

  echo
  echo "DONE"
  echo "lane: $LANE"
  echo "log: $OUT"
} 2>&1 | tee "$OUT"

ln -sf "$OUT" "$TC/probe_gpm_guile_extension_lane_latest.log"
