#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="${1:-$HOME/Braxon}"
cd "$ROOT"

CFG="config/toolchains/braxon_terminal_provenance.json"
DOC="docs/braxon/BRAXON_TERMINAL_PROVENANCE.md"

echo "== verify Braxon terminal provenance =="
echo "root=$ROOT"

test -s "$CFG"
test -s "$DOC"

python3 - "$CFG" <<'PY'
import json, sys

p = sys.argv[1]
data = json.load(open(p, "r", encoding="utf-8"))

assert data["schema"] == "braxon.terminal.provenance.v1"
assert data["identity"] == "Braxon"
assert data["authority"] == "NSQ_COURT"
assert data["nsq_is_bus"] is True
assert data["court_is_compositor"] is True

build = data["absorbed_build_surfaces"]
term = data["absorbed_terminal_surfaces"]
sem = data["semantic_evidence"]
boundary = data["execution_boundary"]

assert build["Alien::Build"] is True
assert build["cargo"] is True
assert build["rustc"] is True

assert term["IPC::Run"] is True
assert term["IO::Interactive"] is True
assert term["Complete::Bash"] is True
assert term["Encode::Locale"] is True
assert term["FFI::CheckLib"] is True

assert sem["alienfile_and_alien_build_root_lines_present"] is True
assert sem["pseudo_terminal_lines_present"] is True
assert sem["terminal_color_lines_present"] is True
assert sem["terminal_width_height_lines_present"] is True

assert boundary["absorbed_source_is_runtime_execution_proof"] is False
assert boundary["translated_nsq_body_is_hot_live_proof"] is False
assert boundary["terminal_wrapper_install_may_use_this_as_guidance"] is True
assert boundary["hot_live_requires_wake_dispatch_or_runtime_route_proof"] is True

print("PASS: terminal provenance is coherent")
PY

grep -q 'absorbed Alien::Build' "$DOC" || grep -q 'Alien::Build' "$DOC"
grep -q 'does not by itself prove hot-live autonomous execution' "$DOC"
grep -q 'Do not pull, reset, force-push' "$DOC"

echo "PASS: terminal provenance doc carries the boundary"
echo "PASS: Braxon terminal provenance verified"
