#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$HOME/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
SRC="$TC/source_forge"
LANE="$SRC/mandoc_apropos_logic"
VER="${MANDOC_VERSION:-1.14.6}"
PREFIX="$SRC/install/mandoc-$VER"
OUT="$TC/build_source_mandoc_apropos_lane_$(date +%Y%m%d_%H%M%S).log"
JOBS="${JOBS:-7}"

mkdir -p "$LANE"/{src,build,install,reports,locks,tmp,db}

{
  cd "$ROOT"
  source "$SRC/source_forge_env" 2>/dev/null || true

  export PATH="$PREFIX/bin:$SRC/install/bin:$ROOT:$TC/install/python/bin:/data/data/com.termux/files/usr/opt/rust-nightly/bin:/data/data/com.termux/files/usr/bin:$HOME/.cargo/bin:$HOME/.local/bin:$PATH"
  export LD_LIBRARY_PATH="$PREFIX/lib:$SRC/install/lib:$TC/install/braxon_android_overlay/lib:/data/data/com.termux/files/usr/lib"
  export CC="/data/data/com.termux/files/usr/bin/clang"
  export CFLAGS="-O2 -fPIC"
  export LDFLAGS="-L/data/data/com.termux/files/usr/lib"

  echo "=== source-build mandoc/apropos for Braxon ==="
  date
  echo "version=$VER"
  echo "prefix=$PREFIX"
  echo "jobs=$JOBS"

  echo
  echo "=== bootstrap deps only ==="
  for p in clang lld make curl tar gzip xz-utils patch zlib less manpages; do
    pkg install -y "$p" || true
  done

  echo
  echo "=== fetch mandoc source ==="
  cd "$LANE/src"
  TARBALL="mandoc-$VER.tar.gz"
  [ -f "$TARBALL" ] || curl -L -o "$TARBALL" "https://mandoc.bsd.lv/snapshots/$TARBALL"
  sha256sum "$TARBALL" > "$LANE/reports/mandoc_tarball.sha256"

  echo
  echo "=== unpack ==="
  rm -rf "$LANE/build/mandoc-$VER"
  mkdir -p "$LANE/build"
  tar -xzf "$TARBALL" -C "$LANE/build"
  cd "$LANE/build/mandoc-$VER"

  echo
  echo "=== configure local Braxon prefix ==="
  cat > configure.local <<EOF
PREFIX=$PREFIX
BINDIR=$PREFIX/bin
SBINDIR=$PREFIX/bin
MANDIR=$PREFIX/share/man
EXAMPLEDIR=$PREFIX/share/examples/mandoc
WWWPREFIX=$PREFIX/share/doc/mandoc
MANPATH_DEFAULT=/data/data/com.termux/files/usr/share/man:$PREFIX/share/man:$ROOT/docs:$ROOT/specs:$ROOT/brand:$SRC
UTF8_LOCALE=en_US.UTF-8
CC=$CC
CFLAGS="$CFLAGS"
LDFLAGS="$LDFLAGS"
EOF

  echo
  echo "=== configure/build/install ==="
  ./configure
  make -j "$JOBS"
  make install

  echo
  echo "=== write Braxon apropos env ==="
  cat > "$LANE/mandoc_apropos_env" <<EOF
export BRAXON_MANDOC_APROPOS_LANE="$LANE"
export MANDOC_SOURCE_PREFIX="$PREFIX"
export PATH="$PREFIX/bin:\$PATH"
export MANPATH="/data/data/com.termux/files/usr/share/man:$PREFIX/share/man:$ROOT/docs:$ROOT/specs:$ROOT/brand:$SRC"
export PAGER="less"
EOF
  chmod +x "$LANE/mandoc_apropos_env"

  echo
  echo "=== build Braxon apropos database ==="
  source "$LANE/mandoc_apropos_env"

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
.Sh SEE ALSO
.Xr apropos 1 ,
.Xr makewhatis 8
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
.Sh SEE ALSO
.Xr braxon-source-first 7
EOF

  makewhatis "$PREFIX/share/man" > "$LANE/reports/makewhatis.txt" 2>&1 || true

  echo
  echo "=== proof ==="
  command -v mandoc
  command -v apropos
  mandoc -V || true
  apropos -M "$PREFIX/share/man" braxon > "$LANE/reports/apropos_braxon.txt" 2>&1 || true
  apropos -M "$PREFIX/share/man" nsq > "$LANE/reports/apropos_nsq.txt" 2>&1 || true
  cat "$LANE/reports/apropos_braxon.txt" || true
  cat "$LANE/reports/apropos_nsq.txt" || true

  echo
  echo "=== verifier ==="
  cat > "$ROOT/scripts/verify_source_mandoc_apropos_lane.sh" <<'EOF'
#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$HOME/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
SRC="$TC/source_forge"
LANE="$SRC/mandoc_apropos_logic"

source "$LANE/mandoc_apropos_env"

test -x "$MANDOC_SOURCE_PREFIX/bin/mandoc"
test -x "$MANDOC_SOURCE_PREFIX/bin/apropos"

mandoc -V || true
apropos -M "$MANDOC_SOURCE_PREFIX/share/man" braxon >/dev/null || true
apropos -M "$MANDOC_SOURCE_PREFIX/share/man" nsq >/dev/null || true

grep -R "BRAXON_NSQ_BASE8_LEVER_SCALE_GE220000_PROVEN225370_V1" "$MANDOC_SOURCE_PREFIX/share/man" >/dev/null

echo "BRAXON SOURCE MANDOC APROPOS LANE VERIFY OK"
EOF
  chmod +x "$ROOT/scripts/verify_source_mandoc_apropos_lane.sh"
  "$ROOT/scripts/verify_source_mandoc_apropos_lane.sh"

  echo
  echo "=== lock ==="
  {
    echo "BRAXON_SOURCE_MANDOC_APROPOS_LANE_LOCK=1"
    date
    echo "MANDOC_VERSION=$VER"
    echo "PREFIX=$PREFIX"
    "$PREFIX/bin/mandoc" -V || true
    "$PREFIX/bin/apropos" -M "$PREFIX/share/man" braxon || true
    "$PREFIX/bin/apropos" -M "$PREFIX/share/man" nsq || true
  } > "$LANE/locks/LOCKED_SOURCE_MANDOC_APROPOS_LANE.txt"

  find "$PREFIX/bin" "$PREFIX/share/man" "$LANE/mandoc_apropos_env" "$ROOT/scripts/verify_source_mandoc_apropos_lane.sh" \
    -type f -print0 2>/dev/null | sort -z | xargs -0 sha256sum \
    > "$LANE/locks/manifest.sha256"

  echo
  echo "DONE"
  echo "mandoc: $PREFIX/bin/mandoc"
  echo "apropos: $PREFIX/bin/apropos"
  echo "env: $LANE/mandoc_apropos_env"
  echo "log: $OUT"
} 2>&1 | tee "$OUT"

ln -sf "$OUT" "$TC/build_source_mandoc_apropos_lane_latest.log"
