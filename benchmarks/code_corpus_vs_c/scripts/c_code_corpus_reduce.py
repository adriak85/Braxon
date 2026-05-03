from pathlib import Path
import json
import subprocess

root = Path.home() / "Braxon" / "benchmarks" / "code_corpus_vs_c"
matrix = json.loads((root / "specs" / "code_corpus_matrix.json").read_text())
corpus_root = root / "corpus"
src_dir = root / "c_src"
out_root = root / "results" / "c"

subprocess.run(
    f'cc -O3 -std=gnu11 "{src_dir / "c_code_corpus_bench.c"}" -o "{src_dir / "c_code_corpus_bench"}"',
    shell=True,
    check=True
)

rows = []
for tier in matrix["tiers"].keys():
    files = sorted((corpus_root / tier).glob("*.code"))
    for repeat in range(1, matrix["repeats"] + 1):
        out_json = out_root / f"{tier}_repeat_{repeat}.json"
        cmd = [str(src_dir / "c_code_corpus_bench"), str(out_json)] + [str(p) for p in files]
        subprocess.run(cmd, check=True)
        rep = json.loads(out_json.read_text())
        sec = rep["elapsed_ms"] / 1000.0 if rep["elapsed_ms"] else None
        units = rep["readable_output_lines"] + rep["structural_nodes"] + rep["relation_edges"]
        rows.append({
            "system": "c",
            "tier": tier,
            "repeat": repeat,
            "elapsed_ms": rep["elapsed_ms"],
            "readable_output_bytes": rep["readable_output_bytes"],
            "readable_output_lines": rep["readable_output_lines"],
            "readable_semantic_units": units,
            "structural_nodes": rep["structural_nodes"],
            "relation_edges": rep["relation_edges"],
            "information_per_second_bytes": (rep["readable_output_bytes"] / sec) if sec else None,
            "information_per_second_lines": (rep["readable_output_lines"] / sec) if sec else None,
            "information_per_second_units": (units / sec) if sec else None,
            "deterministic_repeat_match": rep["deterministic_repeat_match"],
            "failure_mode": rep["failure_mode"]
        })

out = out_root / "c_code_corpus_score.json"
out.write_text(json.dumps({"version": 1, "rows": rows}, indent=2))
print(json.dumps({"version": 1, "rows": rows}, indent=2))
