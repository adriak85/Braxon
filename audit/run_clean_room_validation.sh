#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUTPUT_PATH="${1:-$ROOT_DIR/audit/clean_room_validation_result.json}"
if [[ "$OUTPUT_PATH" != /* ]]; then OUTPUT_PATH="$ROOT_DIR/$OUTPUT_PATH"; fi
TMP_DIR="$(mktemp -d -t braxon-clean-room-XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

REPO_DIR="$TMP_DIR/repo"
START_COMMIT="$(git -C "$ROOT_DIR" rev-parse HEAD)"
START_REMOTE="$(git -C "$ROOT_DIR" rev-parse origin/reconstruction)"

mkdir -p "$(dirname "$OUTPUT_PATH")"
printf 'Cloning reconstruction branch into %s\n' "$REPO_DIR"
gh repo clone adriak85/Braxon "$REPO_DIR" -- --single-branch --branch reconstruction --depth 1 >/dev/null
cd "$REPO_DIR"
CLONED_COMMIT="$(git rev-parse HEAD)"
if [[ "$CLONED_COMMIT" != "$START_COMMIT" || "$CLONED_COMMIT" != "$START_REMOTE" ]]; then
  python3 - "$OUTPUT_PATH" "$START_COMMIT" "$START_REMOTE" "$CLONED_COMMIT" <<'PY'
import json, pathlib, sys
out, source_commit, remote_commit, cloned_commit = sys.argv[1:]
pathlib.Path(out).write_text(json.dumps({
    "schema": "braxon.clean_room_validation.v1",
    "status": "BLOCKED",
    "reason": "clone commit mismatch",
    "source_commit": source_commit,
    "remote_commit": remote_commit,
    "cloned_commit": cloned_commit,
}, indent=2) + "\n")
PY
  cat "$OUTPUT_PATH"
  exit 1
fi

if [[ -f "$HOME/.cargo/env" ]]; then source "$HOME/.cargo/env"; fi
TOOLCHAIN="1.96.0"
RUSTC_VERSION="$(rustc +"$TOOLCHAIN" --version)"
CARGO_VERSION="$(cargo +"$TOOLCHAIN" --version)"
export CARGO_HOME="$TMP_DIR/cargo-home"
export CARGO_TARGET_DIR="$TMP_DIR/cargo-target"
mkdir -p "$CARGO_HOME" "$CARGO_TARGET_DIR"

TEST_LOG="$TMP_DIR/braxon_core_tests.log"
if cargo +"$TOOLCHAIN" test -p Braxon-core --lib >"$TEST_LOG" 2>&1; then
  TEST_STATUS="PASS"
else
  TEST_STATUS="FAIL"
fi

TEST_COUNT="$(grep -Eo 'test result: ok\. [0-9]+ passed' "$TEST_LOG" | tail -1 | grep -Eo '[0-9]+' | head -1 || true)"
if [[ -z "$TEST_COUNT" ]]; then TEST_COUNT="0"; fi

python3 - "$OUTPUT_PATH" "$START_COMMIT" "$CLONED_COMMIT" "$TEST_STATUS" "$TEST_COUNT" "$TEST_LOG" "$RUSTC_VERSION" "$CARGO_VERSION" <<'PY'
import json
import pathlib
import sys
out, source_commit, cloned_commit, status, count, log_path, rustc_version, cargo_version = sys.argv[1:]
log_text = pathlib.Path(log_path).read_text(errors="replace")
result = {
    "schema": "braxon.clean_room_validation.v1",
    "source_commit": source_commit,
    "cloned_commit": cloned_commit,
    "commit_match": source_commit == cloned_commit,
    "braxon_core_test_status": status,
    "braxon_core_test_count": int(count),
    "developer_target_dir_used": False,
    "developer_cargo_home_used": False,
    "requested_toolchain": "1.96.0",
    "rustc_version": rustc_version,
    "cargo_version": cargo_version,
    "test_log_sha256": __import__("hashlib").sha256(pathlib.Path(log_path).read_bytes()).hexdigest(),
    "test_log_tail": log_text[-12000:],
    "scope": "Clean clone and Braxon-core library suite; no Android/device acceptance.",
}
pathlib.Path(out).write_text(json.dumps(result, indent=2) + "\n")
if status != "PASS" or not result["commit_match"]:
    raise SystemExit(1)
PY
cat "$OUTPUT_PATH"
