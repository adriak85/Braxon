#!/usr/bin/env sh
set -eu

ROOT="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
MODE="${1:-status}"
MIN_BUILD_FREE_KIB="${BRAXON_MIN_BUILD_FREE_KIB:-33554432}"

fail() { printf '%s\n' "braxon-reconstruct: $*" >&2; exit 1; }
notice() { printf '%s\n' "braxon-reconstruct: $*"; }

require_ksr_build_authorization() {
  expected_scope="$1"
  [ "${BRAXON_KSR_SEMANTIC_BUILD_CAPABILITY:-}" = "feature:toolchain.semantic_build_dialect" ] || fail "physical build execution is KSR-owned; invoke Braxon toolchain build-dialect $expected_scope execute"
  [ "${BRAXON_KSR_SEMANTIC_BUILD_SCOPE:-}" = "$expected_scope" ] || fail "KSR build scope mismatch; expected $expected_scope"
  [ "${BRAXON_KSR_SEMANTIC_BUILD_ACTION:-}" = "execute" ] || fail "physical build execution requires an explicit KSR execute transition"
  watermark="${BRAXON_KSR_SEMANTIC_BUILD_WATERMARK:-}"
  case "$watermark" in
    ????????*) ;;
    *) fail "physical build execution requires a nonempty functional KSR watermark" ;;
  esac
}

require_contracts() {
  for path in \
    Cargo.lock \
    .cargo/config.toml \
    config/toolchains/contained_semantic_toolchain_inventory.json \
    config/toolchains/source_availability_manifest.json \
    config/toolchains/rust_bootstrap_chain.json \
    config/toolchains/termux_android_aarch64_capacity_profile.json \
    config/toolchains/galaxy_a17_dense_artifact_package_contract.json \
    config/toolchains/termux_nsq_intercept_policy.json \
    config/toolchains/source_built_build_graph.json \
    config/toolchains/extended_repository_integration_manifest.json \
    config/toolchains/license_report.json \
    config/toolchains/gap_report.json \
    config/nsq/complete_semantic_extraction_contract.json \
    config/nsq/semantic_corpus_manifest.json \
    config/nsq/semantic_build_dialect_contract.json \
    scripts/braxon_termux_calibrate.sh \
    scripts/toolchains/rebuild_full_android_language_toolchain.sh \
    scripts/toolchains/capture_galaxy_a17_dense_artifact_package.sh \
    scripts/toolchains/promote_rust_edge_nightly_aarch64.sh \
    scripts/toolchains/resolve_braxon_repository_tool.sh \
    scripts/toolchains/write_braxon_repository_tool_dispatch.sh \
    scripts/toolchains/verify_public_source_archives.sh \
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

verified_existing_source() {
  destination="$1"
  expected_commit="$2"
  required_one="$3"
  required_two="$4"
  [ -d "$destination" ] || return 1
  [ -f "$destination/$required_one" ] || return 1
  [ -f "$destination/$required_two" ] || return 1
  [ "$(git -C "$destination" rev-parse HEAD 2>/dev/null || true)" = "$expected_commit" ] || return 1
  git -C "$destination" diff --quiet --ignore-submodules -- 2>/dev/null || return 1
  git -C "$destination" diff --cached --quiet --ignore-submodules -- 2>/dev/null || return 1
  return 0
}

llvm_source_complete() {
  llvm_check_destination="$1"
  for llvm_required in \
    llvm/CMakeLists.txt \
    llvm/lib/Demangle/CMakeLists.txt \
    llvm/lib/Support/CMakeLists.txt \
    llvm/lib/TableGen/CMakeLists.txt \
    clang/CMakeLists.txt \
    lld/CMakeLists.txt \
    bolt/CMakeLists.txt \
    llvm/tools/llvm-jitlink/CMakeLists.txt; do
    [ -f "$llvm_check_destination/$llvm_required" ] || return 1
  done
  return 0
}

materialize_chunked_llvm_source() {
  llvm_chunks="$1"
  llvm_expected_root="$2"
  llvm_destination="$3"
  llvm_receipt="$ROOT/state/full_android_language_toolchain/source_receipts/llvm-project-eaab4d9841b9a8a12783d927b2df2291c1c79269.txt"
  llvm_expected_sha="0d4b6831708211df28ca4b317c06f6e0078f9df5ad673ba902c73f0318a4fa1c"
  [ -d "$llvm_chunks" ] || fail "repository-contained LLVM chunk directory is absent: $llvm_chunks"
  if [ -d "$llvm_destination" ] && [ "$(find "$llvm_destination" -mindepth 1 -maxdepth 1 -print -quit)" ]; then
    if llvm_source_complete "$llvm_destination" && [ -f "$llvm_receipt" ] && grep -Fxq "archive_sha256=$llvm_expected_sha" "$llvm_receipt"; then
      notice "accepted_verified_complete_llvm_source=$llvm_destination"
      return 0
    fi
    [ "${BRAXON_REPLACE_INCOMPLETE_LLVM_SOURCE:-0}" = "1" ] || fail "LLVM source is incomplete or lacks a contained-archive receipt: $llvm_destination; preserve any local work, then rerun source-edge with BRAXON_REPLACE_INCOMPLETE_LLVM_SOURCE=1"
    rm -rf "$llvm_destination"
  fi
  llvm_staging="$ROOT/state/full_android_language_toolchain/materialization/.${llvm_expected_root}.$$"
  llvm_archive="$llvm_staging/${llvm_expected_root}.tar.gz"
  rm -rf "$llvm_staging"
  mkdir -p "$llvm_staging"
  cat "$llvm_chunks"/*.part > "$llvm_archive"
  llvm_actual_sha="$(sha256sum "$llvm_archive" | awk '{print $1}')"
  [ "$llvm_actual_sha" = "$llvm_expected_sha" ] || fail "reassembled LLVM source archive SHA-256 mismatch"
  tar -xzf "$llvm_archive" -C "$llvm_staging"
  llvm_extracted="$llvm_staging/$llvm_expected_root"
  [ -d "$llvm_extracted" ] || fail "LLVM archive did not contain expected root $llvm_expected_root"
  llvm_source_complete "$llvm_extracted" || fail "LLVM archive extraction is incomplete: required LLVM lib directories are absent"
  mkdir -p "$(dirname "$llvm_destination")" "$(dirname "$llvm_receipt")"
  mv "$llvm_extracted" "$llvm_destination"
  {
    echo "schema=braxon.llvm.contained_source_receipt.v1"
    echo "expected_root=$llvm_expected_root"
    echo "archive_sha256=$llvm_actual_sha"
    echo "required_paths=llvm/CMakeLists.txt,llvm/lib/Demangle/CMakeLists.txt,llvm/lib/Support/CMakeLists.txt,llvm/lib/TableGen/CMakeLists.txt,clang/CMakeLists.txt,lld/CMakeLists.txt"
    echo "materialized_from=repository_contained_chunk_set"
  } > "$llvm_receipt"
  rm -rf "$llvm_staging"
  notice "materialized_verified_complete_llvm_source=$llvm_destination"
}

materialize_archive_source() {
  archive="$1"
  expected_root="$2"
  destination="$3"
  [ -f "$archive" ] || fail "repository-contained source archive is absent: $archive"
  if [ -d "$destination" ] && [ "$(find "$destination" -mindepth 1 -maxdepth 1 -print -quit)" ]; then
    if [ "$destination" = "$ROOT/state/full_android_language_toolchain/src/rust" ] && verified_existing_source "$destination" "f964de49bcb561e5c6c725bb37201e11d852daf0" "x.py" "compiler/rustc/Cargo.toml"; then
      notice "accepted_verified_existing_rust_source=$destination"
      return 0
    fi
    if [ "$destination" = "$ROOT/state/full_android_language_toolchain/src/cpython" ] && verified_existing_source "$destination" "49918f5b0ceb1950c3222fd4fd6be872d2e15c6f" "configure.ac" "Python/ceval.c"; then
      notice "accepted_verified_existing_cpython_source=$destination"
      return 0
    fi
    [ "${BRAXON_REPLACE_SOURCE_EDGE:-0}" = "1" ] || fail "destination already contains unverified or dirty source: $destination; set BRAXON_REPLACE_SOURCE_EDGE=1 only after preserving or removing it intentionally"
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
  require_ksr_build_authorization llvm-source-edge
  cd "$ROOT"
  "$ROOT/scripts/toolchains/verify_public_source_archives.sh" "$ROOT"
  materialize_chunked_llvm_source \
    "$ROOT/state/full_android_language_toolchain/source_archives/llvm-project-eaab4d9841b9a8a12783d927b2df2291c1c79269.chunks" \
    "llvm-project-eaab4d9841b9a8a12783d927b2df2291c1c79269" \
    "$ROOT/state/full_android_language_toolchain/src/llvm-project"
  materialize_archive_source \
    "$ROOT/state/full_android_language_toolchain/source_archives/rust-f964de49bcb561e5c6c725bb37201e11d852daf0.tar.gz" \
    "rust-f964de49bcb561e5c6c725bb37201e11d852daf0" \
    "$ROOT/state/full_android_language_toolchain/src/rust"
  materialize_archive_source \
    "$ROOT/state/full_android_language_toolchain/source_archives/cpython-49918f5b0ceb1950c3222fd4fd6be872d2e15c6f.tar.gz" \
    "cpython-49918f5b0ceb1950c3222fd4fd6be872d2e15c6f" \
    "$ROOT/state/full_android_language_toolchain/src/cpython"
  notice "pinned_llvm_rust_and_cpython_source_edges_verified_or_materialized_from_repository_archives=true"
  notice "network_used=false"
}

offline_verify() {
  cd "$ROOT"
  "$ROOT/scripts/toolchains/verify_public_source_archives.sh" "$ROOT"
  node tools/toolchain/validate_toolchain_contracts.mjs "$ROOT"
  "$ROOT/scripts/toolchains/resolve_braxon_repository_tool.sh" exec cargo test --workspace --locked --offline
  "$ROOT/scripts/toolchains/resolve_braxon_repository_tool.sh" exec cargo run --locked --offline -- toolchain verify
  "$ROOT/scripts/toolchains/resolve_braxon_repository_tool.sh" exec cargo run --locked --offline -- toolchain bionic
}

calibrate() {
  target_preflight
  "${SHELL:-sh}" "$ROOT/scripts/braxon_termux_calibrate.sh" calibrate
  "${SHELL:-sh}" "$ROOT/scripts/braxon_termux_calibrate.sh" verify
}

source_build() {
  require_ksr_build_authorization llvm-aarch64-source-build
  target_preflight
  capacity_preflight
  for path in state/full_android_language_toolchain/src/rust state/full_android_language_toolchain/src/cpython; do
    [ -d "$ROOT/$path" ] && [ "$(find "$ROOT/$path" -mindepth 1 -maxdepth 1 -print -quit)" ] || fail "source-build prerequisite is absent: $path; run source-edge to materialize repository-contained sources"
  done
  llvm_source_complete "$ROOT/state/full_android_language_toolchain/src/llvm-project" || fail "LLVM source is incomplete; run source-edge with BRAXON_REPLACE_INCOMPLETE_LLVM_SOURCE=1 to materialize the verified contained LLVM archive"
  [ -f "$ROOT/state/full_android_language_toolchain/source_receipts/llvm-project-eaab4d9841b9a8a12783d927b2df2291c1c79269.txt" ] || fail "LLVM contained-source receipt is absent; run source-edge with BRAXON_REPLACE_INCOMPLETE_LLVM_SOURCE=1"
  BRAXON_SOURCE_BUILD_APPROVED=1 JOBS="${JOBS:-1}" "$ROOT/scripts/toolchains/rebuild_full_android_language_toolchain.sh" "$ROOT"
}

galaxy_a17_artifact_package() {
  require_ksr_build_authorization galaxy-a17-artifact-package
  target_preflight
  "$ROOT/scripts/toolchains/capture_galaxy_a17_dense_artifact_package.sh" "$ROOT"
}

edge_nightly_build() {
  require_ksr_build_authorization rust-edge-nightly
  target_preflight
  capacity_preflight
  cd "$ROOT"
  "$ROOT/scripts/toolchains/verify_public_source_archives.sh" "$ROOT"
  for path in \
    state/full_android_language_toolchain/install/python/bin/python3 \
    state/full_android_language_toolchain/install/llvm/bin/llvm-config \
    state/full_android_language_toolchain/install/llvm/bin/clang \
    state/full_android_language_toolchain/install/llvm/bin/llvm-readelf; do
    [ -x "$ROOT/$path" ] || fail "edge-nightly prerequisite is absent: $path; complete the repository-contained base source-build first"
  done
  BRAXON_SOURCE_BUILD_APPROVED=1 BRAXON_SOURCE_LLVM="$ROOT/state/full_android_language_toolchain/install/llvm" "$ROOT/scripts/toolchains/unified_android_libc_contract_overlay.sh" "$ROOT"
  BRAXON_SOURCE_BUILD_APPROVED=1 JOBS="${JOBS:-1}" "$ROOT/scripts/toolchains/promote_rust_edge_nightly_aarch64.sh" "$ROOT"
  "$ROOT/scripts/toolchains/write_braxon_repository_tool_dispatch.sh" "$ROOT"
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
  artifact-package)
    require_contracts
    galaxy_a17_artifact_package
    ;;
  *) fail "usage: $0 [status|preflight|source-edge|verify|calibrate|source-build|edge-nightly-build|artifact-package]" ;;
esac
