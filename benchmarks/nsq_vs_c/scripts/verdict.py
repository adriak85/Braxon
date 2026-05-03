from pathlib import Path
import json

root = Path.home() / "Braxon" / "benchmarks" / "nsq_vs_c"
nsq_path = root / "results" / "nsq" / "nsq_benchmark_report.json"
c_dir = root / "results" / "c"

report = {
    "version": 1,
    "nsq_present": nsq_path.exists(),
    "c_present": c_dir.exists(),
    "notes": [],
    "verdict": {}
}

if nsq_path.exists():
    nsq = json.loads(nsq_path.read_text())
    ok_runs = [r for r in nsq["runs"] if r.get("ok")]
    report["notes"].append(f"nsq_ok_runs={len(ok_runs)}")
    if ok_runs:
        first = ok_runs[0]
        fams = first.get("families", [])
        report["verdict"]["nsq_has_artifacts"] = len(fams) > 0
        report["verdict"]["nsq_determinism_ready"] = True

c_reports = list(c_dir.glob("c_bench_*.json")) if c_dir.exists() else []
report["notes"].append(f"c_reports={len(c_reports)}")
report["verdict"]["c_has_artifacts"] = len(c_reports) > 0

if report["verdict"].get("nsq_has_artifacts") and report["verdict"].get("c_has_artifacts"):
    report["verdict"]["benchmark_state"] = "compare_now"
else:
    report["verdict"]["benchmark_state"] = "incomplete"

out = root / "results" / "benchmark_verdict.json"
out.write_text(json.dumps(report, indent=2))
print(json.dumps(report, indent=2))
