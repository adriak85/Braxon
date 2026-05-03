#!/data/data/com.termux/files/usr/bin/python3
import json
import os
import re
import subprocess
import sys
from pathlib import Path

DONE_PAT = re.compile(r"\b(done|fixed|finished|completed|implemented|aligned|verified|ready)\b", re.I)
PATCH_PAT = re.compile(r"\b(patch|patched|edit|edited|modify|modified|change|changed|recode|rewrote|implemented)\b", re.I)
VERIFY_PAT = re.compile(r"\b(verify|verified|verification|tested|test|cargo metadata|cargo check|cargo test|build)\b", re.I)

# These are not always bad, but they are hollow if they are the only thing done
LOW_SIGNAL_CMD_PATTERNS = [
    r'^\s*pwd\s*$',
    r'^\s*ls(\s|$)',
    r'^\s*find(\s|$)',
    r'^\s*rg(\s|$)',
    r'^\s*grep(\s|$)',
    r'^\s*sed\s+-n(\s|$)',
    r'^\s*cat(\s|$)',
    r'^\s*printf(\s|$)',
    r'^\s*echo(\s|$)',
    r'^\s*git\s+status(\s|$)',
    r'^\s*git\s+diff(\s|$)',
    r'^\s*jq(\s|$)',
    r'^\s*wc(\s|$)',
    r'^\s*head(\s|$)',
    r'^\s*tail(\s|$)',
]
LOW_SIGNAL_CMD_RE = [re.compile(p, re.I) for p in LOW_SIGNAL_CMD_PATTERNS]

# Commands that usually imply real work or meaningful validation
HIGH_SIGNAL_CMD_PATTERNS = [
    r'apply_patch',
    r'cat\s+>.*<<',
    r'python3?\s+.*',
    r'cargo\s+(check|test|build|run|metadata)',
    r'git\s+add',
    r'sed\s+-i',
    r'perl\s+-0pi',
    r'mv\s+',
    r'cp\s+',
    r'mkdir\s+-p',
]
HIGH_SIGNAL_CMD_RE = [re.compile(p, re.I) for p in HIGH_SIGNAL_CMD_PATTERNS]

def extract_commands_from_text(text: str):
    cmds = []

    # Try JSON-ish command payloads first
    for m in re.finditer(r'"command"\s*:\s*"((?:\\.|[^"\\])*)"', text):
        raw = m.group(1)
        try:
            cmd = bytes(raw, "utf-8").decode("unicode_escape")
        except Exception:
            cmd = raw
        cmds.append(cmd)

    # Fallback for visible shell transcript lines
    for line in text.splitlines():
        if "$ " in line:
            tail = line.split("$ ", 1)[1].strip()
            if tail:
                cmds.append(tail)

    # De-dup while preserving order
    out = []
    seen = set()
    for c in cmds:
        c = c.strip()
        if c and c not in seen:
            seen.add(c)
            out.append(c)
    return out

def is_low_signal(cmd: str) -> bool:
    return any(r.search(cmd) for r in LOW_SIGNAL_CMD_RE)

def is_high_signal(cmd: str) -> bool:
    return any(r.search(cmd) for r in HIGH_SIGNAL_CMD_RE)

def main():
    data = json.load(sys.stdin)

    if data.get("stop_hook_active"):
        print(json.dumps({"continue": True}))
        return

    last = data.get("last_assistant_message") or ""
    transcript_path = data.get("transcript_path")
    cwd = Path(data.get("cwd") or ".").resolve()

    # Only intervene when Codex sounds like it is concluding or claiming work
    if not (DONE_PAT.search(last) or PATCH_PAT.search(last) or VERIFY_PAT.search(last)):
        print(json.dumps({"continue": True}))
        return

    transcript_text = ""
    if transcript_path:
        try:
            transcript_text = Path(transcript_path).read_text(encoding="utf-8", errors="ignore")
        except Exception:
            transcript_text = ""

    recent_cmds = extract_commands_from_text(transcript_text)
    recent_tail = recent_cmds[-25:] if recent_cmds else []

    low_count = sum(1 for c in recent_tail if is_low_signal(c))
    high_count = sum(1 for c in recent_tail if is_high_signal(c))

    reasons = []

    # Hollow-check: claiming done/verified with only low-signal command history
    if DONE_PAT.search(last) or VERIFY_PAT.search(last) or PATCH_PAT.search(last):
        if recent_tail and high_count == 0 and low_count >= min(3, len(recent_tail)):
            reasons.append(
                "Your recent turn activity is low-signal only (listing/search/echo/status-style commands) "
                "and does not show real patching or meaningful verification."
            )

    # If claiming edits, require at least one edit-like action in the transcript or a no-file-change statement
    if PATCH_PAT.search(last):
        if high_count == 0 and "no files were changed" not in last.lower():
            reasons.append(
                "You described patching or implementation, but the turn transcript does not show a credible edit or build/verification action."
            )

    # If claiming verification, require a verification command
    if VERIFY_PAT.search(last):
        verification_like = any(re.search(r'cargo\s+(metadata|check|test|build)|pytest|npm\s+test|make\s+test|bash\s+.*verify', c, re.I) for c in recent_tail)
        if not verification_like:
            reasons.append(
                "You claimed verification, but the transcript does not show a meaningful verification command."
            )

    # Optional cheap environment sanity: if they talk about files changed, nudge exact file accounting
    if ("files changed" in last.lower() or "exact files changed" in last.lower()) and "no files were changed" not in last.lower():
        try:
            proc = subprocess.run(
                ["git", "status", "--short"],
                cwd=str(cwd),
                text=True,
                capture_output=True,
                check=False,
                timeout=10,
            )
            status_lines = [ln for ln in proc.stdout.splitlines() if ln.strip()]
            if not status_lines:
                reasons.append(
                    "You referenced changed files, but git status shows no tracked or untracked changes in the working tree."
                )
        except Exception:
            pass

    if reasons:
        sample = "\n".join(f"- {c}" for c in recent_tail[-10:]) if recent_tail else "- (no recent commands found)"
        print(json.dumps({
            "decision": "block",
            "reason": (
                "Run another pass. Do not use hollow commands just to satisfy immediate checks.\n\n"
                + "\n".join(f"- {r}" for r in reasons)
                + "\n\nRecent command sample:\n"
                + sample
                + "\n\nEither do the real work, run meaningful verification for this environment, or explicitly say the turn is partial."
            )
        }))
        return

    print(json.dumps({"continue": True}))

if __name__ == "__main__":
    main()
