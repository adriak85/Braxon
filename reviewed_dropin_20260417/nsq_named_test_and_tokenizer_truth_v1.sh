#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$HOME/Braxon"
DL="$HOME/storage/shared/Download"
STAMP="$(date +%Y%m%d_%H%M%S)"
OUT="$DL/nsq_named_test_and_tokenizer_truth_$STAMP"

mkdir -p "$OUT"
cd "$ROOT"

LATEST_PACK="$(find "$DL" -maxdepth 1 -type d -name 'nsq_semantic_extension_pack_*' | sort | tail -n 1)"

cargo test -p nsq-runtime --release -- --list > "$OUT/01_nsq_runtime_test_list.txt" 2>&1 || true
cargo test -p nsq-core --release -- --list > "$OUT/02_nsq_core_test_list.txt" 2>&1 || true

python3 <<'PY' > "$OUT/03_tokenizer_schema_truth.txt"
from pathlib import Path
import json

root = Path.home() / "Braxon"
tok = root / "assets/braxon_core/tokenizer/braxon_unified_tokenizer.json"

print(f"tokenizer_path={tok}")
print(f"exists={tok.exists()}")

if not tok.exists():
    raise SystemExit(0)

raw = tok.read_text(errors="ignore")
print(f"bytes={len(raw.encode())}")

try:
    obj = json.loads(raw)
except Exception as e:
    print(f"json_parse_error={e}")
    raise SystemExit(0)

print(f"top_type={type(obj).__name__}")

if isinstance(obj, dict):
    print(f"top_keys={sorted(obj.keys())[:80]}")
    model = obj.get("model")
    if isinstance(model, dict):
        print(f"model_type={model.get('type')}")
        vocab = model.get("vocab")
        merges = model.get("merges")
        if isinstance(vocab, dict):
            print(f"vocab_entries={len(vocab)}")
            ks = list(vocab.keys())
            for s in ks[:40]:
                print(f"vocab_sample={s}")
        else:
            print("vocab_entries=0")
        if isinstance(merges, list):
            print(f"merge_entries={len(merges)}")
    added = obj.get("added_tokens")
    if isinstance(added, list):
        print(f"added_tokens={len(added)}")
        for item in added[:40]:
            print(f"added_token={item}")
PY

python3 <<'PY' > "$OUT/04_semantic_pack_vs_tokenizer.txt"
from pathlib import Path
import json
import re

dl = Path.home() / "storage/shared/Download"
root = Path.home() / "Braxon"
packs = sorted([p for p in dl.iterdir() if p.is_dir() and p.name.startswith("nsq_semantic_extension_pack_")])
if not packs:
    print("latest_pack=<none>")
    raise SystemExit(0)

pack = packs[-1]
print(f"latest_pack={pack}")

terms_file = pack / "semantic_tokenizer_candidate_terms.json"
tok_file = root / "assets/braxon_core/tokenizer/braxon_unified_tokenizer.json"

terms = json.loads(terms_file.read_text())
tok = json.loads(tok_file.read_text())

if isinstance(tok, dict) and isinstance(tok.get("model"), dict) and isinstance(tok["model"].get("vocab"), dict):
    vocab = set(tok["model"]["vocab"].keys())
else:
    vocab = set(k for k in tok.keys() if isinstance(k, str))

print(f"tokenizer_vocab_size={len(vocab)}")

for group, items in terms.items():
    print(f"group={group}")
    for item in items:
        exact = item in vocab
        partial = any(item.lower() in v.lower() for v in vocab)
        print(f"{group}\t{item}\texact={str(exact).lower()}\tpartial={str(partial).lower()}")
PY

{
  echo "== semantic activation surfaces =="
  rg -n -S -i \
    -e 'grid_26d' \
    -e 'semantic_score_alignment' \
    -e 'delta_extension' \
    -e 'tokenizer_binding_state' \
    -e 'tokenizer_bridge_stamp' \
    crates src config state specs docs nsq || true
} > "$OUT/05_semantic_activation_hits.txt"

{
  echo "out_dir=$OUT"
  echo
  echo "== runtime tests mentioning zlm/canonical/base8 =="
  rg -n -i 'zlm|canonical|base8|semantic' "$OUT/01_nsq_runtime_test_list.txt" || true
  echo
  echo "== core tests mentioning zlm/canonical/base8 =="
  rg -n -i 'zlm|canonical|base8|semantic' "$OUT/02_nsq_core_test_list.txt" || true
  echo
  echo "== tokenizer truth head =="
  sed -n '1,120p' "$OUT/03_tokenizer_schema_truth.txt"
  echo
  echo "== semantic pack vs tokenizer head =="
  sed -n '1,120p' "$OUT/04_semantic_pack_vs_tokenizer.txt"
} > "$OUT/99_summary.txt"

cat "$OUT/99_summary.txt"
