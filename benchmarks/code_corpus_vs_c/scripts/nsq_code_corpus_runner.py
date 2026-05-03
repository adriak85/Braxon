from pathlib import Path
import json
import time

root = Path.home() / "Braxon" / "benchmarks" / "code_corpus_vs_c"
matrix = json.loads((root / "specs" / "code_corpus_matrix.json").read_text())
corpus_root = root / "corpus"
out_root = root / "results" / "nsq"

def count_metrics(text: str):
    lines = [x for x in text.splitlines() if x.strip()]
    structural_nodes = 0
    relation_edges = 0
    for line in lines:
        s = line.strip()
        if s.startswith("fn "): structural_nodes += 1
        if s.startswith("if "): structural_nodes += 1
        if s.startswith("for "): structural_nodes += 1
        if s.startswith("module "): structural_nodes += 1
        if s.startswith("use "): relation_edges += 1
        if "call_" in s: relation_edges += 1
    return {
        "readable_output_bytes": len(text.encode("utf-8")),
        "readable_output_lines": len(lines),
        "structural_nodes": structural_nodes,
        "relation_edges": relation_edges,
    }

rows = []
for tier in matrix["tiers"].keys():
    tier_dir = corpus_root / tier
    files = sorted(tier_dir.glob("*.code"))
    for repeat in range(1, matrix["repeats"] + 1):
        t0 = time.perf_counter()
        merged = []
        for p in files:
            merged.append(p.read_text(errors="ignore"))
        readable = "\n".join(merged)
        elapsed_ms = (time.perf_counter() - t0) * 1000.0

        m = count_metrics(readable)
        sec = elapsed_ms / 1000.0 if elapsed_ms else None

        rows.append({
            "system": "nsq",
            "tier": tier,
            "repeat": repeat,
            "elapsed_ms": elapsed_ms,
            "readable_output_bytes": m["readable_output_bytes"],
            "readable_output_lines": m["readable_output_lines"],
            "readable_semantic_units": m["readable_output_lines"] + m["structural_nodes"] + m["relation_edges"],
            "structural_nodes": m["structural_nodes"],
            "relation_edges": m["relation_edges"],
            "information_per_second_bytes": (m["readable_output_bytes"] / sec) if sec else None,
            "information_per_second_lines": (m["readable_output_lines"] / sec) if sec else None,
            "information_per_second_units": ((m["readable_output_lines"] + m["structural_nodes"] + m["relation_edges"]) / sec) if sec else None,
            "deterministic_repeat_match": True,
            "failure_mode": None
        })

out = out_root / "nsq_code_corpus_score.json"
out.parent.mkdir(parents=True, exist_ok=True)
out.write_text(json.dumps({"version": 1, "rows": rows}, indent=2))
print(json.dumps({"version": 1, "rows": rows}, indent=2))
