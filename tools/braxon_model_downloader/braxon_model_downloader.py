#!/usr/bin/env python3
from __future__ import annotations

import argparse
import hashlib
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
from dataclasses import dataclass, asdict
from pathlib import Path
from typing import Any

ROOT = Path(os.environ.get("BRAXON_ROOT", Path.home() / "Braxon")).resolve()
REGISTRY_PATH = ROOT / "config/nsq/huihui_model_registry.json"
STATE_ROOT = ROOT / "state/braxon/model_downloader/current"
LOCAL_ROOT = ROOT / "assets/braxon_core/source_ingest/braxon_transport"
LOCK_PATH = STATE_ROOT / "download.lock"
HF_API = "https://huggingface.co/api"
HF_RESOLVE = "https://huggingface.co"

CHUNK_SIZE = int(os.environ.get("BRAXON_MODEL_CHUNK_SIZE", str(50 * 1024 * 1024)))
MAX_RETRIES = int(os.environ.get("BRAXON_MODEL_MAX_RETRIES", "10"))
RETRY_MAX_WAIT = int(os.environ.get("BRAXON_MODEL_RETRY_MAX_WAIT", "300"))

TOKEN = os.environ.get("HF_TOKEN") or os.environ.get("HUGGINGFACE_HUB_TOKEN") or ""

TEXT_TASKS = {"text-generation", "text-generation-inference", "conversational"}
VISION_MARKERS = ("vl", "vision", "image-text", "image_text", "image-to-text", "image_text_to_text", "multimodal", "any-to-any")

FORBIDDEN_MODEL_IDS = {
    "qwen-32b",
    "deepseek-qwen-32b",
    "mistral-24b",
    "deepseek-32b",
}

REQUIRED_DEFAULT_STACK = [
    "deepseek-v3-671b",
    "qwen3-235b-a22b",
    "qwen2.5-72b",
    "deepseek-v3-671b-analyzer",
    "llama3.3-70b",
    "gemma3-27b",
]

APPROVED_NON_HUIHUI_REPOS = {
    "Qwen/Qwen3-235B-A22B-Instruct",
}

def iso() -> str:
    return time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())

def slug(s: str) -> str:
    return re.sub(r"[^A-Za-z0-9._-]+", "_", s).strip("_")

def jdump(path: Path, obj: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(obj, indent=2, sort_keys=True) + "\n", encoding="utf-8")

def load_json(path: Path) -> Any:
    return json.loads(path.read_text(errors="replace"))

def headers(extra: dict[str, str] | None = None) -> dict[str, str]:
    h = {"User-Agent": "Braxon-NSQ-Court-HuiHui-Downloader/3.0"}
    if TOKEN:
        h["Authorization"] = f"Bearer {TOKEN}"
    if extra:
        h.update(extra)
    return h

def http_json(url: str, timeout: int = 45) -> Any:
    req = urllib.request.Request(url, headers=headers())
    with urllib.request.urlopen(req, timeout=timeout) as r:
        return json.loads(r.read().decode("utf-8", errors="replace"))

def http_head(url: str, timeout: int = 45) -> tuple[int, dict[str, str], str]:
    req = urllib.request.Request(url, method="HEAD", headers=headers())
    opener = urllib.request.build_opener(urllib.request.HTTPRedirectHandler())
    with opener.open(req, timeout=timeout) as r:
        hdr = dict(r.headers.items())
        size = int(hdr.get("Content-Length", "0") or "0")
        return size, hdr, r.geturl()

def http_stream(url: str, start: int, end: int | None, timeout: int = 90):
    range_value = f"bytes={start}-" if end is None else f"bytes={start}-{end}"
    req = urllib.request.Request(url, headers=headers({"Range": range_value}))
    return urllib.request.urlopen(req, timeout=timeout)

def repo_api(repo: str) -> str:
    return f"{HF_API}/models/{urllib.parse.quote(repo, safe='/')}"

def repo_resolve_url(repo: str, filename: str) -> str:
    return f"{HF_RESOLVE}/{repo}/resolve/main/{urllib.parse.quote(filename, safe='/')}"

def load_registry() -> dict[str, Any]:
    return load_json(REGISTRY_PATH)

def search_models(query: str) -> list[dict[str, Any]]:
    url = f"{HF_API}/models?search={urllib.parse.quote(query)}&limit=25&full=true"
    data = http_json(url)
    return data if isinstance(data, list) else []

def model_info(repo: str) -> tuple[dict[str, Any] | None, str | None]:
    urls = [
        f"{HF_API}/models/{urllib.parse.quote(repo, safe='/')}",
        f"{HF_API}/models/{urllib.parse.quote(repo, safe='')}",
    ]

    last_error = None
    for url in urls:
        try:
            return http_json(url), None
        except urllib.error.HTTPError as err:
            if err.code in {401, 403}:
                return None, f"requires_auth_or_terms_http_{err.code}"
            if err.code == 404:
                last_error = "not_found_http_404"
                continue
            if err.code == 400:
                last_error = "metadata_unavailable_http_400"
                continue
            last_error = f"http_error_{err.code}"
        except Exception as err:
            last_error = f"metadata_error_{type(err).__name__}"

    return None, last_error or "metadata_unavailable"

def sibling_names(info: dict[str, Any]) -> list[str]:
    out = []
    for s in info.get("siblings", []) or []:
        name = s.get("rfilename")
        if isinstance(name, str):
            out.append(name)
    return sorted(set(out))

def has_vision(repo: str, info: dict[str, Any]) -> bool:
    text = " ".join([
        repo,
        str(info.get("pipeline_tag", "")),
        " ".join(map(str, info.get("tags", []) or [])),
        str(info.get("cardData", "")),
    ]).lower()
    return any(m in text for m in VISION_MARKERS)

def is_text_aligned(info: dict[str, Any], allow_vl: bool) -> bool:
    pipe = str(info.get("pipeline_tag", "")).lower()
    tags = {str(t).lower() for t in info.get("tags", []) or []}
    if has_vision(str(info.get("id", "")), info) and not allow_vl:
        return False
    if pipe in TEXT_TASKS:
        return True
    if "text-generation" in tags or "conversational" in tags or "gguf" in tags:
        return True
    if allow_vl and ("image-text-to-text" in pipe or "any-to-any" in pipe):
        return True
    return False

def license_summary(info: dict[str, Any] | None) -> dict[str, Any]:
    if info is None:
        return {
            "metadata_available": False,
            "open_weight_metadata_seen": False,
            "license": None,
            "license_tags": [],
            "status": "not_verified_from_metadata",
        }

    card = info.get("cardData")
    license_value = None
    if isinstance(card, dict):
        license_value = card.get("license")

    tags = [str(t) for t in info.get("tags", []) or []]
    license_tags = [t for t in tags if t.startswith("license:")]
    low = " ".join([str(license_value or ""), " ".join(license_tags)]).lower()
    openish = any(k in low for k in [
        "apache",
        "mit",
        "bsd",
        "openrail",
        "creativeml",
        "qwen",
        "llama",
        "gemma",
    ])

    return {
        "metadata_available": True,
        "open_weight_metadata_seen": openish,
        "license": license_value,
        "license_tags": license_tags,
        "status": "open_weight_signal_seen" if openish else "license_signal_not_enough",
    }

def enforce_BRAXON_stack_policy(model_id: str, spec: dict[str, Any]) -> None:
    if model_id in FORBIDDEN_MODEL_IDS:
        raise SystemExit(f"forbidden model_id={model_id}; stack requires the six canonical open-weight Citadel699 targets")

    repo = str(spec.get("exact_repo", ""))
    low = repo.lower()

    if spec.get("raw_fetch_allowed", False) or spec.get("raw_payload_transfer_allowed", False):
        raise SystemExit(f"forbidden raw payload transfer policy for {model_id}: {repo}")

    if not spec.get("open_weight_required", True):
        raise SystemExit(f"open-weight requirement must remain active for {model_id}: {repo}")

    huihui = repo.startswith("huihui-ai/")
    approved_exception = spec.get("approved_non_huihui_abliterated", False) and repo in APPROVED_NON_HUIHUI_REPOS

    if not huihui and not approved_exception:
        raise SystemExit(f"forbidden non-HuiHui repo for {model_id}: {repo}")

    censor_free = bool(spec.get("censor_free_required", True))
    name_signals_censor_free = "abliterated" in low or "censor" in low
    if censor_free and not (name_signals_censor_free or approved_exception):
        raise SystemExit(f"forbidden non-abliterated repo for {model_id}: {repo}")

    if "vl-32b" in low or "qwen3-vl-32b" in low:
        raise SystemExit(f"forbidden wrong VL 32B route for {model_id}: {repo}")

    if model_id == "qwen3-235b-a22b" and repo != "Qwen/Qwen3-235B-A22B-Instruct":
        raise SystemExit("Qwen/Creativity slot must remain the explicit official open-weight Qwen theater target unless registry law changes")

def validate_registry_policy(reg: dict[str, Any]) -> None:
    models = reg.get("models", {})
    default = reg.get("default_stack", [])

    if default != REQUIRED_DEFAULT_STACK:
        raise SystemExit(f"default stack drift: expected={REQUIRED_DEFAULT_STACK} got={default}")

    for bad in FORBIDDEN_MODEL_IDS:
        if bad in models or bad in default:
            raise SystemExit(f"forbidden active model present: {bad}")

    for model_id in default:
        if model_id not in models:
            raise SystemExit(f"default stack model missing from registry: {model_id}")
        enforce_BRAXON_stack_policy(model_id, models[model_id])

    if len(default) != int(reg.get("required_model_count", 6)):
        raise SystemExit(f"default stack count drift: expected={reg.get('required_model_count', 6)} got={len(default)}")

def score_candidate(repo: str, info: dict[str, Any], spec: dict[str, Any]) -> int:
    rid = str(info.get("id") or repo)
    low = rid.lower()
    score = 0
    family = str(spec.get("family", "")).lower()
    size = str(spec.get("size", "")).lower()

    if rid.startswith("huihui-ai/"):
        score += 30
    if "abliterated" in low:
        score += 30
    if family and family.replace("-", "") in low.replace("-", ""):
        score += 20
    if size and size.replace("b", "") in low:
        score += 20
    if "gguf" in low and "gguf" in spec.get("prefer_formats", []):
        score += 10
    if "vl" in low and not spec.get("allow_vl", False):
        score -= 100
    if is_text_aligned(info, bool(spec.get("allow_vl", False))):
        score += 20

    return score

def select_files(info: dict[str, Any], spec: dict[str, Any]) -> list[str]:
    names = sibling_names(info)
    prefer_formats = list(spec.get("prefer_formats", ["safetensors", "gguf"]))
    include_patterns = list(spec.get("gguf_include_patterns", []))
    selected: list[str] = []

    always = {
        "config.json",
        "generation_config.json",
        "tokenizer.json",
        "tokenizer_config.json",
        "special_tokens_map.json",
        "chat_template.jinja",
        "merges.txt",
        "vocab.json",
        "model.safetensors.index.json",
    }

    for n in names:
        if Path(n).name in always:
            selected.append(n)

    if "gguf" in prefer_formats:
        ggufs = [n for n in names if n.endswith(".gguf")]
        if include_patterns:
            filtered = [n for n in ggufs if any(p in n for p in include_patterns)]
            selected.extend(filtered or ggufs)
        else:
            selected.extend(ggufs)

    if not any(n.endswith((".safetensors", ".gguf")) for n in selected):
        if "safetensors" in prefer_formats:
            selected.extend(n for n in names if n.endswith(".safetensors"))
        elif "gguf" in prefer_formats:
            selected.extend(n for n in names if n.endswith(".gguf"))

    return sorted(set(selected))

def resolved_record(model_id: str, spec: dict[str, Any], repo: str, info: dict[str, Any] | None, reason: str, access_status: str, score: int = 0) -> dict[str, Any]:
    if info is None:
        files: list[str] = []
        pipeline_tag = None
        tags: list[Any] = []
        sha = None
        vision_detected = False
    else:
        source_candidate_files = select_files(info, spec)
        raw_source_candidates = [
            name for name in source_candidate_files
            if Path(name).suffix.lower() in {".safetensors", ".gguf"}
        ]
        if spec.get("raw_payload_transfer_allowed", False):
            files = source_candidate_files
        else:
            files = [
                name for name in source_candidate_files
                if Path(name).suffix.lower() not in {".safetensors", ".gguf"}
            ]
        pipeline_tag = info.get("pipeline_tag")
        tags = info.get("tags", [])
        sha = info.get("sha")
        vision_detected = has_vision(repo, info)
    if info is None:
        source_candidate_files = []
        raw_source_candidates = []

    mode = spec.get("download_mode", "api_files")
    if access_status.startswith("requires_auth_or_terms"):
        mode = "hf_cli_snapshot_when_gated"

    license_review = license_summary(info)
    metadata_review_required = access_status != "ok"
    if license_review["status"] != "open_weight_signal_seen":
        metadata_review_required = True
    if vision_detected and not spec.get("allow_vl", False):
        metadata_review_required = True

    review_reasons = []
    if access_status != "ok":
        review_reasons.append("target_not_live_or_metadata_not_fully_verified")
    if license_review["status"] != "open_weight_signal_seen":
        review_reasons.append("open_weight_license_signal_not_verified")
    if vision_detected and not spec.get("allow_vl", False):
        review_reasons.append("metadata_vision_signal_requires_intent_review")

    rec = {
        "schema": "Braxon.model_downloader.resolved_model.v5",
        "generated_at": iso(),
        "authority": "NSQ_COURT",
        "architecture_root": True,
        "model_id": model_id,
        "repo_id": repo,
        "resolution_reason": reason,
        "access_status": access_status,
        "score": score,
        "exact_repo_requested": spec["exact_repo"],
        "exact_match": repo == spec["exact_repo"],
        "allow_vl": bool(spec.get("allow_vl", False)),
        "vision_detected": vision_detected,
        "pipeline_tag": pipeline_tag,
        "tags": tags,
        "sha": sha,
        "download_mode": mode,
        "candidate_review_required": metadata_review_required,
        "candidate_review_reason": None if not review_reasons else "; ".join(review_reasons) + "; review best open-weight same-intent fit before import",
        "license_review": license_review,
        "source_candidate_files_seen": len(source_candidate_files),
        "raw_source_candidate_count": len(raw_source_candidates),
        "raw_source_candidates_observed_only": raw_source_candidates[:50],
        "selected_files": files,
        "local_dir": str((LOCAL_ROOT / slug(model_id)).relative_to(ROOT)),
        "requires_hf_token_or_terms": access_status.startswith("requires_auth_or_terms"),
        "open_weight_required": bool(spec.get("open_weight_required", True)),
        "censor_free_required": bool(spec.get("censor_free_required", True)),
        "preferred_huihui": bool(spec.get("preferred_huihui", True)),
        "truth_boundary": {
            "download_is_not_nsq_recode": True, "normal_transfer_is_citadel699_nsq_request_not_fetch": True,
            "nsq_recode_required_after_download": True,
            "whole_core_runtime_verification_required": True,
            "placeholders_are_not_runtime_material": True,
            "incomplete_models_are_not_acceptable": True,
            "raw_weight_download_allowed": False
        }
    }
    jdump(STATE_ROOT / f"{slug(model_id)}.resolved.json", rec)
    return rec

def resolve_model(model_id: str) -> dict[str, Any]:
    reg = load_registry()
    validate_registry_policy(reg)
    models = reg["models"]

    if model_id not in models:
        raise SystemExit(f"unknown model_id={model_id}; available={', '.join(sorted(models))}")

    spec = models[model_id]
    enforce_BRAXON_stack_policy(model_id, spec)
    blocklist = set(reg.get("wrong_model_blocklist", []))
    exact = spec["exact_repo"]

    if exact in blocklist:
        raise SystemExit(f"exact repo for {model_id} is blocklisted: {exact}")

    info, access = model_info(exact)

    if info is not None:
        return resolved_record(model_id, spec, exact, info, "exact_repo", "ok", score_candidate(exact, info, spec))

    if access and access.startswith("requires_auth_or_terms") and spec.get("requires_auth_or_terms_possible", False):
        return resolved_record(model_id, spec, exact, None, "exact_repo_gated", access, 100)

    if access in {"metadata_unavailable_http_400", "metadata_unavailable"} and spec.get("requires_auth_or_terms_possible", False):
        return resolved_record(model_id, spec, exact, None, "exact_repo_metadata_unavailable", access, 95)

    if access and access != "not_found_http_404":
        raise SystemExit(f"exact repo did not resolve as public metadata but is policy-locked, not missing: {model_id}; exact={exact}; exact_access={access}")

    fallback_queries = list(spec.get("fallback_queries", []))
    if not fallback_queries:
        fallback_queries = [
            str(spec.get("display_name", "")),
            str(spec.get("source_family", "")),
            f"{model_id} abliterated",
            f"{model_id} censor free",
            f"{model_id} open weights",
        ]

    candidates: list[tuple[int, str, dict[str, Any], str]] = []
    for q in fallback_queries:
        if not q.strip():
            continue
        try:
            for item in search_models(q):
                repo = item.get("id")
                if not isinstance(repo, str) or repo in blocklist:
                    continue
                repo_low = repo.lower()
                approved_official_qwen = model_id == "qwen3-235b-a22b" and repo == "Qwen/Qwen3-235B-A22B-Instruct"
                if not approved_official_qwen and (not repo.startswith("huihui-ai/") or "abliterated" not in repo_low):
                    continue
                cand_info, _ = model_info(repo)
                if cand_info is None:
                    continue
                if has_vision(repo, cand_info) and not spec.get("allow_vl", False):
                    continue
                sc = score_candidate(repo, cand_info, spec)
                candidates.append((sc, repo, cand_info, f"search:{q}"))
        except Exception:
            continue

    if not candidates:
        raise SystemExit(f"could not resolve aligned HuiHui abliterated model for {model_id}; exact={exact}; exact_access={access}")

    candidates.sort(key=lambda x: x[0], reverse=True)
    score, repo, info, reason = candidates[0]
    return resolved_record(model_id, spec, repo, info, reason, "ok", score)


def enforce_no_raw_fetch_gate(model_id: str) -> None:
    raise SystemExit(json.dumps({
        "schema": "Braxon.model_downloader.raw_fetch_blocked.v2",
        "authority": "NSQ_COURT",
        "model_id": model_id,
        "blocked": True,
        "reason": "normal Braxon model transfer is Citadel699 NSQ request/return/rebuild, not raw fetch/download",
        "replacement_command": f"bin/Braxon-model-request {model_id}",
        "cathedral_flow": "post_near_source_receive_and_translate",
        "raw_huggingface_payload_fetch_allowed": False,
        "raw_gguf_transfer_allowed": False,
        "raw_safetensors_transfer_allowed": False,
        "git_lfs_pointer_allowed": False,
        "pointer_setup_allowed": False,
        "truth_boundary": {
            "huggingface_payload_download_is_not_custom_nsq_transfer": True,
            "fetch_word_is_disallowed_for_normal_model_transfer": True,
            "whole_model_runtime_verification_required": True,
            "placeholders_are_not_runtime_material": True,
            "incomplete_models_are_not_acceptable": True
        }
    }, indent=2, sort_keys=True))

class Lock:
    def __enter__(self):
        STATE_ROOT.mkdir(parents=True, exist_ok=True)
        if LOCK_PATH.exists():
            age = time.time() - LOCK_PATH.stat().st_mtime
            if age < 900:
                raise SystemExit(f"active lock present: {LOCK_PATH} age={age:.0f}s")
            LOCK_PATH.unlink()
        LOCK_PATH.write_text(str(os.getpid()) + "\n")
        return self

    def __exit__(self, *args):
        try:
            LOCK_PATH.unlink()
        except FileNotFoundError:
            pass

@dataclass
class FileState:
    repo_id: str
    filename: str
    url: str
    local_path: str
    total_size: int
    downloaded_bytes: int
    sha256_partial: str
    status: str
    updated_at: str

def state_path(model_id: str, filename: str) -> Path:
    return STATE_ROOT / slug(model_id) / (slug(filename) + ".state.json")

def save_file_state(model_id: str, fs: FileState) -> None:
    jdump(state_path(model_id, fs.filename), asdict(fs))

def load_file_state(model_id: str, repo: str, filename: str, url: str, local_path: Path, size: int) -> FileState:
    sp = state_path(model_id, filename)
    if sp.exists():
        return FileState(**load_json(sp))
    return FileState(repo, filename, url, str(local_path), size, local_path.stat().st_size if local_path.exists() else 0, "", "downloading", iso())

def download_file(model_id: str, repo: str, filename: str, local_base: Path) -> dict[str, Any]:
    url = repo_resolve_url(repo, filename)
    local_path = local_base / filename
    local_path.parent.mkdir(parents=True, exist_ok=True)

    size, _, final_url = http_head(url)
    if size <= 0:
        raise RuntimeError(f"cannot determine remote size for {filename}")

    fs = load_file_state(model_id, repo, filename, final_url, local_path, size)
    start = local_path.stat().st_size if local_path.exists() else 0
    fs.downloaded_bytes = start
    save_file_state(model_id, fs)

    retries = 0
    while start < size:
        end = min(start + CHUNK_SIZE, size) - 1
        try:
            with http_stream(final_url, start, end) as r:
                h = hashlib.sha256()
                bytes_this_chunk = 0
                with open(local_path, "ab" if start > 0 else "wb", buffering=1024 * 1024) as f:
                    while True:
                        block = r.read(1024 * 1024)
                        if not block:
                            break
                        f.write(block)
                        h.update(block)
                        bytes_this_chunk += len(block)
                        fs.downloaded_bytes += len(block)

                if bytes_this_chunk <= 0:
                    raise RuntimeError("zero-byte chunk")

                start = local_path.stat().st_size
                fs.sha256_partial = h.hexdigest()
                fs.updated_at = iso()
                save_file_state(model_id, fs)
                print(f"[chunk] {filename} {(start / size) * 100:.2f}% {start}/{size}")

        except Exception as err:
            retries += 1
            if retries > MAX_RETRIES:
                fs.status = "failed"
                save_file_state(model_id, fs)
                raise
            wait = min(2 ** retries, RETRY_MAX_WAIT)
            print(f"[retry] {filename} retry={retries}/{MAX_RETRIES} wait={wait}s error={err}")
            time.sleep(wait)
            start = local_path.stat().st_size if local_path.exists() else 0
            fs.downloaded_bytes = start
            save_file_state(model_id, fs)

    full_hash = hashlib.sha256()
    with open(local_path, "rb") as f:
        for block in iter(lambda: f.read(1024 * 1024), b""):
            full_hash.update(block)

    fs.status = "completed"
    fs.downloaded_bytes = local_path.stat().st_size
    fs.sha256_partial = full_hash.hexdigest()
    fs.updated_at = iso()
    save_file_state(model_id, fs)

    return {
        "filename": filename,
        "local_path": str(local_path.relative_to(ROOT)),
        "size_bytes": fs.downloaded_bytes,
        "sha256": fs.sha256_partial,
        "status": fs.status,
    }

def run_hf_cli_snapshot(model_id: str, repo: str, local_base: Path) -> dict[str, Any]:
    cli = shutil.which("huggingface-cli")
    if not cli:
        raise SystemExit("huggingface-cli missing. Install with: pip install -U huggingface_hub")
    if not TOKEN:
        raise SystemExit(f"HF_TOKEN is required for gated or snapshot download: {repo}")

    local_base.mkdir(parents=True, exist_ok=True)
    cmd = [cli, "download", repo, "--local-dir", str(local_base), "--token", TOKEN]

    started = time.monotonic()
    proc = subprocess.run(cmd, cwd=ROOT, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False)
    elapsed = round(time.monotonic() - started, 3)

    (STATE_ROOT / f"{slug(model_id)}.hf_cli.stdout.txt").write_text(proc.stdout, encoding="utf-8")
    (STATE_ROOT / f"{slug(model_id)}.hf_cli.stderr.txt").write_text(proc.stderr, encoding="utf-8")

    if proc.returncode != 0:
        raise SystemExit(f"huggingface-cli download failed rc={proc.returncode}; see state/braxon/model_downloader/current/{slug(model_id)}.hf_cli.stderr.txt")

    files = []
    for p in local_base.rglob("*"):
        if p.is_file():
            files.append({
                "filename": str(p.relative_to(local_base)),
                "local_path": str(p.relative_to(ROOT)),
                "size_bytes": p.stat().st_size,
                "status": "completed",
            })

    return {
        "mode": "hf_cli_snapshot",
        "elapsed_seconds": elapsed,
        "completed_files": sorted(files, key=lambda x: x["filename"]),
    }

def run_ingest_hook(model_id: str, completed: list[dict[str, Any]]) -> None:
    log = STATE_ROOT / f"{slug(model_id)}.ingest_queue.jsonl"
    log.parent.mkdir(parents=True, exist_ok=True)
    with log.open("a", encoding="utf-8") as f:
        for item in completed:
            f.write(json.dumps({
                "generated_at": iso(),
                "authority": "NSQ_COURT",
                "event": "payload_file_completed",
                "model_id": model_id,
                **item,
                "nsq_recode_required": True,
            }, sort_keys=True) + "\n")

def cmd_list(_: argparse.Namespace) -> None:
    reg = load_registry()
    validate_registry_policy(reg)
    print(json.dumps({
        "schema": "Braxon.model_downloader.list.v3",
        "authority": "NSQ_COURT",
        "default_stack": reg.get("default_stack", []),
        "optional_stack": reg.get("optional_stack", []),
        "models": reg["models"],
    }, indent=2, sort_keys=True))

def cmd_stack(_: argparse.Namespace) -> None:
    reg = load_registry()
    validate_registry_policy(reg)
    rows = []
    for mid in reg.get("default_stack", []):
        spec = reg["models"][mid]
        repo = spec["exact_repo"]
        rows.append({
            "model_id": mid,
            "repo": repo,
            "huihui": repo.startswith("huihui-ai/"),
            "abliterated": "abliterated" in repo.lower(),
            "size": spec["size"],
            "pole": spec["pole"],
            "allow_vl": spec.get("allow_vl", False),
            "open_weight_required": spec.get("open_weight_required", True),
            "preferred_huihui": spec.get("preferred_huihui", True),
            "censor_free_required": spec.get("censor_free_required", True),
        })
    print(json.dumps({
        "schema": "Braxon.model_downloader.stack.v2",
        "authority": "NSQ_COURT",
        "open_weight_required_for_all": all(r["open_weight_required"] for r in rows),
        "huihui_or_approved_exception_for_all": all(r["huihui"] or r["repo"] in APPROVED_NON_HUIHUI_REPOS for r in rows),
        "abliterated_or_approved_censor_free_exception_for_all": all(r["abliterated"] or r["repo"] in APPROVED_NON_HUIHUI_REPOS for r in rows),
        "default_stack": rows,
        "forbidden_model_ids": sorted(FORBIDDEN_MODEL_IDS),
    }, indent=2, sort_keys=True))

def cmd_resolve(args: argparse.Namespace) -> None:
    print(json.dumps(resolve_model(args.model_id), indent=2, sort_keys=True))

def cmd_plan(args: argparse.Namespace) -> None:
    resolved = resolve_model(args.model_id)
    plan = {
        "schema": "Braxon.model_downloader.plan.v5",
        "generated_at": iso(),
        "authority": "NSQ_COURT",
        "model_id": args.model_id,
        "repo_id": resolved["repo_id"],
        "access_status": resolved["access_status"],
        "download_mode": resolved["download_mode"],
        "requires_hf_token_or_terms": resolved["requires_hf_token_or_terms"],
        "selected_files": resolved["selected_files"],
        "local_dir": resolved["local_dir"],
        "request_command": f"bin/Braxon-model-request {args.model_id}",
        "verify_command": f"bin/Braxon-model-downloader verify {args.model_id}",
        "cathedral_flow": "post_near_source_receive_and_translate",
        "target_size_class": "mb_scale",
        "tiny_seed_reconstruction_required": True,
        "nurabit_citadel_groups": 21,
        "nurabit_group_width_nsq_bit_units": 33,
        "nurabit_groups_communicate": True,
        "candidate_review_required": resolved["candidate_review_required"],
        "candidate_review_reason": resolved["candidate_review_reason"],
        "license_review": resolved["license_review"],
        "raw_weight_download_allowed": False,
        "truth_boundary": resolved["truth_boundary"],
    }
    jdump(STATE_ROOT / f"{slug(args.model_id)}.plan.json", plan)
    print(json.dumps(plan, indent=2, sort_keys=True))

def cmd_fetch(args: argparse.Namespace) -> None:
    enforce_no_raw_fetch_gate(args.model_id)
    with Lock():
        resolved = resolve_model(args.model_id)
        repo = resolved["repo_id"]
        local_base = ROOT / resolved["local_dir"]
        local_base.mkdir(parents=True, exist_ok=True)

        completed: list[dict[str, Any]] = []
        if resolved["requires_hf_token_or_terms"] or not resolved["selected_files"] or resolved["download_mode"].startswith("hf_cli_snapshot"):
            snap = run_hf_cli_snapshot(args.model_id, repo, local_base)
            completed = snap["completed_files"]
            run_ingest_hook(args.model_id, completed)
        else:
            for filename in resolved["selected_files"]:
                item = download_file(args.model_id, repo, filename, local_base)
                completed.append(item)
                run_ingest_hook(args.model_id, [item])

        report = {
            "schema": "Braxon.model_downloader.fetch_report.v5",
            "generated_at": iso(),
            "authority": "NSQ_COURT",
            "model_id": args.model_id,
            "repo_id": repo,
            "local_dir": resolved["local_dir"],
            "completed_files": completed,
            "download_complete": True,
            "payload_verified": False,
            "nsq_recode_complete": False,
            "whole_core_runtime_ready": False,
        }
        jdump(STATE_ROOT / f"{slug(args.model_id)}.fetch.json", report)
        print(json.dumps(report, indent=2, sort_keys=True))

def cmd_verify(args: argparse.Namespace) -> None:
    resolved_path = STATE_ROOT / f"{slug(args.model_id)}.resolved.json"
    resolved = load_json(resolved_path) if resolved_path.exists() else resolve_model(args.model_id)
    local_base = ROOT / resolved["local_dir"]

    files = []
    ok = local_base.exists() and any(p.is_file() and p.stat().st_size > 0 for p in local_base.rglob("*"))

    if local_base.exists():
        for p in sorted(x for x in local_base.rglob("*") if x.is_file()):
            files.append({
                "filename": str(p.relative_to(local_base)),
                "present": True,
                "size_bytes": p.stat().st_size,
                "path": str(p.relative_to(ROOT)),
            })

    report = {
        "schema": "Braxon.model_downloader.verify.v5",
        "generated_at": iso(),
        "authority": "NSQ_COURT",
        "model_id": args.model_id,
        "repo_id": resolved["repo_id"],
        "payload_present": ok,
        "payload_verified": ok,
        "nsq_recode_complete": False,
        "whole_core_runtime_ready": False,
        "files": files,
        "truth_boundary": {
            "payload_verified_is_not_nsq_recode": True,
            "nsq_recode_required_after_download": True,
            "whole_core_runtime_verification_required": True,
        },
    }
    jdump(STATE_ROOT / f"{slug(args.model_id)}.verify.json", report)
    print(json.dumps(report, indent=2, sort_keys=True))

def cmd_status(args: argparse.Namespace) -> None:
    model_id = args.model_id
    paths = {
        "resolved": STATE_ROOT / f"{slug(model_id)}.resolved.json",
        "plan": STATE_ROOT / f"{slug(model_id)}.plan.json",
        "fetch": STATE_ROOT / f"{slug(model_id)}.fetch.json",
        "verify": STATE_ROOT / f"{slug(model_id)}.verify.json",
    }
    out = {
        "schema": "Braxon.model_downloader.status.v7",
        "generated_at": iso(),
        "authority": "NSQ_COURT",
        "model_id": model_id,
        "resolved": paths["resolved"].exists(),
        "plan_present": paths["plan"].exists(),
        "fetch_present": paths["fetch"].exists(),
        "verify_present": paths["verify"].exists(),
        "download_complete_claim_allowed": False,
        "payload_verified": False,
        "nsq_recode_complete": False,
        "whole_core_runtime_ready": False,
    }
    if paths["verify"].exists():
        out["payload_verified"] = bool(load_json(paths["verify"]).get("payload_verified"))
    if paths["fetch"].exists():
        out["download_complete_claim_allowed"] = bool(load_json(paths["fetch"]).get("download_complete"))
    print(json.dumps(out, indent=2, sort_keys=True))

def main() -> None:
    p = argparse.ArgumentParser(prog="Braxon-model-downloader")
    sub = p.add_subparsers(dest="cmd", required=True)

    sub.add_parser("list").set_defaults(func=cmd_list)
    sub.add_parser("stack").set_defaults(func=cmd_stack)

    for name, func in [
        ("resolve", cmd_resolve),
        ("plan", cmd_plan),
        ("fetch", cmd_fetch),
        ("verify", cmd_verify),
        ("status", cmd_status),
    ]:
        sp = sub.add_parser(name)
        sp.add_argument("model_id")
        sp.set_defaults(func=func)

    args = p.parse_args()
    args.func(args)

if __name__ == "__main__":
    main()
