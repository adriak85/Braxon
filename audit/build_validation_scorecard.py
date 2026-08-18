import json
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MATRIX_PATH = ROOT / "audit" / "validation_campaign_matrix.json"
OUT_JSON = ROOT / "audit" / "validation_campaign_scorecard.json"
OUT_MD = ROOT / "audit" / "validation_campaign_scorecard.md"

matrix = json.loads(MATRIX_PATH.read_text())
gates = matrix["gates"]
counts = {}
for gate in gates:
    status = gate["status"]
    counts[status] = counts.get(status, 0) + 1
executed = counts.get("PROVEN", 0) + counts.get("MEASURED", 0) + counts.get("EQUIVALENT", 0)
commit = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip()
remote = subprocess.check_output(["git", "rev-parse", "origin/reconstruction"], cwd=ROOT, text=True).strip()
scorecard = {
    "schema": "braxon.validation_campaign_scorecard.v1",
    "branch": "reconstruction",
    "commit": commit,
    "origin_reconstruction": remote,
    "gate_count": len(gates),
    "status_counts": counts,
    "executed_or_evidenced_gate_count": executed,
    "benchmark_coverage_percent": round(executed * 100.0 / len(gates), 2),
    "architecture_completeness_percent": None,
    "native_semantic_coverage_percent": None,
    "execution_coverage_percent": None,
    "equivalence_coverage_percent": None,
    "physical_validation": "BLOCKED",
    "guile_migration": "BLOCKED",
    "zig_migration": "BLOCKED",
    "android_acceptance": "BLOCKED",
    "wowas_universal_global_compliance": "BLOCKED",
    "training_advantage": "NOT_ESTABLISHED",
    "remaining_blocker_gate_ids": [gate["id"] for gate in gates if gate["status"] == "BLOCKED"],
    "remaining_theoretical_gate_ids": [gate["id"] for gate in gates if gate["status"] == "THEORETICAL"],
    "policy": "Null percentages are intentional where no defensible denominator exists. Statuses are evidence classifications, not marketing scores.",
}
OUT_JSON.write_text(json.dumps(scorecard, indent=2) + "\n")
rows = []
for gate in gates:
    evidence = "; ".join(gate.get("evidence", [])) or "—"
    rows.append(f"| {gate['id']} | {gate['name']} | {gate['status']} | {evidence} | {gate['scope']} |")
md = """# Reconstruction Validation Campaign Scorecard\n\nThis scorecard is generated from `validation_campaign_matrix.json`. It classifies evidence conservatively: a gate is not marked proven or equivalent without executable support, and physical/device/model/migration gates remain blocked when they were not actually run.\n\n| Metric | Value |\n|---|---:|\n| Campaign gates | {gate_count} |\n| Proven | {proven} |\n| Measured | {measured} |\n| Equivalent | {equivalent} |\n| Blocked | {blocked} |\n| Theoretical | {theoretical} |\n| Executed or evidenced gate coverage | {coverage}% |\n| Physical validation | BLOCKED |\n| Guile migration | BLOCKED |\n| Zig migration | BLOCKED |\n| Android 16 acceptance | BLOCKED |\n| WOWAS universal/global compliance | BLOCKED |\n| Real training advantage | NOT ESTABLISHED |\n\nThe current repository evidence establishes a coherent native execution mechanism and several measured scaling properties. It does not establish whole-system semantic equivalence, real-device acceptance, or real-model training acceleration.\n\n## Gate Matrix\n\n| ID | Gate | Status | Evidence | Scope and limitation |\n|---:|---|---|---|---|\n{rows}\n\n## Reproducibility\n\n| Field | Value |\n|---|---|\n| Branch | reconstruction |\n| Commit | {commit} |\n| Origin commit | {remote} |\n""".format(
    gate_count=len(gates),
    proven=counts.get("PROVEN", 0),
    measured=counts.get("MEASURED", 0),
    equivalent=counts.get("EQUIVALENT", 0),
    blocked=counts.get("BLOCKED", 0),
    theoretical=counts.get("THEORETICAL", 0),
    coverage=scorecard["benchmark_coverage_percent"],
    rows="\n".join(rows),
    commit=commit,
    remote=remote,
)
OUT_MD.write_text(md)
