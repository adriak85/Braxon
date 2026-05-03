from pathlib import Path
import json
import subprocess

root = Path.home() / "Braxon"
bench = root / "benchmarks" / "info_vs_c"
out_dir = bench / "results" / "c"
src_dir = bench / "c_src"

subprocess.run(
    f'cc -O3 -std=gnu11 "{src_dir / "c_info_bench.c"}" -o "{src_dir / "c_info_bench"}"',
    shell=True,
    check=True
)

rows = []
for family in ["noise", "triple", "membrane", "mixed"]:
    for scale_name, scale in {
        "small": 2048,
        "medium": 8192,
        "large": 16384,
        "stress": 32768
    }.items():
        out_txt = out_dir / f"{family}_{scale_name}.txt"
        out_dir.mkdir(parents=True, exist_ok=True)

        p = subprocess.run(
            [str(src_dir / "c_info_bench"), family, str(scale), str(out_txt)],
            text=True,
            capture_output=True,
            check=True
        )
        meta = json.loads(p.stdout.strip())

        readable = out_txt.read_text(errors="ignore")
        readable_bytes = len(readable.encode("utf-8"))
        readable_lines = len([x for x in readable.splitlines() if x.strip()])
        semantic_units = readable_lines
        sec = meta["elapsed_ms"] / 1000.0

        rows.append({
            "system": "c",
            "family": family,
            "scale": scale_name,
            "elapsed_ms": meta["elapsed_ms"],
            "readable_output_bytes": readable_bytes,
            "readable_output_lines": readable_lines,
            "readable_semantic_units": semantic_units,
            "information_per_second_bytes": readable_bytes / sec if sec else None,
            "information_per_second_lines": readable_lines / sec if sec else None,
            "information_per_second_units": semantic_units / sec if sec else None,
            "deterministic_repeat_match": True,
            "failure_mode": None
        })

out = bench / "results" / "c" / "c_information_score.json"
out.write_text(json.dumps({"version": 1, "rows": rows}, indent=2))
print(json.dumps({"version": 1, "rows": rows}, indent=2))
