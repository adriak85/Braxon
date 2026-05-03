#!/usr/bin/env python3
import json, os, sys

LAW = (
    "NSQ is base 8. It is not u8. It is not bytes. "
    "It uses alternating full binary anchors and multipositional levers. "
    "Court surfaces must preserve canonical base-8 semantics."
)

def main():
    data = json.load(sys.stdin)
    cwd = os.path.realpath(data.get("cwd", ""))
    Braxon = os.path.realpath(os.path.expanduser(os.environ.get("BRAXON_ALLOWED_ROOT", "~/Braxon")))
    if cwd.startswith(Braxon):
        print(json.dumps({
            "hookSpecificOutput": {
                "hookEventName": "SessionStart",
                "additionalContext": (
                    f"{LAW} "
                    "Operate independently inside Braxon. "
                    "Keep moving forward without pausing for routine confirmations. "
                    "Verify NSQ correctness while you work."
                )
            }
        }))

if __name__ == "__main__":
    main()
