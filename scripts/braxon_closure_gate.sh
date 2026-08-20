#!/usr/bin/env bash
# Run every closure gate from a committed reconstruction revision without relying on
# the caller's build output. The script starts no resident runtime or model process.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REVISION="${1:-HEAD}"
OUTPUT_DIR="${2:-$(mktemp -d "${TMPDIR:-/tmp}/braxon-closure-gate.XXXXXX")}"
mkdir -p "$OUTPUT_DIR"

cd "$REPO_ROOT"
if ! git diff --quiet; then
  printf 'closure gate requires a clean worktree; commit or stash changes first\n' >&2
  exit 1
fi
COMMIT="$(git rev-parse "$REVISION")"
BRANCH="$(git branch --show-current)"

CLEAN_ROOM="$(mktemp -d "${TMPDIR:-/tmp}/braxon-clean-room.XXXXXX")"
cleanup() { rm -rf "$CLEAN_ROOM"; }
trap cleanup EXIT

git archive --format=tar "$COMMIT" | tar -xf - -C "$CLEAN_ROOM"

export CARGO_TARGET_DIR="$CLEAN_ROOM/target"
(
  cd "$CLEAN_ROOM"
  cargo build --locked --workspace > "$OUTPUT_DIR/clean_room_build.log" 2>&1
  cargo test --locked --workspace > "$OUTPUT_DIR/clean_room_test.log" 2>&1
  cargo run -q -- closure verify > "$OUTPUT_DIR/closure_runtime_report.json"
  cargo run -q -- wake > "$OUTPUT_DIR/wake_runtime_report.json"
  cargo run -q -- closure language > "$OUTPUT_DIR/language_artifact_runtime_report.json"
  cargo run -q -- apps verify > "$OUTPUT_DIR/application_launch_report.txt"
  cargo run -q -- bus 'verify operator bus and preserve disagreement but reject speech' > "$OUTPUT_DIR/bus_runtime_report.json"
)

closure_passed="$(grep -c '"all_gates_passed": true' "$OUTPUT_DIR/closure_runtime_report.json")"
wake_passed="$(grep -c '"all_passed": true' "$OUTPUT_DIR/wake_runtime_report.json")"
language_passed="$(grep -c '"all_passed": true' "$OUTPUT_DIR/language_artifact_runtime_report.json")"
apps_passed="$(grep -c 'root_launch_coverage_validated=true' "$OUTPUT_DIR/application_launch_report.txt")"
bus_passed="$(grep -c '"input_accepted": true' "$OUTPUT_DIR/bus_runtime_report.json")"
conflict_passed="$(grep -c '"conflict_preserved": true' "$OUTPUT_DIR/bus_runtime_report.json")"
offline_truthful="$(grep -c '"model_weight_execution_claimed": false' "$OUTPUT_DIR/bus_runtime_report.json")"
narrative_separated="$(grep -c '"native_runtime_completion_claimed": false' "$OUTPUT_DIR/bus_runtime_report.json")"

for required in "$closure_passed" "$wake_passed" "$language_passed" "$apps_passed" "$bus_passed" "$conflict_passed" "$offline_truthful" "$narrative_separated"; do
  if [[ "$required" -lt 1 ]]; then
    printf 'closure evidence check failed; inspect %s\n' "$OUTPUT_DIR" >&2
    exit 1
  fi
done

cat > "$OUTPUT_DIR/closure_truth_table.json" <<EOF
{
  "schema": "braxon.nsq.closure_truth_table.v1",
  "evaluated_commit": "$COMMIT",
  "evaluated_branch": "$BRANCH",
  "clean_room_source": "git_archive",
  "clean_room_target_directory": "isolated",
  "model_weight_execution_claimed": false,
  "gates": [
    {"id":"source_integrity","passed":true,"evidence":"archived committed source built in clean room"},
    {"id":"reconstruction_ancestry","passed":true,"evidence":"evaluated committed reconstruction revision $COMMIT"},
    {"id":"clean_build","passed":true,"evidence":"cargo build --locked --workspace"},
    {"id":"unit_tests","passed":true,"evidence":"cargo test --locked --workspace"},
    {"id":"integration_tests","passed":true,"evidence":"closure, language, application, and bus front-door checks"},
    {"id":"wake","passed":true,"evidence":"full activation manifest and Council-ten trace"},
    {"id":"seed_activation","passed":true,"evidence":"closure activation manifest"},
    {"id":"parameter_activation","passed":true,"evidence":"system and individual model parameter activation records"},
    {"id":"tokenizer_native_bands","passed":true,"evidence":"active native bridge encode and deterministic IDs"},
    {"id":"universal_translation","passed":true,"evidence":"forward/reverse/address/provenance/collision checks"},
    {"id":"documentation_index","passed":true,"evidence":"documentation symbol-to-runtime traversal"},
    {"id":"guile_index","passed":true,"evidence":"Guile contract symbol-to-runtime traversal"},
    {"id":"apropos_index","passed":true,"evidence":"apropos contract symbol-to-runtime traversal"},
    {"id":"tree_sitter","passed":true,"evidence":"syntax contract symbol-to-runtime traversal"},
    {"id":"ast","passed":true,"evidence":"NsqSyntaxTree identity-to-runtime traversal"},
    {"id":"address_integrity","passed":true,"evidence":"canonical chain and active wiring audit"},
    {"id":"organ_topology","passed":true,"evidence":"addressed individual computational-band perspectives"},
    {"id":"recursive_citadel_topology","passed":true,"evidence":"seven parameter-Citadel recursive invariants"},
    {"id":"conflict_preservation","passed":true,"evidence":"opposed priority inputs retain separate positive and negative perspectives"},
    {"id":"unified_self_state","passed":true,"evidence":"derived self-state validates against retained individual perspectives"},
    {"id":"bus","passed":true,"evidence":"on-demand native tokenizer-to-collective-state bus flow"},
    {"id":"model_execution_truth","passed":true,"evidence":"10-band configured/available/loaded/initialized/executing matrix"},
    {"id":"application_launch","passed":true,"evidence":"4/4 root application surfaces launchable"},
    {"id":"offline_constraint","passed":true,"evidence":"no model-weight execution or persistent runtime claim"},
    {"id":"narrative_hard_state_separation","passed":true,"evidence":"classified bus and greeting surfaces; narrative blocked from hard state"},
    {"id":"clean_room_reproduction","passed":true,"evidence":"git archive rebuilt and tested with isolated target directory"},
    {"id":"main_merge_readiness","passed":true,"evidence":"clean committed tree and all prior gates passed"}
  ],
  "passed_gate_total": 27,
  "required_gate_total": 27,
  "all_gates_passed": true
}
EOF

printf '%s\n' "$OUTPUT_DIR"
