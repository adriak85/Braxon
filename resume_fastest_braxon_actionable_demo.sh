#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$HOME/Braxon"
STAMP="$(date +%Y%m%d_%H%M%S)"
REPORT_DIR="$ROOT/state/reports"
REPORT="$REPORT_DIR/resume_fastest_braxon_actionable_demo_$STAMP.txt"
DOWNLOAD_COPY="$HOME/storage/shared/Download/resume_fastest_braxon_actionable_demo_$STAMP.txt"

mkdir -p "$REPORT_DIR"
cd "$ROOT"

log() { printf '%s\n' "$*" | tee -a "$REPORT"; }
run() {
  log ""
  log ">>> $*"
  "$@" 2>&1 | tee -a "$REPORT"
}

: > "$REPORT"

log "=== RESUME FASTEST BRAXON ACTIONABLE DEMO ==="
log "root=$ROOT"
log "stamp=$STAMP"
log "fix=add nsq-grid/nsq-wake to workspace if crate dirs exist"

log ""
log "=== 1. patch workspace members safely ==="
python3 - <<'PY'
from pathlib import Path

p = Path("Cargo.toml")
s = p.read_text()
orig = s

needed = []
for member in ["crates/nsq-grid", "crates/nsq-wake"]:
    if Path(member, "Cargo.toml").exists() and f'"{member}"' not in s:
        needed.append(member)

if needed:
    backup = Path(f"Cargo.toml.before_add_nsq_wake_workspace")
    if not backup.exists():
        backup.write_text(orig)

    marker = '    "crates/nsqasm-stamp-db",\n'
    if marker in s:
        insert = "".join(f'    "{m}",\n' for m in needed)
        s = s.replace(marker, marker + insert)
    else:
        end = s.index("]\n", s.index("members = ["))
        insert = "".join(f'    "{m}",\n' for m in needed)
        s = s[:end] + insert + s[end:]

    p.write_text(s)
    print("added workspace members:", ", ".join(needed))
else:
    print("workspace member patch not needed or crate dirs absent")
PY

log ""
log "=== 2. show workspace membership for nsq crates ==="
grep -n 'crates/nsq-\|crates/nsqasm' Cargo.toml | tee -a "$REPORT"

log ""
log "=== 3. Cargo metadata check ==="
run cargo metadata --no-deps --format-version 1

log ""
log "=== 4. rerun proof tests ==="
run cargo test -p Braxon-core council_ten -- --nocapture

if cargo metadata --no-deps --format-version 1 2>/dev/null | grep -q '"name":"nsq-wake"'; then
  run cargo test -p nsq-wake -- --nocapture
else
  log "SKIP: nsq-wake still not present in workspace metadata"
fi

if cargo metadata --no-deps --format-version 1 2>/dev/null | grep -q '"name":"nsqasm-stamp-db"'; then
  run cargo test -p nsqasm-stamp-db -- --nocapture
else
  log "SKIP: nsqasm-stamp-db not present in workspace metadata"
fi

log ""
log "=== 5. build root binary ==="
run cargo build --release

BIN="$ROOT/target/release/Braxon"
if [ ! -x "$BIN" ]; then
  BIN="$ROOT/target/release/braxon"
fi

log ""
log "=== 6. offline actionable commands ==="
if [ -x "$BIN" ]; then
  run "$BIN" status
  run "$BIN" wake
  run "$BIN" apps verify
  run "$BIN" runtime registry
else
  log "ERROR: release binary not found"
  exit 9
fi

log ""
log "=== RESULT LABEL ==="
log "Citadel699_rebuild=already_passed_in_prior_run"
log "route_gates=already_passed_in_prior_run"
log "council_ten_wake=verified"
log "workspace_nsq_wake_membership=repaired_if_crate_present"
log "offline_actionable_demo_complete_if_commands_above_pass=true"
log "hot_live_claim=false_until_model_route_execution_proof"
log "report=$REPORT"

cp "$REPORT" "$DOWNLOAD_COPY" 2>/dev/null || true
log "download_copy=$DOWNLOAD_COPY"
