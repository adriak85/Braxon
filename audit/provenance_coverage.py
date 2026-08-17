#!/usr/bin/env python3
"""Deterministic tracked-source provenance and function-surface comparison."""
from __future__ import annotations
import csv, hashlib, re, subprocess, sys
from pathlib import Path

REPOS = {
    "Braxon": Path("/home/ubuntu/Braxon"),
    "0": Path("/home/ubuntu/related/0"),
    "DAX-FULL": Path("/home/ubuntu/related/DAX-FULL"),
    "Dax": Path("/home/ubuntu/related/Dax"),
    "Dax-Autonomous-System": Path("/home/ubuntu/related/Dax-Autonomous-System"),
    "PAPI": Path("/home/ubuntu/related/PAPI"),
    "f1ux-service": Path("/home/ubuntu/related/f1ux-service"),
    "fastapi-llm-bot": Path("/home/ubuntu/related/fastapi-llm-bot"),
    "termux-packages": Path("/home/ubuntu/related/termux-packages"),
}
EXTENSIONS = {".rs", ".py", ".go", ".js", ".jsx", ".ts", ".tsx", ".c", ".h", ".cc", ".cpp", ".zig", ".java", ".sh"}
PATTERNS = {
    "rust": re.compile(r"\b(?:pub\s+)?(?:async\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)"),
    "python": re.compile(r"^\s*(?:async\s+)?def\s+([A-Za-z_][A-Za-z0-9_]*)", re.M),
    "go": re.compile(r"\bfunc\s+(?:\([^)]*\)\s*)?([A-Za-z_][A-Za-z0-9_]*)"),
    "js": re.compile(r"\bfunction\s+([A-Za-z_$][A-Za-z0-9_$]*)|\b(?:const|let|var)\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*=\s*(?:async\s*)?\("),
    "c": re.compile(r"\b[A-Za-z_][A-Za-z0-9_]*\s+([A-Za-z_][A-Za-z0-9_]*)\s*\([^;{}]*\)\s*\{"),
}

def run(repo: Path, *args: str) -> str:
    return subprocess.check_output(["git", "-C", str(repo), *args], text=True, errors="replace")

def language(path: str) -> str:
    ext = Path(path).suffix.lower()
    if ext == ".rs": return "rust"
    if ext == ".py": return "python"
    if ext == ".go": return "go"
    if ext in {".js", ".jsx", ".ts", ".tsx"}: return "js"
    if ext in {".c", ".h", ".cc", ".cpp", ".java"}: return "c"
    return "other"

def tracked(repo: Path) -> list[str]:
    return [p for p in run(repo, "ls-files", "-z").split("\0") if p]

def read_source(repo: Path, path: str) -> str | None:
    try:
        return (repo / path).read_text(encoding="utf-8", errors="replace")
    except OSError:
        return None

def refs(repo: Path) -> list[str]:
    return [r for r in run(repo, "for-each-ref", "--format=%(refname)", "refs/heads", "refs/remotes").splitlines() if r]

def classify(path: str, source: str, canonical_keys: set[tuple[str, str]], canonical_hashes: dict[str, str], digest: str) -> tuple[str, str]:
    lower = path.lower()
    if digest in canonical_hashes:
        return "incorporated", canonical_hashes[digest]
    if any((language(path), name) in canonical_keys for name in extract(path, source)):
        matches = sorted(name for name in extract(path, source) if (language(path), name) in canonical_keys)
        return "duplicated", matches[0] if matches else "canonical-symbol-match"
    if any(token in lower for token in ("backup", "archive", "deprecated", "before_", ".bak", "/target/")):
        return "deprecated/evidence", "provenance-only-candidate"
    return "missing", "no-canonical-function-match"

def extract(path: str, source: str) -> list[str]:
    lang = language(path)
    pattern = PATTERNS.get(lang)
    if pattern is None:
        return []
    names = []
    for match in pattern.finditer(source):
        names.append(next((group for group in match.groups() if group), ""))
    return sorted(set(n for n in names if n))

def main() -> int:
    out = Path(sys.argv[1] if len(sys.argv) > 1 else "/home/ubuntu/Braxon-final-audit/coverage/function_surface_inventory.tsv")
    out.parent.mkdir(parents=True, exist_ok=True)
    canonical = REPOS["Braxon"]
    canonical_keys: set[tuple[str, str]] = set()
    canonical_hashes: dict[str, str] = {}
    canonical_paths = {}
    for path in tracked(canonical):
        if Path(path).suffix.lower() not in EXTENSIONS: continue
        source = read_source(canonical, path)
        if source is None: continue
        digest = hashlib.sha256(source.encode("utf-8", "replace")).hexdigest()
        canonical_hashes[digest] = path
        for name in extract(path, source):
            canonical_keys.add((language(path), name)); canonical_paths[(language(path), name)] = path
    rows = []
    for repo_name, repo in REPOS.items():
        if not (repo / ".git").exists(): continue
        head = run(repo, "rev-parse", "HEAD").strip()
        for path in tracked(repo):
            if Path(path).suffix.lower() not in EXTENSIONS: continue
            source = read_source(repo, path)
            if source is None: continue
            digest = hashlib.sha256(source.encode("utf-8", "replace")).hexdigest()
            symbols = extract(path, source)
            if not symbols:
                continue
            if repo_name == "Braxon":
                status, status_home = "canonical", "canonical"
            else:
                status, status_home = classify(path, source, canonical_keys, canonical_hashes, digest)
            for symbol in symbols:
                canonical_home = "canonical" if repo_name == "Braxon" else canonical_paths.get((language(path), symbol), status_home)
                rows.append((repo_name, head, path, language(path), symbol, digest, status, canonical_home))
    with out.open("w", newline="") as fh:
        writer = csv.writer(fh, delimiter="\t")
        writer.writerow(["repository", "head", "path", "language", "symbol", "sha256", "classification", "canonical_home"])
        writer.writerows(rows)
    summary = out.with_name("function_surface_summary.tsv")
    counts = {}
    for row in rows: counts[(row[0], row[6])] = counts.get((row[0], row[6]), 0) + 1
    with summary.open("w") as fh:
        fh.write("repository\tclassification\tfunctions\n")
        for (repo, status), count in sorted(counts.items()): fh.write(f"{repo}\t{status}\t{count}\n")
    branch_summary = out.with_name("branch_ref_summary.tsv")
    with branch_summary.open("w") as fh:
        fh.write("repository\tref_count\thead\n")
        for repo_name, repo in REPOS.items():
            if (repo / ".git").exists(): fh.write(f"{repo_name}\t{len(refs(repo))}\t{run(repo, 'rev-parse', 'HEAD').strip()}\n")
    print(f"rows={len(rows)} inventory={out} summary={summary} branches={branch_summary}")
    return 0

if __name__ == "__main__": raise SystemExit(main())
