#!/data/data/com.termux/files/usr/bin/bash
set -u

STAMP="$(date +%Y%m%d_%H%M%S)"
ROOT="${1:-$PWD}"

cd "$ROOT" || exit 1

REQUEST="state/nsq/citadel699/current/request_capsule.json"
TARGET="state/nsq/citadel699/current/target_models.json"
SURFACE="apps/nsq/BRAXON_six_model_stack.nsq"
OUTDIR="state/nsq/citadel699/rebuilds/$STAMP"
PROOF="state/nsq/proofs/citadel699_current_rebuild.json"

mkdir -p "$OUTDIR" state/nsq/proofs

for f in "$REQUEST" "$TARGET" "$SURFACE" config/nsq/BRAXON_six_model_stack.json; do
  if [ ! -s "$f" ]; then
    echo "ERROR: required Citadel699 input missing: $f"
    exit 2
  fi
done

if grep -RInE 'raw_fetch_allowed[[:space:]]*[:=][[:space:]]*true|raw_payload_transfer_allowed[[:space:]]*[:=][[:space:]]*true|pointer_setup_allowed[[:space:]]*[:=][[:space:]]*true|donor_transport_pointer_stub_allowed[[:space:]]*[:=][[:space:]]*true' \
  "$REQUEST" "$TARGET" "$SURFACE" config/nsq/BRAXON_six_model_stack.json
then
  echo "ERROR: forbidden raw/pointer allowance in live Citadel699 input"
  exit 3
fi

for model in deepseek-v3-671b qwen3-235b-a22b qwen2.5-72b deepseek-v3-671b-analyzer llama3.3-70b gemma3-27b; do
  grep -q "$model" "$TARGET" || {
    echo "ERROR: missing model in target manifest: $model"
    exit 4
  }
done

cat > "$OUTDIR/council_six.rebuild.nsq" <<NSQ
CITADEL699_REQUEST_RETURN_REBUILD {
  authority = NSQ_COURT
  stamp = $STAMP
  transfer_method = citadel699_nsq_request_return_rebuild
  transfer_form = nsq_only
  raw_fetch_allowed = false
  raw_payload_transfer_allowed = false
  pointer_setup_allowed = false
  donor_transport_pointer_stub_allowed = false
  separated_raw_shards_allowed = false
  target_size_class = mb_scale
  reconstruction_seed = tiny_nsq_seed
  nurabit_citadel_groups = 21
  nurabit_group_width_nsq_bit_units = 33
  nurabit_groups_communicate = true

  target_models {
    deepseek_v3_671b = deepseek-v3-671b
    qwen3_235b_a22b = qwen3-235b-a22b
    qwen2_5_72b = qwen2.5-72b
    deepseek_v3_671b_analyzer = deepseek-v3-671b-analyzer
    llama3_3_70b = llama3.3-70b
    gemma3_27b = gemma3-27b
  }

  status = rebuild_manifest_materialized
  runtime_claim = verification_required_for_active_unified_load
}
NSQ

cat > "$OUTDIR/council_six.materialization.json" <<JSON
{
  "schema": "Braxon.nsq.citadel699.rebuild_materialization.v1",
  "authority": "NSQ_COURT",
  "stamp": "$STAMP",
  "status": "rebuild_manifest_materialized",
  "transfer_method": "citadel699_nsq_request_return_rebuild",
  "transfer_form": "nsq_only",
  "raw_fetch_allowed": false,
  "raw_payload_transfer_allowed": false,
  "pointer_setup_allowed": false,
  "donor_transport_pointer_stub_allowed": false,
  "separated_raw_shards_allowed": false,
  "target_size_class": "mb_scale",
  "reconstruction_seed": "tiny_nsq_seed",
  "nurabit_citadel_groups": 21,
  "nurabit_group_width_nsq_bit_units": 33,
  "nurabit_groups_communicate": true,
  "required_model_count": 6,
  "models": [
    "deepseek-v3-671b",
    "qwen3-235b-a22b",
    "qwen2.5-72b",
    "deepseek-v3-671b-analyzer",
    "llama3.3-70b",
    "gemma3-27b"
  ],
  "request_capsule": "$REQUEST",
  "target_models": "$TARGET",
  "nsq_surface": "$SURFACE",
  "nsq_rebuild_surface": "$OUTDIR/council_six.rebuild.nsq",
  "whole_core_runtime_verification_required": true
}
JSON

BYTES="$(wc -c < "$OUTDIR/council_six.rebuild.nsq" | tr -d ' ')"
SHA="$(sha256sum "$OUTDIR/council_six.rebuild.nsq" | awk '{print $1}')"

cat > "$PROOF" <<JSON
{
  "schema": "Braxon.nsq.citadel699.rebuild_proof.v1",
  "authority": "NSQ_COURT",
  "stamp": "$STAMP",
  "status": "citadel699_rebuild_manifest_ready",
  "transfer_method": "citadel699_nsq_request_return_rebuild",
  "transfer_form": "nsq_only",
  "raw_fetch_allowed": false,
  "raw_payload_transfer_allowed": false,
  "pointer_setup_allowed": false,
  "separated_raw_shards_allowed": false,
  "target_size_class": "mb_scale",
  "reconstruction_seed": "tiny_nsq_seed",
  "nurabit_citadel_groups": 21,
  "nurabit_group_width_nsq_bit_units": 33,
  "nurabit_groups_communicate": true,
  "required_model_count": 6,
  "rebuild_dir": "$OUTDIR",
  "rebuild_surface": "$OUTDIR/council_six.rebuild.nsq",
  "rebuild_bytes": $BYTES,
  "rebuild_sha256": "$SHA",
  "runtime_claim_verification_required": true
}
JSON

ln -sf "../rebuilds/$STAMP/council_six.materialization.json" state/nsq/citadel699/current/materialization.json
ln -sf "../rebuilds/$STAMP/council_six.rebuild.nsq" state/nsq/citadel699/current/council_six.rebuild.nsq

echo "citadel699_rebuild_status=0"
echo "rebuild_dir=$OUTDIR"
echo "rebuild_surface=$OUTDIR/council_six.rebuild.nsq"
echo "proof=$PROOF"
echo "rebuild_sha256=$SHA"
