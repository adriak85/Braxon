#!/usr/bin/env python3
from __future__ import annotations

import argparse
import hashlib
import json
import os
import shutil
import subprocess
import sys
import textwrap
import time
from pathlib import Path
from typing import Dict, Any


ROOT = Path.cwd()
STAMP_DIR = ROOT / "state/nsq/stamps"
REGISTRY = STAMP_DIR / "stamp_registry.jsonl"
SOURCE_DIR = STAMP_DIR / "sources"
EXPANDED_DIR = STAMP_DIR / "expanded"
CIPHER_REGISTRY = ROOT / "config/nsq/nsq_asm_stamp_cipher_registry.json"


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def maybe_blake3(path: Path) -> str | None:
    for cmd in ("b3sum", "blake3"):
        exe = shutil.which(cmd)
        if not exe:
            continue
        p = subprocess.run([exe, str(path)], capture_output=True, text=True)
        if p.returncode == 0 and p.stdout.strip():
            return p.stdout.split()[0]
    return None


def canonicalize(text: str) -> str:
    lines = []
    for raw in text.splitlines():
        line = raw.strip()
        if not line:
            continue
        if line.startswith("#"):
            continue
        lines.append(" ".join(line.split()))
    return "\n".join(lines) + ("\n" if lines else "")


def load_cipher() -> dict:
    if CIPHER_REGISTRY.exists():
        return json.loads(CIPHER_REGISTRY.read_text())
    return {"cipher_tokens": {}}


def ensure_dirs() -> None:
    SOURCE_DIR.mkdir(parents=True, exist_ok=True)
    EXPANDED_DIR.mkdir(parents=True, exist_ok=True)
    REGISTRY.parent.mkdir(parents=True, exist_ok=True)


def read_registry() -> list[dict]:
    if not REGISTRY.exists():
        return []
    out = []
    for line in REGISTRY.read_text(errors="ignore").splitlines():
        line = line.strip()
        if not line:
            continue
        try:
            out.append(json.loads(line))
        except Exception:
            pass
    return out


def append_registry(entry: dict) -> None:
    ensure_dirs()
    with REGISTRY.open("a", encoding="utf-8") as f:
        f.write(json.dumps(entry, sort_keys=True) + "\n")


def expand_cipher(cipher_text: str, target: str) -> str:
    """
    This is intentionally readable scaffold expansion, not a final assembler.
    It preserves familiar code shape while keeping NSQ cipher intent compact.
    """
    reg = load_cipher()
    known = reg.get("cipher_tokens", {})
    lines = []
    lines.append(f"; NSQ stamp cipher expansion target={target}")
    lines.append("; This scaffold is parseable intent, not final hand-optimized ASM.")
    for raw in cipher_text.splitlines():
        s = raw.strip()
        if not s:
            continue
        parts = s.split()
        tok = parts[0].upper()
        rest = " ".join(parts[1:])
        meaning = known.get(tok, {}).get("meaning", "custom operation")
        if target in ("aarch64_asm", "arm64_asm", "asm", "assembly"):
            if tok == "FN":
                lines.append(f".global {rest}")
                lines.append(f"{rest}:")
            elif tok == "RET":
                lines.append("    ret")
            elif tok == "MOV":
                lines.append(f"    mov {rest}")
            elif tok == "LDR":
                lines.append(f"    ldr {rest}")
            elif tok == "STR":
                lines.append(f"    str {rest}")
            elif tok == "ADD":
                lines.append(f"    add {rest}")
            elif tok == "SUB":
                lines.append(f"    sub {rest}")
            elif tok == "CMP":
                lines.append(f"    cmp {rest}")
            elif tok == "B":
                lines.append(f"    b {rest}")
            elif tok == "BL":
                lines.append(f"    bl {rest}")
            elif tok == "PUSH":
                lines.append(f"    stp {rest}    ; PUSH/save scaffold")
            elif tok == "POP":
                lines.append(f"    ldp {rest}    ; POP/restore scaffold")
            else:
                lines.append(f"    ; {tok} {rest} :: {meaning}")
        elif target == "x86_64_asm":
            if tok == "FN":
                lines.append(f"global {rest}")
                lines.append(f"{rest}:")
            elif tok == "RET":
                lines.append("    ret")
            else:
                lines.append(f"    ; {tok} {rest} :: {meaning}")
        elif target == "c":
            if tok == "FN":
                lines.append(f"void {rest}(void) {{")
            elif tok == "RET":
                lines.append("    return;")
                lines.append("}")
            else:
                lines.append(f"    /* {tok} {rest} :: {meaning} */")
        elif target == "rust":
            if tok == "FN":
                lines.append(f"pub fn {rest}() {{")
            elif tok == "RET":
                lines.append("    return;")
                lines.append("}")
            else:
                lines.append(f"    // {tok} {rest} :: {meaning}")
        else:
            lines.append(f"{tok} {rest} :: {meaning}")
    return "\n".join(lines) + "\n"


def make_stamp(args: argparse.Namespace) -> None:
    ensure_dirs()

    text = ""
    if args.source:
        source_in = Path(args.source)
        text = source_in.read_text(errors="replace")
    elif args.text:
        text = args.text
    elif not sys.stdin.isatty():
        text = sys.stdin.read()
    else:
        raise SystemExit("No source/text/stdin provided")

    stamp_id = args.stamp_id or f"stamp_{int(time.time())}_{hashlib.sha256((args.name + text).encode()).hexdigest()[:12]}"
    ext = args.ext or ("s" if "asm" in args.language else "txt")
    source_path = SOURCE_DIR / f"{stamp_id}.{ext}"
    expanded_path = EXPANDED_DIR / f"{stamp_id}.{args.target.replace('/', '_')}.txt"

    source_path.write_text(text)
    expanded = expand_cipher(text, args.target) if args.cipher else text
    expanded_path.write_text(expanded)

    raw = source_path.read_bytes()
    expanded_raw = expanded_path.read_bytes()
    canonical = canonicalize(expanded)

    entry = {
        "stamp_id": stamp_id,
        "name": args.name,
        "language_surface": args.language,
        "dialect": args.dialect,
        "family": args.family,
        "cipher": bool(args.cipher),
        "target": args.target,
        "canonical_meaning": args.meaning,
        "source_path": str(source_path.relative_to(ROOT)),
        "expanded_path": str(expanded_path.relative_to(ROOT)),
        "source_sha256": sha256_bytes(raw),
        "expanded_sha256": sha256_bytes(expanded_raw),
        "source_blake3": maybe_blake3(source_path),
        "expanded_blake3": maybe_blake3(expanded_path),
        "blake_null_sha256": sha256_bytes(canonical.encode()),
        "byte_count": len(raw),
        "expanded_byte_count": len(expanded_raw),
        "created_at": int(time.time()),
        "reusable": True,
        "court_route": [
            "policer",
            "lexor",
            "lexer",
            "parser",
            "linter",
            "optimizer",
            "router",
            "scheduler",
            "inspector",
            "picker",
            "compositor",
        ],
        "dependencies": args.dep,
        "notes": args.notes,
    }

    append_registry(entry)
    print(json.dumps(entry, indent=2, sort_keys=True))


def list_stamps(args: argparse.Namespace) -> None:
    items = read_registry()
    if args.language:
        items = [x for x in items if x.get("language_surface") == args.language]
    if args.name:
        items = [x for x in items if args.name.lower() in x.get("name", "").lower()]
    print(json.dumps(items, indent=2, sort_keys=True))


def verify_stamps(args: argparse.Namespace) -> None:
    items = read_registry()
    results = []
    for item in items:
        source = ROOT / item.get("source_path", "")
        expanded = ROOT / item.get("expanded_path", "")
        r = {"stamp_id": item.get("stamp_id"), "name": item.get("name")}
        if source.exists():
            raw = source.read_bytes()
            r["source_exists"] = True
            r["source_sha256_match"] = sha256_bytes(raw) == item.get("source_sha256")
        else:
            r["source_exists"] = False
        if expanded.exists():
            raw2 = expanded.read_bytes()
            canon = canonicalize(expanded.read_text(errors="replace")).encode()
            r["expanded_exists"] = True
            r["expanded_sha256_match"] = sha256_bytes(raw2) == item.get("expanded_sha256")
            r["blake_null_sha256_match"] = sha256_bytes(canon) == item.get("blake_null_sha256")
        else:
            r["expanded_exists"] = False
        r["ok"] = all(v is True for k, v in r.items() if k.endswith("_match")) and r.get("source_exists") and r.get("expanded_exists")
        results.append(r)
    ok = all(x.get("ok") for x in results) if results else True
    print(json.dumps({"ok": ok, "count": len(results), "results": results}, indent=2, sort_keys=True))


def main() -> None:
    ap = argparse.ArgumentParser(prog="nsq_stamp.py")
    sub = ap.add_subparsers(dest="cmd", required=True)

    make = sub.add_parser("make")
    make.add_argument("--name", required=True)
    make.add_argument("--stamp-id", default="")
    make.add_argument("--language", default="asm")
    make.add_argument("--dialect", default="aarch64_asm")
    make.add_argument("--family", default="assembly")
    make.add_argument("--target", default="aarch64_asm")
    make.add_argument("--meaning", default="")
    make.add_argument("--source", default="")
    make.add_argument("--text", default="")
    make.add_argument("--cipher", action="store_true")
    make.add_argument("--ext", default="")
    make.add_argument("--dep", action="append", default=[])
    make.add_argument("--notes", default="")
    make.set_defaults(func=make_stamp)

    ls = sub.add_parser("list")
    ls.add_argument("--language", default="")
    ls.add_argument("--name", default="")
    ls.set_defaults(func=list_stamps)

    verify = sub.add_parser("verify")
    verify.set_defaults(func=verify_stamps)

    args = ap.parse_args()
    args.func(args)


if __name__ == "__main__":
    main()
