#!/usr/bin/env python3
from __future__ import annotations

import json
import shutil
import subprocess
from pathlib import Path


ROOT = Path.cwd()
REG = ROOT / "config/nsq/nsq_runtime_language_registry.json"
PLAT = ROOT / "config/nsq/nsq_runtime_platform_registry.json"


def load_json(path: Path) -> dict:
    if not path.exists():
        return {"_missing": str(path)}
    return json.loads(path.read_text())


def cargo_workspace_packages() -> set[str]:
    try:
        p = subprocess.run(
            ["cargo", "metadata", "--format-version", "1", "--no-deps"],
            capture_output=True,
            text=True,
            check=False,
        )
        if p.returncode != 0:
            return set()
        data = json.loads(p.stdout)
        return {pkg.get("name", "") for pkg in data.get("packages", [])}
    except Exception:
        return set()


def main() -> None:
    reg = load_json(REG)
    plat = load_json(PLAT)
    required = set(reg.get("required_core_surfaces", []))
    declared = {s.get("id") for s in reg.get("surfaces", []) if isinstance(s, dict)}
    missing_declared = sorted(required - declared)

    packages = cargo_workspace_packages()
    expected_crates = {
        "nsq-core",
        "nsq-runtime",
        "nsq-source",
        "nsq-lint",
        "nsq-proof",
        "nsq-pack",
        "nsq-inspect",
        "nsq-query",
        "nsq-index",
        "nsq-compress",
        "Braxon-core",
        "Braxon-ingest",
        "Braxon-cli",
    }
    missing_crates = sorted(expected_crates - packages)

    primary = plat.get("primary_platform")
    primary_obj = None
    for p in plat.get("platforms", []):
        if p.get("id") == primary:
            primary_obj = p
            break

    tool_status = {}
    if primary_obj:
        for tool in primary_obj.get("required_tools", []):
            tool_status[tool] = shutil.which(tool) is not None

    report = {
        "registry": str(REG),
        "platform_registry": str(PLAT),
        "required_language_surface_count": len(required),
        "declared_surface_count": len(declared),
        "missing_required_surface_declarations": missing_declared,
        "expected_runtime_crates_missing_from_workspace": missing_crates,
        "primary_platform": primary,
        "primary_required_tool_status": tool_status,
        "ok": not missing_declared and not missing_crates and all(tool_status.values()),
        "note": "This verifies runtime registry coverage and visible crate/platform scaffolding. It does not claim every parser is complete."
    }
    print(json.dumps(report, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
