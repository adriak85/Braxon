#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
import time
import urllib.error
import urllib.parse
import urllib.request
from pathlib import Path
from typing import Any

ROOT = Path(os.environ.get("BRAXON_ROOT", Path.home() / "Braxon")).resolve()
REGISTRY = ROOT / "config/nsq/huihui_model_registry.json"
LOCAL_ROOT = ROOT / "assets/braxon_core/source_ingest/braxon_transport"
HF_API = "https://huggingface.co/api"
HF_RESOLVE = "https://huggingface.co"
TOKEN = os.environ.get("HF_TOKEN") or os.environ.get("HUGGINGFACE_HUB_TOKEN") or ""

MIN_MODEL_BYTES = int(os.environ.get("BRAXON_MIN_MODEL_FILE_BYTES", str(16 * 1024 * 1024)))
SAMPLE_BYTES = int(os.environ.get("BRAXON_TRIPLE_BITE_BYTES", str(64 * 1024)))

POINTER_PREFIX = b"version https://git-lfs.github.com/spec/v1"
GGUF_MAGIC = b"GGUF"

def iso() -> str:
    return time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())

def load_json(path: Path) -> Any:
    return json.loads(path.read_text(errors="replace"))

def headers() -> dict[str, str]:
    h = {"User-Agent": "Braxon-Whole-Model-Pointer-Guard/1.0"}
    if TOKEN:
        h["Authorization"] = f"Bearer {TOKEN}"
    return h

def hf_api_repo(repo: str) -> str:
    return f"{HF_API}/models/{urllib.parse.quote(repo, safe='/')}"

def hf_resolve(repo: str, filename: str) -> str:
    return f"{HF_RESOLVE}/{repo}/resolve/main/{urllib.parse.quote(filename, safe='/')}"

def http_json(url: str) -> tuple[Any | None, str]:
    try:
        req = urllib.request.Request(url, headers=headers())
        with urllib.request.urlopen(req, timeout=45) as r:
            return json.loads(r.read().decode("utf-8", errors="replace")), "ok"
    except urllib.error.HTTPError as err:
        return None, f"http_{err.code}"
    except Exception as err:
        return None, f"error_{type(err).__name__}"

def http_head_size(url: str) -> tuple[int | None, str]:
    try:
        req = urllib.request.Request(url, method="HEAD", headers=headers())
        opener = urllib.request.build_opener(urllib.request.HTTPRedirectHandler())
        with opener.open(req, timeout=45) as r:
            raw = r.headers.get("Content-Length")
            return (int(raw) if raw else None), "ok"
    except urllib.error.HTTPError as err:
        return None, f"http_{err.code}"
    except Exception as err:
        return None, f"error_{type(err).__name__}"

def b3(path: Path) -> tuple[str | None, str]:
    b3sum = shutil.which("b3sum")
    if b3sum:
        proc = subprocess.run([b3sum, str(path)], text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False)
        if proc.returncode == 0 and proc.stdout.strip():
            return proc.stdout.split()[0], "ok"
        return None, f"b3sum_failed:{proc.stderr.strip()[:160]}"

    try:
        import blake3  # type: ignore
        h = blake3.blake3()
        with path.open("rb") as f:
            for block in iter(lambda: f.read(1024 * 1024), b""):
                h.update(block)
        return h.hexdigest(), "ok"
    except Exception as err:
        return None, f"missing_blake3:{type(err).__name__}"

def triple_bite(path: Path) -> dict[str, Any]:
    size = path.stat().st_size
    spans = []
    if size <= 0:
        return {"ok": False, "spans": [], "reason": "empty"}

    offsets = [0, max(0, size // 2 - SAMPLE_BYTES // 2), max(0, size - SAMPLE_BYTES)]
    with path.open("rb") as f:
        for label, off in zip(["head", "middle", "tail"], offsets):
            f.seek(off)
            data = f.read(min(SAMPLE_BYTES, max(0, size - off)))
            spans.append({
                "label": label,
                "offset": off,
                "bytes": len(data),
                "all_zero": bool(data) and all(b == 0 for b in data),
                "looks_lfs_pointer": data.startswith(POINTER_PREFIX),
            })

    ok = all(s["bytes"] > 0 for s in spans) and not any(s["all_zero"] or s["looks_lfs_pointer"] for s in spans)
    return {"ok": ok, "spans": spans}

def inspect_file(path: Path) -> dict[str, Any]:
    rel = str(path.relative_to(ROOT))
    size = path.stat().st_size
    head = path.read_bytes()[:4096] if size else b""

    is_pointer = head.startswith(POINTER_PREFIX)
    suffix = path.suffix.lower()
    kind = "other"
    header_ok = True
    header_reason = "not_model_payload"

    if suffix == ".gguf":
        kind = "gguf"
        header_ok = head.startswith(GGUF_MAGIC)
        header_reason = "gguf_magic_ok" if header_ok else "gguf_magic_missing"
    elif suffix == ".safetensors":
        kind = "safetensors"
        header_ok = False
        header_reason = "safetensors_header_unchecked"
        if size >= 16 and not is_pointer:
            try:
                n = int.from_bytes(head[:8], "little")
                if 2 <= n <= min(size - 8, 100 * 1024 * 1024):
                    header = path.read_bytes()[8:8+n]
                    json.loads(header.decode("utf-8", errors="strict"))
                    header_ok = True
                    header_reason = "safetensors_header_json_ok"
                else:
                    header_reason = f"safetensors_header_len_bad:{n}"
            except Exception as err:
                header_reason = f"safetensors_header_parse_failed:{type(err).__name__}"
    elif path.name.endswith(".json"):
        kind = "json"
        try:
            json.loads(path.read_text(errors="strict"))
            header_ok = True
            header_reason = "json_ok"
        except Exception as err:
            header_ok = False
            header_reason = f"json_parse_failed:{type(err).__name__}"

    model_payload = suffix in {".gguf", ".safetensors"}
    min_size_ok = (not model_payload) or size >= MIN_MODEL_BYTES

    hash_value, hash_status = b3(path) if model_payload else (None, "skipped_non_payload")
    tb = triple_bite(path) if model_payload else {"ok": True, "spans": []}

    ok = (
        size > 0
        and not is_pointer
        and min_size_ok
        and header_ok
        and bool(tb["ok"])
        and (not model_payload or hash_status == "ok")
    )

    return {
        "path": rel,
        "name": path.name,
        "kind": kind,
        "size_bytes": size,
        "is_lfs_pointer": is_pointer,
        "min_size_ok": min_size_ok,
        "header_ok": header_ok,
        "header_reason": header_reason,
        "triple_bite": tb,
        "blake3": hash_value,
        "blake3_status": hash_status,
        "ok": ok,
    }

def remote_siblings(repo: str) -> tuple[list[str], dict[str, int], str]:
    data, status = http_json(hf_api_repo(repo))
    if status != "ok" or not isinstance(data, dict):
        return [], {}, status
    names = []
    sizes = {}
    for s in data.get("siblings", []) or []:
        name = s.get("rfilename")
        if isinstance(name, str):
            names.append(name)
            if isinstance(s.get("size"), int):
                sizes[name] = int(s["size"])
    return sorted(set(names)), sizes, "ok"

def expected_from_index(local_dir: Path) -> list[str]:
    idx = local_dir / "model.safetensors.index.json"
    if not idx.exists():
        return []
    data = load_json(idx)
    vals = sorted(set(data.get("weight_map", {}).values()))
    return [v for v in vals if isinstance(v, str)]

def verify_model(model_id: str, spec: dict[str, Any]) -> dict[str, Any]:
    repo = spec["exact_repo"]
    local_dir = LOCAL_ROOT / model_id
    result: dict[str, Any] = {
        "model_id": model_id,
        "repo_id": repo,
        "local_dir": str(local_dir.relative_to(ROOT)),
        "repo_huihui": repo.startswith("huihui-ai/"),
        "repo_abliterated": "abliterated" in repo.lower(),
        "local_dir_present": local_dir.exists(),
        "remote_manifest_status": None,
        "remote_manifest_verified": False,
        "missing_expected_files": [],
        "extra_note": "",
        "files": [],
        "payload_files": [],
        "remote_size_mismatches": [],
        "whole_model_payload_present": False,
        "no_pointers": False,
        "blake3_complete": False,
        "triple_bite_ok": False,
        "ok": False,
    }

    if not result["repo_huihui"] or not result["repo_abliterated"]:
        result["extra_note"] = "repo identity violates HuiHui abliterated policy"
        return result

    if not local_dir.exists():
        result["extra_note"] = "local model directory is absent; model is not downloaded"
        return result

    files = sorted(p for p in local_dir.rglob("*") if p.is_file())
    inspected = [inspect_file(p) for p in files]
    result["files"] = inspected
    payloads = [f for f in inspected if f["kind"] in {"gguf", "safetensors"}]
    result["payload_files"] = payloads

    remote_names, remote_sizes, remote_status = remote_siblings(repo)
    result["remote_manifest_status"] = remote_status

    expected = []
    if remote_status == "ok":
        expected = [n for n in remote_names if n.endswith((".gguf", ".safetensors", ".json", ".jinja", ".txt"))]
        result["remote_manifest_verified"] = True
    else:
        idx_expected = expected_from_index(local_dir)
        expected = idx_expected
        result["remote_manifest_verified"] = False

    local_rel = {str(p.relative_to(local_dir)) for p in files}
    result["missing_expected_files"] = sorted(x for x in expected if x not in local_rel)

    if remote_status == "ok":
        for rel in sorted(local_rel):
            p = local_dir / rel
            if rel in remote_sizes and p.stat().st_size != remote_sizes[rel]:
                result["remote_size_mismatches"].append({
                    "file": rel,
                    "local_size": p.stat().st_size,
                    "remote_size": remote_sizes[rel],
                })
            elif rel not in remote_sizes and p.suffix.lower() in {".gguf", ".safetensors"}:
                head_size, head_status = http_head_size(hf_resolve(repo, rel))
                if head_status == "ok" and head_size is not None and p.stat().st_size != head_size:
                    result["remote_size_mismatches"].append({
                        "file": rel,
                        "local_size": p.stat().st_size,
                        "remote_size": head_size,
                    })

    has_index = (local_dir / "model.safetensors.index.json").exists()
    has_gguf = any(f["kind"] == "gguf" for f in inspected)
    has_safetensors = any(f["kind"] == "safetensors" for f in inspected)

    result["whole_model_payload_present"] = bool(payloads) and (has_gguf or (has_index and has_safetensors))
    result["no_pointers"] = all(not f["is_lfs_pointer"] for f in inspected)
    result["blake3_complete"] = all(f["blake3_status"] == "ok" for f in payloads)
    result["triple_bite_ok"] = all(f["triple_bite"]["ok"] for f in payloads)

    result["ok"] = bool(
        result["repo_huihui"]
        and result["repo_abliterated"]
        and result["local_dir_present"]
        and result["whole_model_payload_present"]
        and result["no_pointers"]
        and result["blake3_complete"]
        and result["triple_bite_ok"]
        and not result["missing_expected_files"]
        and not result["remote_size_mismatches"]
        and all(f["ok"] for f in inspected if f["kind"] in {"gguf", "safetensors", "json"})
    )

    if not result["ok"] and not result["extra_note"]:
        result["extra_note"] = "model is not proven whole; see missing_expected_files, pointer flags, headers, BLAKE3, triple_bite, and remote_size_mismatches"

    return result

def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--model", action="append", default=[])
    ap.add_argument("--json-out", default="")
    args = ap.parse_args()

    if not REGISTRY.exists():
        raise SystemExit(f"missing registry: {REGISTRY}")

    reg = load_json(REGISTRY)
    default_stack = reg.get("default_stack", [])
    models = reg.get("models", {})
    targets = args.model or default_stack

    reports = []
    for model_id in targets:
        if model_id not in models:
            reports.append({"model_id": model_id, "ok": False, "error": "missing_from_registry"})
            continue
        reports.append(verify_model(model_id, models[model_id]))

    ok = all(bool(r.get("ok")) for r in reports)
    out = {
        "schema": "Braxon.whole_model_pointer_guard.v1",
        "generated_at": iso(),
        "authority": "NSQ_COURT",
        "all_models_requested": targets,
        "all_models_whole": ok,
        "no_lfs_pointers": all(r.get("no_pointers") for r in reports if "no_pointers" in r),
        "blake3_required": True,
        "remote_size_check_attempted": True,
        "reports": reports,
    }

    text = json.dumps(out, indent=2, sort_keys=True)
    print(text)

    if args.json_out:
        p = Path(args.json_out)
        p.parent.mkdir(parents=True, exist_ok=True)
        p.write_text(text + "\n", encoding="utf-8")

    raise SystemExit(0 if ok else 1)

if __name__ == "__main__":
    main()
