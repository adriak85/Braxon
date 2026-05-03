#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
DEBUG_BIN="$HOME/.cargo/target-cache/Braxon/debug/Braxon"

C0=$'\033[0m'
C1=$'\033[38;5;45m'
C2=$'\033[38;5;111m'
C3=$'\033[38;5;141m'
C4=$'\033[38;5;220m'
C5=$'\033[38;5;81m'
BD=$'\033[1m'

clear_screen() { printf '\033c'; }

banner() {
  printf '%s%s' "$BD" "$C1"
  cat <<'TXT'
╔══════════════════════════════════════════════════════════════════════╗
║                            B R A X O N                             ║
║                    commander space · first outfit                  ║
╚══════════════════════════════════════════════════════════════════════╝
TXT
  printf '%s' "$C0"
  printf '%sdeep teal%s · %sviolet%s · %sgold%s\n' "$C5" "$C0" "$C3" "$C0" "$C4" "$C0"
  echo
}

status_line() {
  local control="$ROOT/state/braxon/braxon_weight_ingest.control"
  local pipe="$ROOT/state/braxon/braxon_nsq_pipeline.status"
  local mode="unknown" reason="unknown" source="unknown" recode="unknown" runtime="unknown"

  [ -f "$control" ] && mode="$(awk -F= '/^mode=/{print $2}' "$control" | tail -n 1)"
  [ -f "$control" ] && reason="$(awk -F= '/^reason=/{print $2}' "$control" | tail -n 1)"
  [ -f "$pipe" ] && source="$(awk -F= '/^source_ingest_status=/{print $2}' "$pipe" | tail -n 1)"
  [ -f "$pipe" ] && recode="$(awk -F= '/^nsq_recode_status=/{print $2}' "$pipe" | tail -n 1)"
  [ -f "$pipe" ] && runtime="$(awk -F= '/^whole_core_runtime_status=/{print $2}' "$pipe" | tail -n 1)"

  printf '%sstatus%s mode=%s%s%s ingress=%s%s%s recode=%s%s%s runtime=%s%s%s reason=%s%s%s\n' \
    "$C4" "$C0" "$C2" "$mode" "$C0" "$C2" "$source" "$C0" "$C3" "$recode" "$C0" "$C5" "$runtime" "$C0" "$C1" "$reason" "$C0"
  echo
}

pause_it() {
  echo
  read -r -p "press enter to return > " _
}

truth_view() {
  clear_screen
  banner
  "$ROOT/scripts/braxon_truth_surface.sh" "$ROOT" || true
  pause_it
}

verify_view() {
  clear_screen
  banner
  if [ -x "$DEBUG_BIN" ]; then
    "$DEBUG_BIN" verify || true
    echo
    "$DEBUG_BIN" status || true
    echo
    "$DEBUG_BIN" coverage | sed -n '1,120p' || true
  else
    echo "missing debug binary: $DEBUG_BIN"
  fi
  pause_it
}

wizard_view() {
  clear_screen
  banner
  cat <<'TXT'
WHAT IS REAL NOW

1. Braxon has a real workspace and real native Rust surfaces.
2. The model lane is registered and bound as manifest/stamp truth.
3. The 26D / delta / supermodel layer is present as sealed reference structure.
4. Whole-core runtime inference is not live yet until recode + runtime load are real.
5. The safest current ingest posture is one manual single-flight worker, not a respawn loop.
TXT
  pause_it
}

start_ingest() {
  clear_screen
  banner
  BRAXON_ARIA2_MAX_CONNECTIONS=4 BRAXON_ARIA2_SPLIT=4 "$ROOT/scripts/braxon_manual_ingest_single.sh" "$ROOT" || true
  pause_it
}

stop_ingest() {
  clear_screen
  banner
  "$ROOT/scripts/braxon_stop_ingest_all.sh" "$ROOT" || true
  pause_it
}

tail_log() {
  clear_screen
  banner
  tail -n 120 "$ROOT/state/braxon/logs/braxon_weight_ingest.manual.log" 2>/dev/null || echo "no manual ingest log yet"
  pause_it
}

audit_nsq() {
  clear_screen
  banner
  "$ROOT/scripts/audit_nsq_commands_current.sh" "$ROOT" || true
  pause_it
}

launch_braxon() {
  clear_screen
  banner
  if [ -x "$DEBUG_BIN" ]; then
    "$DEBUG_BIN" || true
  else
    echo "missing debug binary: $DEBUG_BIN"
  fi
  pause_it
}

build_release() {
  clear_screen
  banner
  cd "$ROOT"
  cargo build -p Braxon --release
  pause_it
}

while true; do
  clear_screen
  banner
  status_line
  cat <<'TXT'
  1) truth surface
  2) verify / status / coverage
  3) wizard: what is real now
  4) start manual ingest (aria2c 4x4)
  5) stop all ingest
  6) tail ingest log
  7) audit nsq commands
  8) launch Braxon debug binary
  9) build Braxon release
 10) quit
TXT
  echo
  read -r -p "select > " choice
  case "$choice" in
    1) truth_view ;;
    2) verify_view ;;
    3) wizard_view ;;
    4) start_ingest ;;
    5) stop_ingest ;;
    6) tail_log ;;
    7) audit_nsq ;;
    8) launch_braxon ;;
    9) build_release ;;
    10|q|Q|exit) exit 0 ;;
    *) echo "unknown choice"; sleep 1 ;;
  esac
done
