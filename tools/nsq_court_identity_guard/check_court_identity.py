#!/usr/bin/env python3
from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path.cwd()
FINDINGS: list[dict] = []

REQUIRED = [
    "apps/nsq/court_macro_registry.nsq",
    "apps/nsq/nsq_court_whole_architecture.nsq",
    "config/nsq/nsq_court_whole_architecture.json",
    "state/nsq/court/route_registry.json",
    "state/nsq/court/routes/BRAXON_model_downloader.json",
    "state/nsq/court/routes/semantic_benchmark.json",
    "state/nsq/court/routes/bare_tasker.json",
]

COURT_FILES = [
    "apps/nsq/court_macro_registry.nsq",
    "apps/nsq/nsq_court_whole_architecture.nsq",
    "config/braxon_court.json",
    "config/nsq_court.json",
    "config/kingdom/court_canonical.json",
    "config/nsq/nsq_court_whole_architecture.json",
    "state/nsq/court/bare_metal_macro_equipment.json",
    "state/nsq/court/macro_registry_current.json",
    "state/nsq/court/route_registry.json",
    "state/nsq/court/routes/BRAXON_model_downloader.json",
    "state/nsq/court/routes/semantic_benchmark.json",
    "state/nsq/court/routes/bare_tasker.json",
    "specs/court/COURT_CONSTITUTION.md",
    "specs/nsq/court_of_archons.md",
    "specs/nsq/court_prime.md",
    "specs/nsq/NSQ_COURT_WHOLE_ARCHITECTURE.md",
    "crates/braxon-kingdom-generate/src/main.rs",
]

FORBIDDEN_PATHS = [
    "bin/nsq-semantic-task-runner",
    "tools/nsq_semantic_benchmark/nsq_semantic_task_runner.c",
    "state/nsq/quarantine/c_reference_runner_legacy",
]

GENERIC_DRIFT = re.compile(r"\b(primary_component|lint_component)\b", re.IGNORECASE)

FORBIDDEN_TEXT = [
    "native_c_semantic_reference",
    "c_runner_allowed_for_acceptance=true",
    '"c_runner_allowed_for_acceptance": true',
    "c_reference_runner_acceptance = allowed",
    '"c_reference_runner_acceptance": true',
    "c_reference_runner_storage = true",
    '"c_reference_runner_storage": true',
    "direct_acceptance_runner_allowed = true",
    '"direct_acceptance_runner_allowed": true',
    "duplicate_task_systems_allowed = true",
    '"duplicate_task_systems_allowed": true',
    "court_is_agents = true",
    '"court_is_agents": true',
]

SCAN_TEXT_FILES = [
    "apps/nsq",
    "config/nsq",
    "specs/nsq",
    "state/nsq/court",
    "state/nsq/perpetual_runtime/current",
    "state/nsq/semantic_benchmark/current/report.json",
    "state/nsq/semantic_benchmark/current/report.txt",
    "tools/nsq_semantic_benchmark",
    "bin/nsq-semantic-runtime-bench",
]

SKIP_PARTS = {
    ".git",
    "target",
    "crates/wowas-final-edition-v10",
    "state/nsq/metadata_law/snapshots",
    "state/nsq/metadata_law/impact",
}

SELF = "tools/nsq_court_identity_guard/check_court_identity.py"

def rel(path: Path) -> str:
    try:
        return path.relative_to(ROOT).as_posix()
    except Exception:
        return str(path)

def read(path: Path) -> str:
    try:
        return path.read_text(errors="replace")
    except Exception:
        return ""

def add(kind: str, path: str, line: int, text: str) -> None:
    FINDINGS.append({"kind": kind, "path": path, "line": line, "text": text})

def skip(path: Path) -> bool:
    r = rel(path)
    if r == SELF:
        return True
    return any(part in r for part in SKIP_PARTS)

for p in REQUIRED:
    if not (ROOT / p).exists():
        add("missing_required_court_surface", p, 0, "required")

for p in FORBIDDEN_PATHS:
    if (ROOT / p).exists():
        add("forbidden_c_runner_or_storage_present", p, 0, "remove it; do not quarantine it")

macro = ROOT / "apps/nsq/court_macro_registry.nsq"
if macro.exists():
    text = read(macro)
    if "king = compositor" not in text:
        add("missing_king_compositor_binding", rel(macro), 0, "king = compositor required")
    if "queen = linter" not in text:
        add("missing_queen_linter_binding", rel(macro), 0, "queen = linter required")
    if "court_is_agents = false" not in text and '"court_is_agents": false' not in text:
        add("missing_court_is_agents_false", rel(macro), 0, "court_is_agents=false required")

registry_path = ROOT / "state/nsq/court/route_registry.json"
if registry_path.exists():
    try:
        registry = json.loads(read(registry_path))
        if registry.get("authority") != "NSQ_COURT":
            add("court_authority_not_nsq_court", rel(registry_path), 0, str(registry.get("authority")))
        if registry.get("architecture_root") is not True:
            add("court_not_architecture_root", rel(registry_path), 0, "architecture_root must be true")
        if registry.get("king") != "compositor":
            add("missing_king_compositor_binding", rel(registry_path), 0, f"king={registry.get('king')!r}")
        if registry.get("queen") != "linter":
            add("missing_queen_linter_binding", rel(registry_path), 0, f"queen={registry.get('queen')!r}")
        if registry.get("court_is_agents") is not False:
            add("court_must_not_be_agents", rel(registry_path), 0, f"court_is_agents={registry.get('court_is_agents')!r}")
        if registry.get("duplicate_task_systems_allowed") is not False:
            add("duplicate_task_systems_not_closed", rel(registry_path), 0, "must be false")
        if registry.get("c_reference_runner_storage") is not False:
            add("c_reference_storage_not_forbidden", rel(registry_path), 0, "must be false")
    except Exception as err:
        add("route_registry_parse_error", rel(registry_path), 0, repr(err))

for relname in COURT_FILES:
    p = ROOT / relname
    if not p.exists() or not p.is_file() or skip(p):
        continue
    text = read(p)
    for idx, line in enumerate(text.splitlines(), 1):
        if GENERIC_DRIFT.search(line):
            add("generic_court_reroute_drift", relname, idx, line.strip())

for scan in SCAN_TEXT_FILES:
    base = ROOT / scan
    if not base.exists():
        continue
    files = [base] if base.is_file() else [p for p in base.rglob("*") if p.is_file()]
    for path in files:
        if skip(path):
            continue
        r = rel(path)
        if path.suffix in {".png", ".jpg", ".jpeg", ".webp", ".bin", ".elf"}:
            continue
        text = read(path)
        for idx, line in enumerate(text.splitlines(), 1):
            stripped = line.strip()
            for bad in FORBIDDEN_TEXT:
                if bad in stripped:
                    add("forbidden_acceptance_or_c_reference_text", r, idx, stripped)

if FINDINGS:
    print("NSQ court identity guard: FAIL")
    print("Required: authority=NSQ_COURT, architecture_root=true, king=compositor, queen=linter, court_is_agents=false.")
    print("Generic reroute terms primary_component/lint_component are drift in NSQ Court surfaces.")
    print("C runner acceptance/storage/quarantine is forbidden.")
    print("failure_count=" + str(len(FINDINGS)))
    for f in FINDINGS[:200]:
        print(f"{f['path']}:{f['line']}:{f['kind']}: {f['text']}")
    if len(FINDINGS) > 200:
        print(f"... {len(FINDINGS)-200} additional findings omitted from console display; the guard still fails on the complete finding set")
    sys.exit(1)

print("NSQ court identity guard: ok")
print("authority=NSQ_COURT")
print("architecture_root=true")
print("king=compositor")
print("queen=linter")
print("court_is_agents=false")
print("generic_reroute_terms=blocked")
print("duplicate_task_systems_allowed=false")
print("c_reference_runner_acceptance=false")
print("c_reference_runner_storage=false")
