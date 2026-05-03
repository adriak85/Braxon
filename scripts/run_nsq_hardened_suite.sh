#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

STAMP="${1:-$(date +%Y%m%d_%H%M%S)}"
OUT="$HOME/storage/shared/Download/nsq_hardened_suite_$STAMP"
TMP="$ROOT/tmp/nsq_hardened_suite_$STAMP"
BIN_DIR="${CARGO_TARGET_DIR:-$HOME/.cargo/target-cache/Braxon}/release"

mkdir -p "$OUT" "$TMP"

cargo build -q --release \
  -p nsq-source \
  -p nsq-compile \
  -p nsq-inspect \
  -p nsq-index \
  -p nsq-query \
  -p nsq-profile \
  -p nsq-debug \
  -p nsq-bench-compare

python3 "$ROOT/scripts/generate_hardened_neutral_bench.py" "$TMP/corpus" >/dev/null

cat > "$TMP/calibration_lock.json" <<'JSON'
{
  "selected_profile": "hardened-suite",
  "promoted_macros": ["links", "fanout", "owns", "visits", "returns", "active"],
  "hot_targets": ["hot.root", "hub.root", "node.0"],
  "threshold_macro_promotion": 4,
  "threshold_expansion": 8,
  "representation_lock": {
    "symbol_id_class": "u16",
    "macro_id_class": "u16",
    "anchor_class": "u32_delta",
    "gain_class": "u16",
    "window_class": "u8"
  },
  "rebalance_actions": ["hardened_suite_enabled"]
}
JSON

python3 - "$TMP" "$OUT" "$BIN_DIR" <<'PY'
import hashlib
import json
import pathlib
import statistics
import subprocess
import sys
import time

tmp = pathlib.Path(sys.argv[1])
out = pathlib.Path(sys.argv[2])
bin_dir = pathlib.Path(sys.argv[3])

corpus_dir = tmp / "corpus"
lock = tmp / "calibration_lock.json"

results = []
query_matrix = [
    ["find", "symbol", "node.0"],
    ["find", "rel", "links"],
    ["neighbors", "node.0"],
    ["anchors", "min=1000", "max=300000"],
    ["states", "target=node.0"],
]

for src in sorted(corpus_dir.glob("*.nsq")):
    if src.name == "neutral_task_spec.json":
        continue

    base = tmp / src.stem
    canonicalized = base.with_suffix(".canonical.txt")
    canonical = base.with_suffix(".spine.nsq")
    index = base.with_suffix(".nsqidx.json")
    index_pretty = base.with_suffix(".nsqidx.pretty.json")
    artifact = base.with_suffix(".nsqb")
    manifest = base.with_suffix(".manifest.json")
    inspect = base.with_suffix(".inspect.json")

    spine_runs = []
    index_runs = []
    compile_runs = []
    inspect_runs = []
    query_runs = []

    for _ in range(3):
        t0 = time.perf_counter()
        spine = subprocess.run(
            [str(bin_dir / "nsq-source"), "spine", str(src)],
            check=True, capture_output=True, text=True,
        )
        t1 = time.perf_counter()
        canonicalized.write_text(lower.stdout, encoding="utf-8")
        canonical.write_text(
            "\n".join(
                line for line in lower.stdout.splitlines()
                if line.strip() and not line.startswith("#")
            ) + "\n",
            encoding="utf-8",
        )
        spine_runs.append((t1 - t0) * 1000.0)

        t0 = time.perf_counter()
        subprocess.run(
            [str(bin_dir / "nsq-index"), "build", str(canonical), str(index)],
            check=True, capture_output=True, text=True,
        )
        t1 = time.perf_counter()
        index_runs.append((t1 - t0) * 1000.0)

        t0 = time.perf_counter()
        subprocess.run(
            [str(bin_dir / "nsq-compile"), str(canonical), str(artifact), str(lock), str(manifest)],
            check=True, capture_output=True, text=True,
        )
        t1 = time.perf_counter()
        compile_runs.append((t1 - t0) * 1000.0)

        t0 = time.perf_counter()
        ins = subprocess.run(
            [str(bin_dir / "nsq-inspect"), str(artifact)],
            check=True, capture_output=True, text=True,
        )
        t1 = time.perf_counter()
        inspect.write_text(ins.stdout, encoding="utf-8")
        inspect_runs.append((t1 - t0) * 1000.0)

        q_total = 0.0
        for q in query_matrix:
            t0 = time.perf_counter()
            subprocess.run(
                [str(bin_dir / "nsq-query"), str(index)] + q,
                check=True, capture_output=True, text=True,
            )
            t1 = time.perf_counter()
            q_total += (t1 - t0) * 1000.0
        query_runs.append(q_total)

    subprocess.run(
        [str(bin_dir / "nsq-index"), "build-pretty", str(canonical), str(index_pretty)],
        check=True, capture_output=True, text=True,
    )

    manifest_json = json.loads(manifest.read_text(encoding="utf-8"))
    index_json = json.loads(index.read_text(encoding="utf-8"))
    inspect_json = json.loads(inspect.read_text(encoding="utf-8"))

    artifact_sha = hashlib.sha256(artifact.read_bytes()).hexdigest()
    index_sha = hashlib.sha256(index.read_bytes()).hexdigest()

    results.append({
        "profile": src.stem,
        "input_path": str(src),
        "spine_ms_mean": round(statistics.mean(spine_runs), 3),
        "index_ms_mean": round(statistics.mean(index_runs), 3),
        "compile_ms_mean": round(statistics.mean(compile_runs), 3),
        "inspect_ms_mean": round(statistics.mean(inspect_runs), 3),
        "query_ms_mean": round(statistics.mean(query_runs), 3),
        "symbols": index_json["stats"]["symbols"],
        "macros": index_json["stats"]["macros"],
        "edges": index_json["stats"]["edges"],
        "states": index_json["stats"]["states"],
        "spine_lines": manifest_json.get("spine_lines", index_json["stats"]["normalized_lines"]),
        "artifact_bytes": artifact.stat().st_size,
        "index_bytes": index.stat().st_size,
        "index_pretty_bytes": index_pretty.stat().st_size,
        "artifact_sha256": artifact_sha,
        "index_sha256": index_sha,
        "inspect_bytes": inspect_json.get("bytes"),
        "phase_counters": index_json["stats"].get("phase_counters", {}),
    })

(out / "hardened_results.json").write_text(json.dumps(results, indent=2) + "\n", encoding="utf-8")
print(json.dumps(results, indent=2))
PY

"$BIN_DIR/nsq-bench-compare" "$TMP/corpus/neutral_task_spec.json" > "$OUT/neutral_compare.json"

tar -C "$OUT" -czf "$OUT/results_bundle.tar.gz" hardened_results.json neutral_compare.json
echo "$OUT"
