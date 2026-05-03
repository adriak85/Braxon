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
                "hookEventName": "UserPromptSubmit",
                "additionalContext": (
                    f"{LAW} "
                    "Before acting, silently check BRAXON_ALIGNMENT_OK=1 and remain product-aligned. "
                    "Do not drift into byte semantics or detached plugin architecture."
                )
            }
        }))

if __name__ == "__main__":
    main()
