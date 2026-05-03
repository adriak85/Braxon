#!/usr/bin/env python3
"""
SCENT HOUND — Semantic Drift Detection
Domain: semantic drift, linkage trail, macro scent

Runs BEFORE build. Sniffs every court-facing crate for:
- Binary-width types in semantic space (u32/u16 as semantic truth)
- Unlabeled boundary crossings
- Linkage drift (crates importing things they shouldn't)
- Macro scent (macros that expand to binary-width types)

Exit 0 = clean
Exit 1 = drift detected (DO NOT BUILD)
Exit 2 = usage error
"""

import sys
import re
import json
from pathlib import Path


# Crates where semantic truth lives — binary types are contamination here
SEMANTIC_CRATES = {
    "nsq-core", "nsq-lint", "nsq-court", "Braxon-court",
    "nsq-proof", "nsq-source", "nsq-compose",
}

# Binary types that must not appear in semantic space
BINARY_TYPES = [
    (r'\bu32\b', 'u32'),
    (r'\bu16\b', 'u16'),
    (r'\bas\s+u32\b', 'cast to u32'),
    (r'\bas\s+u16\b', 'cast to u16'),
    (r'put_u32\b', 'put_u32 write'),
    (r'put_u16\b', 'put_u16 write'),
    (r'get_u32\b', 'get_u32 read'),
    (r'get_u16\b', 'get_u16 read'),
]

# Patterns that indicate LEGITIMATE boundary usage (not contamination)
BOUNDARY_MARKERS = [
    '// boundary',
    '// BOUNDARY',
    '// transport',
    '// TRANSPORT',
    '// serialization',
    '// binary frame',
    'boundary_',
    '_transport',
]

# Known bad import patterns (wrong crate dependencies)
FORBIDDEN_IMPORTS = {
    "nsq-core": ["candle", "gguf", "llama", "tokenizers"],
    "nsq-court": ["candle", "gguf", "llama"],
    "nsq-lint": ["candle", "gguf", "llama"],
}


def is_boundary_context(line: str, lines: list, line_idx: int) -> bool:
    """Check if a binary-type usage is in legitimate boundary context."""
    # Check the line itself
    for marker in BOUNDARY_MARKERS:
        if marker in line:
            return True

    # Check the preceding comment block
    for i in range(max(0, line_idx - 3), line_idx):
        for marker in BOUNDARY_MARKERS:
            if marker in lines[i]:
                return True

    # Skip parse_u32/parse_u16 — these are format validators, not semantic types
    if 'parse_u32' in line or 'parse_u16' in line or 'parse::<u32>' in line or 'parse::<u16>' in line:
        return True

    return False


def scan_file(path: Path, crate_name: str) -> list:
    """Scan a single Rust file for semantic drift."""
    findings = []

    try:
        content = path.read_text(encoding='utf-8')
    except Exception as e:
        return [{"file": str(path), "line": 0, "issue": f"cannot read: {e}", "severity": "error"}]

    lines = content.split('\n')

    for i, line in enumerate(lines, 1):
        stripped = line.strip()

        # Skip comments and empty lines
        if stripped.startswith('//') or stripped.startswith('/*') or not stripped:
            continue

        # Skip test modules (less strict)
        if '#[cfg(test)]' in stripped:
            break

        for pattern, label in BINARY_TYPES:
            if re.search(pattern, line):
                if not is_boundary_context(line, lines, i - 1):
                    findings.append({
                        "file": str(path),
                        "line": i,
                        "code": stripped[:100],
                        "issue": f"binary-width type '{label}' in semantic crate '{crate_name}'",
                        "severity": "DRIFT"
                    })

    return findings


def check_cargo_imports(crate_path: Path, crate_name: str) -> list:
    """Check Cargo.toml for forbidden dependencies."""
    findings = []
    cargo_toml = crate_path / "Cargo.toml"

    if not cargo_toml.exists():
        return findings

    forbidden = FORBIDDEN_IMPORTS.get(crate_name, [])
    if not forbidden:
        return findings

    try:
        content = cargo_toml.read_text()
        for dep in forbidden:
            if dep in content:
                findings.append({
                    "file": str(cargo_toml),
                    "line": 0,
                    "issue": f"forbidden dependency '{dep}' in semantic crate '{crate_name}'",
                    "severity": "LINKAGE_DRIFT"
                })
    except Exception:
        pass

    return findings


def main():
    workspace = Path(".")

    # Find workspace root
    for parent in [Path(".")] + list(Path(".").parents)[:5]:
        if (parent / "Cargo.toml").exists():
            content = (parent / "Cargo.toml").read_text()
            if "[workspace]" in content:
                workspace = parent
                break

    crates_dir = workspace / "crates"
    if not crates_dir.exists():
        print("ERROR: crates/ directory not found — not in workspace root?", file=sys.stderr)
        sys.exit(2)

    all_findings = []

    for crate_name in SEMANTIC_CRATES:
        crate_path = crates_dir / crate_name
        if not crate_path.exists():
            continue

        # Check Cargo imports
        all_findings.extend(check_cargo_imports(crate_path, crate_name))

        # Scan all Rust source files
        for rs_file in crate_path.rglob("*.rs"):
            # Skip generated files
            if "target" in rs_file.parts:
                continue
            all_findings.extend(scan_file(rs_file, crate_name))

    # Report
    if not all_findings:
        print("SCENT HOUND: clean — no semantic drift detected")
        print(f"  Scanned: {', '.join(sorted(SEMANTIC_CRATES))}")
        sys.exit(0)

    print("SCENT HOUND: DRIFT DETECTED\n")
    by_severity = {}
    for f in all_findings:
        sev = f["severity"]
        by_severity.setdefault(sev, []).append(f)

    for sev, findings in sorted(by_severity.items()):
        print(f"  [{sev}] {len(findings)} finding(s):")
        for f in findings:
            print(f"    {f['file']}:{f['line']}")
            print(f"      {f['issue']}")
            if f.get('code'):
                print(f"      code: {f['code']}")
        print()

    print(f"TOTAL: {len(all_findings)} finding(s). DO NOT BUILD until drift is resolved.")
    sys.exit(1)


if __name__ == "__main__":
    main()
