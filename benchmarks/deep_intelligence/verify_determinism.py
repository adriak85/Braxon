#!/usr/bin/env python3
"""Compare two deep benchmark reports while excluding only timing fields."""
from __future__ import annotations

import argparse
import copy
import json
from pathlib import Path

TIMING_KEYS = {
    "started_monotonic_ns",
    "finished_monotonic_ns",
    "elapsed_ms",
    "user_cpu_seconds",
    "system_cpu_seconds",
    "max_rss_kib_delta",
}


def stable(value):
    if isinstance(value, dict):
        return {key: stable(item) for key, item in value.items() if key not in TIMING_KEYS}
    if isinstance(value, list):
        return [stable(item) for item in value]
    return value


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("left", type=Path)
    parser.add_argument("right", type=Path)
    args = parser.parse_args()
    left = stable(json.loads(args.left.read_text()))
    right = stable(json.loads(args.right.read_text()))
    same = left == right
    print(json.dumps({"deterministic_state": same, "left": str(args.left), "right": str(args.right)}))
    return 0 if same else 2


if __name__ == "__main__":
    raise SystemExit(main())
