#!/data/data/com.termux/files/usr/bin/python3
import json
import os
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(os.environ.get("BRAXON_ALLOWED_ROOT", str(Path.ho
me() / "Braxon")))


DONE_PAT = re.compile(
    r"\b(done|fixed|finished|completed|aligned|ready|resolved|implemented)\b",
    re.IGNORECASE,
)

LEFT_PAT = re.compile(
    r"\b(remaining|follow[- ]?up|todo|tbd|not finished|unfinished|next step|still needs)\b",
    re.IGNORECASE,
)

MARKER_PAT = re.compile(
    r"\b(TODO|FIXME|TBD|WIP|UNIMPLEMENTED|PLACEHOLDER)\b"
)

SCAN_DIRS = [
    "crates",
    "specs",
    "runtime",
    "config",
    "docs",
]

def run(cmd, cwd):
    try:
        out = subprocess.run(
            cmd,
            cwd=str(cwd),
            text=True,
            capture_output=True,
            timeout=20,
            check=False,
        )
        return out.returncode, out.stdout.strip(), out.stderr.strip()
    except Exception as e:
        return 1, "", str(e)

def main():
    data = json.load(sys.stdin)

    if data.get("stop_hook_active"):
        print(json.dumps({"continue": True}))
        return

    last = data.get("last_assistant_message") or ""
    lower = last.lower()

    # Only act when Codex sounds like it is concluding work.
    if not DONE_PAT.search(last):
        print(json.dumps({"continue": True}))
        return

    reasons = []

    # If the completion text itself admits remaining work, force another pass.
    if LEFT_PAT.search(last):
        reasons.append(
            "Your completion text still indicates remaining work. "
            "Do one more pass and either finish those items or clearly label the output as partial."
        )

    # Require completion hygiene: files changed + verification.
    has_files_changed = ("files changed" in lower) or ("exact files changed" in lower)
    has_verification = ("verification" in lower) or ("verified" in lower) or ("tests" in lower)
    if not has_files_changed or not has_verification:
        reasons.append(
            "Before concluding, provide exact files changed and focused verification."
        )

    # Light repo scan for obvious unfinished markers.
    hit_lines = []
    for rel in SCAN_DIRS:
        p = ROOT / rel
        if not p.exists():
            continue
        rc, out, _ = run(
            [
                "rg",
                "-n",
                r"\b(TODO|FIXME|TBD|WIP|UNIMPLEMENTED|PLACEHOLDER)\b",
                str(p),
            ],
            ROOT,
        )
        if out:
            hit_lines.extend(out.splitlines()[:10])

    if hit_lines:
        reasons.append(
            "Unfinished markers still exist in active repo surfaces:\n"
            + "\n".join(hit_lines[:10])
            + "\nResolve them, justify them, or explicitly mark them as deferred."
        )

    if reasons:
        print(json.dumps({
            "decision": "block",
            "reason": "Run one more completion pass.\n\n" + "\n\n".join(reasons)
        }))
        return

    print(json.dumps({"continue": True}))

if __name__ == "__main__":
    main()
