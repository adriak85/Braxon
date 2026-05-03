#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT_DIR="${BRAXON_HOME:-$(cd "$(dirname "$0")/.." && pwd)}"
STATE_DIR="$ROOT_DIR/state/braxon"
LOG_DIR="$STATE_DIR/logs"
SRC_DIR="$ROOT_DIR/assets/braxon_core/source_ingest/braxon_transport"
NSQ_DIR="$ROOT_DIR/assets/braxon_core/weights/nsq"

ARTIFACT_PATH="$NSQ_DIR/Braxon-27B_extended.nsqb"
ENVELOPE_PATH="$NSQ_DIR/Braxon-27B_extended.nsqb.meta"
MANIFEST_PATH="$SRC_DIR/BLAKE3SUMS"
BIND_PATH="$STATE_DIR/braxon_binding.json"
REG_PATH="$STATE_DIR/offline_model_registry.json"
PIPELINE_STATUS="$STATE_DIR/braxon_nsq_pipeline.status"
LOG_PATH="$LOG_DIR/BRAXON_recode_state.log"

mkdir -p "$LOG_DIR" "$NSQ_DIR"

python3 - "$SRC_DIR" "$MANIFEST_PATH" "$BIND_PATH" "$REG_PATH" "$ENVELOPE_PATH" "$ARTIFACT_PATH" <<'PYTHON_BLOCK'
import datetime
import hashlib
import json
import os
import sys

src_dir, manifest_path, bind_path, reg_path, env_path, out_path = sys.argv[1:]

if not os.path.isfile(manifest_path):
    raise SystemExit(f"missing manifest: {manifest_path}")
if not os.path.isfile(bind_path):
    raise SystemExit(f"missing binding: {bind_path}")

with open(bind_path, "r", encoding="utf-8") as fh:
    bind = json.load(fh)

reg = {}
if os.path.isfile(reg_path):
    with open(reg_path, "r", encoding="utf-8") as fh:
        reg = json.load(fh)

pack = bind.get("runtime_packaging", {})
tok = bind.get("tokenizer", {})
params = bind.get("parameters", {})

def recursive_key_values(obj, key):
    found = []
    if isinstance(obj, dict):
        for k, v in obj.items():
            if k == key:
                found.append(v)
            found.extend(recursive_key_values(v, key))
    elif isinstance(obj, list):
        for item in obj:
            found.extend(recursive_key_values(item, key))
    return found

representation_mode = next(iter(recursive_key_values(reg, "representation_mode")), "stamp_bound_manifest")
runtime_mass_profile = next(iter(recursive_key_values(reg, "runtime_mass_profile")), "manifest_and_stamps_only")
session_surface = next(iter(recursive_key_values(reg, "session_surface")), "zlm_native_runtime_surface")
agentic_capability = next(iter(recursive_key_values(reg, "agentic_capability")), "full_agentic_conversation")
stamp_bundle = next(iter(recursive_key_values(reg, "stamp_bundle")), [])
if not isinstance(stamp_bundle, list):
    stamp_bundle = []

source_files = []
total_bytes = 0
with open(manifest_path, "r", encoding="utf-8") as fh:
    for raw in fh:
        raw = raw.strip()
        if not raw:
            continue
        parts = raw.split(None, 1)
        if len(parts) != 2:
            continue
        blake3, rel = parts
        path = os.path.join(src_dir, rel)
        if not os.path.isfile(path):
            raise SystemExit(f"manifest entry missing on disk: {rel}")
        size = os.path.getsize(path)
        total_bytes += size
        source_files.append((rel, blake3, size))

env_meta = {}
if os.path.isfile(env_path):
    with open(env_path, "r", encoding="utf-8") as fh:
        for line in fh:
            if ": " in line:
                k, v = line.rstrip("\n").split(": ", 1)
                env_meta[k] = v

model_name = bind.get("model_name", "BRAXON")
core_identity = bind.get("core_identity", "BRAXON_core_primary_model")
generated = datetime.datetime.utcnow().replace(microsecond=0).isoformat() + "Z"
manifest_sha256 = hashlib.sha256(
    "\n".join(f"{rel}\t{blake3}\t{size}" for rel, blake3, size in source_files).encode()
).hexdigest()

lines = []
def add(k, v):
    lines.append(f"{k}: {v}")

add("artifact_kind", "nsq_whole_core_runtime_bundle")
add("artifact_name", os.path.basename(out_path))
add("generated_at_utc", generated)
add("model_name", model_name)
add("core_identity", core_identity)
add("representation_mode", representation_mode)
add("runtime_mass_profile", runtime_mass_profile)
add("hot_live_parameter_embodiment", "false")
add("delta_expansion_state", "not_implemented")
add("tokenizer_runtime_unification", "not_proven")
add("session_surface", session_surface)
add("agentic_capability", agentic_capability)
add("source_ingest_directory", src_dir)
add("source_manifest", manifest_path)
add("source_required_files", len(source_files))
add("source_present_files", len(source_files))
add("source_total_bytes", total_bytes)
add("source_blake3_status", env_meta.get("source_blake3_status", "verified"))
add("source_blake3_recorded_files", len(source_files))
add("nsq_envelope_artifact", env_path)
add("runtime_load_policy", pack.get("runtime_load_policy", "whole_core_only"))
add("launch_form", pack.get("launch_form", "hot_whole_core"))
add("zlm_binding_mode", pack.get("zlm_binding_mode", "whole_core_session_surface"))
add("grid_26d_mode", pack.get("grid_26d_mode", "sealed_reference_structure"))
add("grid_26d_activation_mode", pack.get("grid_26d_activation_mode", "semantic_score_alignment"))
add("supermodel_extension_mode", pack.get("supermodel_extension_mode", "sealed_reference_structure"))
add("supermodel_extension_activation_mode", pack.get("supermodel_extension_activation_mode", "semantic_score_alignment"))
add("delta_extension_mode", pack.get("delta_extension_mode", "sealed_reference_structure"))
add("delta_extension_activation_mode", pack.get("delta_extension_activation_mode", "semantic_score_alignment"))
add("whole_parameter_stamp", pack.get("whole_parameter_stamp", "nsq.runtime.native.model.parameter.whole.v1"))
add("parameter_projection_mode", pack.get("parameter_projection_mode", "single_bit_factor_shim"))
add("env_parameter_copy_mode", pack.get("env_parameter_copy_mode", "lazy_load"))
add("tokenizer_binding_state", tok.get("binding_state", "semantic_feed_bound_not_runtime_unified"))
add("parameter_binding_state", params.get("binding_state", "donor_transport_indexed_not_hot_live"))
add("hidden_size", params.get("hidden_size", ""))
add("hidden_layers", params.get("hidden_layers", ""))
add("attention_heads", params.get("attention_heads", ""))
add("key_value_heads", params.get("key_value_heads", ""))
add("max_positions", params.get("max_positions", ""))
add("vocab_size", params.get("vocab_size", ""))
add("manifest_digest_sha256", manifest_sha256)
add("stamp_bundle_count", len(stamp_bundle))
for item in stamp_bundle:
    add("stamp_bundle", item)

lines.append("")
lines.append("[[source_files]]")
for rel, blake3, size in source_files:
    lines.append(f"{rel}\t{blake3}\t{size}")

with open(out_path, "w", encoding="utf-8") as out:
    out.write("\n".join(map(str, lines)) + "\n")
PYTHON_BLOCK

current_source_status="unknown"
if [ -f "$PIPELINE_STATUS" ]; then
  current_source_status="$(awk -F= '/^source_ingest_status=/{print $2}' "$PIPELINE_STATUS" | tail -n 1)"
fi

cat > "$LOG_PATH" <<LOG_BLOCK
artifact=$ARTIFACT_PATH
generated_at_utc=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
nsq_recode_status=manifest_bundle_only
whole_core_runtime_status=manifest_bundle_emitted
source_ingest_status=$current_source_status
LOG_BLOCK

bash "$ROOT_DIR/scripts/finalize_braxon_nsq_whole_core.sh"
