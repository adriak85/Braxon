#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$HOME/Braxon"
cd "$ROOT"

echo "=== rename branch to source wording ==="
git branch -m braxton-source-systems 2>/dev/null || true

echo "=== unstage broad accidental adds only ==="
git restore --staged scripts 2>/dev/null || true
git restore --staged state 2>/dev/null || true
git restore --staged .gitignore 2>/dev/null || true

echo "=== rename packaged wording ==="
find packaged -type f -print0 2>/dev/null | xargs -0 sed -i \
  -e 's/Braxton Systems/Braxton Source Systems/g' \
  -e 's/Braxon Systems/Braxon Source Systems/g' \
  -e 's/braxton-systems/braxton-source-systems/g' \
  -e 's/BRAXTON_SYSTEMS/BRAXTON_SOURCE_SYSTEMS/g' \
  -e 's/BRAXON_SYSTEMS/BRAXON_SOURCE_SYSTEMS/g'

if [ -f packaged/identity/BRAXTON_SYSTEMS_IDENTITY.md ]; then
  mv packaged/identity/BRAXTON_SYSTEMS_IDENTITY.md packaged/identity/BRAXTON_SOURCE_SYSTEMS_IDENTITY.md
fi

echo "=== fix mandoc hardlinks in build AND install rules ==="
M="$(find "$ROOT/state/full_android_language_toolchain/source_forge/mandoc_apropos_logic/build" -maxdepth 1 -type d -name 'mandoc-*' 2>/dev/null | sort | tail -n 1 || true)"

if [ -n "$M" ] && [ -f "$M/Makefile" ]; then
  cd "$M"
  cp -f Makefile "Makefile.before_source_no_hardlinks_$(date +%Y%m%d_%H%M%S)"

  perl -0pi -e 's/\bLN=ln -f\b/LN=cp -f/g' Makefile Makefile.local 2>/dev/null || true
  perl -0pi -e 's/\$\(LN\) mandoc man/cp -f mandoc man/g' Makefile Makefile.local 2>/dev/null || true
  perl -0pi -e 's/ln -f mandoc man/cp -f mandoc man/g' Makefile Makefile.local 2>/dev/null || true
  perl -0pi -e 's/cd ([^\n]+?) && ln -f mandoc man/cd $1 \&\& cp -f mandoc man/g' Makefile Makefile.local 2>/dev/null || true
  perl -0pi -e 's/cd ([^\n]+?) && ln -f mandoc apropos/cd $1 \&\& cp -f mandoc apropos/g' Makefile Makefile.local 2>/dev/null || true
  perl -0pi -e 's/cd ([^\n]+?) && ln -f mandoc whatis/cd $1 \&\& cp -f mandoc whatis/g' Makefile Makefile.local 2>/dev/null || true
  perl -0pi -e 's/cd ([^\n]+?) && ln -f mandoc makewhatis/cd $1 \&\& cp -f mandoc makewhatis/g' Makefile Makefile.local 2>/dev/null || true

  echo "remaining mandoc hardlink lines:"
  grep -n 'ln -f mandoc\|$(LN) mandoc' Makefile Makefile.local 2>/dev/null || true

  make -j "${JOBS:-7}"
  make install
fi

cd "$ROOT"

echo "=== rebuild source package manifests ==="
find packaged -type f ! -path '*/tarballs/*' | sort > packaged/manifests/package_file_list.txt
find packaged -type f ! -path '*/tarballs/*' -print0 | sort -z | xargs -0 sha256sum > packaged/manifests/package_manifest.sha256

STAMP="$(date +%Y%m%d_%H%M%S)"
tar -czf "packaged/tarballs/braxton-source-systems-review-staging-$STAMP.tar.gz" -C "$ROOT" packaged
sha256sum "packaged/tarballs/braxton-source-systems-review-staging-$STAMP.tar.gz" > "packaged/tarballs/braxton-source-systems-review-staging-$STAMP.tar.gz.sha256"

echo "=== stage only source package surfaces ==="
git add \
  build_source_guile_nsq_lane.sh \
  build_source_mandoc_apropos_lane.sh \
  probe_gpm_guile_extension_lane.sh \
  fix_mandoc_android_hardlink_resume.sh \
  packaged

git status --short
