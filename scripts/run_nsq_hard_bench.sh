#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

STAMP="${1:-$(date +%Y%m%d_%H%M%S)}"
OUT="$HOME/storage/shared/Download/nsq_hard_bench_$STAMP"
TMP="$ROOT/tmp/nsq_hard_bench_$STAMP"
BIN_DIR="${CARGO_TARGET_DIR:-$HOME/.cargo/target-cache/Braxon}/release"

mkdir -p "$OUT" "$TMP"

cargo build -q --release -p nsq-source -p nsq-compile -p nsq-inspect

python3 "$ROOT/scripts/generate_nsq_scale.py" "$TMP/corpus" > /dev/null

cat > "$TMP/calibration_lock.json" <<'JSON'
{
  "selected_profile": "hard-bench",
  "promoted_macros": ["links", "fanout", "active"],
  "hot_targets": ["hot.root", "node.0"],
  "threshold_macro_promotion": 4,
  "threshold_expansion": 8,
  "representation_lock": {
    "symbol_id_class": "u16",
    "macro_id_class": "u16",
    "anchor_class": "u32_delta",
    "gain_class": "u16",
    "window_class": "u8"
  },
  "rebalance_actions": ["hard_bench_enabled"]
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
for src in sorted(corpus_dir.glob("*.nsq")):
    if src.name == "scale_manifest.json":
        continue

    base = tmp / src.stem
    canonicalized = base.with_suffix(".canonical.txt")
    canonical = base.with_suffix(".spine.nsq")
    artifact = base.with_suffix(".nsqb")
    manifest = base.with_suffix(".manifest.json")
    inspect = base.with_suffix(".inspect.json")

    spine_runs = []
    compile_runs = []
    inspect_runs = []
    sha = None
    symbols = None
    macros = None
    native_bytes = None
    spine_lines = None
    inspect_bytes = None

    for _ in range(3):
        t0 = time.perf_counter()
        spine = subprocess.run(
            [str(bin_dir / "nsq-source"), "spine", str(src)],
            check=True, capture_output=True, text=True
        )
        t1 = time.perf_counter()
        canonicalized.write_text(lower.stdout, encoding="utf-8")
        canonical.write_text(
            "\n".join(line for line in lower.stdout.splitlines() if line and not line.startswith("#")) + "\n",
            encoding="utf-8"
        )
        spine_runs.append((t1 - t0) * 1000.0)

        t0 = time.perf_counter()
        subprocess.run(
            [str(bin_dir / "nsq-compile"), str(canonical), str(artifact), str(lock), str(manifest)],
            check=True, capture_output=True, text=True
        )
        t1 = time.perf_counter()
        compile_runs.append((t1 - t0) * 1000.0)

        t0 = time.perf_counter()
        ins = subprocess.run(
            [str(bin_dir / "nsq-inspect"), str(artifact)],
            check=True, capture_output=True, text=True
        )
        t1 = time.perf_counter()
        inspect_runs.append((t1 - t0) * 1000.0)
        inspect.write_text(ins.stdout, encoding="utf-8")

    sha = hashlib.sha256(artifact.read_bytes()).hexdigest()
    m = json.loads(manifest.read_text(encoding="utf-8"))
    i = json.loads(inspect.read_text(encoding="utf-8"))

    symbols = m.get("compiled_symbols")
    macros = m.get("compiled_macros")
    native_bytes = m.get("native_bytes")
    spine_lines = m.get("spine_lines")
    inspect_bytes = i.get("bytes")

    results.append({
        "profile": src.stem,
        "input_path": str(src),
        "spine_ms_mean": round(statistics.mean(spine_runs), 3),
        "compile_ms_mean": round(statistics.mean(compile_runs), 3),
        "inspect_ms_mean": round(statistics.mean(inspect_runs), 3),
        "compiled_symbols": symbols,
        "compiled_macros": macros,
        "native_bytes": native_bytes,
        "spine_lines": spine_lines,
        "inspect_bytes": inspect_bytes,
        "artifact_sha256": sha,
    })

(out / "hard_bench_results.json").write_text(json.dumps(results, indent=2) + "\n", encoding="utf-8")
print(json.dumps(results, indent=2))
PY

tar -C "$OUT" -czf "$OUT/results_bundle.tar.gz" hard_bench_results.json
echo "$OUT"
