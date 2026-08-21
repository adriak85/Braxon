#!/usr/bin/env sh
set -eu

ROOT="${1:-$HOME/Braxon}"
MANIFEST="$ROOT/config/toolchains/source_availability_manifest.json"

fail() {
  printf '%s\n' "braxon-source-archive-verify: $*" >&2
  exit 1
}

[ -f "$MANIFEST" ] || fail "source availability manifest is missing: $MANIFEST"
grep -Fq '"schema": "braxon.toolchain.source_availability.v1"' "$MANIFEST" || fail "unsupported source availability manifest schema"
for tool in awk cat grep sha256sum sort wc; do
  command -v "$tool" >/dev/null 2>&1 || fail "required verification tool is missing: $tool"
done

verify_provenance() {
  provenance="$1"
  expected_sha="$2"
  provenance_path="$ROOT/$provenance"
  [ -f "$provenance_path" ] || fail "archive provenance is missing: $provenance"
  grep -Fq '"schema": "braxon.source_archive.provenance.v1"' "$provenance_path" || fail "unsupported source archive provenance schema: $provenance"
  grep -Fq "\"sha256\": \"$expected_sha\"" "$provenance_path" || fail "provenance hash mismatch: $provenance"
}

verify_single_archive() {
  source_id="$1"
  relative_path="$2"
  expected_bytes="$3"
  expected_sha="$4"
  provenance="$5"
  archive="$ROOT/$relative_path"
  [ -f "$archive" ] || fail "repository-contained source archive is missing: $relative_path"
  actual_bytes="$(wc -c < "$archive" | tr -d '[:space:]')"
  actual_sha="$(sha256sum "$archive" | awk '{print $1}')"
  [ "$actual_bytes" = "$expected_bytes" ] || fail "archive byte count mismatch: $source_id"
  [ "$actual_sha" = "$expected_sha" ] || fail "archive identity mismatch: $source_id"
  verify_provenance "$provenance" "$expected_sha"
  printf 'CHECK source_id=%s representation=single_archive bytes=%s sha256=%s path=%s provenance=%s valid=true\n' \
    "$source_id" "$actual_bytes" "$actual_sha" "$relative_path" "$provenance"
}

verify_chunk_set() {
  source_id="$1"
  chunk_directory="$2"
  expected_count="$3"
  maximum_chunk_bytes="$4"
  expected_bytes="$5"
  expected_sha="$6"
  provenance="$7"
  directory="$ROOT/$chunk_directory"
  [ -d "$directory" ] || fail "nested LLVM chunk directory is missing: $chunk_directory"

  chunks="$(find "$directory" -maxdepth 1 -type f -name '*.part' -print | LC_ALL=C sort)"
  [ -n "$chunks" ] || fail "nested LLVM chunks are missing: $chunk_directory"
  chunk_count="$(printf '%s\n' "$chunks" | wc -l | tr -d '[:space:]')"
  [ "$chunk_count" = "$expected_count" ] || fail "nested LLVM chunk count mismatch: expected $expected_count, got $chunk_count"

  largest=0
  while IFS= read -r chunk; do
    chunk_bytes="$(wc -c < "$chunk" | tr -d '[:space:]')"
    [ "$chunk_bytes" -le "$maximum_chunk_bytes" ] || fail "nested LLVM chunk exceeds Git-safe maximum: $chunk"
    [ "$chunk_bytes" -gt "$largest" ] && largest="$chunk_bytes"
  done <<EOF
$chunks
EOF

  actual_bytes="$(while IFS= read -r chunk; do cat "$chunk"; done <<EOF | wc -c | tr -d '[:space:]'
$chunks
EOF
)"
  actual_sha="$(while IFS= read -r chunk; do cat "$chunk"; done <<EOF | sha256sum | awk '{print $1}'
$chunks
EOF
)"
  [ "$actual_bytes" = "$expected_bytes" ] || fail "nested LLVM reassembly byte count mismatch: $source_id"
  [ "$actual_sha" = "$expected_sha" ] || fail "nested LLVM reassembly identity mismatch: $source_id"
  verify_provenance "$provenance" "$expected_sha"
  printf 'CHECK source_id=%s representation=git_safe_chunk_set chunks=%s largest_chunk_bytes=%s bytes=%s sha256=%s directory=%s provenance=%s valid=true\n' \
    "$source_id" "$chunk_count" "$largest" "$actual_bytes" "$actual_sha" "$chunk_directory" "$provenance"
}

printf '%s\n' 'schema=braxon.toolchain.public_source_archive_verification.v1'
printf '%s\n' 'verifier=posix_shell_no_node_runtime'
printf 'root=%s\n' "$ROOT"

verify_single_archive \
  rust \
  state/full_android_language_toolchain/source_archives/rust-f964de49bcb561e5c6c725bb37201e11d852daf0.tar.gz \
  38258116 \
  ea2b7f5abde429b1699ca4fa4f6c44d5533db4b0bccae020baf813da02f0e42e \
  state/full_android_language_toolchain/source_archives/rust-f964de49bcb561e5c6c725bb37201e11d852daf0.provenance.json
verify_chunk_set \
  rust_nested_llvm \
  state/full_android_language_toolchain/source_archives/llvm-project-eaab4d9841b9a8a12783d927b2df2291c1c79269.chunks \
  6 \
  50331648 \
  258299213 \
  0d4b6831708211df28ca4b317c06f6e0078f9df5ad673ba902c73f0318a4fa1c \
  state/full_android_language_toolchain/source_archives/llvm-project-eaab4d9841b9a8a12783d927b2df2291c1c79269.provenance.json
verify_single_archive \
  rust_edge_nightly_1_100_0 \
  state/full_android_language_toolchain/source_archives/rust-f7d782a3be46d6bb4b9792fe69a61db389ba1769.tar.gz \
  39821416 \
  50e6078f413d40a1991b8f7ee0b19c9ec28f93bfbc5f5e7cb22575a610e56cb0 \
  state/full_android_language_toolchain/source_archives/rust-f7d782a3be46d6bb4b9792fe69a61db389ba1769.provenance.json
verify_single_archive \
  cpython \
  state/full_android_language_toolchain/source_archives/cpython-49918f5b0ceb1950c3222fd4fd6be872d2e15c6f.tar.gz \
  43839630 \
  7757cb0e24d9a9598239174580eb018a8197dfcb213bb576d67ffbc499dd2181 \
  state/full_android_language_toolchain/source_archives/cpython-49918f5b0ceb1950c3222fd4fd6be872d2e15c6f.provenance.json

printf '%s\n' 'source_check_total=4'
printf '%s\n' 'valid=true'
