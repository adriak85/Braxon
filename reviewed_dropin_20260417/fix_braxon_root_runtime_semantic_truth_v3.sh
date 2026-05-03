#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${ROOT:-$HOME/Braxon}"
DL="$HOME/storage/shared/Download"
STAMP="$(date +%Y%m%d_%H%M%S)"
OUT="$DL/fix_BRAXON_root_runtime_semantic_truth_$STAMP"

mkdir -p "$OUT"
cd "$ROOT"

CORE="$ROOT/crates/Braxon-core/src/lib.rs"
MAIN="$ROOT/src/main.rs"
REPAIR="$DL/repair_BRAXON_root_runtime_semantic_truth_v2.sh"
VERIFY="$DL/verify_BRAXON_root_runtime_semantic_truth_v2.sh"

for p in "$CORE" "$MAIN"; do
  if [ ! -f "$p" ]; then
    echo "missing_required_path=$p"
    exit 1
  fi
done

cp -f "$CORE" "$OUT/BRAXON_core_lib.rs.before"
cp -f "$MAIN" "$OUT/main.rs.before"
[ -f "$REPAIR" ] && cp -f "$REPAIR" "$OUT/repair_BRAXON_root_runtime_semantic_truth_v2.sh.before" || true
[ -f "$VERIFY" ] && cp -f "$VERIFY" "$OUT/verify_BRAXON_root_runtime_semantic_truth_v2.sh.before" || true

export ROOT OUT CORE MAIN REPAIR VERIFY

python3 <<'PY'
from pathlib import Path
import os
import re

core_path = Path(os.environ["CORE"])
main_path = Path(os.environ["MAIN"])
repair_path = Path(os.environ["REPAIR"])
verify_path = Path(os.environ["VERIFY"])
out = Path(os.environ["OUT"])

core_raw = core_path.read_text(encoding="utf-8", errors="ignore")
main_raw = main_path.read_text(encoding="utf-8", errors="ignore")

status = []

SHORT_BLOCK = (
    "        runtime_semantic_consumers_ready,\n"
    "        runtime_semantic_feed_entries,\n"
    "        runtime_compass_seed_tokens,\n"
    "        runtime_semantic_patch_anchor_count,\n"
    "        runtime_semantic_tests_present,\n"
)

FALSE_BLOCK = (
    "        runtime_semantic_consumers_ready: false,\n"
    "        runtime_semantic_feed_entries: 0,\n"
    "        runtime_compass_seed_tokens: 0,\n"
    "        runtime_semantic_patch_anchor_count: 0,\n"
    "        runtime_semantic_tests_present: false,\n"
)

TRUE_BLOCK = (
    "        runtime_semantic_consumers_ready: true,\n"
    "        runtime_semantic_feed_entries: 0,\n"
    "        runtime_compass_seed_tokens: 0,\n"
    "        runtime_semantic_patch_anchor_count: 0,\n"
    "        runtime_semantic_tests_present: false,\n"
)

def collapse_exact_double(raw: str, block: str) -> tuple[str, int]:
    count = 0
    doubled = block + block
    while doubled in raw:
        raw = raw.replace(doubled, block)
        count += 1
    return raw, count

core_raw, c1 = collapse_exact_double(core_raw, SHORT_BLOCK)
core_raw, c2 = collapse_exact_double(core_raw, FALSE_BLOCK)
core_raw, c3 = collapse_exact_double(core_raw, TRUE_BLOCK)

status.append(f"collapsed_short_blocks={c1}")
status.append(f"collapsed_false_blocks={c2}")
status.append(f"collapsed_true_blocks={c3}")

def dedupe_runtime_semantic_runs(raw: str) -> tuple[str, int]:
    lines = raw.splitlines(True)
    out_lines = []
    removed = 0
    i = 0

    semantic_pat = re.compile(
        r'^(?P<indent>\s*)'
        r'(?P<name>runtime_semantic_(?:consumers_ready|feed_entries|compass_seed_tokens|patch_anchor_count|tests_present))'
        r'(?P<rest>\s*(?::.*)?\,\s*)$'
    )

    while i < len(lines):
        line = lines[i]
        out_lines.append(line)
        i += 1

        stripped = line.rstrip()
        if not stripped.endswith("{"):
            continue

        seen = set()
        while i < len(lines):
            cur = lines[i]
            m = semantic_pat.match(cur)
            if m:
                name = m.group("name")
                key = (name, ":" in m.group("rest"))
                if key in seen:
                    removed += 1
                    i += 1
                    continue
                seen.add(key)

            out_lines.append(cur)

            if cur.lstrip().startswith("}"):
                i += 1
                break

            i += 1

    return "".join(out_lines), removed

core_raw, removed_semantic_dupes = dedupe_runtime_semantic_runs(core_raw)
status.append(f"removed_semantic_duplicate_lines={removed_semantic_dupes}")

# Keep the helper and fields, but make sure the same shorthand pack is not repeated
# immediately after runtime_semantic_truth(...) compute insertion.
compute_pat = re.compile(
    r'(\)\s*=\s*runtime_semantic_truth\(root,\s*&runtime_lib_raw\);\n)'
    r'(?:' + re.escape(SHORT_BLOCK) + r')+',
    re.S
)
core_raw, compute_subs = compute_pat.subn(r'\1', core_raw)
status.append(f"removed_post_compute_field_runs={compute_subs}")

core_path.write_text(core_raw, encoding="utf-8")

# Harden the repair script so it no longer does broad literal insertions.
if repair_path.exists():
    repair_raw = repair_path.read_text(encoding="utf-8", errors="ignore")

    loop_pat = re.compile(
        r'for pat, repl in \[\n.*?\n\]:\n'
        r'    if pat in core_raw and "runtime_semantic_patch_anchor_count" not in core_raw.split\(pat, 1\)\[0\]\[-300:\]:\n'
        r'        core_raw = core_raw.replace\(pat, repl\)\n',
        re.S,
    )

    replacement = (
        'status.append("struct_literal_insertions=disabled_safe")\n'
    )

    repair_raw_new, subs = loop_pat.subn(replacement, repair_raw, count=1)
    if subs:
        repair_path.write_text(repair_raw_new, encoding="utf-8")
    status.append(f"repair_script_literal_loop_replaced={subs}")
else:
    status.append("repair_script_literal_loop_replaced=missing")

# Make the verify script report whether compile errors still mention duplicate fields.
if verify_path.exists():
    verify_raw = verify_path.read_text(encoding="utf-8", errors="ignore")
    marker = '  echo "== cargo check tail =="'
    inject = (
        '  echo "== duplicate field scan == "\n'
        '  rg -n -S "specified more than once|E0062" "$OUT/03_check.txt" || true\n'
        '  echo\n'
        '  echo "== cargo check tail == "\n'
    )
    if marker in verify_raw and 'duplicate field scan' not in verify_raw:
        verify_raw = verify_raw.replace(marker, inject, 1)
        verify_path.write_text(verify_raw, encoding="utf-8")
        status.append("verify_script_augmented=true")
    else:
        status.append("verify_script_augmented=false")
else:
    status.append("verify_script_augmented=missing")

(out / "10_fix_status.txt").write_text("\n".join(status) + "\n", encoding="utf-8")
PY

{
  echo "== fix status =="
  cat "$OUT/10_fix_status.txt"
  echo
  echo "== Braxon-core semantic truth grep =="
  rg -n -S \
    -e 'runtime_semantic_truth' \
    -e 'runtime_semantic_consumers_ready' \
    -e 'runtime_semantic_feed_entries' \
    -e 'runtime_compass_seed_tokens' \
    -e 'runtime_semantic_patch_anchor_count' \
    -e 'runtime_semantic_tests_present' \
    crates/Braxon-core/src/lib.rs src/main.rs || true
  echo
  echo "== repair script status lines =="
  [ -f "$REPAIR" ] && rg -n -S 'struct_literal_insertions=disabled_safe|verify_compute=|helper_block=|cargo_changed=' "$REPAIR" || true
} > "$OUT/11_source_grep.txt"

cargo fmt --all > "$OUT/20_fmt.txt" 2>&1 || true
cargo check --workspace --bins --lib --all-targets --all-features --release --keep-going -j6 > "$OUT/21_check.txt" 2>&1 || true
cargo test -p nsq-runtime -- --nocapture > "$OUT/22_nsq_runtime_tests.txt" 2>&1 || true

if [ -x "$HOME/.cargo/target-cache/Braxon/release/Braxon" ]; then
  BIN="$HOME/.cargo/target-cache/Braxon/release/Braxon"
else
  BIN="cargo run --release --"
fi

bash -lc "$BIN status" > "$OUT/30_status.txt" 2>&1 || true
bash -lc "$BIN verify" > "$OUT/31_verify.txt" 2>&1 || true
bash -lc "$BIN plan" > "$OUT/32_plan.txt" 2>&1 || true

{
  echo "out_dir=$OUT"
  echo
  echo "== fix status =="
  cat "$OUT/10_fix_status.txt"
  echo
  echo "== source grep head =="
  sed -n '1,260p' "$OUT/11_source_grep.txt"
  echo
  echo "== duplicate field scan =="
  rg -n -S 'specified more than once|E0062' "$OUT/21_check.txt" || true
  echo
  echo "== cargo check tail =="
  tail -n 160 "$OUT/21_check.txt" || true
  echo
  echo "== nsq-runtime tests tail =="
  tail -n 120 "$OUT/22_nsq_runtime_tests.txt" || true
  echo
  echo "== Braxon semantic lines =="
  rg -n -S \
    -e 'runtime_semantic_consumers_ready' \
    -e 'runtime_semantic_feed_entries' \
    -e 'runtime_compass_seed_tokens' \
    -e 'runtime_semantic_patch_anchor_count' \
    -e 'runtime_semantic_tests_present' \
    "$OUT/30_status.txt" "$OUT/31_verify.txt" "$OUT/32_plan.txt" || true
} > "$OUT/99_summary.txt"

cat "$OUT/99_summary.txt"
