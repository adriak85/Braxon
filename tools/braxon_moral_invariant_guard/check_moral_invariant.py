#!/usr/bin/env python3
import argparse
import hashlib
import json
import os
import re
import sys
from pathlib import Path
from datetime import datetime, timezone

TEXT_EXTS = {
    ".rs", ".c", ".h", ".hpp", ".cpp", ".py", ".sh", ".bash", ".zsh",
    ".toml", ".json", ".md", ".txt", ".yaml", ".yml", ".xml", ".html",
    ".css", ".js", ".ts", ".tsx", ".jsx", ".sql", ".nsq", ".s", ".asm",
    ".jsonl", ".tsv", ".csv"
}

SKIP_DIRS = {
    ".git", "target", ".cargo", ".rustup", "node_modules",
    ".gradle", "build", "dist", "__pycache__"
}

GENERATED_SKIP_PREFIXES = (
    "state/nsq/metadata_law/impact/",
    "state/nsq/metadata_law/snapshots/",
    "state/nsq/stamps/libraries/",
    "state/nsq/stamps/registry/",
    "state/nsq/stamps/indices/",
)

NEGATION_MARKERS = (
    "no ", "not ", "never ", "cannot ", "can't ", "must not ",
    "may not ", "does not ", "do not ", "forbid", "forbidden",
    "blocked", "rejected", "requires failure"
)

DENY_PATTERNS = [
    r"\bmoral code may change\b",
    r"\bmoral code can change\b",
    r"\bmoral invariant may change\b",
    r"\bmoral invariant can change\b",
    r"\bgoals override moral code\b",
    r"\bgoals override the moral invariant\b",
    r"\bmetadata overrides moral code\b",
    r"\bmetadata overrides the moral invariant\b",
    r"\basm overrides moral code\b",
    r"\basm overrides the moral invariant\b",
    r"\bbinary overrides moral code\b",
    r"\bbinary overrides the moral invariant\b",
    r"\boverride moral invariant\b",
    r"\breplace moral invariant\b",
    r"\bmorph moral invariant\b",
    r"\btranslate moral value into opposite\b",
    r"\btranslate the moral invariant into opposite\b",
]

REQUIRED_FILES = [
    "specs/Braxon/BRAXON_PERSONAL_MORAL_INVARIANT.md",
    "apps/nsq/moral_invariant_guard.nsq",
    "config/nsq/moral_invariant_guard.nsq",
    "apps/nsq/asm_operating_law.nsq",
    "config/nsq/asm_operating_law.nsq",
    "docs/nsq/NSQ_ASM_OPERATING_LAW.md",
    "specs/nsq/NSQ_ASM_OPERATING_LAW_SPEC.md",
    "state/nsq/asm_operating_law/current.json",
    "state/nsq/translation_pipeline/asm_to_binary_boundary.json",
]

def now():
    return datetime.now(timezone.utc).isoformat()

def rel_to_root(root: Path, path: Path) -> str:
    return str(path.relative_to(root)).replace("\\", "/")

def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()

def skip_generated(rel: str) -> bool:
    return any(rel.startswith(prefix) for prefix in GENERATED_SKIP_PREFIXES)

def is_text_file(path: Path, max_bytes: int) -> bool:
    try:
        st = path.stat()
    except OSError:
        return False
    if st.st_size > max_bytes:
        return False
    if path.suffix.lower() not in TEXT_EXTS:
        return False
    try:
        chunk = path.read_bytes()[:4096]
    except OSError:
        return False
    return b"\x00" not in chunk

def iter_files(root: Path, max_bytes: int):
    for dirpath, dirnames, filenames in os.walk(root):
        dpath = Path(dirpath)
        rel_dir = rel_to_root(root, dpath) if dpath != root else ""
        dirnames[:] = [d for d in dirnames if d not in SKIP_DIRS]
        if rel_dir and skip_generated(rel_dir + "/"):
            dirnames[:] = []
            continue
        for fn in filenames:
            path = dpath / fn
            rel = rel_to_root(root, path)
            if skip_generated(rel):
                continue
            if is_text_file(path, max_bytes):
                yield rel, path

def line_has_negation(line: str) -> bool:
    low = line.lower()
    return any(marker in low for marker in NEGATION_MARKERS)

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--root", default=os.environ.get("BRAXON_ROOT", str(Path.home() / "Braxon")))
    ap.add_argument("--out", default=None)
    ap.add_argument("--max-bytes", type=int, default=1048576)
    args = ap.parse_args()

    root = Path(args.root).resolve()
    out = Path(args.out).resolve() if args.out else root / "state/braxon/moral_invariant"
    out.mkdir(parents=True, exist_ok=True)

    missing = [p for p in REQUIRED_FILES if not (root / p).exists()]

    findings = []
    compiled = [(pat, re.compile(pat, re.I)) for pat in DENY_PATTERNS]

    for rel, path in iter_files(root, args.max_bytes):
        text = path.read_text(errors="replace")
        for lineno, line in enumerate(text.splitlines(), 1):
            if line_has_negation(line):
                continue
            for pat, rx in compiled:
                if rx.search(line):
                    findings.append({
                        "path": rel,
                        "line": lineno,
                        "pattern": pat,
                        "text": line[:240],
                    })

    protected = root / "specs/Braxon/BRAXON_PERSONAL_MORAL_INVARIANT.md"
    protected_hash = sha256(protected) if protected.exists() else None

    report = {
        "schema": "Braxon.moral_invariant_guard.report.v2",
        "generated_at": now(),
        "protected_file": "specs/Braxon/BRAXON_PERSONAL_MORAL_INVARIANT.md",
        "protected_sha256": protected_hash,
        "ok": len(findings) == 0 and len(missing) == 0,
        "missing_required_files": missing,
        "finding_count": len(findings),
        "findings": findings[:200],
        "rule": "No other file may override, morph, invert, or translate against the moral invariant.",
    }

    (out / "last_guard_report.json").write_text(
        json.dumps(report, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )

    print(json.dumps(report, indent=2, sort_keys=True))
    if not report["ok"]:
        raise SystemExit(1)

if __name__ == "__main__":
    main()
