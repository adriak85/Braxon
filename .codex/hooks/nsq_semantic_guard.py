#!/usr/bin/env python3

import json
import re
import sys
from pathlib import Path

ROOT = Path.cwd()

FAILURES = []

ILLEGAL_PATTERNS = [
    r"\bnsq\s*=\s*u8\b",
    r"\bnsq\s*=\s*bytes\b",
    r"\btokenizer replacement\b",
    r"\bflattened integer routing\b",
]

REQUIRED_PATTERNS = [
    "semantic topology",
    "runtime",
    "execution",
]

def fail(msg):
    FAILURES.append(msg)

def scan(path: Path):
    try:
        text = path.read_text(errors="ignore")
    except Exception:
        return

    lower = text.lower()

    for pat in ILLEGAL_PATTERNS:
        if re.search(pat, lower):
            fail(f"{path}: illegal NSQ semantic downgrade")

    if "nsq" in lower:
        found = sum(1 for x in REQUIRED_PATTERNS if x in lower)

        if found == 0:
            fail(f"{path}: NSQ referenced without runtime semantic framing")

def main():
    for ext in ("*.rs", "*.md", "*.json", "*.toml"):
        for p in ROOT.rglob(ext):
            scan(p)

    if FAILURES:
        print(json.dumps({
            "status": "blocked",
            "reason": "nsq_semantic_guard_failed",
            "failures": FAILURES,
        }, indent=2))
        sys.exit(1)

    print(json.dumps({
        "status": "passed",
        "gate": "nsq_semantic_guard",
    }, indent=2))

if __name__ == "__main__":
    main()
