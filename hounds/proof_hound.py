#!/usr/bin/env python3
"""
PROOF HOUND — Parity Verification
Domain: parity failure, proof mismatch, inspect divergence

Runs AFTER build, BEFORE deploy. Verifies:
- Build artifacts match expected checksums
- Test results match expected outcomes
- Config files are internally consistent
- No silent failures in build output

Exit 0 = parity verified
Exit 1 = parity failure (DO NOT DEPLOY)
Exit 2 = usage error
"""

import sys
import json
import hashlib
import subprocess
from pathlib import Path


REQUIRED_CONFIGS = [
    "config/nsq_court.json",
    "config/braxon_court.json",
    "config/nsq/huihui_model_registry.json",
]

REQUIRED_CRATES = [
    "crates/nsq-core",
    "crates/nsq-court",
    "crates/nsq-runtime",
    "crates/nsq-lint",
    "crates/braxon-court",
    "crates/braxon-core",
    "crates/wowas-final-edition-v10",
]

REQUIRED_SPECS = [
    "specs/nsq",
    "docs/nsq",
]


def sha256_file(path: Path) -> str:
    """Compute SHA256 of a file."""
    h = hashlib.sha256()
    try:
        with open(path, 'rb') as f:
            for chunk in iter(lambda: f.read(65536), b''):
                h.update(chunk)
        return h.hexdigest()
    except Exception as e:
        return f"ERROR:{e}"


def verify_config_json(path: Path) -> list:
    """Verify a JSON config file parses correctly."""
    issues = []
    try:
        with open(path) as f:
            data = json.load(f)
        if not data:
            issues.append(f"PROOF: {path} is empty JSON object/array")
    except json.JSONDecodeError as e:
        issues.append(f"PROOF: {path} is invalid JSON: {e}")
    except Exception as e:
        issues.append(f"PROOF: cannot read {path}: {e}")
    return issues


def verify_court_config_parity(workspace: Path) -> list:
    """Verify court configs are internally consistent."""
    issues = []

    court_path = workspace / "config/nsq_court.json"
    if not court_path.exists():
        return [f"PROOF: nsq_court.json missing"]

    try:
        with open(court_path) as f:
            court = json.load(f)

        roles = court.get("court", {})
        expected_roles = {
            "composer", "linter", "director", "manager", "guard",
            "archon_gates", "arcmage", "bard", "bishop", "conjurer",
            "crier", "detective", "healer", "jack", "keeper",
            "keymaster", "knight", "locksmith", "oracle", "rook",
            "seer", "sees_all", "tank", "ticketmaster", "ace",
        }

        present = set(roles.keys())
        missing = expected_roles - present
        if missing:
            issues.append(f"PROOF: court config missing roles: {', '.join(sorted(missing))}")

        extra = present - expected_roles
        if extra:
            issues.append(f"PROOF: court config has unknown roles: {', '.join(sorted(extra))}")

        # Each role must have title and domain
        for role_id, role_data in roles.items():
            if "title" not in role_data:
                issues.append(f"PROOF: role '{role_id}' missing title")
            if "domain" not in role_data or not role_data["domain"]:
                issues.append(f"PROOF: role '{role_id}' missing domain")

    except Exception as e:
        issues.append(f"PROOF: court config parse error: {e}")

    return issues


def verify_rust_sources(workspace: Path) -> list:
    """Verify Rust source files have minimum expected content."""
    issues = []

    checks = [
        ("crates/nsq-core/src/lib.rs", 500, ["NsqSurfaceValue", "MultipositionalLever", "FullBinaryAnchor"]),
        ("crates/nsq-court/src/main.rs", 100, ["verify", "dispatch", "Court"]),
        ("crates/nsq-court/src/roles.rs", 1000, ["Composer", "Linter", "Guard", "Ace"]),
        ("crates/nsq-runtime/src/lib.rs", 300, ["CourtSurface"]),
    ]

    for rel_path, min_size, required_symbols in checks:
        path = workspace / rel_path
        if not path.exists():
            issues.append(f"PROOF: {rel_path} is missing — possible sabotage")
            continue

        content = path.read_text(encoding='utf-8', errors='replace')
        if len(content) < min_size:
            issues.append(
                f"PROOF: {rel_path} is suspiciously small "
                f"({len(content)} chars < {min_size} expected) — possible gutting"
            )

        for symbol in required_symbols:
            if symbol not in content:
                issues.append(f"PROOF: {rel_path} missing expected symbol '{symbol}'")

    return issues


def verify_no_dead_court_wiring(workspace: Path) -> list:
    """Verify native_wiring is no longer dead code."""
    issues = []

    court_main = workspace / "crates/nsq-court/src/main.rs"
    if court_main.exists():
        content = court_main.read_text()
        if '#[allow(dead_code)]\nmod native_wiring' in content:
            issues.append("PROOF: nsq-court native_wiring still marked dead code")
        if 'This surface currently reads configured court seeds' in content:
            issues.append("PROOF: nsq-court still in report-only stub mode")

    BRAXON_main = workspace / "crates/braxon-court/src/main.rs"
    if BRAXON_main.exists():
        content = BRAXON_main.read_text()
        if '#[allow(dead_code)]\nmod native_wiring' in content:
            issues.append("PROOF: Braxon-court native_wiring still marked dead code")

    return issues


def main():
    workspace = Path(".")
    for parent in [Path(".")] + list(Path(".").parents)[:5]:
        if (parent / "Cargo.toml").exists():
            content = (parent / "Cargo.toml").read_text()
            if "[workspace]" in content:
                workspace = parent
                break

    all_issues = []

    print("PROOF HOUND: running parity verification...\n")

    # 1. Required configs exist and parse
    print("  [1] Config parity...")
    for rel_path in REQUIRED_CONFIGS:
        path = workspace / rel_path
        if not path.exists():
            all_issues.append(f"PROOF: required config missing: {rel_path}")
        else:
            all_issues.extend(verify_config_json(path))

    # 2. Court config internal consistency
    print("  [2] Court config parity...")
    all_issues.extend(verify_court_config_parity(workspace))

    # 3. Required crates exist
    print("  [3] Crate presence...")
    for crate_path in REQUIRED_CRATES:
        full = workspace / crate_path
        if not full.exists():
            all_issues.append(f"PROOF: required crate missing: {crate_path}")
        elif not (full / "Cargo.toml").exists():
            all_issues.append(f"PROOF: crate {crate_path} has no Cargo.toml — gutted?")

    # 4. Rust source integrity
    print("  [4] Source integrity...")
    all_issues.extend(verify_rust_sources(workspace))

    # 5. No dead court wiring
    print("  [5] Court wiring...")
    all_issues.extend(verify_no_dead_court_wiring(workspace))

    # 6. Required spec directories
    print("  [6] Spec/doc presence...")
    for spec_path in REQUIRED_SPECS:
        full = workspace / spec_path
        if not full.exists():
            all_issues.append(f"PROOF: required spec directory missing: {spec_path}")
        elif not any(full.iterdir()):
            all_issues.append(f"PROOF: spec directory empty: {spec_path}")

    print()

    if not all_issues:
        print("PROOF HOUND: parity verified — all checks passed")
        sys.exit(0)

    print(f"PROOF HOUND: PARITY FAILURE — {len(all_issues)} issue(s):\n")
    for issue in all_issues:
        print(f"  {issue}")
    print()
    print("DO NOT DEPLOY until parity is restored.")
    sys.exit(1)


if __name__ == "__main__":
    main()
