#!/usr/bin/env python3
from __future__ import annotations

import hashlib
import json
import os
import subprocess
import sys
import time
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[2]
STATE = ROOT / "state/braxon/music_app/current"
STATE.mkdir(parents=True, exist_ok=True)

CONFIG = ROOT / "config/nsq/BRAXON_music_app.json"
CITADEL = ROOT / "state/nsq/citadel699/current/audit.json"
SEED_PACK = ROOT / "state/nsq/site_rebuild/current/beyond_699_seed_pack.jsonl"
SYNTH = ROOT / "bin/Braxon-music-synth"

def rel(p: Path) -> str:
    try:
        return p.relative_to(ROOT).as_posix()
    except Exception:
        return str(p)

def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()

def load_json(path: Path, fallback: Any) -> Any:
    try:
        return json.loads(path.read_text(errors="replace"))
    except Exception:
        return fallback

def write_json(path: Path, obj: Any) -> None:
    path.write_text(json.dumps(obj, indent=2, sort_keys=True) + "\n", encoding="utf-8")

def command_doctor() -> int:
    cfg = load_json(CONFIG, {})
    cit = load_json(CITADEL, {})
    seed_exists = SEED_PACK.exists()
    synth_exists = SYNTH.exists() and os.access(SYNTH, os.X_OK)

    report = {
        "schema": "Braxon.music_app.doctor.v1",
        "ok": bool(cfg and cit and seed_exists and synth_exists),
        "config": rel(CONFIG),
        "config_present": CONFIG.exists(),
        "citadel_audit": rel(CITADEL),
        "citadel_hit_count": cit.get("citadel_hit_count", 0),
        "beyond_699_hit_count": cit.get("beyond_699_hit_count", 0),
        "seed_pack": rel(SEED_PACK),
        "seed_pack_present": seed_exists,
        "synth": rel(SYNTH),
        "synth_present": synth_exists,
        "claims": {
            "native_preview_synth": synth_exists,
            "asm_seed_fold": synth_exists,
            "full_music_generation": False,
            "bare_metal": False,
            "perpetual_runtime": False
        }
    }
    write_json(STATE / "doctor.json", report)
    print(json.dumps(report, indent=2, sort_keys=True))
    return 0 if report["ok"] else 1

def command_plan() -> int:
    cit = load_json(CITADEL, {})
    cfg = load_json(CONFIG, {})
    seed_entries = 0
    if SEED_PACK.exists():
        with SEED_PACK.open("r", encoding="utf-8", errors="replace") as f:
            seed_entries = sum(1 for _ in f)

    sections = cfg.get("audio", {}).get("sections", ["intro", "verse", "pre_chorus", "chorus", "bridge", "outro"])

    plan = {
        "schema": "Braxon.music_app.site_rebuild_plan.v1",
        "created_at_unix": int(time.time()),
        "strategy": "transfer_enough_of_each_section_to_rebuild_rest_on_site",
        "sections": [],
        "citadel_699_review": {
            "path": rel(CITADEL),
            "citadel_hit_count": cit.get("citadel_hit_count", 0),
            "beyond_699_hit_count": cit.get("beyond_699_hit_count", 0)
        },
        "seed_pack": {
            "path": rel(SEED_PACK),
            "entries": seed_entries,
            "sha256": sha256_file(SEED_PACK) if SEED_PACK.exists() else None
        },
        "truth_boundary": {
            "can_rebuild_deterministic_sections": True,
            "can_rebuild_unknown_source_without_verifier": False,
            "full_music_generation_claim": False
        }
    }

    for idx, section in enumerate(sections):
        section_seed = hashlib.sha256(f"{section}|{idx}|BRAXON_music_app|citadel699".encode()).hexdigest()
        plan["sections"].append({
            "section": section,
            "ordinal": idx,
            "transfer": {
                "required": [
                    "section_name",
                    "ordinal",
                    "seed_hash",
                    "duration_hint",
                    "local_generation_rule"
                ],
                "omit_if_rebuildable": [
                    "full_pcm",
                    "expanded_stems",
                    "duplicated_pattern_body"
                ]
            },
            "seed_hash": section_seed,
            "local_generation_rule": "native_preview_synth_seeded_section_pattern_v1"
        })

    write_json(STATE / "site_rebuild_plan.json", plan)
    print(json.dumps(plan, indent=2, sort_keys=True))
    return 0

def command_synth() -> int:
    if not SYNTH.exists():
        print(f"missing synth: {SYNTH}", file=sys.stderr)
        return 2

    plan_path = STATE / "site_rebuild_plan.json"
    if not plan_path.exists():
        command_plan()

    plan_hash = sha256_file(plan_path)
    out_wav = STATE / "preview.wav"
    seed_text = f"BRAXON_music_app|{plan_hash}|citadel699|beyond699|asm"

    proc = subprocess.run(
        [str(SYNTH), str(out_wav), seed_text],
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )

    (STATE / "synth.stdout.txt").write_text(proc.stdout, encoding="utf-8")
    (STATE / "synth.stderr.txt").write_text(proc.stderr, encoding="utf-8")

    manifest = {
        "schema": "Braxon.music_app.preview_manifest.v1",
        "ok": proc.returncode == 0 and out_wav.exists(),
        "returncode": proc.returncode,
        "output_wav": rel(out_wav),
        "output_wav_sha256": sha256_file(out_wav) if out_wav.exists() else None,
        "runtime_kind": "native_c_plus_aarch64_asm_seed_fold",
        "tracking": False,
        "macro_discovery": False,
        "tracers": False,
        "bare_metal_claim": False,
        "stdout": rel(STATE / "synth.stdout.txt"),
        "stderr": rel(STATE / "synth.stderr.txt")
    }
    write_json(STATE / "preview_manifest.json", manifest)
    print(json.dumps(manifest, indent=2, sort_keys=True))
    return 0 if manifest["ok"] else 1

def command_manifest() -> int:
    doctor = load_json(STATE / "doctor.json", {})
    preview = load_json(STATE / "preview_manifest.json", {})
    plan = load_json(STATE / "site_rebuild_plan.json", {})

    manifest = {
        "schema": "Braxon.music_app.current_manifest.v1",
        "app": "BRAXON_music_app",
        "created_at_unix": int(time.time()),
        "doctor_ok": doctor.get("ok"),
        "preview_ok": preview.get("ok"),
        "site_rebuild_plan": rel(STATE / "site_rebuild_plan.json"),
        "preview_manifest": rel(STATE / "preview_manifest.json"),
        "preview_wav": preview.get("output_wav"),
        "features": {
            "asm_seed_fold": True,
            "native_preview_synth": True,
            "site_rebuild_seed_pack": SEED_PACK.exists(),
            "citadel_699_review": CITADEL.exists(),
            "offline_first": True,
            "full_generation_claim": False,
            "perpetual_runtime_claim": False
        }
    }
    write_json(STATE / "manifest.json", manifest)
    print(json.dumps(manifest, indent=2, sort_keys=True))
    return 0

def main() -> int:
    cmd = sys.argv[1] if len(sys.argv) > 1 else "all"

    if cmd == "doctor":
        return command_doctor()
    if cmd == "plan":
        return command_plan()
    if cmd == "synth":
        return command_synth()
    if cmd == "manifest":
        return command_manifest()
    if cmd == "all":
        rc = 0
        rc |= command_plan()
        rc |= command_synth()
        rc |= command_doctor()
        rc |= command_manifest()
        return rc

    print("usage: Braxon-music-app [all|doctor|plan|synth|manifest]", file=sys.stderr)
    return 2

if __name__ == "__main__":
    raise SystemExit(main())
