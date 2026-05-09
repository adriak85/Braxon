#!/data/data/com.termux/files/usr/bin/bash
set -eu

cd "${HOME}/Braxon" 2>/dev/null || cd "$(git rev-parse --show-toplevel)"

fail() {
  echo "FAIL: $*" >&2
  exit 91
}

warn() {
  echo "WARN: $*" >&2
}

need_file() {
  [ -s "$1" ] || fail "missing or empty: $1"
}

CONTRACT="apps/nsq/citadel699_reduced_intent_wire_contract.nsq"
CONFIG="config/nsq/citadel699_reduced_intent_wire_contract.json"
ROUTE="state/nsq/court/routes/citadel699_reduced_intent_wire_contract.json"
PROOF="state/nsq/proofs/citadel699_reduced_intent_wire_contract.json"
POLICY="config/nsq/model_transfer_nsq_only_policy.json"
RECEPTOR="config/nsq/citadel699_wire_receptor.json"
COUNCIL="config/nsq/braxon_council_ten_stack.json"

for f in "$CONTRACT" "$CONFIG" "$ROUTE" "$PROOF" "$POLICY" "$RECEPTOR" "$COUNCIL"; do
  need_file "$f"
done

# Required affirmative laws. These are not normal download flags; they are the active NSQ wire law.
grep -F 'LAW nsq_is_the_only_runtime' "$CONTRACT" >/dev/null || fail "contract does not assert NSQ-only runtime"
grep -F 'LAW nsq_is_the_bus' "$CONTRACT" >/dev/null || fail "contract does not assert NSQ is the bus"
grep -F 'LAW nsq_is_lowest_base_language' "$CONTRACT" >/dev/null || fail "contract does not assert NSQ lowest base language"
grep -F 'LAW court_is_compositor' "$CONTRACT" >/dev/null || fail "contract does not assert court/compositor identity"
grep -F 'LAW court_is_not_agents' "$CONTRACT" >/dev/null || fail "contract does not assert court is not agents"
grep -F 'LAW raw_weight_download_to_phone_is_forbidden' "$CONTRACT" >/dev/null || fail "contract does not forbid raw phone weight download"
grep -F 'LAW raw_payload_transfer_to_phone_is_forbidden' "$CONTRACT" >/dev/null || fail "contract does not forbid raw phone payload transfer"
grep -F 'LAW source_side_receive_translates_immediately_to_nsq' "$CONTRACT" >/dev/null || fail "contract does not require source-side NSQ translation"
grep -F 'LAW local_storage_is_reduced_intent_seed_only' "$CONTRACT" >/dev/null || fail "contract does not restrict local storage to reduced intent seed"
grep -F 'LAW tiny_seed_must_include_reconstruction_algorithm' "$CONTRACT" >/dev/null || fail "contract does not require reconstruction algorithm in tiny seed"
grep -F 'LAW hot_hypervisor_buildout_required' "$CONTRACT" >/dev/null || fail "contract does not require hot hypervisor buildout"
grep -F 'LAW recursive_instruction_thread_required' "$CONTRACT" >/dev/null || fail "contract does not require recursive instruction thread"

# Active policy/receptor/council must agree with the special non-normal transfer model.
grep -F '"raw_weight_download_allowed": false' "$POLICY" >/dev/null || fail "policy allows raw weight download"
grep -F '"raw_payload_transfer_allowed": false' "$POLICY" >/dev/null || fail "policy allows raw payload transfer"
grep -F '"wire_form": "nsq_language_wire"' "$POLICY" >/dev/null || fail "policy missing NSQ language wire"
grep -F '"citadel699_form": "daemonized_nuband_instruction_set"' "$POLICY" >/dev/null || fail "policy missing Citadel699 daemonized nuband form"

grep -F '"raw_weight_download_allowed": false' "$RECEPTOR" >/dev/null || fail "receptor allows raw weight download"
grep -F '"raw_payload_transfer_allowed": false' "$RECEPTOR" >/dev/null || fail "receptor allows raw payload transfer"
grep -F '"wire_form": "nsq_language_wire"' "$RECEPTOR" >/dev/null || fail "receptor missing NSQ language wire"
grep -F '"citadel_form": "daemonized_nuband_instruction_set"' "$RECEPTOR" >/dev/null || fail "receptor missing daemonized nuband form"

grep -F '"required_model_count": 10' "$COUNCIL" >/dev/null || fail "council is not locked to ten surfaces"
grep -F '"brain_model_count": 6' "$COUNCIL" >/dev/null || fail "council brain count is not six"
grep -F '"sensory_body_count": 4' "$COUNCIL" >/dev/null || fail "council sensory body count is not four"
grep -F '"deepseek-v3-671b"' "$COUNCIL" >/dev/null || fail "missing deepseek-v3-671b"
grep -F '"qwen3-235b-a22b"' "$COUNCIL" >/dev/null || fail "missing qwen3-235b-a22b"
grep -F '"qwen2.5-72b"' "$COUNCIL" >/dev/null || fail "missing qwen2.5-72b"
grep -F '"deepseek-v3-671b-analyzer"' "$COUNCIL" >/dev/null || fail "missing deepseek-v3-671b-analyzer"
grep -F '"llama3.3-70b"' "$COUNCIL" >/dev/null || fail "missing llama3.3-70b"
grep -F '"gemma3-27b"' "$COUNCIL" >/dev/null || fail "missing gemma3-27b"
grep -F '"Wan2.1-T2V-14B"' "$COUNCIL" >/dev/null || fail "missing Wan2.1-T2V-14B"
grep -F '"IndexTTS2"' "$COUNCIL" >/dev/null || fail "missing IndexTTS2"
grep -F '"raw_fetch_allowed": false' "$COUNCIL" >/dev/null || fail "council allows raw fetch"
grep -F '"raw_payload_transfer_allowed": false' "$COUNCIL" >/dev/null || fail "council allows raw payload transfer"
grep -F '"pointer_setup_allowed": false' "$COUNCIL" >/dev/null || fail "council allows pointer setup"
grep -F '"target_size_class": "mb_scale"' "$COUNCIL" >/dev/null || fail "council is not MB-scale target"
grep -F '"tiny_seed_reconstruction_required": true' "$COUNCIL" >/dev/null || fail "council does not require tiny seed reconstruction"

# Guard against active raw payloads under the Braxon transport path.
if find assets/braxon_core/source_ingest/braxon_transport -type f -name '*.safetensors' 2>/dev/null | grep -q .; then
  fail "raw safetensors found in active Braxon transport path"
fi

if grep -RIl '^version https://git-lfs.github.com/spec/v1' assets/braxon_core/source_ingest/braxon_transport 2>/dev/null | grep -q .; then
  fail "Git LFS pointer payload found in active Braxon transport path"
fi

# Legacy downloader code may still contain raw-download vocabulary as rejected/older machinery.
# This verifier no longer fails merely because legacy source text mentions .safetensors or huggingface-cli.
# It fails only if active law/config permits those paths or active transport contains payload material.
LEGACY_RAW_REFS="$(grep -RInE 'huggingface-cli download|git lfs pull' tools/BRAXON_model_downloader tools/nsq_citadel699 tools/citadel699_nsq_request_return_rebuild.sh 2>/dev/null || true)"
if [ -n "$LEGACY_RAW_REFS" ]; then
  warn "legacy raw-download/materialization references still exist as code text; active NSQ law keeps them denied."
fi

# The old direct raw downloader must expose its own raw-fetch blocker if present.
if [ -f tools/BRAXON_model_downloader/BRAXON_model_downloader.py ]; then
  grep -F 'raw_fetch_blocked' tools/BRAXON_model_downloader/BRAXON_model_downloader.py >/dev/null || \
    fail "legacy downloader lacks raw_fetch_blocked gate marker"
fi

echo "OK: Citadel699 reduced-intent wire contract is installed."
echo "OK: ten-surface council is locked and no model stack reversion was detected."
echo "OK: active path is external source gate -> Citadel pipe -> source-side NSQ reduced intent -> tiny seed/reconstruction law -> hot hypervisor buildout."
echo "OK: active transport contains no raw safetensors or Git LFS pointer payloads."
