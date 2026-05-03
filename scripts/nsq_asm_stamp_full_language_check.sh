#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${BRAXON_ROOT:-$HOME/Braxon}"
DL="$HOME/storage/shared/Download"
STAMP="$(date +%Y%m%d_%H%M%S)"
OUT="$DL/nsq_asm_stamp_full_language_check_${STAMP}"

mkdir -p "$OUT"
cd "$ROOT"

{
  echo "== NSQ ASM stamp full language check =="
  date
  echo "root=$ROOT"
  echo

  echo "== language audit after fill =="
  python3 tools/nsq_finish/nsq_language_runtime_audit.py | tee "$OUT/language_runtime_audit_after.json"
  echo

  echo "== create sample ASM cipher stamp =="
  python3 tools/nsq_finish/nsq_stamp.py make \
    --name "nsq_anchor_lever_pack_kernel_scaffold" \
    --stamp-id "stamp_asm_anchor_lever_pack_v1" \
    --language asm \
    --dialect aarch64_asm \
    --family assembly \
    --target aarch64_asm \
    --cipher \
    --meaning "Pack NSQ anchor and lever units into a bounded hot-kernel scaffold for later ASM optimization." \
    --notes "Reusable ASM scaffold stamp. Cipher expands to familiar AArch64-shaped code but remains governed by NSQ stamp/court validation." \
    --text $'FN nsq_anchor_lever_pack_v1\nPUSH x29, x30, [sp, #-16]!\nMOV x29, sp\nSCAN anchor_lever_input\nPACK anchor lever groups\nHASH blake_null_semantic_digest\nPOP x29, x30, [sp], #16\nRET' \
    | tee "$OUT/sample_stamp.json"
  echo

  echo "== verify stamps =="
  python3 tools/nsq_finish/nsq_stamp.py verify | tee "$OUT/stamp_verify.json"
  echo

  echo "== stamp list asm =="
  python3 tools/nsq_finish/nsq_stamp.py list --language asm | tee "$OUT/stamp_list_asm.json"
  echo

  echo "== docs and configs =="
  ls -la docs/nsq config/nsq tools/nsq_finish state/nsq/stamps | sed -n '1,240p'
  echo

  echo "== cargo check quick =="
  if command -v cargo >/dev/null 2>&1 && [ -f Cargo.toml ]; then
    cargo check --workspace > "$OUT/cargo_check.out" 2> "$OUT/cargo_check.err" || true
    tail -n 60 "$OUT/cargo_check.err" || true
  else
    echo "cargo unavailable or no Cargo.toml"
  fi
  echo

  echo "== git status relevant =="
  git status --short docs/nsq config/nsq tools/nsq_finish scripts/nsq_asm_stamp_full_language_check.sh state/nsq 2>/dev/null || true

} | tee "$OUT/summary.txt"

echo "report_dir=$OUT"
