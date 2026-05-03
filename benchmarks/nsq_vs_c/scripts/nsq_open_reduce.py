from pathlib import Path
import json
from statistics import median

root = Path.home() / "Braxon" / "benchmarks" / "nsq_vs_c"
src = root / "results" / "nsq" / "nsq_benchmark_report.json"

rep = json.loads(src.read_text())
runs = rep["runs"]

families = {}
for run in runs:
    for fam in run.get("families", []):
        name = fam["family"]
        families.setdefault(name, {
            "artifact_bytes": [],
            "decoded_bytes": [],
            "decoded_records": [],
            "run_ms": [],
            "hashes": [],
            "coverage": [],
            "failures": 0,
            "successes": 0
        })

        row = families[name]

        complete = (
            fam.get("artifact_bytes") is not None and
            fam.get("decoded_bytes") is not None and
            fam.get("decoded_records") is not None and
            fam.get("replay_sha256") is not None
        )

        if complete:
            row["successes"] += 1
            row["artifact_bytes"].append(fam["artifact_bytes"])
            row["decoded_bytes"].append(fam["decoded_bytes"])
            row["decoded_records"].append(fam["decoded_records"])
            row["run_ms"].append(run.get("super_run_ms"))
            row["hashes"].append(fam["replay_sha256"])

            coverage = 0
            if fam.get("unique_symbols") is not None:
                coverage += 1
            if fam.get("unique_macros") is not None:
                coverage += 1
            if fam.get("decoded_records") is not None:
                coverage += 1
            row["coverage"].append(coverage / 3.0)
        else:
            row["failures"] += 1

out_rows = []
for family, row in families.items():
    unique_hashes = sorted(set(row["hashes"]))
    deterministic = len(unique_hashes) <= 1 and len(unique_hashes) > 0
    out_rows.append({
        "system": "nsq",
        "family": family,
        "scale": "mixed_scales",
        "parse_ms": None,
        "build_ms": median(row["run_ms"]) if row["run_ms"] else None,
        "decode_ms": None,
        "artifact_bytes": median(row["artifact_bytes"]) if row["artifact_bytes"] else None,
        "decoded_bytes": median(row["decoded_bytes"]) if row["decoded_bytes"] else None,
        "decoded_records": median(row["decoded_records"]) if row["decoded_records"] else None,
        "replay_hash": unique_hashes[0] if deterministic and unique_hashes else None,
        "deterministic_repeat_match": deterministic,
        "semantic_coverage": median(row["coverage"]) if row["coverage"] else 0.0,
        "failure_mode": None if row["failures"] == 0 else f"partial_null_outputs:{row['failures']}",
        "successes": row["successes"],
        "failures": row["failures"]
    })

out = root / "results" / "nsq" / "nsq_open_score.json"
out.write_text(json.dumps({"version": 1, "rows": out_rows}, indent=2))
print(json.dumps({"version": 1, "rows": out_rows}, indent=2))
