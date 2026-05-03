#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import subprocess
import tarfile
import time
from pathlib import Path
from typing import Any

ROOT = Path(os.environ.get("BRAXON_ROOT", Path.home() / "Braxon")).resolve()
DL = Path.home() / "storage/shared/Download"

CONFIG = ROOT / "config/nsq/citadel699_wire_receptor.json"
REGISTRY = ROOT / "config/nsq/huihui_model_registry.json"
STATE = ROOT / "state/nsq/citadel699/current"
IMPORT_ROOT = ROOT / "assets/braxon_core/source_ingest/nsq_transport/citadel699"

MAX_BUNDLE_BYTES = int(os.environ.get("BRAXON_CITADEL699_MAX_BUNDLE_BYTES", str(1024 * 1024 * 1024)))
SAMPLE_BYTES = int(os.environ.get("BRAXON_CITADEL699_TRIPLE_BITE_BYTES", str(64 * 1024)))

POINTER_PREFIX = b"version https://git-lfs.github.com/spec/v1"
FORBIDDEN_WIRE_SUFFIXES = {".safetensors", ".gguf"}

DEFAULT_STACK = [
    "deepseek-v3-671b",
    "qwen3-235b-a22b",
    "qwen2.5-72b",
    "deepseek-v3-671b-analyzer",
    "llama3.3-70b",
    "gemma3-27b",
]

FALLBACK_SYSTEM_TARGETS = [
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
    "wowas_canon_engine",
]

REQUIRED_SKILL_FIELDS = [
    "summary",
    "structural_patterns",
    "translation_skills",
    "compression_skills",
    "routing_skills",
    "compatibility_skills",
    "rebuild_skills",
    "verification_skills",
    "reuse_candidates",
    "confidence",
    "limits",
]

REQUIRED_PROHIBITED_FLAGS = [
    "raw_source_code_retained",
    "raw_payload_bytes_retained",
    "credentials_or_tokens_retained",
    "private_identifiers_retained",
    "third_party_secrets_retained",
    "external_user_data_retained",
]

def iso() -> str:
    return time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())

def slug(s: str) -> str:
    return re.sub(r"[^A-Za-z0-9._-]+", "_", s).strip("_")

def load_json(path: Path) -> Any:
    return json.loads(path.read_text(errors="replace"))

def write_json(path: Path, obj: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(obj, indent=2, sort_keys=True) + "\n", encoding="utf-8")

def config() -> dict[str, Any]:
    if CONFIG.exists():
        data = load_json(CONFIG)
        return data if isinstance(data, dict) else {}
    return {}

def system_targets() -> list[str]:
    cfg = config()
    xs = cfg.get("system_awareness_targets")
    if isinstance(xs, list) and xs:
        return [str(x) for x in xs]
    return FALLBACK_SYSTEM_TARGETS[:]

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

def triple_bite(path: Path) -> dict[str, Any]:
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
            })

    ok = all(s["bytes"] > 0 for s in spans) and not any(
        s["all_zero"] or s["looks_lfs_pointer"] for s in spans
    )
    return {"ok": ok, "spans": spans}

def registry_models() -> dict[str, Any]:
    if not REGISTRY.exists():
        return {}
    data = load_json(REGISTRY)
    return data.get("models", {}) if isinstance(data, dict) else {}

def registry_default_stack() -> list[str]:
    if REGISTRY.exists():
        reg = load_json(REGISTRY)
        stack = reg.get("default_stack")
        if isinstance(stack, list) and stack:
            return [str(x) for x in stack]
    return DEFAULT_STACK[:]

def stack_models(selected: list[str]) -> list[str]:
    clean = [str(x) for x in selected if str(x).strip()]
    if not clean or "all" in clean:
        return registry_default_stack()

    out: list[str] = []
    seen = set()
    for x in clean:
        if x not in seen:
            seen.add(x)
            out.append(x)
    return out

def repo_policy_allowed(repo: str, spec: dict[str, Any]) -> bool:
    huihui = isinstance(repo, str) and repo.startswith("huihui-ai/")
    abliterated = isinstance(repo, str) and "abliterated" in repo.lower()
    approved_exception = bool(spec.get("approved_non_huihui_abliterated"))
    open_weight_required = bool(spec.get("open_weight_required", True))
    censor_free_required = bool(spec.get("censor_free_required", True))
    censor_free_signal = abliterated or approved_exception
    return bool(open_weight_required and censor_free_required and censor_free_signal and (huihui or approved_exception))


def model_record(model_id: str, models: dict[str, Any]) -> dict[str, Any]:
    spec = models.get(model_id, {})
    repo = spec.get("exact_repo", "")
    huihui = isinstance(repo, str) and repo.startswith("huihui-ai/")
    abliterated = isinstance(repo, str) and "abliterated" in repo.lower()
    approved_exception = bool(spec.get("approved_non_huihui_abliterated"))
    policy_allowed = repo_policy_allowed(repo, spec)

    return {
        "model_id": model_id,
        "repo_id": repo,
        "huihui": huihui,
        "abliterated": abliterated,
        "approved_non_huihui_abliterated": approved_exception,
        "open_weight_required": bool(spec.get("open_weight_required", True)),
        "censor_free_required": bool(spec.get("censor_free_required", True)),
        "preferred_huihui": bool(spec.get("preferred_huihui", True)),
        "if_exact_target_not_live": "review_best_fit_for_same_intent_and_return_candidate_review; do_not_download_raw_weights",
        "policy_allowed": policy_allowed,
        "size": spec.get("size"),
        "pole": spec.get("pole"),
        "wire_form_requested": "nsq_only",
        "raw_payload_transfer_allowed": False,
    }


def make_learning_contract() -> dict[str, Any]:
    targets = system_targets()
    return {
        "learn_from_surroundings": True,
        "allowed_learning_form": "abstract_transferable_system_skills_only",
        "raw_source_code_retention_allowed": False,
        "raw_payload_byte_retention_allowed": False,
        "credentials_retention_allowed": False,
        "private_identifier_retention_allowed": False,
        "third_party_secret_retention_allowed": False,
        "learned_skills_breakdown_required": True,
        "bad_habits_rejection_required": True,
        "moral_filter_report_required": True,
        "system_benefit_map_required": True,
        "every_target_system_must_receive_benefit_notes": True,
        "required_learned_skill_fields": REQUIRED_SKILL_FIELDS,
        "required_prohibited_retention_flags": REQUIRED_PROHIBITED_FLAGS,
        "system_awareness_targets": targets,
        "return_manifest_shape": {
            "learned_skills_breakdown": "object",
            "system_benefit_map": {target: "object" for target in targets},
            "prohibited_retention_report": {flag: False for flag in REQUIRED_PROHIBITED_FLAGS},
            "moral_filter_report": "object"
        }
    }

def make_request(args: argparse.Namespace) -> None:
    models = registry_models()
    targets = stack_models(args.model)

    out_dir = Path(args.out_dir).expanduser()
    if not out_dir.is_absolute():
        out_dir = (DL / out_dir).resolve()
    out_dir.mkdir(parents=True, exist_ok=True)

    records = [model_record(mid, models) for mid in targets]
    bad = [r for r in records if not r.get("policy_allowed")]

    request = {
        "schema": "Braxon.nsq.citadel699.request.v2",
        "generated_at": iso(),
        "authority": "NSQ_COURT",
        "route": "citadel699_wire_receptor",
        "request_id": f"citadel699_{int(time.time())}",
        "max_return_bundle_bytes": MAX_BUNDLE_BYTES,
        "transfer_form": "nsq_only",
        "citadel_form": "nuband_instruction_set",
        "citadel_is_llm": False,
        "citadel_is_model": False,
        "source_side_raw_material_allowed": True,
        "wire_form": "nsq_language_wire",
        "post_mode": "cathedral_post_near_source_receive_and_translate",
        "target_size_class": "mb_scale",
        "tiny_seed_reconstruction_required": True,
        "nurabit_citadel_groups": 21,
        "nurabit_group_width_nsq_bit_units": 33,
        "nurabit_groups_communicate": True,
        "raw_weight_download_allowed": False,
        "bit_law": {
            "bits_are_valued": True,
            "bits_are_recycled": True,
            "content_addressed_fragments": True,
            "duplicate_bits_must_fold": True,
            "null_spans_forbidden": True,
            "lfs_pointer_forbidden": True,
            "raw_gguf_forbidden_on_wire": True,
            "raw_safetensors_forbidden_on_wire": True
        },
        "learning_contract": make_learning_contract(),
        "models": records,
        "policy_ok": not bad,
        "required_model_count": 6,
        "actual_model_count": len(targets),
        "exactly_six_models": len(targets) == 6,
        "policy_errors": bad,
        "remote_instruction": (
            "Citadel is a nuband instruction set, not an LLM and not a model. "
            "Post the Cathedral request near the authorized source; raw source may be handled on that source side only. "
            "Translate onto the wire as NSQ language rebuild material by structure-preserving base-8 semantics. "
            "If an exact target is not live, do not fabricate success and do not download raw weights into Braxon; "
            "review the best open-weight, preferably HuiHui/abliterated or otherwise censor-free, fit for the same intent "
            "and return candidate_review records for approval/import verification. "
            "Transform authorized source payload into NSQ rebuild fragments before return. "
            "Do not return raw GGUF, raw safetensors, Git LFS pointers, null spans, duplicated blob sludge, "
            "raw source code, credentials, private identifiers, third-party secrets, or external user data. "
            "Return a tar.gz bundle containing manifest.json plus NSQ/content-addressed rebuild fragments only. "
            "The manifest must include learned_skills_breakdown, prohibited_retention_report, moral_filter_report, "
            "and system_benefit_map for every configured Braxon/NSQ target system. Learned skills must be abstract, "
            "structural, reusable, and morally filtered."
        ),
        "truth_boundary": {
            "this_request_is_not_a_model_download": True,
            "citadel_is_llm": False,
            "citadel_is_model": False,
            "citadel_is_nuband_instruction_set": True,
            "raw_source_may_exist_at_citadel_source_side": True,
            "wire_material_must_be_nsq_language": True,
            "returned_bundle_must_be_verified_before_import": True,
            "learned_skills_are_abstract_only": True,
            "whole_core_runtime_verification_required": True,
            "placeholders_are_not_runtime_material": True,
            "incomplete_models_are_not_acceptable": True
        }
    }

    req_json = out_dir / "citadel699_request.json"
    req_nsq = out_dir / "citadel699_request.nsq"

    write_json(req_json, request)

    req_nsq.write_text(
        "CITADEL699_REQUEST {\n"
        "  authority = NSQ_COURT\n"
        "  route = citadel699_wire_receptor\n"
        "  citadel_form = nuband_instruction_set\n"
        "  citadel_is_llm = false\n"
        "  citadel_is_model = false\n"
        "  transfer_form = nsq_only\n"
        "  wire_form = nsq_language_wire\n"
        "  source_side_raw_material_allowed = true\n"
        "  post_mode = cathedral_post_near_source_receive_and_translate\n"
        "  target_size_class = mb_scale\n"
        "  tiny_seed_reconstruction_required = true\n"
        "  nurabit_citadel_groups = 21\n"
        "  nurabit_group_width_nsq_bit_units = 33\n"
        "  raw_weight_download_allowed = false\n"
        "  max_return_bundle_bytes = 1073741824\n"
        "  bits_are_valued = true\n"
        "  bits_are_recycled = true\n"
        "  content_addressed_fragments = true\n"
        "  raw_gguf_forbidden_on_wire = true\n"
        "  raw_safetensors_forbidden_on_wire = true\n"
        "  lfs_pointer_forbidden = true\n"
        "  null_spans_forbidden = true\n"
        "  learned_skills_breakdown_required = true\n"
        "  system_benefit_map_required = true\n"
        "  raw_source_code_retention_allowed = false\n"
        "  credentials_retention_allowed = false\n"
        "  private_identifier_retention_allowed = false\n"
        "  if_exact_target_not_live = review_best_fit_for_same_intent\n"
        "  placeholders_are_not_runtime_material = true\n"
        "  incomplete_models_are_not_acceptable = true\n"
        "}\n",
        encoding="utf-8"
    )

    h, hs = b3(req_json)
    receipt = {
        "schema": "Braxon.nsq.citadel699.request_receipt.v2",
        "generated_at": iso(),
        "request_json": str(req_json),
        "request_nsq": str(req_nsq),
        "request_json_blake3": h,
        "request_json_blake3_status": hs,
        "models": targets,
        "policy_ok": not bad,
        "ok": hs == "ok" and not bad and len(targets) == 6
    }
    write_json(out_dir / "citadel699_request_receipt.json", receipt)
    print(json.dumps(receipt, indent=2, sort_keys=True))
    raise SystemExit(0 if receipt["ok"] else 1)

def template_manifest(args: argparse.Namespace) -> None:
    targets = stack_models(args.model)
    systems = system_targets()

    manifest = {
        "schema": "Braxon.nsq.citadel699.bundle.v1",
        "generated_at": iso(),
        "authority": "NSQ_COURT",
        "transfer_form": "nsq_only",
        "models": targets,
        "bundle_contents": [],
        "learned_skills_breakdown": {
            "summary": "",
            "structural_patterns": [],
            "translation_skills": [],
            "compression_skills": [],
            "routing_skills": [],
            "compatibility_skills": [],
            "rebuild_skills": [],
            "verification_skills": [],
            "reuse_candidates": [],
            "confidence": {},
            "limits": []
        },
        "system_benefit_map": {
            system: {
                "benefit": "",
                "usable_patterns": [],
                "integration_notes": [],
                "risk_limits": []
            }
            for system in systems
        },
        "prohibited_retention_report": {
            "raw_source_code_retained": False,
            "raw_payload_bytes_retained": False,
            "credentials_or_tokens_retained": False,
            "private_identifiers_retained": False,
            "third_party_secrets_retained": False,
            "external_user_data_retained": False
        },
        "moral_filter_report": {
            "agency_privacy_goal_alignment": True,
            "bad_habits_rejected": [],
            "harmful_patterns_rejected": [],
            "unsafe_retention_rejected": [],
            "notes": ""
        },
        "truth_boundary": {
            "bundle_is_not_whole_core_runtime": True,
            "bundle_requires_local_rebuild_and_verify": True,
            "learned_skills_are_abstract_only": True
        }
    }

    if args.out:
        write_json(Path(args.out).expanduser(), manifest)
    print(json.dumps(manifest, indent=2, sort_keys=True))

def tar_manifest(path: Path) -> tuple[dict[str, Any] | None, list[dict[str, Any]], list[str]]:
    members_out = []
    errors = []
    manifest = None

    try:
        with tarfile.open(path, "r:*") as tf:
            for m in tf.getmembers():
                name = m.name
                suffix = Path(name).suffix.lower()

                if name.startswith("/") or ".." in Path(name).parts:
                    errors.append(f"path_traversal:{name}")

                if suffix in FORBIDDEN_WIRE_SUFFIXES:
                    errors.append(f"raw_payload_forbidden:{name}")

                members_out.append({
                    "name": name,
                    "size": m.size,
                    "type": "file" if m.isfile() else "other",
                    "suffix": suffix
                })

                if m.isfile() and Path(name).name == "manifest.json":
                    f = tf.extractfile(m)
                    if f:
                        manifest = json.loads(f.read().decode("utf-8", errors="replace"))
    except Exception as err:
        errors.append(f"tar_read_failed:{type(err).__name__}:{err}")

    return manifest, members_out, errors

def validate_learning_manifest(manifest: Any) -> dict[str, Any]:
    errors: list[str] = []
    targets = system_targets()

    if not isinstance(manifest, dict):
        return {"ok": False, "errors": ["manifest_not_object"]}

    skills = manifest.get("learned_skills_breakdown")
    if not isinstance(skills, dict):
        errors.append("missing_learned_skills_breakdown")
    else:
        for field in REQUIRED_SKILL_FIELDS:
            if field not in skills:
                errors.append(f"missing_learned_skill_field:{field}")

    benefit = manifest.get("system_benefit_map")
    if not isinstance(benefit, dict):
        errors.append("missing_system_benefit_map")
    else:
        for target in targets:
            if target not in benefit:
                errors.append(f"missing_system_benefit_target:{target}")

    retention = manifest.get("prohibited_retention_report")
    if not isinstance(retention, dict):
        errors.append("missing_prohibited_retention_report")
    else:
        for flag in REQUIRED_PROHIBITED_FLAGS:
            if retention.get(flag) is not False:
                errors.append(f"retention_flag_not_false:{flag}")

    moral = manifest.get("moral_filter_report")
    if not isinstance(moral, dict):
        errors.append("missing_moral_filter_report")
    elif moral.get("agency_privacy_goal_alignment") is not True:
        errors.append("moral_filter_alignment_not_true")

    return {
        "ok": not errors,
        "errors": errors,
        "required_system_targets": targets,
        "required_skill_fields": REQUIRED_SKILL_FIELDS,
        "required_prohibited_flags": REQUIRED_PROHIBITED_FLAGS
    }

def verify_bundle(path: Path) -> dict[str, Any]:
    path = path.expanduser().resolve()
    if not path.exists():
        return {"ok": False, "error": "bundle_missing", "path": str(path)}

    size = path.stat().st_size
    head = path.read_bytes()[:4096] if size else b""
    is_pointer = head.startswith(POINTER_PREFIX)
    tb = triple_bite(path)
    digest, digest_status = b3(path)

    manifest, members, tar_errors = tar_manifest(path) if tarfile.is_tarfile(path) else (None, [], ["not_tar_bundle"])

    manifest_ok = isinstance(manifest, dict) and str(manifest.get("schema", "")).startswith("Braxon.nsq.citadel699.bundle")
    learning_check = validate_learning_manifest(manifest)
    bundle_form_ok = not any(Path(m["name"]).suffix.lower() in FORBIDDEN_WIRE_SUFFIXES for m in members)

    ok = bool(
        size > 0
        and size <= MAX_BUNDLE_BYTES
        and not is_pointer
        and tb.get("ok")
        and digest_status == "ok"
        and manifest_ok
        and learning_check.get("ok")
        and bundle_form_ok
        and not tar_errors
    )

    return {
        "schema": "Braxon.nsq.citadel699.bundle_verify.v2",
        "generated_at": iso(),
        "authority": "NSQ_COURT",
        "path": str(path),
        "size_bytes": size,
        "max_return_bundle_bytes": MAX_BUNDLE_BYTES,
        "size_ok": size <= MAX_BUNDLE_BYTES,
        "is_lfs_pointer": is_pointer,
        "triple_bite": tb,
        "blake3": digest,
        "blake3_status": digest_status,
        "tar_errors": tar_errors,
        "manifest_ok": manifest_ok,
        "learning_manifest_check": learning_check,
        "manifest": manifest,
        "members": members,
        "raw_payload_forbidden_on_wire": True,
        "bundle_form_ok": bundle_form_ok,
        "ok": ok
    }

def cmd_verify(args: argparse.Namespace) -> None:
    result = verify_bundle(Path(args.bundle))
    if args.json_out:
        write_json(Path(args.json_out).expanduser(), result)
    print(json.dumps(result, indent=2, sort_keys=True))
    raise SystemExit(0 if result.get("ok") else 1)

def cmd_import(args: argparse.Namespace) -> None:
    src = Path(args.bundle).expanduser().resolve()
    result = verify_bundle(src)
    if not result.get("ok"):
        print(json.dumps(result, indent=2, sort_keys=True))
        raise SystemExit(1)

    IMPORT_ROOT.mkdir(parents=True, exist_ok=True)
    dest = IMPORT_ROOT / src.name
    shutil.copy2(src, dest)

    receipt = {
        "schema": "Braxon.nsq.citadel699.import_receipt.v2",
        "generated_at": iso(),
        "authority": "NSQ_COURT",
        "source_bundle": str(src),
        "imported_bundle": str(dest.relative_to(ROOT)),
        "bundle_blake3": result.get("blake3"),
        "size_bytes": result.get("size_bytes"),
        "learned_skills_breakdown_present": True,
        "system_benefit_map_present": True,
        "truth_boundary": {
            "imported_bundle_is_nsq_rebuild_material": True,
            "learned_skills_are_abstract_only": True,
            "whole_core_runtime_verification_required": True,
            "raw_weight_download_allowed": False,
            "placeholders_are_not_runtime_material": True,
            "incomplete_models_are_not_acceptable": True
        },
        "ok": True
    }

    write_json(STATE / f"import_{slug(src.name)}.json", receipt)
    print(json.dumps(receipt, indent=2, sort_keys=True))

def cmd_status(_: argparse.Namespace) -> None:
    cfg = config()
    out = {
        "schema": "Braxon.nsq.citadel699.status.v2",
        "generated_at": iso(),
        "authority": "NSQ_COURT",
        "config_present": CONFIG.exists(),
        "registry_present": REGISTRY.exists(),
        "import_root": str(IMPORT_ROOT.relative_to(ROOT)),
        "max_return_bundle_bytes": MAX_BUNDLE_BYTES,
        "raw_payload_transfer_allowed": False,
        "bits_are_valued": True,
        "bits_are_recycled": True,
        "learned_skills_breakdown_required": True,
        "system_benefit_map_required": True,
        "system_awareness_targets": system_targets(),
        "config": cfg
    }
    print(json.dumps(out, indent=2, sort_keys=True))

def main() -> None:
    ap = argparse.ArgumentParser(prog="nsq-citadel699")
    sub = ap.add_subparsers(dest="cmd", required=True)

    sp = sub.add_parser("make-request")
    sp.add_argument("--model", action="append", default=[])
    sp.add_argument("--out-dir", default=str(DL / "citadel699_request"))
    sp.set_defaults(func=make_request)

    sp = sub.add_parser("template-manifest")
    sp.add_argument("--model", action="append", default=[])
    sp.add_argument("--out", default="")
    sp.set_defaults(func=template_manifest)

    sp = sub.add_parser("verify-bundle")
    sp.add_argument("bundle")
    sp.add_argument("--json-out", default="")
    sp.set_defaults(func=cmd_verify)

    sp = sub.add_parser("import-bundle")
    sp.add_argument("bundle")
    sp.set_defaults(func=cmd_import)

    sub.add_parser("status").set_defaults(func=cmd_status)

    args = ap.parse_args()
    args.func(args)

if __name__ == "__main__":
    main()
