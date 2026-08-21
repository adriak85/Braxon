#!/usr/bin/env sh
set -eu

ROOT="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
MODE="${1:-status}"
MIN_BUILD_FREE_KIB="${BRAXON_MIN_BUILD_FREE_KIB:-33554432}"

fail() { printf '%s\n' "braxon-reconstruct: $*" >&2; exit 1; }
notice() { printf '%s\n' "braxon-reconstruct: $*"; }

require_contracts() {
  for path in \
    Cargo.lock \
    .cargo/config.toml \
    config/toolchains/contained_semantic_toolchain_inventory.json \
    config/toolchains/source_availability_manifest.json \
    config/toolchains/rust_bootstrap_chain.json \
    config/toolchains/termux_android_aarch64_capacity_profile.json \
    config/toolchains/termux_nsq_intercept_policy.json \
    config/toolchains/source_built_build_graph.json \
    config/toolchains/extended_repository_integration_manifest.json \
    config/toolchains/license_report.json \
    config/toolchains/gap_report.json \
    config/nsq/complete_semantic_extraction_contract.json \
    config/nsq/semantic_corpus_manifest.json \
    scripts/braxon_termux_calibrate.sh \
    scripts/toolchains/rebuild_full_android_language_toolchain.sh \
    scripts/toolchains/promote_rust_edge_nightly_aarch64.sh \
    tools/toolchain/validate_toolchain_contracts.mjs \
    tools/toolchain/verify_public_source_archives.mjs; do
    [ -e "$ROOT/$path" ] || fail "required reconstruction contract is missing: $path"
  done
}

target_preflight() {
  machine="$(uname -m 2>/dev/null || printf unknown)"
  is_android=0
  command -v getprop >/dev/null 2>&1 && is_android=1
  case "${PREFIX:-}" in *com.termux*) is_android=1 ;; esac
  if [ "$machine" != "aarch64" ] || [ "$is_android" != 1 ]; then
    [ "${BRAXON_ALLOW_NON_TARGET_RECONSTRUCTION:-0}" = "1" ] || fail "requires native Android AArch64 Termux; use BRAXON_ALLOW_NON_TARGET_RECONSTRUCTION=1 only to validate script control flow"
  fi
  page_size="$(getconf PAGESIZE 2>/dev/null || printf unknown)"
  notice "target_machine=$machine"
  notice "android_detected=$is_android"
  notice "page_size=$page_size"
}

capacity_preflight() {
  available="$(df -Pk "$ROOT" | awk 'NR==2 {print $4}')"
  case "$available" in ''|*[!0-9]*) fail "unable to determine free KiB for $ROOT" ;; esac
  notice "available_kib=$available"
  notice "required_build_kib=$MIN_BUILD_FREE_KIB"
  [ "$available" -ge "$MIN_BUILD_FREE_KIB" ] || fail "insufficient executable-workspace capacity; do not start source build until available KiB meets BRAXON_MIN_BUILD_FREE_KIB"
}

materialize_archive_source() {
  archive="$1"
  expected_root="$2"
  destination="$3"
  [ -f "$archive" ] || fail "repository-contained source archive is absent: $archive"
  if [ -d "$destination" ] && [ "$(find "$destination" -mindepth 1 -maxdepth 1 -print -quit)" ]; then
    [ "${BRAXON_REPLACE_SOURCE_EDGE:-0}" = "1" ] || fail "destination already contains source: $destination; set BRAXON_REPLACE_SOURCE_EDGE=1 only after preserving or removing it intentionally"
    rm -rf "$destination"
  fi
  staging="$ROOT/state/full_android_language_toolchain/materialization/.${expected_root}.$$"
  rm -rf "$staging"
  mkdir -p "$staging"
  tar -xzf "$archive" -C "$staging"
  extracted="$staging/$expected_root"
  [ -d "$extracted" ] || fail "archive did not contain expected root $expected_root: $archive"
  mkdir -p "$(dirname "$destination")"
  mv "$extracted" "$destination"
  rm -rf "$staging"
  notice "materialized_repository_contained_source=$destination"
}

source_edge() {
  cd "$ROOT"
  node tools/toolchain/verify_public_source_archives.mjs "$ROOT"
  materialize_archive_source \
    "$ROOT/state/full_android_language_toolchain/source_archives/rust-f964de49bcb561e5c6c725bb37201e11d852daf0.tar.gz" \
    "rust-f964de49bcb561e5c6c725bb37201e11d852daf0" \
    "$ROOT/state/full_android_language_toolchain/src/rust"
  materialize_archive_source \
    "$ROOT/state/full_android_language_toolchain/source_archives/cpython-49918f5b0ceb1950c3222fd4fd6be872d2e15c6f.tar.gz" \
    "cpython-49918f5b0ceb1950c3222fd4fd6be872d2e15c6f" \
    "$ROOT/state/full_android_language_toolchain/src/cpython"
  notice "pinned_rust_and_cpython_source_edges_materialized_from_repository_archives=true"
  notice "network_used=false"
}

offline_verify() {
  cd "$ROOT"
  node tools/toolchain/verify_public_source_archives.mjs "$ROOT"
  node tools/toolchain/validate_toolchain_contracts.mjs "$ROOT"
  cargo test --workspace --locked --offline
  cargo run --locked --offline -- toolchain verify
  cargo run --locked --offline -- toolchain bionic
}

calibrate() {
  target_preflight
  "${SHELL:-sh}" "$ROOT/scripts/braxon_termux_calibrate.sh" calibrate
  "${SHELL:-sh}" "$ROOT/scripts/braxon_termux_calibrate.sh" verify
}

source_build() {
  target_preflight
  capacity_preflight
  for path in state/full_android_language_toolchain/src/llvm-project state/full_android_language_toolchain/src/rust state/full_android_language_toolchain/src/cpython; do
    [ -d "$ROOT/$path" ] && [ "$(find "$ROOT/$path" -mindepth 1 -maxdepth 1 -print -quit)" ] || fail "source-build prerequisite is absent: $path; run source-edge to materialize repository-contained sources"
  done
  BRAXON_SOURCE_BUILD_APPROVED=1 JOBS="${JOBS:-1}" "$ROOT/scripts/toolchains/rebuild_full_android_language_toolchain.sh" "$ROOT"
}

edge_nightly_build() {
  target_preflight
  capacity_preflight
  cd "$ROOT"
  node tools/toolchain/verify_public_source_archives.mjs "$ROOT"
  for path in \
    state/full_android_language_toolchain/install/python/bin/python3 \
    state/full_android_language_toolchain/install/llvm/bin/llvm-config \
    state/full_android_language_toolchain/install/llvm/bin/clang \
    state/full_android_language_toolchain/install/llvm/bin/llvm-readelf; do
    [ -x "$ROOT/$path" ] || fail "edge-nightly prerequisite is absent: $path; complete the repository-contained base source-build first"
  done
  BRAXON_SOURCE_BUILD_APPROVED=1 BRAXON_SOURCE_LLVM="$ROOT/state/full_android_language_toolchain/install/llvm" "$ROOT/scripts/toolchains/unified_android_libc_contract_overlay.sh" "$ROOT"
  BRAXON_SOURCE_BUILD_APPROVED=1 JOBS="${JOBS:-1}" "$ROOT/scripts/toolchains/promote_rust_edge_nightly_aarch64.sh" "$ROOT"
}

case "$MODE" in
  status)
    require_contracts
    notice "root=$ROOT"
    notice "bootstrap_authority=Rust_1.97.1_AArch64_Android_preserved"
    notice "edge_authority=Rust_1.100.0-nightly_f7d782a3b_not_promoted_without_target_receipt"
    notice "promotion_sequence=preflight -> source-edge -> source-build -> edge-nightly-build -> calibrate -> verify"
    ;;
  preflight)
    require_contracts
    target_preflight
    capacity_preflight
    ;;
  source-edge)
    require_contracts
    source_edge
    ;;
  verify)
    require_contracts
    offline_verify
    ;;
  calibrate)
    require_contracts
    calibrate
    ;;
  source-build)
    require_contracts
    source_build
    ;;
  edge-nightly-build)
    require_contracts
    edge_nightly_build
    ;;
  *) fail "usage: $0 [status|preflight|source-edge|verify|calibrate|source-build|edge-nightly-build]" ;;
esac
