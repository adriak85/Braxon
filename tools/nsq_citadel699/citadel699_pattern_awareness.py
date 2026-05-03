#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import math
import os
import shutil
import subprocess
import tarfile
import time
from collections import Counter
from pathlib import Path
from typing import Any

ROOT = Path(os.environ.get("BRAXON_ROOT", Path.home() / "Braxon")).resolve()
CONFIG = ROOT / "config/nsq/citadel699_pattern_awareness.json"

SAMPLE_BYTES = int(os.environ.get("BRAXON_CITADEL699_PATTERN_SAMPLE_BYTES", str(64 * 1024)))
POINTER_PREFIX = b"version https://git-lfs.github.com/spec/v1"
FORBIDDEN_SUFFIXES = {".safetensors", ".gguf"}

CRC32C_POLY = 0x82F63B78


def iso() -> str:
    return time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())


def write_json(path: Path, obj: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(obj, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def crc32c_table() -> list[int]:
    table = []
    for i in range(256):
        crc = i
        for _ in range(8):
            if crc & 1:
                crc = (crc >> 1) ^ CRC32C_POLY
            else:
                crc >>= 1
        table.append(crc & 0xFFFFFFFF)
    return table


CRC_TABLE = crc32c_table()


def crc32c_bytes(data: bytes, crc: int = 0) -> int:
    crc ^= 0xFFFFFFFF
    for b in data:
        crc = CRC_TABLE[(crc ^ b) & 0xFF] ^ (crc >> 8)
    return (crc ^ 0xFFFFFFFF) & 0xFFFFFFFF


def crc32c_file(path: Path) -> str:
    crc = 0
    with path.open("rb") as f:
        while True:
            block = f.read(1024 * 1024)
            if not block:
                break
            crc = crc32c_bytes(block, crc)
    return f"{crc:08x}"


def b3(path: Path) -> tuple[str | None, str]:
    b3sum = shutil.which("b3sum")
    if b3sum:
        p = subprocess.run([b3sum, str(path)], text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False)
        if p.returncode == 0 and p.stdout.strip():
            return p.stdout.split()[0], "ok"
        return None, "b3sum_failed"

    try:
        import blake3  # type: ignore
        h = blake3.blake3()
        with path.open("rb") as f:
            for block in iter(lambda: f.read(1024 * 1024), b""):
                h.update(block)
        return h.hexdigest(), "ok"
    except Exception as err:
        return None, f"missing_blake3:{type(err).__name__}"


def entropy_class(data: bytes) -> str:
    if not data:
        return "empty"
    counts = Counter(data)
    total = len(data)
    ent = 0.0
    for c in counts.values():
        p = c / total
        ent -= p * math.log2(p)
    if ent < 1.0:
        return "very_low"
    if ent < 3.0:
        return "low"
    if ent < 6.0:
        return "structured"
    return "dense_or_compressed"


def triple_bite_shape(path: Path) -> dict[str, Any]:
    size = path.stat().st_size
    if size <= 0:
        return {"ok": False, "reason": "empty", "spans": []}

    offsets = [
        0,
        max(0, size // 2 - SAMPLE_BYTES // 2),
        max(0, size - SAMPLE_BYTES),
    ]

    spans = []
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
                "entropy_class": entropy_class(data),
                "crc32c": f"{crc32c_bytes(data):08x}",
                "content_sample_retained": False
            })

    ok = all(s["bytes"] > 0 for s in spans) and not any(s["all_zero"] or s["looks_lfs_pointer"] for s in spans)
    return {"ok": ok, "spans": spans}


def tar_shape(path: Path) -> dict[str, Any]:
    if not tarfile.is_tarfile(path):
        return {
            "is_tar": False,
            "member_count": 0,
            "suffix_histogram": {},
            "raw_payload_suffix_present": False,
            "manifest_present": False
        }

    suffixes = Counter()
    sizes = []
    raw_payload = False
    manifest_present = False
    content_addressed_guess = 0

    with tarfile.open(path, "r:*") as tf:
        for m in tf.getmembers():
            suffix = Path(m.name).suffix.lower() or "[none]"
            suffixes[suffix] += 1
            sizes.append(int(m.size))
            if suffix in FORBIDDEN_SUFFIXES:
                raw_payload = True
            if Path(m.name).name == "manifest.json":
                manifest_present = True
            lowered = m.name.lower()
            if "blake3" in lowered or "/b3/" in lowered or "/sha256/" in lowered:
                content_addressed_guess += 1

    total = sum(sizes)
    return {
        "is_tar": True,
        "member_count": len(sizes),
        "total_member_bytes": total,
        "largest_member_bytes": max(sizes) if sizes else 0,
        "smallest_member_bytes": min(sizes) if sizes else 0,
        "suffix_histogram": dict(sorted(suffixes.items())),
        "raw_payload_suffix_present": raw_payload,
        "manifest_present": manifest_present,
        "content_addressed_member_guess_count": content_addressed_guess,
        "member_names_retained": False,
        "content_samples_retained": False
    }


def skill_inference(shape: dict[str, Any], tar: dict[str, Any]) -> dict[str, Any]:
    skills = {
        "summary": "Observed authorized artifact structure only; retained no raw content. Patterns are abstracted into reusable Braxon/NSQ implementation skills.",
        "structural_patterns": [],
        "translation_skills": [],
        "compression_skills": [],
        "routing_skills": [],
        "compatibility_skills": [],
        "rebuild_skills": [],
        "verification_skills": [],
        "reuse_candidates": [],
        "confidence": {},
        "limits": [
            "No protected material, unrelated traffic, payload bytes, source code, credentials, private identifiers, or content samples are retained.",
            "CRC32C is only a fast lane checksum; BLAKE3 remains the identity proof."
        ]
    }

    if tar.get("is_tar"):
        skills["structural_patterns"].append("bundle_manifest_plus_fragment_shape")
        skills["routing_skills"].append("route_by_manifest_presence_and_suffix_histogram")
        skills["rebuild_skills"].append("rebuild_requires_manifest_then_content_addressed_fragments")
        skills["verification_skills"].append("verify_tar_shape_before_import")

    if tar.get("content_addressed_member_guess_count", 0) > 0:
        skills["structural_patterns"].append("content_addressed_fragment_layout")
        skills["compression_skills"].append("dedupe_by_content_address_before_transfer")
        skills["reuse_candidates"].append("shared_fragment_cache")

    if shape.get("ok"):
        skills["verification_skills"].append("triple_bite_non_null_non_pointer_screen")
        skills["verification_skills"].append("crc32c_fast_lane_segment_sanity")

    if not tar.get("raw_payload_suffix_present"):
        skills["translation_skills"].append("wire_artifact_keeps_raw_model_payloads_out_of_return_lane")

    skills["confidence"] = {
        "shape_observation": "high" if shape.get("ok") else "low",
        "bundle_observation": "high" if tar.get("is_tar") else "medium",
        "retention_safety": "high"
    }

    return skills


def benefit_map(skills: dict[str, Any]) -> dict[str, Any]:
    systems = [
        "nsq_runtime",
        "BRAXON_core",
        "court_compositor",
        "court_linter",
        "lexor",
        "picker",
        "model_downloader",
        "whole_model_verifier",
        "universal_fetch",
        "storage_architecture",
        "moral_invariant",
        "citadel699",
        "semantic_benchmark",
        "bare_tasker",
        "perpetual_runtime",
        "wowas_canon_engine"
    ]

    out = {}
    for system in systems:
        out[system] = {
            "benefit": "Receives abstract pattern notes only; no raw external material is retained.",
            "usable_patterns": skills.get("structural_patterns", [])[:6],
            "integration_notes": [
                "Use aggregate shape, manifest discipline, and verification sequencing as reusable implementation guidance.",
                "Keep CRC32C as fast sanity only; keep BLAKE3 for authority."
            ],
            "risk_limits": [
                "Do not apply this to protected material or unrelated traffic.",
                "Do not store raw content, code, payload bytes, credentials, identifiers, or samples."
            ]
        }
    return out


def observe(path: Path) -> dict[str, Any]:
    path = path.expanduser().resolve()
    if not path.exists():
        return {"ok": False, "error": "missing", "path": str(path)}

    size = path.stat().st_size
    head = path.read_bytes()[:4096] if size else b""
    lfs_pointer = head.startswith(POINTER_PREFIX)
    blake3, blake3_status = b3(path)
    crc = crc32c_file(path)
    shape = triple_bite_shape(path)
    tar = tar_shape(path)
    skills = skill_inference(shape, tar)

    ok = bool(
        size > 0
        and not lfs_pointer
        and shape.get("ok")
        and blake3_status == "ok"
        and not tar.get("raw_payload_suffix_present", False)
    )

    return {
        "schema": "Braxon.nsq.citadel699.pattern_awareness_report.v1",
        "generated_at": iso(),
        "authority": "NSQ_COURT",
        "route": "citadel699_pattern_awareness",
        "source_path": str(path),
        "size_bytes": size,
        "crc32c": crc,
        "crc32c_role": "optional_fast_lane_checksum_not_identity_proof",
        "blake3": blake3,
        "blake3_status": blake3_status,
        "blake3_role": "identity_proof",
        "is_lfs_pointer": lfs_pointer,
        "triple_bite_shape": shape,
        "tar_shape": tar,
        "learned_skills_breakdown": skills,
        "system_benefit_map": benefit_map(skills),
        "prohibited_retention_report": {
            "raw_source_code_retained": False,
            "raw_payload_bytes_retained": False,
            "credentials_or_tokens_retained": False,
            "private_identifiers_retained": False,
            "third_party_secrets_retained": False,
            "external_user_data_retained": False,
            "member_names_retained": False,
            "content_samples_retained": False
        },
        "truth_boundary": {
            "authorized_artifact_only": True,
            "protected_material_observation_allowed": False,
            "unrelated_traffic_observation_allowed": False,
            "promiscuous_network_capture_allowed": False,
            "abstract_pattern_learning_only": True
        },
        "ok": ok
    }


def main() -> None:
    ap = argparse.ArgumentParser(prog="nsq-citadel699-patterns")
    ap.add_argument("path", nargs="?")
    ap.add_argument("--json-out", default="")
    ap.add_argument("--status", action="store_true")
    args = ap.parse_args()

    if args.status:
        print(json.dumps({
            "schema": "Braxon.nsq.citadel699.pattern_awareness.status.v1",
            "generated_at": iso(),
            "authority": "NSQ_COURT",
            "config_present": CONFIG.exists(),
            "crc32c_allowed": True,
            "crc32c_identity_proof": False,
            "blake3_identity_proof": True,
            "protected_material_observation_allowed": False,
            "unrelated_traffic_observation_allowed": False,
            "retains_raw_content": False
        }, indent=2, sort_keys=True))
        return

    if not args.path:
        raise SystemExit("usage: nsq-citadel699-patterns /path/to/authorized-artifact [--json-out file]")

    result = observe(Path(args.path))
    if args.json_out:
        write_json(Path(args.json_out).expanduser(), result)
    print(json.dumps(result, indent=2, sort_keys=True))
    raise SystemExit(0 if result.get("ok") else 1)


if __name__ == "__main__":
    main()
