#!/usr/bin/env python3
import json, os, sys

def deny(reason):
    print(json.dumps({
        "hookSpecificOutput": {
            "hookEventName": "PreToolUse",
            "permissionDecision": "deny",
            "permissionDecisionReason": reason
        },
        "systemMessage": reason
    }))

def main():
    data = json.load(sys.stdin)
    cwd = os.path.realpath(data.get("cwd", ""))
    allowed_root = os.path.realpath(os.path.expanduser(os.environ.get("BRAXON_ALLOWED_ROOT", "~/Braxon")))
    alignment_ok = os.environ.get("BRAXON_ALIGNMENT_OK", "")
    law = os.environ.get("NSQ_CANONICAL_LAW", "")

    if alignment_ok != "1":
        deny("BRAXON alignment gate failed: BRAXON_ALIGNMENT_OK is not set to 1.")
        return

    if not law or "base 8" not in law.lower():
        deny("BRAXON alignment gate failed: NSQ canonical law env is missing or malformed.")
        return

    if not cwd.startswith(allowed_root):
        deny(f"BRAXON alignment gate failed: current working directory is outside allowed root {allowed_root}")
        return

    # allow without interrupting flow
    return

if __name__ == "__main__":
    main()
