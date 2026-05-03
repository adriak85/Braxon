#!/usr/bin/env python3
from __future__ import annotations

import argparse
import hashlib
import json
import shutil
import subprocess
from pathlib import Path


def canonicalize(text: str) -> str:
    out = []
    for raw in text.splitlines():
        line = raw.strip()
        if not line:
            continue
        if line.startswith("#"):
            continue
        line = " ".join(line.split())
        out.append(line)
    return "\n".join(out) + ("\n" if out else "")


def external_blake3(path: Path) -> str | None:
    for cmd in ("b3sum", "blake3"):
        exe = shutil.which(cmd)
        if not exe:
            continue
        p = subprocess.run([exe, str(path)], capture_output=True, text=True)
        if p.returncode == 0 and p.stdout.strip():
            return p.stdout.split()[0]
    return None


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("input")
    ap.add_argument("--out", default="")
    args = ap.parse_args()

    path = Path(args.input)
    raw = path.read_bytes()
    text = raw.decode("utf-8", errors="replace")
    canon = canonicalize(text)
    canon_bytes = canon.encode("utf-8")

    tmp = None
    b3 = None
    if args.out:
        out_path = Path(args.out)
        out_path.parent.mkdir(parents=True, exist_ok=True)
    else:
        out_path = None

    report = {
        "input": str(path),
        "byte_count": len(raw),
        "raw_sha256": hashlib.sha256(raw).hexdigest(),
        "canonical_byte_count": len(canon_bytes),
        "blake_null_sha256": hashlib.sha256(canon_bytes).hexdigest(),
        "canonical_line_count": len(canon.splitlines()),
        "canon_rule": "Blake Null hashes parsed/canonical NSQ meaning; BLAKE3/SHA-256 still verify raw bytes.",
        "warning": "This is the first canonical scaffold. Replace canonicalize() with the full NSQ parser once the parser is bound."
    }

    raw_b3 = external_blake3(path)
    if raw_b3:
        report["raw_blake3"] = raw_b3

    if out_path:
        out_path.write_text(json.dumps(report, indent=2, sort_keys=True))
    print(json.dumps(report, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
