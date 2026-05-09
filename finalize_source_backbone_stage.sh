#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$HOME/Braxon"
cd "$ROOT"

echo "=== clean stale staged package names ==="
git rm --cached -f packaged/identity/BRAXTON_SYSTEMS_IDENTITY.md 2>/dev/null || true
git rm --cached -f packaged/tarballs/braxton-systems-review-staging-20260508_142558.tar.gz.sha256 2>/dev/null || true

echo "=== fix bake-over filename typo ==="
if [ -f packaged/bake-over/BAXON_SOURCE_BAKE_OVER_PLAN.md ]; then
  mv packaged/bake-over/BAXON_SOURCE_BAKE_OVER_PLAN.md packaged/bake-over/BRAXON_SOURCE_BAKE_OVER_PLAN.md
fi

echo "=== force mandoc install linker variable away from hardlinks ==="
M="$(find "$ROOT/state/full_android_language_toolchain/source_forge/mandoc_apropos_logic/build" -maxdepth 1 -type d -name 'mandoc-*' 2>/dev/null | sort | tail -n 1 || true)"
if [ -n "$M" ] && [ -f "$M/Makefile" ]; then
  cd "$M"
  cp -f Makefile "Makefile.before_force_LN_cp_$(date +%Y%m%d_%H%M%S)"
  cp -f Makefile.local "Makefile.local.before_force_LN_cp_$(date +%Y%m%d_%H%M%S)" 2>/dev/null || true

  {
    echo
    echo '# Braxon Android/Termux: no hardlinks in app storage'
    echo 'LN=cp -f'
  } >> Makefile.local

  perl -0pi -e 's/\$\(LN\) mandoc/cp -f mandoc/g' Makefile Makefile.local 2>/dev/null || true
  perl -0pi -e 's/ln -f mandoc/cp -f mandoc/g' Makefile Makefile.local 2>/dev/null || true

  echo "remaining hardlink refs:"
  grep -n 'ln -f mandoc\|$(LN) mandoc' Makefile Makefile.local 2>/dev/null || true

  make install
fi

cd "$ROOT"

echo "=== rebuild linked-source manifests ==="
find packaged -type f ! -path '*/tarballs/*' | sort > packaged/manifests/package_file_list.txt
find packaged -maxdepth 4 -type l -exec sh -c 'for x; do printf "%s -> %s\n" "$x" "$(readlink "$x")"; done' sh {} + \
  > packaged/manifests/backbone_symlink_map.txt

find packaged -type f ! -path '*/tarballs/*' -print0 | sort -z | xargs -0 sha256sum \
  > packaged/manifests/package_manifest.sha256

STAMP="$(date +%Y%m%d_%H%M%S)"
tar -czhf "packaged/tarballs/braxton-source-systems-linked-index-$STAMP.tar.gz" \
  -C "$ROOT" \
  packaged/identity \
  packaged/review-notes \
  packaged/backbone-links \
  packaged/bake-over \
  packaged/manifests \
  packaged/scripts \
  packaged/source-lanes

sha256sum "packaged/tarballs/braxton-source-systems-linked-index-$STAMP.tar.gz" \
  > "packaged/tarballs/braxton-source-systems-linked-index-$STAMP.tar.gz.sha256"

echo "=== stage exact source-package surfaces only ==="
git add \
  build_source_guile_nsq_lane.sh \
  build_source_mandoc_apropos_lane.sh \
  probe_gpm_guile_extension_lane.sh \
  fix_mandoc_android_hardlink_resume.sh \
  packaged/identity \
  packaged/review-notes \
  packaged/backbone-links \
  packaged/bake-over \
  packaged/manifests \
  packaged/scripts \
  packaged/source-lanes \
  packaged/tarballs/*.sha256

git status --short
