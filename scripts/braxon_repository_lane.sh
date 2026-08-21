#!/usr/bin/env sh
set -eu

ROOT="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
MANIFEST="$ROOT/config/toolchains/extended_repository_integration_manifest.json"
EDGE_ROOT="${BRAXON_REPOSITORY_EDGE_ROOT:-${XDG_CACHE_HOME:-$HOME/.cache}/braxon/repository_edges}"
MODE="${1:-status}"
REPOSITORY_ID="${2:-}"

fail() { printf '%s\n' "braxon-repository-lane: $*" >&2; exit 1; }
[ -f "$MANIFEST" ] || fail "integration manifest is missing: $MANIFEST"

repo_name() {
  case "$1" in
    0) printf '%s\n' 0 ;;
    dax_full) printf '%s\n' DAX-FULL ;;
    dax) printf '%s\n' Dax ;;
    dax_autonomous_system) printf '%s\n' Dax-Autonomous-System ;;
    papi) printf '%s\n' PAPI ;;
    f1ux_service) printf '%s\n' f1ux-service ;;
    fastapi_llm_bot) printf '%s\n' fastapi-llm-bot ;;
    termux_packages) printf '%s\n' termux-packages ;;
    *) fail "unknown repository ID '$1'" ;;
  esac
}

revision() {
  case "$1" in
    0) printf '%s\n' 07a64aa98e3e3f8d8a7c6f10ace7764880e26e68 ;;
    dax_full) printf '%s\n' c6a10388a4359aebfe3be1ea73688a5014078441 ;;
    dax) printf '%s\n' 6ef895e97454e59a3ecd1ce8ad5192f50c052176 ;;
    dax_autonomous_system) printf '%s\n' 9e2b0cc3d7b1f402fd22403369915fa35bab6c0e ;;
    papi) printf '%s\n' 926d2eb7fb5fe0369225bda44c716f8b4e4f6ca2 ;;
    f1ux_service) printf '%s\n' 4eac6b3e5b694bcbe022242298efc9be77b91b20 ;;
    fastapi_llm_bot) printf '%s\n' 07541c8a75183bba7333643f9cf84fe6d807fdf9 ;;
    termux_packages) printf '%s\n' 24a20bcd2d717a160d711f2e8487e7f565096507 ;;
    *) fail "unknown repository ID '$1'" ;;
  esac
}

legal_build_allowed() {
  case "$1" in
    dax_autonomous_system|termux_packages) return 0 ;;
    *) return 1 ;;
  esac
}

quarantine_check() {
  repository_path="$1"
  matches="$(find "$repository_path" -type f \( -iname '*api*key*' -o -iname '*secret*' -o -iname '*.pem' -o -iname '*.p12' -o -iname '*.keystore' \) -print 2>/dev/null || true)"
  [ -z "$matches" ] || {
    printf '%s\n' "$matches" >&2
    fail "credential-like files detected; repository remains quarantined and must not be executed or copied until secrets are removed and rotated"
  }
}

acquire() {
  [ -n "$REPOSITORY_ID" ] || fail "usage: $0 acquire <repository-id>"
  name="$(repo_name "$REPOSITORY_ID")"
  expected="$(revision "$REPOSITORY_ID")"
  destination="$EDGE_ROOT/$REPOSITORY_ID"
  mkdir -p "$EDGE_ROOT"
  if [ ! -d "$destination/.git" ]; then
    gh repo clone "adriak85/$name" "$destination" -- --no-checkout
  fi
  git -C "$destination" fetch --depth 1 origin "$expected"
  git -C "$destination" checkout --detach "$expected"
  actual="$(git -C "$destination" rev-parse HEAD)"
  [ "$actual" = "$expected" ] || fail "revision mismatch for $REPOSITORY_ID: expected $expected, got $actual"
  quarantine_check "$destination"
  printf '%s\n' "ACQUIRED_SOURCE_EDGE=$destination"
  printf '%s\n' "REVISION=$actual"
  printf '%s\n' 'STATUS=source_acquired_not_built'
}

status_one() {
  id="$1"
  path="$EDGE_ROOT/$id"
  expected="$(revision "$id")"
  if [ -d "$path/.git" ]; then
    actual="$(git -C "$path" rev-parse HEAD 2>/dev/null || true)"
    if [ "$actual" = "$expected" ]; then
      printf '%s\t%s\t%s\n' "$id" "pinned_source_edge_present" "$actual"
    else
      printf '%s\t%s\t%s\n' "$id" "source_edge_revision_mismatch" "$actual"
    fi
  else
    printf '%s\t%s\n' "$id" "source_edge_not_acquired"
  fi
}

prepare() {
  [ -n "$REPOSITORY_ID" ] || fail "usage: $0 prepare <repository-id>"
  legal_build_allowed "$REPOSITORY_ID" || fail "manifest legal boundary does not authorize a build for '$REPOSITORY_ID'; acquire and preserve it as a source edge or obtain written authorization"
  path="$EDGE_ROOT/$REPOSITORY_ID"
  [ -d "$path/.git" ] || fail "pinned source edge is absent; run acquire first"
  quarantine_check "$path"
  printf '%s\n' "PREPARE_ALLOWED=$REPOSITORY_ID"
  printf '%s\n' 'NEXT=run only the repository-specific Android source build defined by an approved Braxon build contract'
}

case "$MODE" in
  acquire) acquire ;;
  status)
    for id in 0 dax_full dax dax_autonomous_system papi f1ux_service fastapi_llm_bot termux_packages; do
      status_one "$id"
    done
    ;;
  prepare) prepare ;;
  *) fail "usage: $0 [status|acquire|prepare] [repository-id]" ;;
esac
