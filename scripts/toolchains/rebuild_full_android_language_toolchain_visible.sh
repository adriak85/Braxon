#!/data/data/com.termux/files/usr/bin/bash
# Compatibility entry point. It intentionally adds no source acquisition, compiler
# selection, or build semantics beyond the canonical repository-contained source lane.
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
ROOT="$(cd "$ROOT" && pwd)"
CANONICAL="$ROOT/scripts/toolchains/rebuild_full_android_language_toolchain.sh"

[ -x "$CANONICAL" ] || { echo "FAIL: canonical repository-contained Android rebuild is absent: $CANONICAL" >&2; exit 1; }
echo "Braxon visible rebuild delegates to the canonical repository-contained source-build chain."
exec "$CANONICAL" "$ROOT"
