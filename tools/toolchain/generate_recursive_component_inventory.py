#!/usr/bin/env python3
"""Generate a bounded, exhaustive recursive component-improvement inventory.

Every tracked non-vendor file is covered by a deterministic Git-index aggregate.
Active implementation files receive individual records. Data, corpus, generated, and
historical material are grouped at stable repository component boundaries; each group
records the number of covered files and a SHA-256 over sorted path/blob pairs. This
keeps the repository synchronizable while allowing a clone to regenerate or verify
coverage without treating hundreds of thousands of copies as separate runtime data.
"""
from __future__ import annotations

import hashlib
import json
import os
import subprocess
from collections import defaultdict
from pathlib import Path, PurePosixPath

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "config/nsq/recursive_component_improvement_inventory.json"
IMPLEMENTATION_SUFFIXES = {
    ".rs", ".c", ".h", ".cc", ".cpp", ".cxx", ".m", ".mm", ".S", ".s",
    ".py", ".sh", ".bash", ".scm", ".ss", ".lisp", ".clj", ".zig", ".go",
    ".java", ".kt", ".js", ".ts", ".wat", ".ll", ".ml", ".mli", ".hs",
    ".erl", ".ex", ".exs", ".nim", ".dart", ".swift", ".f90", ".adb",
}
ACTIVE_CODE_PREFIXES = ("src/", "scripts/", "tools/", "crates/")
EXCLUDED_PREFIXES = ("vendor/", "target/")


def run(*args: str) -> str:
    return subprocess.check_output(args, cwd=ROOT, text=True).strip()


def index_entries() -> list[tuple[str, str]]:
    entries: list[tuple[str, str]] = []
    for line in run("git", "ls-files", "-s").splitlines():
        meta, path = line.split("\t", 1)
        blob = meta.split()[1]
        entries.append((path, blob))
    return entries


def component_bucket(path: str) -> str:
    parts = PurePosixPath(path).parts
    if not parts:
        return "repository-root"
    root = parts[0]
    if root == "crates":
        # A workspace crate is a component; canonical corpus is one component per crate.
        return "/".join(parts[:2]) if len(parts) >= 2 else root
    if root in {"src", "scripts", "tools", "config", "docs", "specs", ".cargo"}:
        return "/".join(parts[:2]) if len(parts) >= 2 else root
    if root in {"state", "reconstruction"}:
        # Preserve recursive boundaries without repeating every historical copy.
        return "/".join(parts[:3]) if len(parts) >= 3 else "/".join(parts)
    return "/".join(parts[:2]) if len(parts) >= 2 else root


def sha256_lines(lines: list[str]) -> str:
    digest = hashlib.sha256()
    for line in sorted(lines):
        digest.update(line.encode())
        digest.update(b"\n")
    return digest.hexdigest()


def marker_summary(text: str) -> dict[str, bool]:
    return {
        "process_execution": ("Command::new" in text or "subprocess." in text or "os.system" in text),
        "network_spelling": any(item in text for item in ("https://", "http://", "curl ", "wget ", "git clone")),
        "unsafe_or_ffi": ("unsafe" in text or "extern \"C\"" in text or "ctypes" in text),
        "todo_or_fixme": ("TODO" in text or "FIXME" in text),
        "resident_loop": any(item in text for item in ("loop {", "while True", "serve_forever")),
    }


def improvement_paths(scope: str, markers: dict[str, bool] | None = None) -> list[dict[str, str]]:
    flagged = ", ".join(name for name, present in (markers or {}).items() if present) or "component-boundary review"
    return [
        {
            "category": "abstraction_removal",
            "applicability": "mandatory_evidence_review",
            "action": "Trace semantic callers and routes; remove or fuse a layer only when before/after behavior, error paths, and provenance remain equivalent.",
            "completion_evidence": "route inventory and equivalence test",
        },
        {
            "category": "direct_functionality_and_performance",
            "applicability": "mandatory_evidence_review",
            "action": "Strengthen deterministic input, precondition, state-transition, output, error, and bounded-resource contracts; benchmark concrete operation rather than presentation.",
            "completion_evidence": "operation test and benchmark or semantic equivalence receipt",
        },
        {
            "category": "security_and_reproducibility",
            "applicability": "mandatory_evidence_review",
            "action": "Constrain effects to canonical local paths, checksummed inputs, locked sources, bounded resources, and explicit missing-artifact guidance; reject hidden network and ambient-toolchain behavior.",
            "completion_evidence": f"static-or-component markers={flagged}; clean-clone negative-path test",
        },
        {
            "category": "nsq_reflexor_integration",
            "applicability": "mandatory_evidence_review",
            "action": "Map operations to NSQ semantic identity, intent, preconditions, postconditions, target requirements, and Kinetic Reflexor capability; retain non-operational material as provenance only.",
            "completion_evidence": "declared capability or explicit provenance-only classification",
        },
        {
            "category": "provenance_and_private_eligibility",
            "applicability": "mandatory_evidence_review",
            "action": "Record origin, license, transformation, notices, and redistribution status. Private eligibility requires independent authorship and cannot be inferred from wrapping, renaming, or semantic translation.",
            "completion_evidence": "source-level provenance and license matrix decision",
        },
    ]


def source_and_component_records() -> tuple[list[dict[str, object]], dict[str, object]]:
    active: list[tuple[str, str]] = []
    grouped: dict[str, list[tuple[str, str]]] = defaultdict(list)
    all_covered: list[str] = []
    for path, blob in index_entries():
        if path.startswith(EXCLUDED_PREFIXES):
            continue
        all_covered.append(f"{path}\t{blob}")
        is_active_code = path.startswith(ACTIVE_CODE_PREFIXES) and PurePosixPath(path).suffix in IMPLEMENTATION_SUFFIXES
        if is_active_code:
            active.append((path, blob))
        else:
            grouped[component_bucket(path)].append((path, blob))

    records: list[dict[str, object]] = []
    for path, blob in active:
        file_path = ROOT / path
        try:
            text = file_path.read_text(encoding="utf-8", errors="replace")
            data = file_path.read_bytes()
            checksum = hashlib.sha256(data).hexdigest()
            metrics = {"bytes": len(data), "lines": text.count("\n") + (1 if text else 0), "static_markers": marker_summary(text)}
            state = "available"
        except FileNotFoundError:
            checksum = "missing_worktree_symlink_target"
            metrics = {"bytes": 0, "lines": 0, "static_markers": {}, "missing_worktree_target": True}
            state = "missing_local_artifact"
        records.append({
            "id": f"source:{path}",
            "kind": "active_implementation_source",
            "path": path,
            "git_blob": blob,
            "sha256": checksum,
            "materialization_state": state,
            "metrics": metrics,
            "private_eligibility": "undetermined_pending_provenance",
            "improvement_paths": improvement_paths(path, metrics.get("static_markers", {})),
        })

    for bucket, entries in sorted(grouped.items()):
        record_lines = [f"{path}\t{blob}" for path, blob in entries]
        records.append({
            "id": f"component:{bucket}",
            "kind": "recursive_repository_component_group",
            "component_boundary": bucket,
            "tracked_file_total": len(entries),
            "aggregate_sha256": sha256_lines(record_lines),
            "sample_paths": [path for path, _ in sorted(entries)[:12]],
            "private_eligibility": "undetermined_pending_component_provenance",
            "improvement_paths": improvement_paths(bucket),
        })

    coverage = {
        "covered_nonvendor_tracked_file_total": len(all_covered),
        "covered_nonvendor_git_index_sha256": sha256_lines(all_covered),
        "active_implementation_source_total": len(active),
        "recursive_component_group_total": len(grouped),
        "coverage_rule": "each non-vendor tracked file contributes exactly once either as an active implementation source or a recursive component group member",
    }
    return records, coverage


def dependency_records() -> list[dict[str, object]]:
    metadata = json.loads(run("cargo", "metadata", "--locked", "--offline", "--format-version", "1"))
    workspace_members = set(metadata["workspace_members"])
    records: list[dict[str, object]] = []
    for package in sorted(metadata["packages"], key=lambda item: (item["name"], item["version"])):
        if package["id"] in workspace_members:
            continue
        records.append({
            "id": f"dependency:{package['name']}@{package['version']}",
            "kind": "vendored_dependency",
            "source": package.get("source") or "path",
            "license": package.get("license") or "unresolved",
            "manifest_path": os.path.relpath(package["manifest_path"], ROOT),
            "private_eligibility": "not_private_eligible_without_independent_reimplementation",
            "improvement_paths": improvement_paths("vendored dependency"),
        })
    return records


def main() -> None:
    components, coverage = source_and_component_records()
    dependencies = dependency_records()
    inventory = {
        "schema": "braxon.recursive_component_improvement_inventory.v3",
        "authority": "BRAXON_RECURSIVE_COMPONENT_REVIEW",
        "owner": "Michael David Norris",
        "repository_commit": run("git", "rev-parse", "HEAD"),
        "generation_policy": {
            "all_applicable_improvement_pass_required": True,
            "minimum_improvement_paths_per_component": 5,
            "private_eligibility_requires_independent_authorship": True,
            "upstream_license_reclassification_forbidden": True,
            "abstraction_removal_requires_equivalence_evidence": True,
            "full_file_coverage_is_verified_by_git_index_aggregate": True,
        },
        "source_coverage": coverage,
        "vendored_dependency_total": len(dependencies),
        "components": components + dependencies,
    }
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(json.dumps(inventory, indent=2, sort_keys=True) + "\n")
    print(json.dumps({
        "output": OUT.relative_to(ROOT).as_posix(),
        **coverage,
        "vendored_dependency_total": len(dependencies),
        "component_record_total": len(components) + len(dependencies),
        "minimum_improvement_paths_per_component": 5,
    }))


if __name__ == "__main__":
    main()
