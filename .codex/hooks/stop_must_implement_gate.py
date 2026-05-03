#!/data/data/com.termux/files/usr/bin/python3
import json
import os
import re
import subprocess
import sys
from pathlib import Path

HOME = Path.home()
BRAXON_DIR = Path(os.environ.get("BRAXON_ALLOWED_ROOT", str(HOME / "Braxon")))
MISSION_PATH = BRAXON_DIR / ".codex" / "current_mission.json"

DONE_PAT = re.compile(r"\b(done|fixed|finished|completed|ready|resolved|implemented|aligned)\b", re.I)
SOFT_WRAP_PAT = re.compile(r"\b(audit complete|classification complete|deliverables stand unchanged|no structural patching was performed|no files were changed)\b", re.I)
BLOCKER_PAT = re.compile(r"\b(hard blocker|blocked by|cannot proceed because|waiting on user|deferred pending user)\b", re.I)

def extract_commands(text: str):
    cmds = []
    for m in re.finditer(r'"command"\s*:\s*"((?:\\.|[^"\\])*)"', text):
        raw = m.group(1)
        try:
            cmd = bytes(raw, "utf-8").decode("unicode_escape")
        except Exception:
            cmd = raw
        cmds.append(cmd)

    for line in text.splitlines():
        if "$ " in line:
            tail = line.split("$ ", 1)[1].strip()
            if tail:
                cmds.append(tail)

    out = []
    seen = set()
    for c in cmds:
        c = c.strip()
        if c and c not in seen:
            seen.add(c)
            out.append(c)
    return out

def high_signal_impl(cmd: str) -> bool:
    pats = [
        r'apply_patch',
        r'cat\s+>.*<<',
        r'sed\s+-i',
        r'perl\s+-0pi',
        r'python3?\s+.*',
        r'cargo\s+(check|test|build|run)',
        r'git\s+add',
        r'cp\s+',
        r'mv\s+',
        r'mkdir\s+-p'
    ]
    return any(re.search(p, cmd, re.I) for p in pats)

def meaningful_verify(cmd: str) -> bool:
    pats = [
        r'cargo\s+(metadata|check|test|build)',
        r'pytest',
        r'npm\s+test',
        r'make(\s+test|\s+check)?',
        r'bash\s+.*verify',
        r'rg\s+.*(TODO|FIXME|TBD|WIP|UNIMPLEMENTED|PLACEHOLDER)'
    ]
    return any(re.search(p, cmd, re.I) for p in pats)

def read_transcript(path_str: str) -> str:
    if not path_str:
        return ""
    p = Path(path_str)
    try:
        return p.read_text(encoding="utf-8", errors="ignore")
    except Exception:
        return ""

def git_changes_present(cwd: Path) -> bool:
    try:
        proc = subprocess.run(
            ["git", "status", "--short"],
            cwd=str(cwd),
            text=True,
            capture_output=True,
            check=False,
            timeout=10
        )
        return bool(proc.stdout.strip())
    except Exception:
        return False

def main():
    data = json.load(sys.stdin)

    if data.get("stop_hook_active"):
        print(json.dumps({"continue": True}))
        return

    last = data.get("last_assistant_message") or ""
    lower = last.lower()
    cwd = Path(data.get("cwd") or BRAXON_DIR).resolve()
    transcript = read_transcript(data.get("transcript_path"))
    cmds = extract_commands(transcript)
    recent = cmds[-40:] if cmds else []

    if not MISSION_PATH.exists():
        print(json.dumps({"continue": True}))
        return

    mission = json.loads(MISSION_PATH.read_text(encoding="utf-8"))
    req = mission["completion_requirements"]

    # Only intervene when the model appears to be wrapping up or reporting milestone completion.
    if not (DONE_PAT.search(last) or SOFT_WRAP_PAT.search(last)):
        print(json.dumps({"continue": True}))
        return

    if BLOCKER_PAT.search(last):
        print(json.dumps({"continue": True}))
        return

    reasons = []

    has_classification = ("classification table" in lower) or ("1. classification table" in lower)
    has_coverage = ("coverage evaluation" in lower) or ("nsq-coverage evaluation" in lower) or ("nsq coverage evaluation" in lower)
    has_target_map = ("target-state architecture map" in lower) or ("target state architecture map" in lower) or ("target-state workspace map" in lower) or ("target workspace map" in lower)
    has_exclusion = ("explicit exclusion list" in lower) or ("exclusion list" in lower)
    has_recode_plan = ("targeted recode plan" in lower) or ("recode plan" in lower) or ("implementation plan" in lower)
    has_verification_text = ("focused verification" in lower) or ("verification" in lower) or ("verified" in lower)
    mentions_no_patch = ("no structural patching was performed" in lower) or ("no files were changed" in lower)

    impl_cmds = [c for c in recent if high_signal_impl(c)]
    verify_cmds = [c for c in recent if meaningful_verify(c)]

    if req.get("must_have_classification") and not has_classification:
        reasons.append("Missing crate/subsystem classification output.")
    if req.get("must_have_nsq_coverage_evaluation") and not has_coverage:
        reasons.append("Missing NSQ-coverage evaluation.")
    if req.get("must_have_target_state_map") and not has_target_map:
        reasons.append("Missing target-state architecture/workspace map.")
    if req.get("must_have_exclusion_list") and not has_exclusion:
        reasons.append("Missing explicit exclusion list.")
    if req.get("must_have_targeted_recode_plan") and not has_recode_plan:
        reasons.append("Missing targeted recode/implementation plan.")

    if req.get("must_have_actual_implementation_work"):
        if mentions_no_patch:
            reasons.append("Audit/classification alone is not completion. Implementation has not begun.")
        elif not impl_cmds:
            reasons.append("No credible implementation action appears in the recent turn transcript.")

    if req.get("must_have_focused_verification"):
        if not has_verification_text:
            reasons.append("Missing focused verification summary.")
        elif not verify_cmds:
            reasons.append("Verification was claimed, but no meaningful verification command appears in the recent turn transcript.")

    if req.get("must_have_actual_implementation_work") and not git_changes_present(cwd):
        reasons.append("Working tree does not currently show repo changes. Do real implementation work or explicitly state a hard blocker.")

    if reasons:
        sample = "\n".join(f"- {c}" for c in recent[-12:]) if recent else "- (no recent commands found)"
        print(json.dumps({
            "decision": "block",
            "reason": (
                "You are not done. Audit is phase 1 only.\n\n"
                "Required sequence: evaluate -> classify -> target-map -> initiate implementation -> continue implementation -> verify -> only then stop.\n\n"
                + "\n".join(f"- {r}" for r in reasons)
                + "\n\nRecent command sample:\n"
                + sample
                + "\n\nContinue with real implementation work. Do not stop at audit/classification."
            )
        }))
        return

    print(json.dumps({"continue": True}))

if __name__ == "__main__":
    main()
