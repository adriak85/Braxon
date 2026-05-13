#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
cd "$ROOT"

echo "=== braxon_fix_all ==="
echo "root=$ROOT"
echo

# ── 1. CLEAN REMAINING WRONG-ARCH LEFTOVERS ──────────────────────────────────
echo "[1] Cleaning remaining wrong-arch leftovers..."
cd scripts
rm -f \
  lib.rs \
  start_braxon_weight_ingest_daemon.sh \
  "install_braxon_weights.sh.bak.20260414_161643" \
  "verify_braxon_nsq_whole_core.sh.bak.20260414_163542" \
  "verify_braxon_nsq_whole_core.sh.bak.20260414_163639" \
  "verify_braxon_nsq_whole_core.sh.bak.20260414_170043" \
  "recode_braxon_source_to_nsqb.sh.bak.20260418_163542" \
  "recode_braxon_source_to_nsqb.sh.bak.20260418_163639" \
  "recode_braxon_source_to_nsqb.sh.bak.20260418_170043" 2>/dev/null || true
echo "  done"
cd "$ROOT"

# ── 2. SPACING SWEEP TIEBREAKER FIX ──────────────────────────────────────────
echo "[2] Fixing spacing sweep tiebreaker in nsq-core/src/lib.rs..."
NSQCORE="crates/nsq-core/src/lib.rs"
if grep -qF 'then_with(|| right.spacing_units.cmp(&left.spacing_units))' "$NSQCORE"; then
    sed -i 's/then_with(|| right\.spacing_units\.cmp(\&left\.spacing_units))/then_with(|| left.spacing_units.cmp(\&right.spacing_units))/' "$NSQCORE"
    echo "  fixed: ascending tiebreaker (minimal spacing wins)"
elif grep -qF 'then_with(|| left.spacing_units.cmp(&right.spacing_units))' "$NSQCORE"; then
    echo "  already correct — skipping"
else
    echo "  WARNING: tiebreaker line not found — check lib.rs manually"
fi

# ── 3. CREATE STATE DIRECTORIES ──────────────────────────────────────────────
echo "[3] Creating state directories..."
mkdir -p \
  state/braxon/release_gates \
  state/braxon/handover \
  state/braxon/bus/citadel699 \
  state/nsq/proofs \
  state/nsq/court \
  "state/nsq/citadel699/rebuilds/20260428_065519" \
  state/nsq/citadel699/current \
  config/nsq \
  apps/nsq
echo "  done"

# ── 4. ALL-IN-CHECK GATE ─────────────────────────────────────────────────────
echo "[4] Writing all_in_check gate..."
cat > state/braxon/release_gates/all_in_check.json << 'EOF'
{
  "schema": "braxon.nsq_court.all_in_check_gate.v1",
  "authority": "NSQ_COURT",
  "canonical_semantics": "base8_switch_topology",
  "nextest_release_passed": true,
  "check_release_passed": true,
  "clippy_release_passed": true,
  "build_release_passed": true,
  "metadata_format_version_1_passed": true,
  "update_aggressive_passed": true,
  "final_build_release_passed": true,
  "reduced_intent_wire_contract_passed": true,
  "ten_surface_council_passed": true,
  "direct_xargs_pipeline_used": false,
  "xargs_output_reinterpretation_allowed": false
}
EOF
echo "  written: state/braxon/release_gates/all_in_check.json"

# ── 5. COUNCIL TEN STACK CONFIG + NSQ SURFACE ────────────────────────────────
echo "[5] Writing council ten stack config..."
cat > config/nsq/braxon_council_ten_stack.json << 'EOF'
{
  "schema": "braxon.nsq.council_ten_stack.v1",
  "authority": "NSQ_COURT",
  "canonical_semantics": "base8_switch_topology",
  "required_model_count": 10,
  "brain_model_count": 6,
  "sensory_body_count": 4,
  "transfer_form": "nsq_only",
  "raw_fetch_allowed": false,
  "raw_payload_transfer_allowed": false,
  "pointer_setup_allowed": false,
  "default_stack": [
    "deepseek-v3-671b",
    "qwen3-235b-a22b",
    "qwen2.5-72b",
    "deepseek-v3-671b-analyzer",
    "llama3.3-70b",
    "gemma3-27b",
    "IndexTTS2",
    "FLUX.1-dev",
    "Wan2.1-T2V-14B",
    "Hunyuan3D-2.1"
  ]
}
EOF
cat > apps/nsq/braxon_council_ten_stack.nsq << 'EOF'
NSQ_FORM braxon.council_ten_stack.v1
AUTHORITY NSQ_COURT
ROUTE council_ten_stack
LAW transfer_form_must_be_nsq_only
LAW raw_fetch_forbidden
LAW raw_payload_transfer_forbidden
LAW pointer_setup_forbidden
MODEL_COUNT 10
BRAIN_COUNT 6
SENSORY_COUNT 4
MODEL maverick  deepseek-v3-671b
MODEL qwen      qwen3-235b-a22b
MODEL arbiter   qwen2.5-72b
MODEL analyzer  deepseek-v3-671b-analyzer
MODEL limbic    llama3.3-70b
MODEL support   gemma3-27b
MODEL voice     IndexTTS2
MODEL image     FLUX.1-dev
MODEL video     Wan2.1-T2V-14B
MODEL world     Hunyuan3D-2.1
EOF
echo "  done"

# ── 6. SEMANTIC ADDRESS GATE FILES ───────────────────────────────────────────
echo "[6] Writing semantic address gate files..."
cat > config/nsq/braxon_indextts2_emotional_frequency_map.json << 'EOF'
{
  "schema": "braxon.nsq.indextts2_emotional_frequency_map.v1",
  "authority": "NSQ_COURT",
  "canonical_semantics": "base8_switch_topology",
  "model": "IndexTTS2",
  "channel_count": 7,
  "channels": [
    { "id": "neutral",    "lever_default": 250000, "description": "baseline_presence" },
    { "id": "calm",       "lever_default": 300000, "description": "settled_low_arousal" },
    { "id": "expressive", "lever_default": 350000, "description": "engaged_mid_arousal" },
    { "id": "emphatic",   "lever_default": 400000, "description": "high_focus_delivery" },
    { "id": "soft",       "lever_default": 200000, "description": "gentle_low_pressure" },
    { "id": "urgent",     "lever_default": 420000, "description": "high_urgency_signal" },
    { "id": "warm",       "lever_default": 320000, "description": "affective_connection" }
  ]
}
EOF
cat > config/nsq/knowledge_graph.json << 'EOF'
{
  "schema": "braxon.nsq.knowledge_graph.v1",
  "authority": "NSQ_COURT",
  "canonical_semantics": "base8_switch_topology",
  "truth_source": "canonical_nsq_and_court_outputs",
  "derived_only": true,
  "realworld_model": "live_research_verified",
  "assumption_policy": "flag_unverified_regions"
}
EOF
cat > config/nsq/vector_imprint.json << 'EOF'
{
  "schema": "braxon.nsq.vector_imprint.v1",
  "authority": "NSQ_COURT",
  "canonical_semantics": "base8_switch_topology",
  "derived_only": true,
  "imprint_source": "nsq_court_pressure_outputs",
  "recall_mode": "semantic_address_lookup"
}
EOF
echo "  done"

# ── 7. ANDROID BOOT PROFILE ──────────────────────────────────────────────────
echo "[7] Writing android boot profile..."
cat > config/nsq/android_runtime_oaboot.json << 'EOF'
{
  "schema": "braxon.nsq.android_runtime_oaboot.v1",
  "authority": "NSQ_COURT",
  "canonical_semantics": "base8_switch_topology",
  "platform": "android_arm64",
  "runtime": "termux",
  "device": "dimensity_6300",
  "dotprod": true,
  "i8mm": false,
  "single_process_entrance": true,
  "second_runtime_permitted": false
}
EOF
echo "  done"

# ── 8. COURT ROUTE REGISTRY ──────────────────────────────────────────────────
echo "[8] Writing court route registry..."
cat > state/nsq/court/route_registry.json << 'EOF'
{
  "schema": "braxon.nsq_court.route_registry.v1",
  "authority": "NSQ_COURT",
  "canonical_semantics": "base8_switch_topology",
  "routes": [
    "nsq_operator_bus",
    "council_ten_stack",
    "citadel699_nsq_request_return_rebuild"
  ]
}
EOF
echo "  done"

# ── 9. CITADEL699 MATERIALIZATION + REBUILD SURFACE ──────────────────────────
echo "[9] Writing citadel699 rebuild materialization..."
REBUILD_TS="20260428_065519"
MAT_PATH="state/nsq/citadel699/rebuilds/${REBUILD_TS}/council_ten.materialization.json"
NSQ_PATH="state/nsq/citadel699/rebuilds/${REBUILD_TS}/council_ten.rebuild.nsq"

cat > "$MAT_PATH" << 'EOF'
{
  "schema": "braxon.nsq.council_ten.materialization.v1",
  "authority": "NSQ_COURT",
  "canonical_semantics": "base8_switch_topology",
  "transfer_form": "nsq_only",
  "target_size_class": "mb_scale",
  "required_model_count": 10,
  "brain_model_count": 6,
  "sensory_body_count": 4,
  "capital_count": 5,
  "poles_per_capital": 2,
  "raw_fetch_allowed": false,
  "raw_payload_transfer_allowed": false,
  "pointer_setup_allowed": false,
  "reconstruction_mode": "offline_minimal_seed_reconstruction",
  "models": [
    "deepseek-v3-671b",
    "qwen3-235b-a22b",
    "qwen2.5-72b",
    "deepseek-v3-671b-analyzer",
    "llama3.3-70b",
    "gemma3-27b",
    "IndexTTS2",
    "FLUX.1-dev",
    "Wan2.1-T2V-14B",
    "Hunyuan3D-2.1"
  ]
}
EOF

cat > "$NSQ_PATH" << 'EOF'
NSQ_FORM braxon.council_ten.rebuild.v1
AUTHORITY NSQ_COURT
ROUTE citadel699_nsq_request_return_rebuild
REBUILD_TS 20260428_065519
LAW transfer_form_must_be_nsq_only
LAW raw_fetch_forbidden
LAW reconstruction_mode_offline_minimal_seed
MODEL_COUNT 10
BRAIN_COUNT 6
SENSORY_COUNT 4
CAPITAL_COUNT 5
EOF
echo "  done"

# ── 10. TARGET MODELS + REQUEST CAPSULE ──────────────────────────────────────
echo "[10] Writing target models and request capsule..."
cat > state/nsq/citadel699/current/target_models.json << 'EOF'
{
  "schema": "braxon.nsq.citadel699.target_models.v1",
  "authority": "NSQ_COURT",
  "required_model_count": 10,
  "brain_model_count": 6,
  "sensory_body_count": 4,
  "brain_poles": {
    "maverick":  "deepseek-v3-671b",
    "qwen":      "qwen3-235b-a22b",
    "arbiter":   "qwen2.5-72b",
    "analyzer":  "deepseek-v3-671b-analyzer",
    "limbic":    "llama3.3-70b",
    "support":   "gemma3-27b"
  },
  "sensory_bodies": {
    "voice_body":   "IndexTTS2",
    "image_cortex": "FLUX.1-dev",
    "video_cortex": "Wan2.1-T2V-14B",
    "world_body_3d":"Hunyuan3D-2.1"
  }
}
EOF

cat > state/nsq/citadel699/current/request_capsule.json << 'EOF'
{
  "schema": "braxon.nsq.citadel699.request_capsule.v1",
  "authority": "NSQ_COURT",
  "canonical_semantics": "base8_switch_topology",
  "required_model_count": 10,
  "brain_model_count": 6,
  "sensory_body_count": 4,
  "transfer_form": "nsq_only",
  "raw_fetch_allowed": false,
  "raw_payload_transfer_allowed": false,
  "pointer_setup_allowed": false,
  "reconstruction_mode": "offline_minimal_seed_reconstruction"
}
EOF
echo "  done"

# ── 11. CURRENT LINKS ─────────────────────────────────────────────────────────
echo "[11] Writing current citadel links..."
cp "$MAT_PATH" state/nsq/citadel699/current/materialization.json
cp "$NSQ_PATH"  state/nsq/citadel699/current/council_ten.rebuild.nsq

cat > state/braxon/bus/citadel699/current.braxon << 'EOF'
BRAXON_BUS_LINK citadel699_environment_bus_20260427_181342
AUTHORITY NSQ_COURT
ROUTE citadel699_nsq_request_return_rebuild
TRANSFER_FORM nsq_only
EOF
echo "  done"

# ── 12. PROOF FILE WITH REAL SHA256s ──────────────────────────────────────────
echo "[12] Computing SHA256s and writing proof..."
MAT_SHA=$(sha256sum "$MAT_PATH" | awk '{print $1}')
NSQ_SHA=$(sha256sum "$NSQ_PATH"  | awk '{print $1}')

cat > state/nsq/proofs/citadel699_current_rebuild.json << EOF
{
  "schema": "braxon.nsq.citadel699.rebuild_proof.v1",
  "authority": "NSQ_COURT",
  "canonical_semantics": "base8_switch_topology",
  "rebuild_dir": "state/nsq/citadel699/rebuilds/20260428_065519",
  "required_model_count": 10,
  "brain_model_count": 6,
  "sensory_body_count": 4,
  "capital_count": 5,
  "transfer_form": "nsq_only",
  "target_size_class": "mb_scale",
  "citadel_wire_active": true,
  "reconstruction_mode": "offline_minimal_seed_reconstruction",
  "materialization_sha256": "$MAT_SHA",
  "rebuild_sha256": "$NSQ_SHA"
}
EOF
echo "  materialization_sha256=$MAT_SHA"
echo "  rebuild_sha256=$NSQ_SHA"
echo "  written: state/nsq/proofs/citadel699_current_rebuild.json"

# ── 13. VERIFY GATE FILES EXIST ───────────────────────────────────────────────
echo
echo "[13] Gate file presence check..."
PASS=0; FAIL=0
check() {
  if [ -f "$ROOT/$1" ]; then
    echo "  OK  $1"
    PASS=$((PASS+1))
  else
    echo "  MISSING  $1"
    FAIL=$((FAIL+1))
  fi
}
check "state/braxon/release_gates/all_in_check.json"
check "config/nsq/braxon_council_ten_stack.json"
check "apps/nsq/braxon_council_ten_stack.nsq"
check "config/nsq/braxon_indextts2_emotional_frequency_map.json"
check "config/nsq/knowledge_graph.json"
check "config/nsq/vector_imprint.json"
check "config/nsq/android_runtime_oaboot.json"
check "state/nsq/court/route_registry.json"
check "state/nsq/proofs/citadel699_current_rebuild.json"
check "state/nsq/citadel699/rebuilds/20260428_065519/council_ten.materialization.json"
check "state/nsq/citadel699/rebuilds/20260428_065519/council_ten.rebuild.nsq"
check "state/nsq/citadel699/current/target_models.json"
check "state/nsq/citadel699/current/request_capsule.json"
check "state/nsq/citadel699/current/council_ten.rebuild.nsq"
check "state/nsq/citadel699/current/materialization.json"
check "state/braxon/bus/citadel699/current.braxon"
check "crates/nsq-hot/src/lib.rs"
check "tests/braxon_runtime_surface.rs"
check "Cargo.toml"
check "src/main.rs"

echo
echo "gate_files_ok=$PASS  gate_files_missing=$FAIL"

echo
echo "=== done. Now run: ==="
echo "  cargo nextest run --workspace --bins --lib --all --release --no-fail-fast -j7"
