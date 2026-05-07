#!/usr/bin/env python3

import json
import re
import sys
from pathlib import Path

ROOT = Path.cwd()

FAILURES = []

STAMP_TERMS = [
    "stamp",
    "wake",
    "materialization",
    "reconstruction",
    "execution",
]

BAD_PATTERNS = [
    r"stamp.*metadata only",
    r"stamp.*passive",
    r"stamp.*archive only",
]

def fail(msg):
    FAILURES.append(msg)

def scan(path: Path):
    try:
        text = path.read_text(errors="ignore")
    except Exception:
        return

    lower = text.lower()

    if "stamp" in lower:
        operational = any(x in lower for x in [
            "wake",
            "execute",
            "execution",
            "materialization",
            "reconstruction",
            "route",
        ])

        if not operational:
            fail(f"{path}: stamp present without operational behavior")

    for pat in BAD_PATTERNS:
        if re.search(pat, lower):
            fail(f"{path}: prohibited passive stamp pattern matched")

def main():
    for ext in ("*.rs", "*.json", "*.md", "*.toml"):
        for p in ROOT.rglob(ext):
            scan(p)

    if FAILURES:
        print(json.dumps({
            "status": "blocked",
            "reason": "stamp_execution_gate_failed",
            "failures": FAILURES,
        }, indent=2))
        sys.exit(1)

    print(json.dumps({
        "status": "passed",
        "gate": "stamp_execution_gate",
    }, indent=2))

if __name__ == "__main__":
    main()
