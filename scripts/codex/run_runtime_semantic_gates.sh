#!/usr/bin/env bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"

cd "$ROOT"

echo
echo "== Braxon / NSQ Runtime Semantic Gate Run =="
echo

run_gate() {
    local gate="$1"

    echo
    echo "== RUNNING: $gate =="
    echo

    python3 "$gate"
}

run_gate ".codex/hooks/runtime_truth_gate.py"
run_gate ".codex/hooks/stamp_execution_gate.py"
run_gate ".codex/hooks/nsq_semantic_guard.py"

echo
echo "== ALL RUNTIME SEMANTIC GATES PASSED =="
echo
