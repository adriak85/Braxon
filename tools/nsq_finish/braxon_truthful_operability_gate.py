#!/usr/bin/env python3
import json
import hashlib
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(".").resolve()

def read_json(path):
    p = ROOT / path
    if not p.exists():
        return None
    try:
        return json.loads(p.read_text())
    except Exception as e:
        return {"__parse_error__": str(e)}

def sha256_file(path):
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()

def exists_nonempty(path):
    p = ROOT / path
    return p.exists() and p.is_file() and p.stat().st_size > 0

def file_record(path):
    p = ROOT / path
    return {
        "path": path,
        "present": p.exists(),
        "bytes": p.stat().st_size if p.exists() and p.is_file() else 0,
        "sha256": sha256_file(p) if p.exists() and p.is_file() and p.stat().st_size > 0 and p.stat().st_size < 128 * 1024 * 1024 else None,
    }

now = datetime.now(timezone.utc).isoformat()

transport = ROOT / "assets/braxon_core/source_ingest/braxon_transport"
weight_index = transport / "model.safetensors.index.json"
index = read_json("assets/braxon_core/source_ingest/braxon_transport/model.safetensors.index.json")

expected_shards = []
if isinstance(index, dict):
    wm = index.get("weight_map") or {}
    expected_shards = sorted(set(wm.values()))

if not expected_shards:
    expected_shards = [f"model-{i:05d}-of-00014.safetensors" for i in range(1, 15)]

shard_records = []
for name in expected_shards:
    p = transport / name
    shard_records.append({
        "path": str(p.relative_to(ROOT)),
        "present": p.exists(),
        "bytes": p.stat().st_size if p.exists() else 0,
    })

required_assets = [
    "assets/braxon_core/source_ingest/braxon_transport/config.json",
    "assets/braxon_core/source_ingest/braxon_transport/generation_config.json",
    "assets/braxon_core/source_ingest/braxon_transport/tokenizer.json",
    "assets/braxon_core/source_ingest/braxon_transport/tokenizer_config.json",
    "assets/braxon_core/source_ingest/braxon_transport/merges.txt",
    "assets/braxon_core/source_ingest/braxon_transport/vocab.json",
    "assets/braxon_core/source_ingest/braxon_transport/model.safetensors.index.json",
    "assets/braxon_core/model_config/config.json",
    "assets/braxon_core/model_config/generation_config.json",
    "assets/braxon_core/tokenizer/braxon_unified_tokenizer.json",
    "assets/braxon_core/tokenizer/braxon_supermodel_tokenizer.json",
    "assets/braxon_core/weights/nsq/Braxon-27B_extended.nsqb",
    "assets/braxon_core/weights/nsq/Braxon-27B_extended.nsqb.meta",
    "state/braxon/offline_model_registry.json",
    "state/braxon/braxon_binding.json",
    "models/braxon/manifest.json",
]

asset_records = [file_record(x) for x in required_assets]
required_assets_present = all(r["present"] and r["bytes"] > 0 for r in asset_records)
all_shards_present = bool(shard_records) and all(r["present"] and r["bytes"] > 0 for r in shard_records)

registry = read_json("state/braxon/offline_model_registry.json")
binding = read_json("state/braxon/braxon_binding.json")
manifest = read_json("models/braxon/manifest.json")
runtime_targets = read_json("config/nsq/nsq_model_install_targets.json")

def json_contains_text(obj, needle):
    return needle.lower() in json.dumps(obj, sort_keys=True, ensure_ascii=False).lower() if obj is not None else False

stamp_bound = json_contains_text(registry, "stamp_bound_manifest_registered_core")
binding_file_present = binding is not None and "__parse_error__" not in binding
manifest_file_present = manifest is not None and "__parse_error__" not in manifest
runtime_route_available = json_contains_text(runtime_targets, "runtime_route_available")

assets_ready = required_assets_present and all_shards_present and stamp_bound and binding_file_present and manifest_file_present

# Can attempt launch means the asset/binding prerequisites are present.
# It does NOT mean hot-live inference is proven.
can_attempt_launch = assets_ready

# Do not mark these true until a live command proves them.
runtime_route_proven = False
loaded_binding_proven = False
runtime_hot_live_proven = False
final_active_digest_present = False

missing = []
if not required_assets_present: missing.append("required_asset_files")
if not all_shards_present: missing.append("safetensors_shards")
if not stamp_bound: missing.append("stamp_bound_registry_status")
if not binding_file_present: missing.append("BRAXON_binding_json")
if not manifest_file_present: missing.append("models_BRAXON_manifest_json")
if not runtime_route_available: missing.append("runtime_route_available_config")
missing += [
    "runtime_route_proof",
    "loaded_model_binding_proof",
    "runtime_hot_live_proof",
    "final_active_digest",
]

launch = {
    "schema": "Braxon.model_launch_readiness.v1",
    "generated_at": now,
    "assets_ready": assets_ready,
    "can_attempt_launch": can_attempt_launch,
    "runtime_route_available": runtime_route_available,
    "runtime_route_proven": runtime_route_proven,
    "loaded_binding_proven": loaded_binding_proven,
    "runtime_hot_live_proven": runtime_hot_live_proven,
    "final_active_digest_present": final_active_digest_present,
    "status": "can_attempt_launch_runtime_proof_pending" if can_attempt_launch else "not_ready_assets_missing",
    "missing_for_full_launch": missing,
    "evidence": {
        "required_assets": asset_records,
        "safetensors_shards": shard_records,
        "expected_shard_count": len(expected_shards),
        "present_shard_count": sum(1 for r in shard_records if r["present"] and r["bytes"] > 0),
        "offline_registry_status_contains_stamp_bound_manifest_registered_core": stamp_bound,
        "binding_file_present": binding_file_present,
        "manifest_file_present": manifest_file_present,
        "runtime_route_available_config": runtime_route_available,
    },
    "rule": "This gate may mark assets_ready and can_attempt_launch from local files, but must not mark runtime_route_proven, loaded_binding_proven, runtime_hot_live_proven, or final_active_digest_present without live proof.",
}

bus = {
    "schema": "Braxon.llm_bus_parameter_pressure_gate.v1",
    "generated_at": now,
    "llm_bus_launch_ready": False,
    "status": "bus_assets_present_runtime_proof_pending" if can_attempt_launch else "not_bus_launch_ready",
    "component_status": {
        "assets_ready": assets_ready,
        "can_attempt_launch": can_attempt_launch,
        "runtime_route_available": runtime_route_available,
        "runtime_route_proven": runtime_route_proven,
        "loaded_binding_proven": loaded_binding_proven,
        "runtime_hot_live_proven": runtime_hot_live_proven,
        "final_active_digest_present": final_active_digest_present,
        "stamp_bound_registry_status": stamp_bound,
    },
    "missing_for_launch": [
        "runtime_route_proven",
        "loaded_binding_proven",
        "runtime_hot_live_proven",
        "final_active_digest_present",
    ],
    "errors": [] if can_attempt_launch else ["asset prerequisites are not fully ready"],
    "warnings": [
        "runtime route proof is not present",
        "loaded model binding proof is not present",
        "runtime hot-live proof is not present",
        "final active digest / launch-route execution proof is not found",
    ],
}

nsq = {
    "schema": "nsq.core_release_gate.v1",
    "generated_at": now,
    "nsq_core_complete_ready": False,
    "complete_ready": False,
    "status": "core_assets_present_release_proof_pending",
    "evidence": {
        "local_finish_manifest_present": exists_nonempty("state/nsq/nsq_local_finish_manifest.json"),
        "runtime_languages_models_manifest_present": exists_nonempty("state/nsq/nsq_runtime_languages_models_manifest.json"),
        "asm_stamp_manifest_present": exists_nonempty("state/nsq/nsq_asm_stamp_full_language_manifest.json"),
        "stamp_registry_present": exists_nonempty("state/nsq/stamps/stamp_registry.jsonl"),
        "model_reconstruction_manifest_present": exists_nonempty("state/nsq/model_reconstruction_manifest.json"),
    },
    "missing_for_release": [
        "formal nsq core release proof",
        "runtime execution proof",
        "whole release gate proof",
    ],
}

Path("state/braxon").mkdir(parents=True, exist_ok=True)
Path("state/nsq").mkdir(parents=True, exist_ok=True)
Path("docs/Braxon").mkdir(parents=True, exist_ok=True)
Path("specs/Braxon").mkdir(parents=True, exist_ok=True)

Path("state/braxon/braxon_model_launch_readiness.json").write_text(json.dumps(launch, indent=2, sort_keys=True) + "\n")
Path("state/braxon/braxon_llm_bus_parameter_pressure_gate.json").write_text(json.dumps(bus, indent=2, sort_keys=True) + "\n")
Path("state/nsq/nsq_core_release_gate.json").write_text(json.dumps(nsq, indent=2, sort_keys=True) + "\n")

Path("docs/Braxon/BRAXON_TRUTHFUL_OPERABILITY_GATE.md").write_text(f"""# Braxon Truthful Operability Gate

Generated: `{now}`

## Result

- `assets_ready`: `{str(assets_ready).lower()}`
- `can_attempt_launch`: `{str(can_attempt_launch).lower()}`
- `runtime_route_available`: `{str(runtime_route_available).lower()}`
- `runtime_route_proven`: `false`
- `loaded_binding_proven`: `false`
- `runtime_hot_live_proven`: `false`
- `final_active_digest_present`: `false`

## Rule

This gate proves only what the phone actually has.

It may mark local assets ready when the full safetensors shard set, tokenizer/config files, NSQ weight artifact, registry, binding file, and manifest are present.

It must not mark runtime hot-live, loaded binding, runtime route proof, or final digest as true until those are proven by a live route execution.
""")

Path("specs/Braxon/BRAXON_TRUTHFUL_OPERABILITY_GATE_CONTRACT.md").write_text("""# Braxon Truthful Operability Gate Contract

The launch gate is evidence based.

Allowed from static local files:

- asset presence
- shard completeness
- tokenizer/config presence
- NSQ weight artifact presence
- registry/binding/manifest presence
- can-attempt-launch when asset prerequisites pass

Not allowed from static local files:

- runtime hot-live proof
- loaded model binding proof
- final active digest
- completed launch-route execution proof
""")

print(json.dumps({
    "ok": True,
    "assets_ready": assets_ready,
    "can_attempt_launch": can_attempt_launch,
    "runtime_route_available": runtime_route_available,
    "runtime_route_proven": runtime_route_proven,
    "loaded_binding_proven": loaded_binding_proven,
    "runtime_hot_live_proven": runtime_hot_live_proven,
    "final_active_digest_present": final_active_digest_present,
    "present_shard_count": sum(1 for r in shard_records if r["present"] and r["bytes"] > 0),
    "expected_shard_count": len(expected_shards),
    "status": launch["status"],
    "next": [
        "generate runtime route proof",
        "generate loaded model binding proof",
        "run hot-live proof",
        "write final active digest",
    ],
}, indent=2, sort_keys=True))
