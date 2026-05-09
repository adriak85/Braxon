#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$HOME/Braxon"
STAMP="$(date +%Y%m%d_%H%M%S)"
REPORT_DIR="$ROOT/state/reports"
REPORT="$REPORT_DIR/fastest_braxon_actionable_demo_$STAMP.txt"
DOWNLOAD_COPY="$HOME/storage/shared/Download/fastest_braxon_actionable_demo_$STAMP.txt"

mkdir -p "$REPORT_DIR" \
  "$ROOT/state/nsq/citadel699/current" \
  "$ROOT/state/nsq/proofs" \
  "$ROOT/state/braxon/bus/citadel699" \
  "$ROOT/config/nsq" \
  "$ROOT/apps/nsq" \
  "$ROOT/scripts"

cd "$ROOT"

log() { printf '%s\n' "$*" | tee -a "$REPORT"; }
run() {
  log ""
  log ">>> $*"
  "$@" 2>&1 | tee -a "$REPORT"
}

: > "$REPORT"

log "=== FASTEST BRAXON ACTIONABLE DEMO ==="
log "root=$ROOT"
log "stamp=$STAMP"
log "goal=offline_actionable_braxon_before_terminal_polish"
log "j_pref=7"

log ""
log "=== 0. preserve local state, no destructive git sync ==="
run git status --short || true
run git branch --show-current || true

log ""
log "=== 1. ensure Council Ten NSQ surface exists ==="
if [ ! -s apps/nsq/braxon_council_ten_stack.nsq ]; then
  cat > apps/nsq/braxon_council_ten_stack.nsq <<'NSQ'
BRAXON_COUNCIL_TEN_STACK {
  authority = NSQ_COURT
  architecture_root = true
  required_model_count = 10
  brain_model_count = 6
  sensory_body_count = 4
  registry_default_stack_is_authoritative = true
  video_and_voice_required = true

  transfer_method = citadel699_nsq_request_return_rebuild
  transfer_form = nsq_only
  raw_fetch_allowed = false
  raw_payload_transfer_allowed = false
  pointer_setup_allowed = false
  donor_transport_pointer_stub_allowed = false

  wiring_protocol = nsq_macro_stamping
  substrate = base8_switch_topology_nurabit_21x33
  round_table_assembly = true
  citadel_699_wire_station = true

  brain_models {
    deepseek_v3_671b = deepseek-v3-671b
    qwen3_235b_a22b = qwen3-235b-a22b
    qwen2_5_72b = qwen2.5-72b
    deepseek_v3_671b_analyzer = deepseek-v3-671b-analyzer
    llama3_3_70b = llama3.3-70b
    gemma3_27b = gemma3-27b
  }

  sensory_bodies {
    image_cortex = FLUX.1-dev
    video_cortex = Wan2.1-T2V-14B
    voice_body = IndexTTS2
    world_body_3d = Hunyuan3D-2.1
  }

  truth_boundary {
    request_capsule_is_not_raw_download = true
    whole_core_runtime_verification_required = true
    raw_fetch_allowed = false
    raw_payload_transfer_allowed = false
    pointer_setup_allowed = false
    target_size_class = mb_scale
  }
}
NSQ
fi

log ""
log "=== 2. ensure Council Ten JSON config exists ==="
if [ ! -s config/nsq/braxon_council_ten_stack.json ]; then
  cat > config/nsq/braxon_council_ten_stack.json <<'JSON'
{
  "schema": "Braxon.nsq.council_ten_stack.v1",
  "authority": "NSQ_COURT",
  "architecture_root": true,
  "required_model_count": 10,
  "brain_model_count": 6,
  "sensory_body_count": 4,
  "default_stack": [
    "deepseek-v3-671b",
    "qwen3-235b-a22b",
    "qwen2.5-72b",
    "deepseek-v3-671b-analyzer",
    "llama3.3-70b",
    "gemma3-27b",
    "FLUX.1-dev",
    "Wan2.1-T2V-14B",
    "IndexTTS2",
    "Hunyuan3D-2.1"
  ],
  "wiring_protocol": "nsq_macro_stamping",
  "substrate": "base8_switch_topology_nurabit_21x33",
  "transfer_method": "citadel699_nsq_request_return_rebuild",
  "transfer_form": "nsq_only",
  "raw_fetch_allowed": false,
  "raw_payload_transfer_allowed": false,
  "pointer_setup_allowed": false,
  "donor_transport_pointer_stub_allowed": false,
  "truth_boundary": {
    "ten_surface_stack_is_authoritative": true,
    "brain_model_count_is_six": true,
    "sensory_body_count_is_four": true,
    "video_and_voice_are_required": true,
    "request_capsule_is_not_raw_download": true,
    "raw_weight_download_allowed": false,
    "whole_core_runtime_verification_required": true,
    "target_size_class": "mb_scale"
  },
  "target_size_class": "mb_scale",
  "tiny_seed_reconstruction_required": true,
  "nurabit_citadel_groups": 21,
  "nurabit_group_width_nsq_bit_units": 33,
  "nurabit_groups_communicate": true,
  "raw_weight_download_allowed": false,
  "open_weight_required": true,
  "censor_free_required": true,
  "huihui_preferred_source": true
}
JSON
fi

log ""
log "=== 3. create missing Citadel699 current inputs if absent ==="
if [ ! -s state/nsq/citadel699/current/request_capsule.json ]; then
  cat > state/nsq/citadel699/current/request_capsule.json <<'JSON'
{
  "schema": "Braxon.nsq.citadel699.request_capsule.v1",
  "authority": "NSQ_COURT",
  "identity": "Braxon",
  "citadel": "Citadel699",
  "required_model_count": 10,
  "brain_model_count": 6,
  "sensory_body_count": 4,
  "transfer_method": "citadel699_nsq_request_return_rebuild",
  "transfer_form": "nsq_only",
  "raw_fetch_allowed": false,
  "raw_payload_transfer_allowed": false,
  "pointer_setup_allowed": false,
  "donor_transport_pointer_stub_allowed": false,
  "separated_raw_shards_allowed": false,
  "ordinary_full_weight_download_is_source_truth": false,
  "full_local_fp32_weight_storage_required": false,
  "bus_manifestation_required": true,
  "stamp_wake_materialization_required": true,
  "whole_core_runtime_verification_required": true,
  "runtime_claim_verification_required": true,
  "target_size_class": "mb_scale"
}
JSON
fi

if [ ! -s state/nsq/citadel699/current/target_models.json ]; then
  cat > state/nsq/citadel699/current/target_models.json <<'JSON'
{
  "schema": "Braxon.nsq.citadel699.target_models.v1",
  "authority": "NSQ_COURT",
  "identity": "Braxon",
  "citadel": "Citadel699",
  "required_model_count": 10,
  "brain_model_count": 6,
  "sensory_body_count": 4,
  "raw_fetch_allowed": false,
  "raw_payload_transfer_allowed": false,
  "pointer_setup_allowed": false,
  "brain_models": {
    "deepseek_v3_671b": "deepseek-v3-671b",
    "qwen3_235b_a22b": "qwen3-235b-a22b",
    "qwen2_5_72b": "qwen2.5-72b",
    "deepseek_v3_671b_analyzer": "deepseek-v3-671b-analyzer",
    "llama3_3_70b": "llama3.3-70b",
    "gemma3_27b": "gemma3-27b"
  },
  "sensory_bodies": {
    "image_cortex": "FLUX.1-dev",
    "video_cortex": "Wan2.1-T2V-14B",
    "voice_body": "IndexTTS2",
    "world_body_3d": "Hunyuan3D-2.1"
  }
}
JSON
fi

log ""
log "=== 4. patch common REPL lever compile mismatch if present ==="
python3 - <<'PY'
from pathlib import Path
p = Path("src/main.rs")
if not p.exists():
    raise SystemExit("src/main.rs missing")
s = p.read_text()
orig = s
# Only fixes the known bad local state where repl_levers uses LeverSweetSpotReport
# but reads fields that belong to lever_spacing_sweet_spot_report.
s = s.replace(
    "fn repl_levers() {\n    let report = lever_sweet_spot_report(0.001);",
    "fn repl_levers() {\n    let report = lever_spacing_sweet_spot_report(0.001);"
)
if s != orig:
    backup = Path(f"src/main.rs.before_fastest_actionable_demo")
    if not backup.exists():
        backup.write_text(orig)
    p.write_text(s)
    print("patched src/main.rs repl_levers spacing report mismatch")
else:
    print("src/main.rs needed no repl_levers patch")
PY

log ""
log "=== 5. run Citadel699 rebuild/materialization ==="
if [ -x tools/citadel699_nsq_request_return_rebuild.sh ]; then
  run tools/citadel699_nsq_request_return_rebuild.sh "$ROOT"
else
  chmod +x tools/citadel699_nsq_request_return_rebuild.sh 2>/dev/null || true
  run bash tools/citadel699_nsq_request_return_rebuild.sh "$ROOT"
fi

log ""
log "=== 6. verify Citadel route gates ==="
[ -x scripts/verify_citadel699_reconstruction_route_gate.sh ] && run scripts/verify_citadel699_reconstruction_route_gate.sh "$ROOT" || true
[ -x scripts/verify_citadel699_emotion_route_boundary.sh ] && run scripts/verify_citadel699_emotion_route_boundary.sh "$ROOT" || true

log ""
log "=== 7. build/test the quickest proof surfaces ==="
run cargo test -p Braxon-core council_ten -- --nocapture
run cargo test -p nsq-wake -- --nocapture
run cargo test -p nsqasm-stamp-db -- --nocapture

log ""
log "=== 8. build root binary ==="
run cargo build --release

BIN="$ROOT/target/release/Braxon"
if [ ! -x "$BIN" ]; then
  BIN="$ROOT/target/release/braxon"
fi

log ""
log "=== 9. offline actionable demo commands ==="
if [ -x "$BIN" ]; then
  run "$BIN" status
  run "$BIN" wake
  run "$BIN" apps verify
  run "$BIN" runtime registry
else
  log "ERROR: release binary not found after build"
  exit 9
fi

log ""
log "=== 10. final proof files ==="
for f in \
  state/nsq/citadel699/current/request_capsule.json \
  state/nsq/citadel699/current/target_models.json \
  state/nsq/proofs/citadel699_current_rebuild.json \
  state/braxon/bus/citadel699/current.braxon \
  config/nsq/citadel699_reconstruction_route_gate.json \
  config/nsq/citadel699_uncensored_emotion_route_boundary.json
do
  if [ -e "$f" ] || [ -L "$f" ]; then
    sha256sum "$f" 2>/dev/null | tee -a "$REPORT" || true
  else
    log "MISSING: $f"
  fi
done

log ""
log "=== RESULT LABEL ==="
log "fastest_path=braxon_actionable_first"
log "terminal_environment_second=true"
log "guile_front_door_later_multiplier=true"
log "hot_live_claim=false_until_executable_route_proof"
log "offline_demo_ready_if_all_steps_above_passed=true"
log "report=$REPORT"

cp "$REPORT" "$DOWNLOAD_COPY" 2>/dev/null || true
log "download_copy=$DOWNLOAD_COPY"

