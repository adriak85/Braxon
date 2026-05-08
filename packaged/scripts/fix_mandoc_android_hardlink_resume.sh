#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$HOME/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
SRC="$TC/source_forge"
LANE="$SRC/mandoc_apropos_logic"
VER="${MANDOC_VERSION:-1.14.6}"
BUILD="$LANE/build/mandoc-$VER"
PREFIX="$SRC/install/mandoc-$VER"
OUT="$TC/fix_mandoc_android_hardlink_resume_$(date +%Y%m%d_%H%M%S).log"
JOBS="${JOBS:-7}"

{
  echo "=== fix mandoc Android hardlink issue ==="
  date
  cd "$BUILD"

  echo "=== backup Makefile ==="
  cp -f Makefile "Makefile.before_android_no_hardlinks_$(date +%Y%m%d_%H%M%S)"

  echo "=== replace hard-link commands with copies/symlinks ==="
  perl -0pi -e 's/\bln -f mandoc man\b/cp -f mandoc man/g' Makefile
  perl -0pi -e 's/\bln -f mandoc apropos\b/cp -f mandoc apropos/g' Makefile
  perl -0pi -e 's/\bln -f mandoc whatis\b/cp -f mandoc whatis/g' Makefile
  perl -0pi -e 's/\bln -f mandoc makewhatis\b/cp -f mandoc makewhatis/g' Makefile

  echo "=== verify patched link rules ==="
  grep -n 'mandoc man\|mandoc apropos\|mandoc whatis\|mandoc makewhatis\|ln -f mandoc' Makefile || true

  echo "=== resume build j$JOBS ==="
  make -j "$JOBS"

  echo "=== install ==="
  make install

  echo "=== ensure command aliases exist ==="
  mkdir -p "$PREFIX/bin"
  for x in man apropos whatis makewhatis mandocdb; do
    if [ ! -e "$PREFIX/bin/$x" ] && [ -x "$PREFIX/bin/mandoc" ]; then
      ln -sf mandoc "$PREFIX/bin/$x" 2>/dev/null || cp -f "$PREFIX/bin/mandoc" "$PREFIX/bin/$x"
    fi
  done

  echo "=== rebuild Braxon man database ==="
  mkdir -p "$PREFIX/share/man/man7"

  cat > "$PREFIX/share/man/man7/braxon-source-first.7" <<'EOF'
.Dd May 8, 2026
.Dt BRAXON-SOURCE-FIRST 7
.Os Braxon
.Sh NAME
.Nm braxon-source-first
.Nd Braxon source-first forge policy
.Sh DESCRIPTION
Braxon prefers source-built lanes over package-manager binaries when practical.
Package-manager tools are bootstrap or fallback surfaces.
The default phone-local strain is j7.
State registry surfaces are first-class build and proof surfaces.
EOF

  cat > "$PREFIX/share/man/man7/nsq-law.7" <<'EOF'
.Dd May 8, 2026
.Dt NSQ-LAW 7
.Os Braxon
.Sh NAME
.Nm nsq-law
.Nd NSQ base-eight and watermark law
.Sh DESCRIPTION
NSQ is the lowest substrate and bus in the Braxon system.
It is not u8, not bytes, and not host-width truth.
The active watermark is BRAXON_NSQ_BASE8_LEVER_SCALE_GE220000_PROVEN225370_V1.
Legacy 1126 and 2254 references are legacy references unless explicitly marked.
EOF

  "$PREFIX/bin/makewhatis" "$PREFIX/share/man" > "$LANE/reports/source_makewhatis_after_hardlink_fix.txt" 2>&1 || true

  echo "=== proof ==="
  "$PREFIX/bin/mandoc" -V || true
  "$PREFIX/bin/apropos" -M "$PREFIX/share/man" braxon || true
  "$PREFIX/bin/apropos" -M "$PREFIX/share/man" nsq || true

  echo "=== lock ==="
  {
    echo "BRAXON_MANDOC_ANDROID_HARDLINK_FIX_LOCK=1"
    date
    echo "PREFIX=$PREFIX"
    "$PREFIX/bin/mandoc" -V || true
    "$PREFIX/bin/apropos" -M "$PREFIX/share/man" braxon || true
    "$PREFIX/bin/apropos" -M "$PREFIX/share/man" nsq || true
  } > "$LANE/locks/LOCKED_MANDOC_ANDROID_HARDLINK_FIX.txt"

  find "$PREFIX/bin" "$PREFIX/share/man" "$LANE/locks/LOCKED_MANDOC_ANDROID_HARDLINK_FIX.txt" \
    -type f -print0 2>/dev/null | sort -z | xargs -0 sha256sum \
    > "$LANE/locks/hardlink_fix_manifest.sha256"

  echo "DONE"
  echo "mandoc: $PREFIX/bin/mandoc"
  echo "apropos: $PREFIX/bin/apropos"
  echo "log: $OUT"
} 2>&1 | tee "$OUT"
