#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${ROOT:-$HOME/Braxon}"
DL="$HOME/storage/shared/Download"
STAMP="$(date +%Y%m%d_%H%M%S)"
OUT="$DL/repair_BRAXON_root_runtime_semantic_truth_$STAMP"

mkdir -p "$OUT"
cd "$ROOT"

CORE="$ROOT/crates/Braxon-core/src/lib.rs"
MAIN="$ROOT/src/main.rs"
CARGO="$ROOT/crates/Braxon-core/Cargo.toml"
ROOT_CARGO="$ROOT/Cargo.toml"

for p in "$CORE" "$MAIN" "$CARGO"; do
  if [ ! -f "$p" ]; then
    echo "missing_required_path=$p"
    exit 1
  fi
done

cp -f "$CORE" "$OUT/BRAXON_core_lib.rs.before"
cp -f "$MAIN" "$OUT/main.rs.before"
cp -f "$CARGO" "$OUT/BRAXON_core_Cargo.toml.before"

export ROOT OUT CORE MAIN CARGO ROOT_CARGO

python3 <<'PY'
from pathlib import Path
import os
import re

root = Path(os.environ["ROOT"])
out = Path(os.environ["OUT"])
core_path = Path(os.environ["CORE"])
main_path = Path(os.environ["MAIN"])
cargo_path = Path(os.environ["CARGO"])
root_cargo_path = Path(os.environ["ROOT_CARGO"])

core_raw = core_path.read_text(encoding="utf-8", errors="ignore")
main_raw = main_path.read_text(encoding="utf-8", errors="ignore")
cargo_raw = cargo_path.read_text(encoding="utf-8", errors="ignore")
root_cargo_raw = root_cargo_path.read_text(encoding="utf-8", errors="ignore") if root_cargo_path.exists() else ""

status = []

def workspace_has_serde_json(root_raw: str) -> bool:
    return bool(re.search(r'(?ms)^\[workspace\.dependencies\].*?^\s*serde_json\s*=', root_raw))

def normalize_serde_json_dependency(cargo_raw: str, root_raw: str) -> tuple[str, bool]:
    desired = 'serde_json = { workspace = true }' if workspace_has_serde_json(root_raw) else 'serde_json = "1"'
    lines = cargo_raw.splitlines()
    out_lines = []
    in_deps = False
    saw_deps = False
    inserted = False
    changed = False

    dep_pat = re.compile(r'^\s*serde_json(?:\.workspace\s*=\s*true|\s*=.*)$')

    for line in lines:
        stripped = line.strip()

        if stripped.startswith('[') and stripped.endswith(']'):
            if in_deps and not inserted:
                out_lines.append(desired)
                inserted = True
                changed = True
            in_deps = (stripped == '[dependencies]')
            if in_deps:
                saw_deps = True
            out_lines.append(line)
            continue

        if in_deps and dep_pat.match(line):
            if not inserted:
                out_lines.append(desired)
                inserted = True
            changed = True
            continue

        out_lines.append(line)

    if in_deps and not inserted:
        out_lines.append(desired)
        inserted = True
        changed = True

    if not saw_deps:
        if out_lines and out_lines[-1].strip():
            out_lines.append("")
        out_lines.extend(["[dependencies]", desired])
        changed = True

    normalized = "\n".join(out_lines).rstrip() + "\n"
    return normalized, changed

cargo_raw, cargo_changed = normalize_serde_json_dependency(cargo_raw, root_cargo_raw)
if cargo_changed:
    cargo_path.write_text(cargo_raw, encoding="utf-8")
status.append(f"cargo_changed={str(cargo_changed).lower()}")

field_block = """    pub runtime_semantic_consumers_ready: bool,
    pub runtime_semantic_feed_entries: usize,
    pub runtime_compass_seed_tokens: usize,
    pub runtime_semantic_patch_anchor_count: usize,
    pub runtime_semantic_tests_present: bool,
"""

if "pub runtime_semantic_consumers_ready: bool," not in core_raw:
    if "pub native_runtime_authority_ok: bool," in core_raw:
        core_raw = core_raw.replace(
            "pub native_runtime_authority_ok: bool,\n",
            "pub native_runtime_authority_ok: bool,\n" + field_block,
            1,
        )
        status.append("struct_fields=patched")
    else:
        status.append("struct_fields=anchor_missing")
else:
    status.append("struct_fields=already_present")

helper_marker = "fn runtime_semantic_truth("
helper_block = r'''
fn runtime_semantic_truth(
    root: &std::path::Path,
    runtime_lib_raw: &str,
) -> (bool, usize, usize, usize, bool) {
    let tok_path = root.join("assets/braxon_core/tokenizer/braxon_unified_tokenizer.json");
    let tests_path = root.join("crates/nsq-runtime/tests/runtime_semantic_context_patch.rs");

    let mut feed_entries = 0usize;
    let mut compass_tokens = 0usize;

    if let Ok(raw) = std::fs::read_to_string(&tok_path) {
        if let Ok(obj) = serde_json::from_str::<serde_json::Value>(&raw) {
            if let Some(feed) = obj.get("semantic_feed").and_then(serde_json::Value::as_object) {
                if let Some(n) = feed.get("entry_count").and_then(serde_json::Value::as_u64) {
                    feed_entries = n as usize;
                } else if let Some(entries) = feed.get("entries").and_then(serde_json::Value::as_array) {
                    feed_entries = entries.len();
                }
            }
            if let Some(seed) = obj.get("compass_seed").and_then(serde_json::Value::as_object) {
                if let Some(tokens) = seed.get("tokens").and_then(serde_json::Value::as_array) {
                    compass_tokens = tokens.len();
                }
            }
        }
    }

    let patch_anchor_count = [
        "BRAXON_runtime_semantic_patch::lane",
        "BRAXON_runtime_semantic_patch::execute_slice",
        "BRAXON_runtime_semantic_patch::algorithm_lever_from_semantic_text",
        "BRAXON_runtime_semantic_patch::execute_request",
    ]
    .iter()
    .filter(|m| runtime_lib_raw.contains(**m))
    .count();

    let tests_present = tests_path.exists();

    let ready =
        feed_entries > 0 &&
        compass_tokens > 0 &&
        patch_anchor_count >= 2 &&
        tests_present;

    (
        ready,
        feed_entries,
        compass_tokens,
        patch_anchor_count,
        tests_present,
    )
}
'''.lstrip()

if helper_marker not in core_raw:
    core_raw = core_raw.rstrip() + "\n\n" + helper_block + "\n"
    status.append("helper_block=patched")
else:
    status.append("helper_block=already_present")

compute_block = """
    let (
        runtime_semantic_consumers_ready,
        runtime_semantic_feed_entries,
        runtime_compass_seed_tokens,
        runtime_semantic_patch_anchor_count,
        runtime_semantic_tests_present,
    ) = runtime_semantic_truth(root, &runtime_lib_raw);
""".rstrip()

if "runtime_semantic_truth(root, &runtime_lib_raw)" not in core_raw:
    pat = re.compile(r'let\s+native_runtime_authority_ok\s*=\s*.*?;\n', re.S)
    m = pat.search(core_raw)
    if m:
        core_raw = core_raw[:m.end()] + "\n" + compute_block + "\n" + core_raw[m.end():]
        status.append("verify_compute=patched")
    else:
        status.append("verify_compute=anchor_missing")
else:
    status.append("verify_compute=already_present")

status.append("struct_literal_insertions=disabled_safe")

main_marker = 'println!("tokenizer_bridge_stamp={}", report.tokenizer_bridge_stamp);'
main_block = """    println!(
        "runtime_semantic_consumers_ready={}",
        report.runtime_semantic_consumers_ready
    );
    println!(
        "runtime_semantic_feed_entries={}",
        report.runtime_semantic_feed_entries
    );
    println!(
        "runtime_compass_seed_tokens={}",
        report.runtime_compass_seed_tokens
    );
    println!(
        "runtime_semantic_patch_anchor_count={}",
        report.runtime_semantic_patch_anchor_count
    );
    println!(
        "runtime_semantic_tests_present={}",
        report.runtime_semantic_tests_present
    );
"""

if "runtime_semantic_consumers_ready={}" not in main_raw:
    if main_marker in main_raw:
        main_raw = main_raw.replace(main_marker, main_marker + "\n" + main_block, 1)
        status.append("main_prints=patched")
    else:
        status.append("main_prints=anchor_missing")
else:
    status.append("main_prints=already_present")

core_path.write_text(core_raw, encoding="utf-8")
main_path.write_text(main_raw, encoding="utf-8")

(out / "10_patch_status.txt").write_text("\n".join(status) + "\n", encoding="utf-8")
PY

{
  echo "== patch status =="
  cat "$OUT/10_patch_status.txt"
  echo
  echo "== cargo manifest serde_json lines =="
  rg -n -S 'serde_json' "$CARGO" || true
  echo
  echo "== root semantic truth grep =="
  rg -n -S \
    -e 'runtime_semantic_truth' \
    -e 'runtime_semantic_consumers_ready' \
    -e 'runtime_semantic_feed_entries' \
    -e 'runtime_compass_seed_tokens' \
    -e 'runtime_semantic_patch_anchor_count' \
    -e 'runtime_semantic_tests_present' \
    crates/Braxon-core/src/lib.rs src/main.rs || true
} > "$OUT/11_patch_grep.txt"

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
  echo "== patch status =="
  cat "$OUT/10_patch_status.txt"
  echo
  echo "== patch grep head =="
  sed -n '1,220p' "$OUT/11_patch_grep.txt"
  echo
  echo "== cargo check tail =="
  tail -n 140 "$OUT/21_check.txt" || true
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
