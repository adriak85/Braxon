#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
. "$ROOT/env/BRAXON_env.sh"

cd "$BRAXON_HOME"
mkdir -p tmp/nsq_family_proof

cat > tmp/nsq_family_proof/calibration_lock.json <<'JSON'
{
  "selected_profile": "native-family-proof",
  "promoted_macros": ["auth_link", "state_open"],
  "hot_targets": ["wake.self", "graph.core"],
  "threshold_macro_promotion": 4,
  "threshold_expansion": 8,
  "representation_lock": {
    "symbol_id_class": "u16",
    "macro_id_class": "u16",
    "anchor_class": "u32_delta",
    "gain_class": "u16",
    "window_class": "u8"
  },
  "rebalance_actions": ["family_native_semantics_enforced"]
}
JSON

cat > tmp/nsq_family_proof/sample_canonical.nsq <<'NSQ'
noise wake.self :macro ping :a 1 :b 2 :pos 10 :amp 5
triple wake.self -> auth_link -> gate.alpha :layer 1 :plane 2 :anchor 100 :weight 20 :flags 3
membrane gate.alpha :state state_open :flux 7 :gate 1 :phase 2
NSQ

cat > tmp/nsq_family_proof/sample_sexpr.nsq <<'NSQ'
@dialect sexpr
(noise wake.self :macro ping :a 1 :b 2 :pos 10 :amp 5)
(triple wake.self auth_link gate.alpha :layer 1 :plane 2 :anchor 100 :weight 20 :flags 3)
(membrane gate.alpha :state state_open :flux 7 :gate 1 :phase 2)
NSQ

cat > tmp/nsq_family_proof/sample_lua_shape.nsq <<'NSQ'
@dialect lua_shape
noise wake.self macro=ping a=1 b=2 pos=10 amp=5
triple wake.self rel=auth_link obj=gate.alpha layer=1 plane=2 anchor=100 weight=20 flags=3
membrane gate.alpha state=state_open flux=7 gate=1 phase=2
NSQ

cat > tmp/nsq_family_proof/sample_python_shape.nsq <<'NSQ'
@dialect python_shape
noise(wake.self, macro=ping, a=1, b=2, pos=10, amp=5)
triple(wake.self, rel=auth_link, obj=gate.alpha, layer=1, plane=2, anchor=100, weight=20, flags=3)
membrane(gate.alpha, state=state_open, flux=7, gate=1, phase=2)
NSQ

BIN="$CARGO_TARGET_DIR/release/nsq-compile"
cargo build -q --release -p nsq-compile

for name in canonical sexpr lua_shape python_shape; do
  "$BIN" \
    "tmp/nsq_family_proof/sample_${name}.nsq" \
    "tmp/nsq_family_proof/${name}.nsqb" \
    "tmp/nsq_family_proof/calibration_lock.json" \
    "tmp/nsq_family_proof/${name}.manifest.json"
done

echo "== manifests =="
for f in tmp/nsq_family_proof/*.manifest.json; do
  echo "-- $f --"
  cat "$f"
  echo
done

echo "== artifact hashes =="
sha256sum tmp/nsq_family_proof/*.nsqb
