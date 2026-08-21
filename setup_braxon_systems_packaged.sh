#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$HOME/Braxon"
BRANCH="braxon-systems"
PACKAGED="$ROOT/packaged"
TC="$ROOT/state/full_android_language_toolchain"
SRC="$TC/source_forge"
STAMP="$(date +%Y%m%d_%H%M%S)"
OUT="$TC/setup_braxon_systems_packaged_$STAMP.log"
JOBS="${JOBS:-7}"

cd "$ROOT"

{
  echo "=== Braxon Systems packaged staging ==="
  date
  echo "NO pull, NO reset, NO release."
  echo "Source-first. Package manager only bootstrap."

  echo
  echo "=== branch ==="
  git status --short || true
  if git show-ref --verify --quiet "refs/heads/$BRANCH"; then
    git switch "$BRANCH"
  else
    git switch -c "$BRANCH"
  fi

  echo
  echo "=== fix Termux package names ==="
  for f in \
    "$ROOT/build_source_guile_nsq_lane.sh" \
    "$ROOT/build_source_mandoc_apropos_lane.sh"
  do
    [ -f "$f" ] || continue
    sed -i \
      -e 's/\bpkg install -y xz\b/pkg install -y xz-utils/g' \
      -e 's/\bpkg install -y gmp\b/pkg install -y libgmp/g' \
      -e 's/for p in clang lld make pkg-config curl tar gzip xz patch gawk sed grep/for p in clang lld make pkg-config curl tar gzip xz-utils patch gawk sed grep/g' \
      -e 's/for p in clang lld make curl tar gzip xz patch zlib/for p in clang lld make curl tar gzip xz-utils patch zlib/g' \
      "$f"
  done

  echo
  echo "=== fix Guile tar hardlink extraction ==="
  if [ -f "$ROOT/build_source_guile_nsq_lane.sh" ]; then
    cp -f "$ROOT/build_source_guile_nsq_lane.sh" "$ROOT/build_source_guile_nsq_lane.sh.before_hardlink_fix_$STAMP"

    python - <<'PY'
from pathlib import Path
p = Path.home() / "Braxon" / "build_source_guile_nsq_lane.sh"
s = p.read_text()
s = s.replace(
'''  tar -xzf "$TARBALL" -C "$GUILE_LANE/src"
  cp -a "$GUILE_LANE/src/guile-$VER" "$GUILE_LANE/build/guile-$VER"''',
'''  set +e
  tar --no-same-owner --no-same-permissions -xzf "$TARBALL" -C "$GUILE_LANE/src"
  TAR_RC="$?"
  set -e
  if [ ! -f "$GUILE_LANE/src/guile-$VER/configure" ]; then
    echo "Guile source did not unpack enough to configure; tar rc=$TAR_RC"
    exit 1
  fi
  if [ "$TAR_RC" != "0" ]; then
    echo "Guile tar reported Android hardlink warnings; continuing because configure exists."
  fi
  cp -R "$GUILE_LANE/src/guile-$VER" "$GUILE_LANE/build/guile-$VER"'''
)
p.write_text(s)
PY
  fi

  echo
  echo "=== fix mandoc hardlink build rule ==="
  MANDOC_BUILD="$(find "$SRC/mandoc_apropos_logic/build" -maxdepth 1 -type d -name 'mandoc-*' 2>/dev/null | sort | tail -n 1 || true)"
  if [ -n "${MANDOC_BUILD:-}" ] && [ -f "$MANDOC_BUILD/Makefile" ]; then
    cd "$MANDOC_BUILD"
    cp -f Makefile "Makefile.before_android_cp_link_$STAMP"

    python - <<'PY'
from pathlib import Path
for name in ["Makefile", "Makefile.local"]:
    p = Path(name)
    if not p.exists():
        continue
    s = p.read_text()
    s = s.replace("LN=ln -f", "LN=cp -f")
    s = s.replace("LN = ln -f", "LN = cp -f")
    s = s.replace("$(LN) mandoc man", "cp -f mandoc man")
    s = s.replace("ln -f mandoc man", "cp -f mandoc man")
    p.write_text(s)
PY

    grep -n 'LN=\|mandoc man\|ln -f mandoc' Makefile Makefile.local 2>/dev/null || true
    make -j "$JOBS"
    make install || true
    cd "$ROOT"
  fi

  echo
  echo "=== make packaged identity ==="
  mkdir -p "$PACKAGED"/{identity,source-lanes,scripts,reports,manifests,review-notes,tarballs}

  cat > "$PACKAGED/identity/BRAXON_SYSTEMS_IDENTITY.md" <<'EOF'
# Braxon Systems Identity

Status: packaged for review, not released.

Core laws:
- do no harm
- respect user privacy
- respect user agency
- support user goals
- fail closed on false proof
- preserve source-first build lanes
- do not fake hot-live state
- state registry is first-class
- NSQ is the bus
- court is compositor/internal machine component

Resolver strategies:
- current_config_path
- tool_config_path
- pkg_config_path
- overlay_include_path
- adoption_include_path
- dereferenced_integrated_prefix
- copied_native_header_prefix
- patched_sysconfig_or_metadata
- env_override_flags
- generated_config_shim

Packaging rule:
This branch is release-prep only. Nothing is released until reviewed.
EOF

  cat > "$PACKAGED/review-notes/SOURCE_FIRST_RELEASE_PREP.md" <<'EOF'
# Source-first release prep

Braxon should build as much as practical from source.

Package manager installs are bootstrap surfaces only:
- compiler
- linker
- make
- tar/gzip/xz-utils
- headers/libraries needed to build source lanes

Release-prep artifacts belong in packaged/.
Installed build products and full state registry trees are not automatically released.
EOF

  echo
  echo "=== copy packageable artifacts ==="
  rsync -a --ignore-missing-args \
    "$ROOT"/build_source_guile_nsq_lane.sh \
    "$ROOT"/build_source_mandoc_apropos_lane.sh \
    "$ROOT"/probe_gpm_guile_extension_lane.sh \
    "$ROOT"/fix_mandoc_android_hardlink_resume.sh \
    "$ROOT"/scripts/verify_source_guile_nsq_lane.sh \
    "$ROOT"/scripts/verify_source_mandoc_apropos_lane.sh \
    "$ROOT"/scripts/verify_gpm_guile_extension_probe.sh \
    "$PACKAGED/scripts/" 2>/dev/null || true

  for lane in guile_nsq_logic mandoc_apropos_logic gpm_guile_extension_probe; do
    if [ -d "$SRC/$lane" ]; then
      mkdir -p "$PACKAGED/source-lanes/$lane"
      rsync -a --ignore-missing-args \
        "$SRC/$lane/docs" \
        "$SRC/$lane/reports" \
        "$SRC/$lane/locks" \
        "$SRC/$lane/"*_env \
        "$PACKAGED/source-lanes/$lane/" 2>/dev/null || true
    fi
  done

  echo
  echo "=== manifest and tarball ==="
  find "$PACKAGED" -type f | sort > "$PACKAGED/manifests/package_file_list.txt"
  find "$PACKAGED" -type f -print0 | sort -z | xargs -0 sha256sum > "$PACKAGED/manifests/package_manifest.sha256"

  tar -czf "$PACKAGED/tarballs/braxon-systems-review-staging-$STAMP.tar.gz" -C "$ROOT" packaged
  sha256sum "$PACKAGED/tarballs/braxon-systems-review-staging-$STAMP.tar.gz" > "$PACKAGED/tarballs/braxon-systems-review-staging-$STAMP.tar.gz.sha256"

  echo
  echo "=== git stage only ==="
  git add packaged scripts build_source_guile_nsq_lane.sh build_source_mandoc_apropos_lane.sh probe_gpm_guile_extension_lane.sh fix_mandoc_android_hardlink_resume.sh 2>/dev/null || true
  git status --short

  echo
  echo "DONE"
  echo "branch=$BRANCH"
  echo "packaged=$PACKAGED"
  echo "tarball=$PACKAGED/tarballs/braxon-systems-review-staging-$STAMP.tar.gz"
  echo "log=$OUT"
} 2>&1 | tee "$OUT"

ln -sf "$OUT" "$TC/setup_braxon_systems_packaged_latest.log"
