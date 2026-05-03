#!/usr/bin/env python3
from __future__ import annotations

import hashlib
import json
import shutil
import subprocess
from pathlib import Path


ROOT = Path.cwd()
TARGETS = ROOT / "config/nsq/nsq_model_install_targets.json"


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def blake3_file(path: Path) -> str | None:
    for cmd in ("b3sum", "blake3"):
        exe = shutil.which(cmd)
        if not exe:
            continue
        p = subprocess.run([exe, str(path)], capture_output=True, text=True)
        if p.returncode == 0 and p.stdout.strip():
            return p.stdout.split()[0]
    return None


def read_json(path: Path) -> dict:
    if not path.exists():
        return {}
    try:
        return json.loads(path.read_text())
    except Exception as e:
        return {"_parse_error": str(e)}


def inspect_file(rel: str) -> dict:
    path = ROOT / rel
    item = {"path": rel, "exists": path.exists()}
    if path.exists() and path.is_file():
        item["byte_count"] = path.stat().st_size
        item["sha256"] = sha256_file(path)
        b3 = blake3_file(path)
        if b3:
            item["blake3"] = b3
    elif path.exists() and path.is_dir():
        item["kind"] = "directory"
        item["entry_count"] = sum(1 for _ in path.iterdir())
    return item


def main() -> None:
    cfg = read_json(TARGETS)
    reports = []
    for model in cfg.get("models", []):
        required = [inspect_file(p) for p in model.get("required_small_targets", [])]
        optional = [inspect_file(p) for p in model.get("optional_source_targets", [])]
        control = [
            inspect_file(model.get("manifest", "")),
            inspect_file(model.get("registry", "")),
            inspect_file(model.get("binding", "")),
            inspect_file(model.get("pipeline_status", "")),
            inspect_file(model.get("source_directory", "")),
            inspect_file(model.get("weight_directory", "")),
            inspect_file(model.get("reconstruction_manifest", "")),
        ]

        required_ok = all(x.get("exists") for x in required)
        source_any = any(x.get("exists") for x in optional) or any(
            x.get("path", "").endswith("source_ingest/braxon_transport") and x.get("exists")
            for x in control
        )
        weight_dir = next((x for x in control if x.get("path", "").endswith("weights/nsq")), {})
        weights_present = bool(weight_dir.get("exists") and weight_dir.get("entry_count", 0) > 0)

        state = "catalog_only"
        if required_ok:
            state = "manifest_bound"
        if required_ok and source_any:
            state = "source_targets_present"
        if required_ok and source_any and weights_present:
            state = "local_artifacts_present"

        reports.append({
            "id": model.get("id"),
            "install_state": state,
            "required_small_targets": required,
            "optional_source_targets": optional,
            "control_targets": control,
            "models_installing": required_ok and source_any,
            "hot_live": False,
            "hot_live_reason": "Hot-live requires runtime route plus exact byte-count/hash/semantic digest proof.",
            "note": "This audit confirms install scaffolding and local artifacts. It does not download model weights."
        })

    print(json.dumps({
        "target_config": str(TARGETS),
        "models": reports,
        "ok": all(m["models_installing"] for m in reports) if reports else False
    }, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
