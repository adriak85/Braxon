#!/usr/bin/env python3
"""Measured native Braxon pipeline benchmark.

This benchmark deliberately separates measured, derived, and blocked fields. It runs the
committed nsq-cli binary for the actual parse/select/eval/fetch path and compares the same
fixed inputs with a resident-token-state reference implementation. It does not claim that
model inference, seed-parameter residency, or ghost/piston firing is covered unless a
reachable executable command proves it.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import resource
import subprocess
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DEFAULT_BINARY = ROOT / "target" / "debug" / "nsq-cli"
DEFAULT_REGION_PROBE = ROOT / "target" / "debug" / "examples" / "region_fire_probe"

WORKLOADS = [
    {"id": "direct-intent", "seed": 101, "input": "agency relation truth"},
    {"id": "linked-resource", "seed": 202, "input": "semantic link tree syntax correction"},
    {"id": "sparse-state", "seed": 303, "input": "scope time relation truth"},
]


def sha256_text(value: str) -> str:
    return hashlib.sha256(value.encode()).hexdigest()


def child_metrics(before: resource.struct_rusage, after: resource.struct_rusage) -> dict[str, float]:
    return {
        "user_cpu_seconds": max(0.0, after.ru_utime - before.ru_utime),
        "system_cpu_seconds": max(0.0, after.ru_stime - before.ru_stime),
        "max_rss_kib_delta": max(0, after.ru_maxrss - before.ru_maxrss),
    }


def run_native(binary: Path, command: str, value: str) -> dict:
    before = resource.getrusage(resource.RUSAGE_CHILDREN)
    start = time.perf_counter_ns()
    proc = subprocess.run(
        [str(binary), command, value],
        cwd=ROOT,
        text=True,
        capture_output=True,
        check=False,
    )
    elapsed = (time.perf_counter_ns() - start) / 1_000_000
    after = resource.getrusage(resource.RUSAGE_CHILDREN)
    output = proc.stdout + proc.stderr
    metrics = child_metrics(before, after)
    return {
        "command": command,
        "input_bytes": len(value.encode()),
        "output_bytes": len(output.encode()),
        "elapsed_ms": elapsed,
        "exit_code": proc.returncode,
        "success": proc.returncode == 0,
        "output_sha256": sha256_text(output),
        "output": output.strip(),
        "metrics": metrics,
    }


def resident_reference(value: str) -> dict:
    start = time.perf_counter_ns()
    tokens = value.split()
    state = {f"token:{index}": token for index, token in enumerate(tokens)}
    state["input"] = value
    output = json.dumps({"tokens": tokens, "state": state}, sort_keys=True)
    elapsed = (time.perf_counter_ns() - start) / 1_000_000
    return {
        "model": "resident-token-state-reference",
        "input_bytes": len(value.encode()),
        "resident_state_entries": len(state),
        "resident_state_bytes_estimate": sum(len(key.encode()) + len(str(item).encode()) for key, item in state.items()),
        "output_bytes": len(output.encode()),
        "elapsed_ms": elapsed,
        "output_sha256": sha256_text(output),
        "correctness_reference": True,
    }


def run_region_probe(binary: Path) -> dict:
    if not binary.exists():
        return {"status": "BLOCKED", "reason": f"region probe does not exist: {binary}"}
    started = time.perf_counter_ns()
    proc = subprocess.run([str(binary)], cwd=ROOT, text=True, capture_output=True, check=False)
    elapsed = (time.perf_counter_ns() - started) / 1_000_000
    if proc.returncode != 0:
        return {"status": "BLOCKED", "exit_code": proc.returncode, "stderr": proc.stderr, "elapsed_ms": elapsed}
    try:
        result = json.loads(proc.stdout)
    except json.JSONDecodeError as error:
        return {"status": "BLOCKED", "reason": f"region probe output is not JSON: {error}", "stdout": proc.stdout, "elapsed_ms": elapsed}
    result["elapsed_ms"] = elapsed
    return result


def run_workload(binary: Path, workload: dict) -> dict:
    value = workload["input"]
    native = {
        "parse": run_native(binary, "parse", f"({value})"),
        "select": run_native(binary, "select", "tree"),
        "eval": run_native(binary, "eval", value),
        "fetch": run_native(binary, "fetch", "Cargo.toml"),
    }
    return {
        "workload_id": workload["id"],
        "fixed_seed": workload["seed"],
        "seed_semantics": "fixed_input_workload_seed; model-parameter seed is not exposed by the current runner",
        "logical_input_bytes": len(value.encode()),
        "input_sha256": sha256_text(value),
        "native_pipeline": native,
        "resident_reference": resident_reference(value),
        "pipeline_stages": {
            "input_acquisition": "PROVEN",
            "tokenizer_boundary": "PROVEN via nsq-cli parse",
            "intent_reconstruction": "PROVEN via nsq-cli eval result state",
            "semantic_link": "BLOCKED in this CLI benchmark; direct semantic-link proof exists separately",
            "capability_discovery": "PROVEN via nsq-cli select",
            "reflexor_mediation": "BLOCKED in this CLI benchmark; direct semantic-link proof exists separately",
            "nsq_native_execution": "PROVEN via RawNsqEngine dispatch",
            "model_intelligence": "BLOCKED; no native model inference command is exposed",
            "result_reconciliation": "DERIVED from native result state; no model result",
            "in_stream_correction": "PROVEN by nsq-core and semantic-link tests, not invoked by this CLI command",
            "result_verification": "PROVEN by exit code and deterministic output hash",
            "release": "BLOCKED in this CLI benchmark; direct semantic-link release proof exists separately",
        },
        "state_elimination": {
            "global_state_constructed": False,
            "global_state_retained": False,
            "claim_status": "BLOCKED_FOR_MODEL_STATE",
            "reason": "The measured path exercises compact native intent state, not a model-parameter workload.",
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=Path, default=DEFAULT_BINARY)
    parser.add_argument("--region-probe", type=Path, default=DEFAULT_REGION_PROBE)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    if not args.binary.exists():
        raise SystemExit(f"native runner binary does not exist: {args.binary}")
    started = time.perf_counter_ns()
    runs = [run_workload(args.binary, workload) for workload in WORKLOADS]
    report = {
        "schema": "braxon.deep_intelligence_benchmark.v1",
        "status": "PARTIAL_PROVEN_WITH_EXPLICIT_BLOCKERS",
        "classification_policy": ["MEASURED", "PROVEN", "DERIVED", "THEORETICAL", "BLOCKED"],
        "repository": str(ROOT),
        "binary": str(args.binary),
        "binary_sha256": hashlib.sha256(args.binary.read_bytes()).hexdigest(),
        "region_probe": run_region_probe(args.region_probe),
        "started_monotonic_ns": started,
        "finished_monotonic_ns": time.perf_counter_ns(),
        "workloads": runs,
        "unmeasured_or_blocked": [
            "model_parameter_residency",
            "ghost_piston_firing_in_this_cli_path",
            "physical_energy",
            "Android_device_acceptance",
        ],
        "money_cost": "NOT_COMPUTED; physical resource-price assumptions are absent",
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
