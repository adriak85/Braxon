#!/data/data/com.termux/files/usr/bin/bash
# Resolve a canonical Braxon tool only from a verified repository-built artifact.
# Ambient Termux tools are bootstrap-only and are never selected by this resolver.
set -euo pipefail

ROOT="${BRAXON_ROOT:-$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)}"
CHAIN="$ROOT/state/full_android_language_toolchain"
DISPATCH_MANIFEST="$CHAIN/install/braxon_repository_tool_dispatch.json"
MODE="${1:-}"
TOOL="${2:-}"

fail() { printf '%s\n' "braxon-repository-tool: $*" >&2; exit 126; }
require_tool_name() {
  case "$1" in
    rustc|cargo|rustdoc|rustfmt|clippy-driver|clang|clang++|ld.lld|llvm-ar|llvm-ranlib|llvm-nm|llvm-objdump|llvm-readelf|llvm-strip|python3|guile|zig) ;;
    *) fail "undeclared tool '$1'" ;;
  esac
}

[ -n "$MODE" ] || fail "usage: $0 [resolve|exec|status] <declared-tool> [args...]"
[ -n "$TOOL" ] || fail "declared tool is required"
require_tool_name "$TOOL"

[ -f "$DISPATCH_MANIFEST" ] || fail "repository-built tool dispatch manifest is absent: $DISPATCH_MANIFEST; complete the native source-build and verified promotion route before tool use. Ambient Termux fallback is prohibited"

tool_record="$(grep -F "\"name\":\"$TOOL\"" "$DISPATCH_MANIFEST" || true)"
[ -n "$tool_record" ] || fail "tool '$TOOL' is absent from $DISPATCH_MANIFEST"
manifest_field() {
  field="$1"
  printf '%s\n' "$tool_record" | sed -n "s/.*\"$field\":\"\\([^\"]*\\)\".*/\\1/p"
}

declared_status="$(manifest_field status)"
declared_path="$(manifest_field path)"
declared_sha="$(manifest_field sha256)"

[ "$declared_status" = "verified_repository_built" ] || fail "tool '$TOOL' is not verified_repository_built in $DISPATCH_MANIFEST; ambient Termux fallback is prohibited"
[ -n "$declared_path" ] || fail "tool '$TOOL' has no declared repository-built path"
[ -n "$declared_sha" ] || fail "tool '$TOOL' has no declared SHA-256"
case "$declared_path" in
  "$ROOT"/*) ;;
  *) fail "tool '$TOOL' resolves outside the Braxon repository: $declared_path" ;;
esac
[ -x "$declared_path" ] || fail "repository-built tool '$TOOL' is absent or not executable: $declared_path"
actual_sha="$(sha256sum "$declared_path" | awk '{print $1}')"
[ "$actual_sha" = "$declared_sha" ] || fail "repository-built tool '$TOOL' SHA-256 mismatch"

case "$MODE" in
  resolve)
    printf '%s\n' "$declared_path"
    ;;
  status)
    printf '{"schema":"braxon.repository_tool_resolution.v1","tool":"%s","status":"verified_repository_built","path":"%s","sha256":"%s","ambient_termux_fallback":false}\n' "$TOOL" "$declared_path" "$declared_sha"
    ;;
  exec)
    shift 2
    exec "$declared_path" "$@"
    ;;
  *)
    fail "usage: $0 [resolve|exec|status] <declared-tool> [args...]"
    ;;
esac
