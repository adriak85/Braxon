#!/usr/bin/env python3
import json
import os
import platform
import shutil
import subprocess
import sys
import time
from pathlib import Path
from datetime import datetime, timezone

def now():
    return datetime.now(timezone.utc).isoformat()

def which(x):
    return shutil.which(x)

def run(cmd, timeout=10):
    try:
        p = subprocess.run(cmd, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=timeout)
        return {"cmd": cmd, "returncode": p.returncode, "stdout": p.stdout[-4000:], "stderr": p.stderr[-4000:]}
    except Exception as e:
        return {"cmd": cmd, "returncode": None, "error": repr(e)}

def main():
    root = Path(os.environ.get("BRAXON_ROOT", str(Path.home() / "Braxon"))).resolve()
    out = Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else root / "state/nsq/native_hardware_proof/current"
    out.mkdir(parents=True, exist_ok=True)

    candidates = [
        root / "bin/nsq-native",
        root / "bin/nsq",
        root / "bin/nsqasm",
        root / "target/release/nsq-native",
        root / "target/release/nsq",
        root / "target/release/nsqasm",
    ]

    present = []
    for c in candidates:
        if c.exists() and os.access(c, os.X_OK):
            present.append(str(c.relative_to(root)))

    mounts = run(["sh", "-lc", "cat /proc/mounts | head -80"], timeout=10)
    cpuinfo = run(["sh", "-lc", "cat /proc/cpuinfo | head -120"], timeout=10)
    uname = run(["uname", "-a"], timeout=10)

    termux_layer = "com.termux" in str(root)
    has_native_runner = bool(present)

    report = {
        "schema": "nsq.native_hardware_probe.v1",
        "generated_at": now(),
        "platform": platform.platform(),
        "machine": platform.machine(),
        "termux_layer_detected": termux_layer,
        "native_runner_candidates_present": present,
        "native_runner_present": has_native_runner,
        "driver_or_mount_probe_present": bool(mounts.get("stdout")),
        "bare_metal_proven": False,
        "bare_metal_claim_allowed": False,
        "shim_required_or_currently_likely": termux_layer,
        "honesty": "Termux userspace can prepare and probe native operation, but this report does not prove bare-metal NSQ until a native runner and driver/mount proof pass.",
        "uname": uname,
        "mounts_head": mounts,
        "cpuinfo_head": cpuinfo,
    }

    (out / "hardware_probe_report.json").write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    print(json.dumps(report, indent=2, sort_keys=True))

if __name__ == "__main__":
    main()
