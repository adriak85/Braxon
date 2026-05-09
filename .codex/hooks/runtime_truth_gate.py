#!/usr/bin/env python3

import json
import re
import sys
from pathlib import Path

ROOT = Path.cwd()

FAILURES = []

RUNTIME_WORDS = [
    "hot_live",
    "materialized",
    "executed",
    "runtime",
    "wake",
    "execution",
]

PLACEHOLDER_WORDS = [
    "todo",
    "placeholder",
    "stub",
    "mock",
    "fake",
    "simulated",
]

def fail(msg):
    FAILURES.append(msg)

def scan_file(path: Path):
    try:
        text = path.read_text(errors="ignore")
    except Exception:
        return

    lower = text.lower()

    if "hot_live_verified" in lower:
        if "materialization" not in lower:
            fail(f"{path}: hot_live_verified without materialization reference")

    if "executed_as_second_runtime\": true" in text:
        fail(f"{path}: illegal second runtime claim")

    runtime_present = any(w in lower for w in RUNTIME_WORDS)
    placeholder_present = any(w in lower for w in PLACEHOLDER_WORDS)

    if runtime_present and placeholder_present:
        fail(f"{path}: runtime language mixed with placeholder language")

def main():
    for ext in ("*.rs", "*.json", "*.md", "*.toml", "*.py"):
        for p in ROOT.rglob(ext):
            scan_file(p)

    if FAILURES:
        print(json.dumps({
            "status": "blocked",
            "reason": "runtime_truth_gate_failed",
            "failures": FAILURES,
        }, indent=2))
        sys.exit(1)

    print(json.dumps({
        "status": "passed",
        "gate": "runtime_truth_gate",
    }, indent=2))

if __name__ == "__main__":
    main()
