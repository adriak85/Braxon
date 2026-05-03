#!/usr/bin/env python3
import argparse
import hashlib
import json
import os
import re
import subprocess
import sys
import time
from collections import defaultdict, deque
from datetime import datetime, timezone
from pathlib import Path

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

GENERATED_SOURCE_SKIP_PREFIXES = (
    "state/nsq/stamps/libraries/",
    "state/nsq/stamps/registry/",
    "state/nsq/stamps/indices/",
    "state/nsq/asm_macro_builder/harm_report_",
    "state/nsq/quarantine/",
    "state/nsq/metadata_law/snapshots/",
    "state/nsq/metadata_law/impact/",
    "state/nsq/metadata_law/current/",
)

PATH_RE = re.compile(
    r'(?P<path>(?:apps|config|specs|docs|crates|state|tools|bin|src|assets|models|benchmarks)/[A-Za-z0-9._/\-]+)'
)

RUST_MOD_RE = re.compile(r'^\s*(?:pub\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*;', re.M)

def now():
    return datetime.now(timezone.utc).isoformat()

def stamp():
    return datetime.now().strftime("%Y%m%d_%H%M%S")

def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()

def rel_to_root(root: Path, path: Path) -> str:
    return str(path.relative_to(root)).replace("\\", "/")

def is_generated_source(rel: str) -> bool:
    return any(rel.startswith(prefix) for prefix in GENERATED_SOURCE_SKIP_PREFIXES)

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

def surface_for(path: str) -> str:
    ext = Path(path).suffix.lower()
    return {
        ".nsq": "nsq",
        ".s": "asm",
        ".asm": "asm",
        ".rs": "rust",
        ".c": "c",
        ".h": "c_header",
        ".cpp": "cpp",
        ".hpp": "cpp_header",
        ".py": "python",
        ".sh": "shell",
        ".bash": "shell",
        ".zsh": "shell",
        ".toml": "toml",
        ".xml": "xml",
        ".json": "json",
        ".jsonl": "jsonl",
        ".md": "markdown",
        ".txt": "text",
        ".yaml": "yaml",
        ".yml": "yaml",
        ".ts": "typescript",
        ".tsx": "typescript",
        ".js": "javascript",
        ".jsx": "javascript",
        ".html": "html",
        ".css": "css",
        ".sql": "sql",
        ".csv": "csv",
        ".tsv": "tsv",
    }.get(ext, ext[1:] if ext else "no_extension")

def library_for(rel: str) -> str:
    parts = rel.split("/")
    if len(parts) >= 2 and parts[0] == "crates":
        return f"crates__{parts[1]}"
    if rel.startswith("apps/nsq/"):
        return "apps__nsq"
    if rel.startswith("config/nsq/"):
        return "config__nsq"
    if rel.startswith("specs/nsq/"):
        return "specs__nsq"
    if rel.startswith("docs/nsq/"):
        return "docs__nsq"
    if rel.startswith("tools/nsq_"):
        return "tools__" + (parts[1] if len(parts) > 1 else "nsq")
    if rel.startswith("state/nsq/"):
        return "state__nsq"
    return "repo__" + (parts[0] if parts else "root")

def authority_role(rel: str) -> str:
    if rel.startswith("apps/nsq/") and rel.endswith(".nsq"):
        return "nsq_carrier"
    if rel.startswith("config/nsq/") and rel.endswith(".nsq"):
        return "nsq_config"
    if rel.startswith("specs/nsq/"):
        return "law_spec"
    if rel.startswith("tools/nsq_") or rel.startswith("bin/nsq-"):
        return "support_tool"
    if rel.startswith("state/nsq/"):
        return "generated_index"
    if rel.startswith("crates/nsq-"):
        return "nsq_crate_source"
    return "source_authority"

def iter_repo_files(root: Path, max_bytes: int):
    for dirpath, dirnames, filenames in os.walk(root):
        dpath = Path(dirpath)
        rel_dir = rel_to_root(root, dpath) if dpath != root else ""
        dirnames[:] = [d for d in dirnames if d not in SKIP_DIRS]
        if rel_dir and is_generated_source(rel_dir + "/"):
            dirnames[:] = []
            continue
        for fn in filenames:
            path = dpath / fn
            rel = rel_to_root(root, path)
            if is_generated_source(rel):
                continue
            if is_text_file(path, max_bytes):
                yield path, rel

def existing_rel(root: Path, candidate: Path):
    try:
        if candidate.exists() and candidate.is_file():
            return rel_to_root(root, candidate.resolve())
    except OSError:
        pass
    return None

def extract_dependencies(root: Path, path: Path, rel: str, text: str):
    deps = set()

    for m in PATH_RE.finditer(text):
        dep = m.group("path").rstrip('",\')]}')
        if dep != rel and (root / dep).exists():
            deps.add(dep)

    if rel.endswith("Cargo.toml") and rel.startswith("crates/"):
        crate_root = path.parent
        for local in ["src/lib.rs", "src/main.rs", "build.rs"]:
            dep = existing_rel(root, crate_root / local)
            if dep and dep != rel:
                deps.add(dep)

    if path.suffix.lower() == ".rs":
        for m in RUST_MOD_RE.finditer(text):
            name = m.group(1)
            for local in [path.parent / f"{name}.rs", path.parent / name / "mod.rs"]:
                dep = existing_rel(root, local)
                if dep and dep != rel:
                    deps.add(dep)

    if path.suffix.lower() == ".nsq":
        for key in ["source_carrier", "compiler_carrier", "generated_authority", "library_stamp_root", "registry_root", "index_root"]:
            pat = re.compile(rf'{re.escape(key)}\s*=\s*([A-Za-z0-9._/\-]+)')
            for m in pat.finditer(text):
                dep = m.group(1).strip()
                if dep != rel and (root / dep).exists() and (root / dep).is_file():
                    deps.add(dep)

    return sorted(deps)

def snapshot(root: Path, out: Path, max_bytes: int):
    st = stamp()
    snap_dir = root / "state/nsq/metadata_law/snapshots" / st
    snap_dir.mkdir(parents=True, exist_ok=True)

    inventory_path = snap_dir / "inventory.jsonl"
    edges_path = snap_dir / "dependency_edges.jsonl"
    reverse_path = snap_dir / "reverse_dependency_edges.jsonl"
    summary_path = snap_dir / "summary.json"

    inventory = []
    edges = []
    started = time.time()

    with inventory_path.open("w", encoding="utf-8") as inv_f, edges_path.open("w", encoding="utf-8") as edge_f:
        for path, rel in iter_repo_files(root, max_bytes):
            data = path.read_bytes()
            text = data.decode("utf-8", errors="replace")
            deps = extract_dependencies(root, path, rel, text)

            row = {
                "path": rel,
                "sha256": sha256_bytes(data),
                "bytes": len(data),
                "lines": text.count("\n") + 1,
                "surface": surface_for(rel),
                "library": library_for(rel),
                "authority_role": authority_role(rel),
                "metadata_class": "source_tracking_identity",
                "generated_source_excluded": False,
                "tracked_at": now(),
            }
            inventory.append(row)
            inv_f.write(json.dumps(row, sort_keys=True) + "\n")

            for dep in deps:
                e = {
                    "from": rel,
                    "to": dep,
                    "edge": "depends_on",
                    "tracked_at": now(),
                }
                edges.append(e)
                edge_f.write(json.dumps(e, sort_keys=True) + "\n")

    reverse = defaultdict(list)
    for e in edges:
        reverse[e["to"]].append(e["from"])

    with reverse_path.open("w", encoding="utf-8") as f:
        for dep, parents in sorted(reverse.items()):
            for parent in sorted(set(parents)):
                f.write(json.dumps({
                    "from": dep,
                    "to": parent,
                    "edge": "is_required_by",
                    "tracked_at": now(),
                }, sort_keys=True) + "\n")

    by_surface = defaultdict(int)
    by_role = defaultdict(int)
    total_bytes = 0
    for row in inventory:
        by_surface[row["surface"]] += 1
        by_role[row["authority_role"]] += 1
        total_bytes += row["bytes"]

    summary = {
        "schema": "nsq.global_metadata_snapshot.v1",
        "snapshot": st,
        "generated_at": now(),
        "inventory": str(inventory_path.relative_to(root)),
        "dependency_edges": str(edges_path.relative_to(root)),
        "reverse_dependency_edges": str(reverse_path.relative_to(root)),
        "tracked_files": len(inventory),
        "dependency_edges_count": len(edges),
        "reverse_dependency_roots": len(reverse),
        "source_bytes_total": total_bytes,
        "source_mib_total": round(total_bytes / (1024 * 1024), 3),
        "by_surface": dict(sorted(by_surface.items())),
        "by_authority_role": dict(sorted(by_role.items())),
        "elapsed_seconds": round(time.time() - started, 3),
        "generated_noise_excluded": True,
    }

    summary_path.write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    current = root / "state/nsq/metadata_law/current"
    current.mkdir(parents=True, exist_ok=True)
    (current / "snapshot_path.txt").write_text(str(snap_dir.relative_to(root)) + "\n", encoding="utf-8")
    (current / "summary.json").write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    print(json.dumps(summary, indent=2, sort_keys=True))
    return summary

def load_current_snapshot(root: Path):
    p = root / "state/nsq/metadata_law/current/snapshot_path.txt"
    if not p.exists():
        raise SystemExit("missing current snapshot; run snapshot first")
    snap_dir = root / p.read_text().strip()
    inv = {}
    inv_path = snap_dir / "inventory.jsonl"
    edge_path = snap_dir / "dependency_edges.jsonl"
    if not inv_path.exists():
        raise SystemExit(f"missing inventory: {inv_path}")

    for line in inv_path.read_text(errors="replace").splitlines():
        if line.strip():
            row = json.loads(line)
            inv[row["path"]] = row

    edges = []
    if edge_path.exists():
        for line in edge_path.read_text(errors="replace").splitlines():
            if line.strip():
                edges.append(json.loads(line))

    return snap_dir, inv, edges

def live_inventory(root: Path, max_bytes: int):
    live = {}
    for path, rel in iter_repo_files(root, max_bytes):
        data = path.read_bytes()
        live[rel] = {
            "path": rel,
            "sha256": sha256_bytes(data),
            "bytes": len(data),
            "surface": surface_for(rel),
            "library": library_for(rel),
            "authority_role": authority_role(rel),
        }
    return live

def transitive_affected(changed, edges, max_depth: int):
    reverse = defaultdict(set)
    for e in edges:
        reverse[e["to"]].add(e["from"])

    affected = []
    seen = set(changed)
    q = deque((c, 0) for c in changed)

    while q:
        node, depth = q.popleft()
        if depth >= max_depth:
            continue
        for parent in sorted(reverse.get(node, [])):
            if parent in seen:
                continue
            seen.add(parent)
            affected.append({
                "path": parent,
                "reason": f"depends_on_changed:{node}",
                "depth": depth + 1,
                "status": "stale_due_to_dependency",
            })
            q.append((parent, depth + 1))

    return affected

def impact(root: Path, out: Path, max_bytes: int, max_depth: int, changed_args):
    st = stamp()
    impact_dir = root / "state/nsq/metadata_law/impact"
    impact_dir.mkdir(parents=True, exist_ok=True)

    snap_dir, old, edges = load_current_snapshot(root)
    live = live_inventory(root, max_bytes)

    changed = []
    added = []
    removed = []

    if changed_args:
        changed = [c.strip().replace("\\", "/") for c in changed_args if c.strip()]
    else:
        for path, row in live.items():
            if path not in old:
                added.append(path)
            elif old[path]["sha256"] != row["sha256"]:
                changed.append(path)
        for path in old:
            if path not in live:
                removed.append(path)

    affected = transitive_affected(changed + removed, edges, max_depth=max_depth)

    actions = []
    for path in sorted(set(changed + added + removed)):
        role = live.get(path, old.get(path, {})).get("authority_role", "unknown")
        if role in {"nsq_carrier", "nsq_config", "law_spec"}:
            actions.append({"path": path, "action": "rerun_law_doctor_and_metadata_snapshot"})
        elif role in {"support_tool", "nsq_crate_source"}:
            actions.append({"path": path, "action": "rerun_tool_or_crate_proof"})
        elif role == "generated_index":
            actions.append({"path": path, "action": "verify_generated_lineage_not_source_authority"})
        else:
            actions.append({"path": path, "action": "refresh_metadata_and_check_dependents"})

    for row in affected:
        actions.append({"path": row["path"], "action": "mark_stale_until_dependency_refresh", "reason": row["reason"]})

    report = {
        "schema": "nsq.global_metadata_impact.v1",
        "generated_at": now(),
        "source_snapshot": str(snap_dir.relative_to(root)),
        "changed": sorted(set(changed)),
        "added": sorted(set(added)),
        "removed": sorted(set(removed)),
        "affected": affected,
        "action_count": len(actions),
        "actions": actions,
        "max_depth": max_depth,
        "generated_noise_excluded": True,
    }

    json_path = impact_dir / f"impact_{st}.json"
    txt_path = impact_dir / f"impact_{st}.txt"

    json_path.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    with txt_path.open("w", encoding="utf-8") as f:
        f.write("== NSQ global metadata impact ==\n")
        f.write(f"source_snapshot={report['source_snapshot']}\n")
        f.write(f"changed={len(report['changed'])}\n")
        f.write(f"added={len(report['added'])}\n")
        f.write(f"removed={len(report['removed'])}\n")
        f.write(f"affected={len(report['affected'])}\n")
        f.write(f"actions={len(report['actions'])}\n\n")

        for label in ["changed", "added", "removed"]:
            f.write(f"== {label} ==\n")
            for item in report[label][:200]:
                f.write(f"{item}\n")
            f.write("\n")

        f.write("== affected ==\n")
        for row in affected[:300]:
            f.write(f"{row['path']} :: {row['status']} :: {row['reason']} :: depth={row['depth']}\n")

        f.write("\n== actions ==\n")
        for row in actions[:300]:
            f.write(json.dumps(row, sort_keys=True) + "\n")

    print(json.dumps({
        "ok": True,
        "impact_json": str(json_path.relative_to(root)),
        "impact_txt": str(txt_path.relative_to(root)),
        "changed": len(report["changed"]),
        "added": len(report["added"]),
        "removed": len(report["removed"]),
        "affected": len(report["affected"]),
        "actions": len(actions),
    }, indent=2, sort_keys=True))

def doctor(root: Path):
    required = [
        "apps/nsq/global_metadata_law.nsq",
        "config/nsq/global_metadata_law.nsq",
        "specs/nsq/NSQ_GLOBAL_METADATA_LAW.md",
        "tools/nsq_metadata_law/nsq_metadata_law.py",
        "bin/nsq-metadata-law",
    ]
    missing = [p for p in required if not (root / p).exists()]
    current = root / "state/nsq/metadata_law/current/summary.json"
    ok = not missing and current.exists()

    print(json.dumps({
        "ok": ok,
        "missing": missing,
        "current_snapshot_present": current.exists(),
        "law": "alteration_creates_impact_event",
        "generated_reports_source_authority": False,
        "metadata_role": "tracking_identity_lineage_impact",
    }, indent=2, sort_keys=True))

    if not ok:
        raise SystemExit(1)

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("command", choices=["snapshot", "impact", "doctor"])
    ap.add_argument("--root", default=os.environ.get("BRAXON_ROOT", str(Path.home() / "Braxon")))
    ap.add_argument("--out", default=None)
    ap.add_argument("--max-bytes", type=int, default=int(os.environ.get("NSQ_METADATA_MAX_FILE_BYTES", "1048576")))
    ap.add_argument("--max-depth", type=int, default=8)
    ap.add_argument("--changed", action="append", default=[])
    args = ap.parse_args()

    root = Path(args.root).resolve()
    out = Path(args.out).resolve() if args.out else root / "state/nsq/metadata_law"
    out.mkdir(parents=True, exist_ok=True)

    if args.command == "snapshot":
        snapshot(root, out, args.max_bytes)
    elif args.command == "impact":
        impact(root, out, args.max_bytes, args.max_depth, args.changed)
    elif args.command == "doctor":
        doctor(root)

if __name__ == "__main__":
    main()
