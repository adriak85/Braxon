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
from pathlib import Path
from typing import Any

ROOT = Path(os.environ.get("BRAXON_ROOT", Path.cwd()))
CONFIG = ROOT / "config/nsq/BRAXON_model_downloader_app.json"
STATE = ROOT / "state/braxon/model_downloader/current"
TARGET_DEFAULT = ROOT / "assets/braxon_core/source_ingest/braxon_transport"
CITADEL = ROOT / "state/nsq/citadel699/current"
STATE.mkdir(parents=True, exist_ok=True)
CITADEL.mkdir(parents=True, exist_ok=True)

def load_config() -> dict[str, Any]:
    if CONFIG.exists():
        return json.loads(CONFIG.read_text(errors="replace"))
    return {
        "repo_id": "huihui-ai/Huihui-Qwen3-VL-32B-Instruct-abliterated",
        "local_dir": "assets/braxon_core/source_ingest/braxon_transport",
        "include_patterns": [
            "config.json",
            "generation_config.json",
            "tokenizer.json",
            "tokenizer_config.json",
            "special_tokens_map.json",
            "chat_template.jinja",
            "model.safetensors.index.json",
            "model-*.safetensors",
        ],
    }

CFG = load_config()

def rel(path: Path) -> str:
    try:
        return path.relative_to(ROOT).as_posix()
    except Exception:
        return str(path)

def write_json(path: Path, obj: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(obj, indent=2, sort_keys=True) + "\n", encoding="utf-8")

def command_exists(name: str) -> bool:
    return shutil.which(name) is not None

def run(argv: list[str], name: str, timeout: int | None = None) -> dict[str, Any]:
    started = time.monotonic()
    try:
        proc = subprocess.run(
            argv,
            cwd=ROOT,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=timeout,
            check=False,
        )
        outp = STATE / f"{name}.stdout.txt"
        errp = STATE / f"{name}.stderr.txt"
        outp.write_text(proc.stdout, encoding="utf-8")
        errp.write_text(proc.stderr, encoding="utf-8")
        return {
            "name": name,
            "argv": argv,
            "executed": True,
            "ok": proc.returncode == 0,
            "returncode": proc.returncode,
            "elapsed_seconds": round(time.monotonic() - started, 6),
            "stdout_path": rel(outp),
            "stderr_path": rel(errp),
            "stdout_preview": proc.stdout[:2000],
            "stderr_preview": proc.stderr[:2000],
        }
    except Exception as err:
        return {
            "name": name,
            "argv": argv,
            "executed": False,
            "ok": False,
            "elapsed_seconds": round(time.monotonic() - started, 6),
            "error": repr(err),
        }

def is_lfs_pointer(path: Path) -> bool:
    try:
        head = path.read_bytes()[:256]
    except Exception:
        return False
    text = head.decode("utf-8", errors="ignore")
    return (
        text.startswith("version https://git-lfs.github.com/spec/v1")
        or "oid sha256:" in text
        or "git-lfs" in text.lower()
    )

def materialized_payload(path: Path) -> bool:
    if not path.exists() or not path.is_file():
        return False
    if is_lfs_pointer(path):
        return False
    return path.stat().st_size > 1024 * 1024

def expected_shards(target: Path) -> list[str]:
    index = target / "model.safetensors.index.json"
    if index.exists() and not is_lfs_pointer(index):
        try:
            obj = json.loads(index.read_text(errors="replace"))
            wm = obj.get("weight_map", {})
            vals = sorted({str(v) for v in wm.values() if str(v).endswith(".safetensors")})
            if vals:
                return vals
        except Exception:
            pass
    found = sorted(p.name for p in target.glob("model-*.safetensors"))
    if found:
        return found
    return [f"model-{i:05d}-of-00014.safetensors" for i in range(1, 15)]

def shard_rebuild_patterns(target: Path) -> dict[str, Any]:
    names = sorted(p.name for p in target.glob("*.safetensors"))
    pattern_rows = []
    rx = re.compile(r"^(?P<prefix>.*?)(?P<num>\d+)-of-(?P<total>\d+)(?P<suffix>\.safetensors)$")
    for name in names:
        m = rx.match(name)
        if not m:
            continue
        width = len(m.group("num"))
        total = int(m.group("total"))
        prefix = m.group("prefix")
        suffix = m.group("suffix")
        expected = [f"{prefix}{i:0{width}d}-of-{total:0{len(m.group('total'))}d}{suffix}" for i in range(1, total + 1)]
        pattern_rows.append({
            "observed": name,
            "prefix": prefix,
            "width": width,
            "total": total,
            "suffix": suffix,
            "expected_count": len(expected),
            "expected": expected,
        })
    return {
        "schema": "Braxon.model_downloader.shard_rebuild_patterns.v1",
        "target": rel(target),
        "observed_safetensors": names,
        "patterns": pattern_rows,
    }

def citadel699_audit() -> dict[str, Any]:
    roots = [ROOT / "apps/nsq", ROOT / "config/nsq", ROOT / "state/nsq/court"]
    hits = []
    num_rx = re.compile(r"\b([6-9]\d\d|[1-9]\d{3,})\b")
    word_rx = re.compile(r"citadel\s*699|citadel699|699", re.IGNORECASE)
    for base in roots:
        if not base.exists():
            continue
        files = [base] if base.is_file() else [p for p in base.rglob("*") if p.is_file()]
        for p in files:
            if any(part in p.as_posix() for part in ["/target/", "/metadata_law/snapshots/"]):
                continue
            try:
                text = p.read_text(errors="replace")
            except Exception:
                continue
            for idx, line in enumerate(text.splitlines(), 1):
                if word_rx.search(line) or num_rx.search(line):
                    hits.append({
                        "path": rel(p),
                        "line": idx,
                        "text": line.strip()[:500],
                    })
    obj = {
        "schema": "nsq.citadel699.pattern_audit.v1",
        "generated_at_unix": int(time.time()),
        "hit_count": len(hits),
        "hits": hits[:1000],
        "truth": "Audit records existing 699-or-higher pattern references only; it does not invent a duplicate Citadel system.",
    }
    write_json(CITADEL / "pattern_audit.json", obj)
    return obj

def read_perpetual_claim() -> dict[str, Any]:
    p = ROOT / "state/nsq/perpetual_runtime/current/claim.json"
    if not p.exists():
        return {}
    try:
        return json.loads(p.read_text(errors="replace"))
    except Exception:
        return {}

def court_ready() -> bool:
    claim = read_perpetual_claim()
    return (
        claim.get("authority") == "NSQ_COURT"
        and claim.get("architecture_root") is True
        and claim.get("king") == "compositor"
        and claim.get("queen") == "linter"
        and claim.get("court_is_agents") is False
        and claim.get("perpetual_runtime_allowed") is True
        and claim.get("c_runner_used") is False
    )

def identity(args: argparse.Namespace) -> int:
    obj = {
        "schema": "Braxon.model_downloader.identity.v2",
        "authority": "NSQ_COURT",
        "architecture_root": True,
        "king": "compositor",
        "queen": "linter",
        "court_is_agents": False,
        "route": "BRAXON_model_downloader",
        "repo_id": CFG.get("repo_id"),
        "local_dir": CFG.get("local_dir"),
        "truth": "Downloader is court-owned. Download verification is not NSQ recode and not whole-core runtime readiness.",
    }
    write_json(STATE / "identity.json", obj)
    print(json.dumps(obj, indent=2, sort_keys=True))
    return 0

def doctor(args: argparse.Namespace) -> int:
    obj = {
        "schema": "Braxon.model_downloader.doctor.v2",
        "authority": "NSQ_COURT",
        "architecture_root": True,
        "perpetual_runtime_ready": court_ready(),
        "tools": {
            "python3": command_exists("python3"),
            "git": command_exists("git"),
            "git_lfs": command_exists("git-lfs") or (command_exists("git") and run(["git", "lfs", "version"], "doctor_git_lfs", timeout=10).get("ok") is True),
            "hf": command_exists("hf"),
            "huggingface_cli": command_exists("huggingface-cli"),
            "aria2c": command_exists("aria2c"),
        },
        "install_help": [
            "pip install -U huggingface_hub",
            "pkg install git git-lfs aria2",
            "git lfs install"
        ],
    }
    obj["ok"] = obj["perpetual_runtime_ready"] and (obj["tools"]["hf"] or obj["tools"]["huggingface_cli"] or obj["tools"]["git_lfs"])
    write_json(STATE / "doctor.json", obj)
    print(json.dumps(obj, indent=2, sort_keys=True))
    return 0 if obj["ok"] else 1

def plan(args: argparse.Namespace) -> int:
    target = ROOT / str(CFG.get("local_dir", "assets/braxon_core/source_ingest/braxon_transport"))
    obj = {
        "schema": "Braxon.model_downloader.plan.v2",
        "authority": "NSQ_COURT",
        "route": "BRAXON_model_downloader",
        "repo_id": CFG.get("repo_id"),
        "local_dir": rel(target),
        "include_patterns": CFG.get("include_patterns", []),
        "expected_shards": expected_shards(target),
        "reject_lfs_pointer_stubs": True,
        "nsq_recode_required_after_download": True,
        "payload_verified_is_not_nsq_recode": True,
        "whole_core_runtime_verification_required": True,
        "raw_weight_download_allowed": False,
        "target_size_class": "mb_scale",
        "tiny_seed_reconstruction_required": True,
    }
    write_json(STATE / "plan.json", obj)
    print(json.dumps(obj, indent=2, sort_keys=True))
    return 0

def fetch(args: argparse.Namespace) -> int:
    if not court_ready() and not args.force:
        obj = {
            "schema": "Braxon.model_downloader.fetch.v2",
            "ok": False,
            "blocked": True,
            "reason": "perpetual runtime proof is not green through NSQ Court",
            "next": "run bin/nsq-court-perpetual-runtime-proof",
        }
        write_json(STATE / "fetch.json", obj)
        print(json.dumps(obj, indent=2, sort_keys=True))
        return 2

    target = ROOT / str(CFG.get("local_dir", "assets/braxon_core/source_ingest/braxon_transport"))
    target.mkdir(parents=True, exist_ok=True)
    repo = str(CFG.get("repo_id"))
    includes = [str(x) for x in CFG.get("include_patterns", [])]

    commands = []
    if command_exists("hf"):
        argv = ["hf", "download", repo, "--local-dir", str(target)]
        for pat in includes:
            argv += ["--include", pat]
        commands.append(("hf_download", argv))
    if command_exists("huggingface-cli"):
        argv = ["huggingface-cli", "download", repo, "--local-dir", str(target), "--resume-download"]
        for pat in includes:
            argv += ["--include", pat]
        commands.append(("huggingface_cli_download", argv))
    if (target / ".git").exists() and command_exists("git"):
        commands.append(("git_lfs_pull", ["git", "-C", str(target), "lfs", "pull"]))

    if not commands:
        obj = {
            "schema": "Braxon.model_downloader.fetch.v2",
            "ok": False,
            "blocked": True,
            "reason": "no downloader backend found",
            "install_help": [
                "pip install -U huggingface_hub",
                "pkg install git git-lfs aria2",
                "git lfs install"
            ],
        }
        write_json(STATE / "fetch.json", obj)
        print(json.dumps(obj, indent=2, sort_keys=True))
        return 3

    results = []
    ok = False
    for name, argv in commands:
        result = run(argv, name, timeout=None)
        results.append(result)
        if result.get("ok") is True:
            ok = True
            break

    obj = {
        "schema": "Braxon.model_downloader.fetch.v2",
        "authority": "NSQ_COURT",
        "route": "BRAXON_model_downloader",
        "repo_id": repo,
        "local_dir": rel(target),
        "ok": ok,
        "commands": results,
        "next": "run verify",
    }
    write_json(STATE / "fetch.json", obj)
    print(json.dumps(obj, indent=2, sort_keys=True))
    return 0 if ok else 4

def verify(args: argparse.Namespace) -> int:
    target = ROOT / str(CFG.get("local_dir", "assets/braxon_core/source_ingest/braxon_transport"))
    target.mkdir(parents=True, exist_ok=True)

    minimum = [str(x) for x in CFG.get("required_minimum", ["config.json", "tokenizer.json", "model.safetensors.index.json"])]
    expected = expected_shards(target)

    files = []
    for p in sorted(target.glob("*")):
        if p.is_file():
            files.append({
                "name": p.name,
                "path": rel(p),
                "size_bytes": p.stat().st_size,
                "is_lfs_pointer": is_lfs_pointer(p),
                "materialized_payload": materialized_payload(p) if p.name.endswith(".safetensors") else (p.exists() and not is_lfs_pointer(p)),
            })

    missing_minimum = [x for x in minimum if not (target / x).exists()]
    missing_shards = [x for x in expected if not (target / x).exists()]
    pointer_shards = [x for x in expected if (target / x).exists() and is_lfs_pointer(target / x)]
    materialized_shards = [x for x in expected if materialized_payload(target / x)]

    rebuild = shard_rebuild_patterns(target)
    write_json(STATE / "rebuild_patterns.json", rebuild)

    citadel = citadel699_audit()

    payload_verified = (
        not missing_minimum
        and not missing_shards
        and not pointer_shards
        and len(materialized_shards) == len(expected)
        and len(expected) > 0
    )

    obj = {
        "schema": "Braxon.model_downloader.verify.v2",
        "authority": "NSQ_COURT",
        "architecture_root": True,
        "king": "compositor",
        "queen": "linter",
        "court_is_agents": False,
        "route": "BRAXON_model_downloader",
        "repo_id": CFG.get("repo_id"),
        "local_dir": rel(target),
        "required_minimum": minimum,
        "missing_minimum": missing_minimum,
        "expected_shards": expected,
        "expected_shard_count": len(expected),
        "missing_shards": missing_shards,
        "pointer_shards": pointer_shards,
        "materialized_shards": materialized_shards,
        "materialized_shard_count": len(materialized_shards),
        "payload_verified": payload_verified,
        "download_complete_claim_allowed": payload_verified,
        "nsq_recode_complete": False,
        "nsq_recode_required_after_download": True,
        "payload_verified_is_not_nsq_recode": True,
        "whole_core_runtime_ready": False,
        "whole_core_runtime_verification_required": True,
        "raw_weight_download_allowed": False,
        "rebuild_patterns": rebuild,
        "citadel699_pattern_audit": {
            "path": "state/nsq/citadel699/current/pattern_audit.json",
            "hit_count": citadel.get("hit_count"),
        },
        "files": files,
    }
    write_json(STATE / "verify.json", obj)
    print(json.dumps(obj, indent=2, sort_keys=True))
    return 0 if payload_verified else 5

def status(args: argparse.Namespace) -> int:
    verify_path = STATE / "verify.json"
    fetch_path = STATE / "fetch.json"
    doctor_path = STATE / "doctor.json"
    verify_obj = json.loads(verify_path.read_text(errors="replace")) if verify_path.exists() else {}
    fetch_obj = json.loads(fetch_path.read_text(errors="replace")) if fetch_path.exists() else {}
    doctor_obj = json.loads(doctor_path.read_text(errors="replace")) if doctor_path.exists() else {}

    obj = {
        "schema": "Braxon.model_downloader.status.v2",
        "authority": "NSQ_COURT",
        "route": "BRAXON_model_downloader",
        "perpetual_runtime_ready": court_ready(),
        "doctor_ok": doctor_obj.get("ok"),
        "fetch_ok": fetch_obj.get("ok"),
        "payload_verified": verify_obj.get("payload_verified", False),
        "download_complete_claim_allowed": verify_obj.get("download_complete_claim_allowed", False),
        "nsq_recode_complete": False,
        "whole_core_runtime_ready": False,
        "next": "fetch then verify" if not verify_obj.get("payload_verified", False) else "run NSQ recode gate",
    }
    write_json(STATE / "status.json", obj)
    print(json.dumps(obj, indent=2, sort_keys=True))
    return 0 if obj["perpetual_runtime_ready"] else 1

def main() -> int:
    parser = argparse.ArgumentParser(prog="Braxon-model-downloader")
    sub = parser.add_subparsers(dest="cmd", required=True)

    sub.add_parser("identity")
    sub.add_parser("doctor")
    sub.add_parser("plan")
    f = sub.add_parser("fetch")
    f.add_argument("--force", action="store_true")
    sub.add_parser("verify")
    sub.add_parser("status")

    args = parser.parse_args()

    if args.cmd == "identity":
        return identity(args)
    if args.cmd == "doctor":
        return doctor(args)
    if args.cmd == "plan":
        return plan(args)
    if args.cmd == "fetch":
        return fetch(args)
    if args.cmd == "verify":
        return verify(args)
    if args.cmd == "status":
        return status(args)

    return 2

if __name__ == "__main__":
    raise SystemExit(main())
