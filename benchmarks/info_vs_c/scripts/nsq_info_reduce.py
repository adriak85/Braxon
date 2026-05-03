from pathlib import Path
import json

root = Path.home() / "Braxon"
src = root / "benchmarks" / "nsq_vs_c" / "results" / "nsq" / "nsq_benchmark_report.json"

if not src.exists():
    raise SystemExit(f"missing {src}")

rep = json.loads(src.read_text())
rows = []

def safe_read_text(path: Path):
    try:
        return path.read_text(errors="ignore")
    except Exception:
        return ""

for run in rep["runs"]:
    elapsed_ms = run.get("super_run_ms")
    for fam in run.get("families", []):
        inspect_path = Path(fam["inspect_path"]) if fam.get("inspect_path") else None
        readable = safe_read_text(inspect_path) if inspect_path and inspect_path.exists() else ""
        readable_bytes = len(readable.encode("utf-8"))
        readable_lines = len([x for x in readable.splitlines() if x.strip()])

        semantic_units = 0
        if fam.get("decoded_records") is not None:
            semantic_units += int(fam["decoded_records"])
        if fam.get("unique_symbols") is not None:
            semantic_units += int(fam["unique_symbols"])
        if fam.get("unique_macros") is not None:
            semantic_units += int(fam["unique_macros"])

        sec = (elapsed_ms / 1000.0) if elapsed_ms else None

        complete = (
            elapsed_ms is not None and
            readable_bytes > 0
        )

        rows.append({
            "system": "nsq",
            "family": fam.get("family"),
            "scale": run.get("scale_name"),
            "elapsed_ms": elapsed_ms,
            "readable_output_bytes": readable_bytes,
            "readable_output_lines": readable_lines,
            "readable_semantic_units": semantic_units,
            "information_per_second_bytes": (readable_bytes / sec) if complete and sec else None,
            "information_per_second_lines": (readable_lines / sec) if complete and sec else None,
            "information_per_second_units": (semantic_units / sec) if complete and sec else None,
            "deterministic_repeat_match": fam.get("replay_sha256") is not None,
            "failure_mode": None if complete else "missing_readable_output_or_elapsed"
        })

out = root / "benchmarks" / "info_vs_c" / "results" / "nsq" / "nsq_information_score.json"
out.parent.mkdir(parents=True, exist_ok=True)
out.write_text(json.dumps({"version": 1, "rows": rows}, indent=2))
print(json.dumps({"version": 1, "rows": rows}, indent=2))
