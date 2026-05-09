#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
OUT="$TC/explore_braxon_perl_native_$(date +%Y%m%d_%H%M%S).log"

{
  echo "=== Braxon Perl native exploration ==="
  date
  echo

  echo "=== Braxon terminal env ==="
  if [ -f "$TC/terminal/braxon-term-1/braxon-terminal.env" ]; then
    source "$TC/terminal/braxon-term-1/braxon-terminal.env"
    echo "loaded: $TC/terminal/braxon-term-1/braxon-terminal.env"
  else
    echo "missing Braxon terminal env"
  fi
  echo "CC=${CC:-}"
  echo "CFLAGS=${CFLAGS:-}"
  echo "LDFLAGS=${LDFLAGS:-}"
  echo "PATH=$PATH"
  echo

  echo "=== compiler target ==="
  command -v clang || true
  clang --version | head -n 5 || true
  clang -dumpmachine || true
  echo

  echo "=== current Perl surface ==="
  command -v perl || true
  perl -V 2>/dev/null || true
  echo

  echo "=== Perl source candidates ==="
  find "$HOME" "$ROOT" "$TC/src" -maxdepth 4 \( \
    -iname 'perl-*' -o \
    -iname 'perl5*' -o \
    -iname 'metacpan*' -o \
    -iname '*alien*' \
  \) 2>/dev/null | sort
  echo

  echo "=== Alien/build tools ==="
  for x in cpan cpanm prove make gmake ninja cmake pkg-config ar ranlib ld.lld llvm-ar llvm-ranlib; do
    printf "%-16s " "$x"
    command -v "$x" || true
  done
  echo

  echo "=== installed Perl build modules probe ==="
  perl -MExtUtils::MakeMaker -MConfig -MFile::Spec -MIPC::Cmd -e '
    print "ExtUtils::MakeMaker OK\n";
    print "archname=$Config::Config{archname}\n";
    print "cc=$Config::Config{cc}\n";
    print "ccflags=$Config::Config{ccflags}\n";
    print "ld=$Config::Config{ld}\n";
    print "ldflags=$Config::Config{ldflags}\n";
    print "libperl=$Config::Config{libperl}\n";
  ' 2>&1 || true
  echo

  echo "=== Alien modules probe ==="
  perl -MAlien::Base -e 'print "Alien::Base OK\n"' 2>&1 || true
  perl -MAlien::Build -e 'print "Alien::Build OK\n"' 2>&1 || true
  perl -MFFI::Platypus -e 'print "FFI::Platypus OK\n"' 2>&1 || true
  echo

  echo "=== native compile smoke with Braxon overlay ==="
  TMP="$TC/tmp/perl_native_probe"
  rm -rf "$TMP"
  mkdir -p "$TMP"
  cat > "$TMP/probe.c" <<'C'
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
int main(void) {
  printf("braxon native c probe ok\n");
  printf("strlen=%zu\n", strlen("braxon"));
  return 0;
}
C
  "${CC:-clang}" ${CFLAGS:-} "$TMP/probe.c" ${LDFLAGS:-} -o "$TMP/probe"
  "$TMP/probe"
  file "$TMP/probe" || true
  echo

  echo "=== recommendation marker ==="
  echo "If this log shows Perl source present and Alien::Build usable, next step is source-build Perl nightly/dev into:"
  echo "$TC/install/perl-native"
  echo "Then lock launcher:"
  echo "$ROOT/braxon-perl"
} 2>&1 | tee "$OUT"

ln -sf "$OUT" "$TC/explore_braxon_perl_native_latest.log"
echo "log: $OUT"
