#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${OUTPUT_DIR:-$ROOT/audit/intelligent_operation_gate}"
BIN="$ROOT/target/debug/Braxon"
mkdir -p "$OUT_DIR"
cd "$ROOT"

cargo build --locked --quiet
cargo test --locked --test braxon_intelligent_operations_surface --test braxon_runtime_surface --test braxon_speech_surface --quiet

"$BIN" reflex verify > "$OUT_DIR/reflex_verify.json"
"$BIN" bus verify terminal launch path through operator bus > "$OUT_DIR/operator_bus.json"
"$BIN" runtime parameter-citadel --signal 8 --context 5 > "$OUT_DIR/parameter_citadel.json"
"$BIN" runtime native-equivalence > "$OUT_DIR/native_equivalence.json"
"$BIN" runtime native-recovery > "$OUT_DIR/native_recovery.json"
"$BIN" language guile '(integrate signal context)' > "$OUT_DIR/guile_language.json"
"$BIN" language zig 'const action = operate;' > "$OUT_DIR/zig_language.json"
"$BIN" status > "$OUT_DIR/status.json"
"$BIN" rescue > "$OUT_DIR/rescue.json"

if "$BIN" runtime infer Braxon 'is truth' > "$OUT_DIR/tensor_infer.stdout" 2> "$OUT_DIR/tensor_infer.stderr"; then
  echo "tensor inference unexpectedly succeeded without a donor index" >&2
  exit 1
fi

grep -q '"valid": true' "$OUT_DIR/reflex_verify.json"
grep -q '"reflex_capability": "feature:operator.intelligence"' "$OUT_DIR/operator_bus.json"
grep -q '"lease_released": true' "$OUT_DIR/operator_bus.json"
grep -q '"capability": "feature:parameter.citadel"' "$OUT_DIR/parameter_citadel.json"
grep -q '"inference_replay_equivalent": true' "$OUT_DIR/native_equivalence.json"
grep -q '"replay_equivalent": true' "$OUT_DIR/native_recovery.json"
grep -q '"language_capability": "language:guile"' "$OUT_DIR/guile_language.json"
grep -q '"language_capability": "language:zig"' "$OUT_DIR/zig_language.json"
grep -q '"reflex_valid": true' "$OUT_DIR/status.json"
grep -q '"closure_all_passed": true' "$OUT_DIR/rescue.json"
grep -q 'authoritative donor index is absent' "$OUT_DIR/tensor_infer.stderr"

if grep -R -n -I -E 'The void is listening|Rescue lane reserved|request_recorded_without_runtime_claim|Measured operator request' \
  src/main.rs crates/braxon-core/src/bus.rs crates/braxon-core/src/greeting.rs crates/braxon-core/src/nsq_native.rs crates/braxon-core/src/intelligent_operation.rs; then
  echo "obsolete narrative or report-only runtime marker remains in an active execution surface" >&2
  exit 1
fi

cat > "$OUT_DIR/intelligent_operation_truth_table.json" <<'JSON'
{
  "schema": "braxon.nsq.intelligent_operation_gate.v1",
  "authority": "NSQ kinetic semantic reflexor",
  "gates": [
    {"id":"reflexor_inventory","passed":true,"evidence":"reflex_verify.json"},
    {"id":"operator_intelligent_action","passed":true,"evidence":"operator_bus.json"},
    {"id":"operator_native_release","passed":true,"evidence":"operator_bus.json"},
    {"id":"parameter_citadel_operation","passed":true,"evidence":"parameter_citadel.json"},
    {"id":"native_equivalence_benchmark","passed":true,"evidence":"native_equivalence.json"},
    {"id":"native_recovery_benchmark","passed":true,"evidence":"native_recovery.json"},
    {"id":"guile_language_operation","passed":true,"evidence":"guile_language.json"},
    {"id":"zig_language_operation","passed":true,"evidence":"zig_language.json"},
    {"id":"measured_status_operation","passed":true,"evidence":"status.json"},
    {"id":"recovery_assessment_operation","passed":true,"evidence":"rescue.json"},
    {"id":"tensor_missing_artifact_guidance","passed":true,"evidence":"tensor_infer.stderr"},
    {"id":"active_runtime_no_obsolete_narrative_or_report_marker","passed":true,"evidence":"source_scan"}
  ],
  "all_passed": true
}
JSON

cat "$OUT_DIR/intelligent_operation_truth_table.json"
