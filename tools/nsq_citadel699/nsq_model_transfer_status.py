#!/usr/bin/env python3
from __future__ import annotations

import json
import os
import time
from pathlib import Path
from typing import Any

ROOT = Path(os.environ.get("BRAXON_ROOT", Path.home() / "Braxon")).resolve()
DL = Path.home() / "storage/shared/Download"

POINTER_PREFIX = b"version https://git-lfs.github.com/spec/v1"
RAW_SUFFIXES = {".safetensors", ".gguf"}

def iso() -> str:
    return time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())

def inspect_file(p: Path) -> dict[str, Any]:
    size = p.stat().st_size
    head = p.read_bytes()[:512] if size else b""
    return {
        "path": str(p.relative_to(ROOT)) if str(p).startswith(str(ROOT)) else str(p),
        "size_bytes": size,
        "suffix": p.suffix.lower(),
        "is_lfs_pointer": head.startswith(POINTER_PREFIX),
        "looks_empty": size == 0
    }

def scan_raw_payloads() -> list[dict[str, Any]]:
    roots = [
        ROOT / "assets/braxon_core/source_ingest/braxon_transport",
        ROOT / "assets/braxon_core/source_ingest/nsq_transport/citadel699"
    ]
    out = []
    for root in roots:
        if not root.exists():
            continue
        for p in root.rglob("*"):
            if p.is_file() and p.suffix.lower() in RAW_SUFFIXES:
                out.append(inspect_file(p))
    return sorted(out, key=lambda x: x["path"])

def scan_requests() -> list[str]:
    hits = []
    if DL.exists():
        for name in ("citadel699_request.json", "citadel699_request_receipt.json", "citadel699_request.nsq"):
            for p in DL.rglob(name):
                hits.append(str(p))
    return sorted(set(hits))[-30:]

raw = scan_raw_payloads()
requests = scan_requests()

out = {
    "schema": "Braxon.nsq.model_transfer_status.v2",
    "generated_at": iso(),
    "authority": "NSQ_COURT",
    "active_transfer_method": "citadel699_nsq_request_return_rebuild",
    "normal_command": "bin/Braxon-model-request MODEL",
    "blocked_commands": [
        "bin/Braxon-model-downloader fetch MODEL",
        "bin/Braxon-model-downloader download MODEL",
        "bin/Braxon-model-downloader raw-fetch MODEL",
        "bin/Braxon-model-downloader raw-download MODEL"
    ],
    "network_used_by_this_status_tool": False,
    "raw_huggingface_payload_fetch_allowed": False,
    "raw_payload_files_found_locally": len(raw),
    "raw_payload_pointer_count_locally": sum(1 for x in raw if x.get("is_lfs_pointer")),
    "raw_payload_files_local_audit": raw,
    "citadel_request_artifacts_found": len(requests),
    "recent_citadel_request_artifacts": requests,
    "runtime_ready": False,
    "status_meaning": {
        "citadel_request_artifact": "request capsule only; not a model download and not runtime ready",
        "raw_payload_file": "legacy/forbidden path unless deliberately quarantined or removed later",
        "lfs_pointer": "not a model and not acceptable as payload",
        "runtime_ready": "false until returned NSQ bundle is imported, rebuilt locally, and verified"
    },
    "truth_boundary": {
        "download_check_is_not_success": True,
        "huggingface_payload_download_is_not_custom_nsq_transfer": True,
        "fetch_word_is_disallowed_for_normal_model_transfer": True,
        "whole_core_runtime_verification_required": True,
        "raw_weight_download_allowed": False,
        "placeholders_are_not_runtime_material": True,
        "incomplete_models_are_not_acceptable": True
    },
    "ok": True
}

print(json.dumps(out, indent=2, sort_keys=True))
