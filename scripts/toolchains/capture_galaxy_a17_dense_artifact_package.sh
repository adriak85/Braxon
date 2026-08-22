#!/usr/bin/env bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
cd "$ROOT"

fail() { printf 'FAIL: %s\n' "$*" >&2; exit 1; }
notice() { printf '%s\n' "$*"; }

[ "${BRAXON_KSR_SEMANTIC_BUILD_CAPABILITY:-}" = "feature:toolchain.semantic_build_dialect" ] || fail "artifact packaging is KSR-owned; invoke Braxon toolchain build-dialect galaxy-a17-artifact-package execute"
[ "${BRAXON_KSR_SEMANTIC_BUILD_SCOPE:-}" = "galaxy-a17-artifact-package" ] || fail "KSR scope mismatch for Galaxy A17 artifact packaging"
[ "${BRAXON_KSR_SEMANTIC_BUILD_ACTION:-}" = "execute" ] || fail "artifact packaging requires an explicit KSR execute transition"
case "${BRAXON_KSR_SEMANTIC_BUILD_WATERMARK:-}" in ????????*) ;; *) fail "artifact packaging requires a functional KSR watermark" ;; esac

machine="$(uname -m 2>/dev/null || true)"
[ "$machine" = "aarch64" ] || fail "Galaxy A17 package capture requires native AArch64; observed '$machine'"
command -v getprop >/dev/null 2>&1 || fail "Galaxy A17 package capture requires Android getprop"
[ "$(getprop ro.product.cpu.abi 2>/dev/null || true)" = "arm64-v8a" ] || fail "Galaxy A17 package capture requires arm64-v8a ABI"
for tool in tar xz sha256sum file awk find sort getconf; do command -v "$tool" >/dev/null 2>&1 || fail "required package-capture tool is absent: $tool"; done

CHAIN="$ROOT/state/full_android_language_toolchain"
BAKED="$CHAIN/baked/current"
OUT="$CHAIN/artifact_packages/galaxy_a17_aarch64"
RUNTIME="$OUT/braxon-galaxy-a17-aarch64-runtime.tar.xz"
DEBUG="$OUT/braxon-galaxy-a17-aarch64-debug-evidence.tar.xz"
MANIFEST="$OUT/artifact_manifest.json"
RECEIPT="$OUT/target_receipt.json"

[ -d "$BAKED/bin" ] || fail "verified baked tool root is absent: $BAKED/bin"
for proof in proofs/bootstrap_authority.txt proofs/llvm_verify.txt proofs/baked_probe_output.txt proofs/braxon_repository_tool_dispatch.json SHA256SUMS; do
  [ -f "$BAKED/$proof" ] || fail "required baked proof is absent: $BAKED/$proof"
done
for tool in clang clang++ ld.lld llvm-readelf llvm-dwarfdump llvm-jitlink llvm-mc llvm-bolt perf2bolt; do
  [ -x "$BAKED/bin/$tool" ] || fail "required baked LLVM artifact is absent: $BAKED/bin/$tool"
done
file "$BAKED/bin/clang" | grep -q 'ARM aarch64' || fail "baked clang is not an AArch64 ELF artifact"
grep -q 'BRAXON_FULL_REBUILD_C_OK' "$BAKED/proofs/baked_probe_output.txt" || fail "baked C target probe is absent"
grep -q 'BRAXON_FULL_REBUILD_RUST_OK' "$BAKED/proofs/baked_probe_output.txt" || fail "baked Rust target probe is absent"

rm -rf "$OUT"
mkdir -p "$OUT"
mem_kib="$(awk '/MemTotal:/ {print $2; exit}' /proc/meminfo 2>/dev/null || true)"
cpu_summary="$(awk -F: '/model name|CPU part|Hardware/ {gsub(/^ +| +$/, "", $2); print $1 "=" $2}' /proc/cpuinfo 2>/dev/null | sort -u | tr '\n' ';')"
cat > "$RECEIPT" <<JSON
{
  "schema": "braxon.toolchain.galaxy_a17_target_receipt.v1",
  "device_family": "Samsung Galaxy A17",
  "device_identity_status": "user_confirmed_target_observation",
  "machine": "$machine",
  "android_abi": "$(getprop ro.product.cpu.abi 2>/dev/null || true)",
  "android_release": "$(getprop ro.build.version.release 2>/dev/null || true)",
  "memory_total_kib": "${mem_kib}",
  "cpu_observation": "${cpu_summary}",
  "ksr_watermark": "${BRAXON_KSR_SEMANTIC_BUILD_WATERMARK}",
  "baked_root": "state/full_android_language_toolchain/baked/current",
  "package_state": "TARGET_PROVEN_ARTIFACT_CAPTURED_NOT_RUNTIME_ACTIVATED"
}
JSON

(
  cd "$CHAIN"
  tar --sort=name --mtime='@0' --owner=0 --group=0 --numeric-owner \
    --exclude='baked/current/bin/*.debug' \
    -cJf "$RUNTIME" baked/current
  tar --sort=name --mtime='@0' --owner=0 --group=0 --numeric-owner \
    --wildcards --no-recursion -cJf "$DEBUG" 'baked/current/bin/*.debug'
)
sha256sum "$RUNTIME" "$DEBUG" "$RECEIPT" > "$OUT/SHA256SUMS"
cat > "$MANIFEST" <<JSON
{
  "schema": "braxon.toolchain.galaxy_a17_dense_artifact_manifest.v1",
  "target": "aarch64-linux-android",
  "device_family": "Samsung Galaxy A17",
  "runtime_archive": "$(basename "$RUNTIME")",
  "debug_archive": "$(basename "$DEBUG")",
  "target_receipt": "$(basename "$RECEIPT")",
  "sha256_manifest": "SHA256SUMS",
  "ksr_watermark": "${BRAXON_KSR_SEMANTIC_BUILD_WATERMARK}",
  "clone_admission": "requires_target_reobservation_exact_hash_verification_and_repository_dispatch_verification",
  "runtime_activated": false,
  "publication_note": "Archive capture is complete. Do not commit archives above hosted Git limits without an explicitly configured large-artifact transport."
}
JSON
sha256sum "$MANIFEST" >> "$OUT/SHA256SUMS"

notice "PASS: Galaxy A17 dense KSR artifact package captured"
notice "runtime_archive=$RUNTIME"
notice "debug_archive=$DEBUG"
notice "manifest=$MANIFEST"
notice "publication=review archive sizes and configure large-artifact transport before committing archives"
