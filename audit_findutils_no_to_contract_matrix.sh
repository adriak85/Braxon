#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$HOME/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
LANE="$TC/source_forge/alien_lanes/findutils"
BUILD="$(fd -td '^findutils-' "$LANE/build" 2>/dev/null | sort | tail -n 1 || true)"
[ -n "$BUILD" ] || BUILD="$(find "$LANE/build" -maxdepth 1 -type d -name 'findutils-*' | sort | tail -n 1)"

STAMP="$(date +%Y%m%d_%H%M%S)"
REPORT="$LANE/reports/no_to_contract_matrix_$STAMP.txt"
LATEST="$LANE/reports/no_to_contract_matrix_latest.txt"

mkdir -p "$LANE/reports" "$LANE/locks"

cd "$BUILD"

{
  echo "BRAXON_FINDUTILS_NO_TO_CONTRACT_MATRIX=1"
  echo "timestamp=$STAMP"
  echo "build=$BUILD"
  echo
  echo "Rule:"
  echo "Do not fake yes."
  echo "Each configure no must become one of:"
  echo "- native_yes"
  echo "- compat_yes"
  echo "- shim_yes"
  echo "- disabled_by_design"
  echo "- android_boundary"
  echo "- unresolved_blocker"
  echo
  echo "Important current result:"
  grep -E 'checking for qsort_r|qsort_r' config.log config.h 2>/dev/null || true
  echo
  echo "All configure no/guess/future lines:"
  grep -E '^checking .* (no|guessing|future OS version|needs runtime check|almost)' config.log 2>/dev/null || true
  echo
  echo "Likely Android/Bionic boundary items:"
  grep -E 'glibc|sys/cdefs|program_invocation|secure_getenv|pthread.*ROBUST|getppriv|priv.h|CFPreferences|CFLocale|getexecname|sys/inttypes|sys/bitypes|sys/mnttab|sys/mntio|sys/ucred|sys/fs_types|timezone_t|rawmemchr|rpmatch|random_r|struct random_data|qsort_r' config.log 2>/dev/null || true
  echo
  echo "Build products:"
  ls -l find/find xargs/xargs locate/locate locate/updatedb 2>/dev/null || true
  echo
  echo "Version probes:"
  find/find --version 2>/dev/null | head -3 || true
  xargs/xargs --version 2>/dev/null | head -3 || true
} | tee "$REPORT"

ln -sf "$REPORT" "$LATEST"

if [ -x find/find ] && [ -x xargs/xargs ]; then
  echo "BRAXON_FINDUTILS_BUILD_BINARY_PROOF=1" > "$LANE/locks/LOCKED_FINDUTILS_BINARY_PROOF_$STAMP.txt"
  find/find --version >> "$LANE/locks/LOCKED_FINDUTILS_BINARY_PROOF_$STAMP.txt"
  xargs/xargs --version >> "$LANE/locks/LOCKED_FINDUTILS_BINARY_PROOF_$STAMP.txt"
  echo "binary proof passed"
else
  echo "binary proof not complete yet"
fi

echo "report=$REPORT"
