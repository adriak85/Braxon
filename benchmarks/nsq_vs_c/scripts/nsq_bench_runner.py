from pathlib import Path
import json
import subprocess
import time
import hashlib
import os
import sys

HOME = Path.home()
BRAXON_HOME = Path(os.environ.get("BRAXON_HOME", str(HOME / "Braxon")))
BENCH_DIR = BRAXON_HOME / "benchmarks" / "nsq_vs_c"
MATRIX = json.loads((BENCH_DIR / "specs" / "benchmark_matrix.json").read_text())

def sh(cmd):
    return subprocess.run(cmd, shell=True, text=True, capture_output=True)

def timed(cmd):
    t0 = time.perf_counter()
    p = sh(cmd)
    dt = (time.perf_counter() - t0) * 1000.0
    return p, dt

def file_sha256(path: Path):
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()

def parse_score_json(path: Path):
    if not path.exists():
        return {}
    return json.loads(path.read_text())

def parse_manifest_json(path: Path):
    if not path.exists():
        return {}
    return json.loads(path.read_text())

def run_once(scale_name, scale, density, repeat_idx):
    run_root = BRAXON_HOME / "artifacts" / "nsq_native_truth"
    cmd = f'nsq-super-run {scale} {density} 3'
    p, run_ms = timed(cmd)
    if p.returncode != 0:
        return {
            "ok": False,
            "scale_name": scale_name,
            "scale": scale,
            "density": density,
            "repeat": repeat_idx,
            "stage": "super_run",
            "stderr": p.stderr,
            "stdout": p.stdout,
            "run_ms": run_ms
        }

    out_dir = None
    for line in p.stdout.splitlines():
        if line.startswith("super_run_out="):
            out_dir = Path(line.split("=", 1)[1].strip())
            break

    if out_dir is None or not out_dir.exists():
        return {
            "ok": False,
            "scale_name": scale_name,
            "scale": scale,
            "density": density,
            "repeat": repeat_idx,
            "stage": "extract_run_dir",
            "stdout": p.stdout,
            "stderr": p.stderr
        }

    fams = ["noise_large", "structured_large", "membrane_large"]
    results = []
    for fam in fams:
        manifest = parse_manifest_json(out_dir / f"{fam}.compile_manifest.json")
        score = parse_score_json(out_dir / f"{fam}.score.json")
        artifact_path = out_dir / f"{fam}.nsqb"
        inspect_path = out_dir / f"{fam}.inspect.txt"

        results.append({
            "family": fam,
            "artifact_path": str(artifact_path),
            "inspect_path": str(inspect_path),
            "artifact_sha256": file_sha256(artifact_path) if artifact_path.exists() else None,
            "source_bytes": (out_dir / "composed_super_surface.nsq").stat().st_size if (out_dir / "composed_super_surface.nsq").exists() else None,
            "artifact_bytes": score.get("artifact_bytes", manifest.get("native_bytes")),
            "decoded_bytes": score.get("decoded_bytes"),
            "decoded_records": score.get("decoded_records"),
            "unique_symbols": score.get("unique_symbols", manifest.get("compiled_symbols")),
            "unique_macros": manifest.get("compiled_macros"),
            "decoded_bytes_per_artifact_byte": score.get("decoded_bytes_per_artifact_byte"),
            "information_density": score.get("information_density"),
            "replay_sha256": score.get("replay_sha256")
        })

    return {
        "ok": True,
        "scale_name": scale_name,
        "scale": scale,
        "density": density,
        "repeat": repeat_idx,
        "super_run_ms": run_ms,
        "out_dir": str(out_dir),
        "families": results
    }

def main():
    out_root = BENCH_DIR / "results" / "nsq"
    out_root.mkdir(parents=True, exist_ok=True)

    all_runs = []
    for scale_name, scale in MATRIX["scales"].items():
        density = MATRIX["densities"][scale_name]
        for repeat_idx in range(1, MATRIX["repeats"] + 1):
            all_runs.append(run_once(scale_name, scale, density, repeat_idx))

    report = {
        "version": 1,
        "system": "nsq",
        "runs": all_runs
    }

    out = out_root / "nsq_benchmark_report.json"
    out.write_text(json.dumps(report, indent=2))
    print(json.dumps(report, indent=2))

if __name__ == "__main__":
    main()
