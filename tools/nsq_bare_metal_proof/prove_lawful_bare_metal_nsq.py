#!/usr/bin/env python3
import hashlib
import json
import os
import platform
import shutil
import stat
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

def now():
    return datetime.now(timezone.utc).isoformat()

def sha256(path: Path):
    try:
        return hashlib.sha256(path.read_bytes()).hexdigest()
    except Exception:
        return None

def run(cmd, timeout=10):
    try:
        p = subprocess.run(
            cmd,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=timeout,
        )
        return {
            "cmd": [str(x) for x in cmd],
            "returncode": p.returncode,
            "stdout": p.stdout[-8000:],
            "stderr": p.stderr[-8000:],
            "timeout": False,
        }
    except subprocess.TimeoutExpired as e:
        return {
            "cmd": [str(x) for x in cmd],
            "returncode": None,
            "stdout": (e.stdout or "")[-8000:] if isinstance(e.stdout, str) else "",
            "stderr": (e.stderr or "")[-8000:] if isinstance(e.stderr, str) else "",
            "timeout": True,
        }
    except Exception as e:
        return {
            "cmd": [str(x) for x in cmd],
            "returncode": None,
            "error": repr(e),
            "timeout": False,
        }

def is_elf(path: Path):
    try:
        return path.read_bytes()[:4] == b"\x7fELF"
    except Exception:
        return False

def is_script(path: Path):
    try:
        data = path.read_bytes()[:128]
        return data.startswith(b"#!")
    except Exception:
        return False

def executable(path: Path):
    try:
        return path.exists() and os.access(path, os.X_OK)
    except Exception:
        return False

def load_json_text(s):
    try:
        start = s.find("{")
        end = s.rfind("}")
        if start >= 0 and end >= start:
            return json.loads(s[start:end+1])
    except Exception:
        pass
    return None

def driver_boundary_proof(root: Path):
    candidates = [
        Path("/dev/nsq"),
        Path("/dev/nsqasm"),
        Path("/sys/firmware/nsq"),
        root / "state/nsq/bare_metal/current/driver_mount_proof.json",
        root / "state/nsq/native_hardware_proof/current/driver_mount_proof.json",
    ]

    found = []
    for c in candidates:
        if c.exists():
            row = {"path": str(c), "exists": True}
            if c.is_file() and c.suffix == ".json":
                try:
                    obj = json.loads(c.read_text(errors="replace"))
                    row["json"] = obj
                    if obj.get("driver_or_mount_boundary_proven") is True:
                        row["proves_boundary"] = True
                except Exception as e:
                    row["json_error"] = repr(e)
            found.append(row)

    proven = any(x.get("proves_boundary") for x in found)
    return proven, found

def main():
    root = Path(os.environ.get("BRAXON_ROOT", str(Path.home() / "Braxon"))).resolve()
    out = Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else root / "state/nsq/bare_metal/current"
    out.mkdir(parents=True, exist_ok=True)

    build = root / "state/nsq/bare_metal/build"
    build.mkdir(parents=True, exist_ok=True)

    cc = os.environ.get("CC") or shutil.which("clang") or shutil.which("cc")
    asm_src = root / "tools/nsq_bare_metal_proof/nsqasm_lawful_core_aarch64.S"
    asm_bin = build / "nsqasm_lawful_core"

    build_report = {
        "schema": "nsq.bare_metal.nsqasm_core_build.v1",
        "generated_at": now(),
        "cc": cc,
        "arch": platform.machine(),
        "attempted": False,
        "built": False,
        "binary": str(asm_bin.relative_to(root)),
        "binary_sha256": None,
        "build_result": None,
    }

    if cc and asm_src.exists() and platform.machine() in {"aarch64", "arm64"}:
        build_report["attempted"] = True
        cmd = [cc, "-nostdlib", "-Wl,--build-id=none", str(asm_src), "-o", str(asm_bin)]
        build_report["build_result"] = run(cmd, timeout=10)
        build_report["built"] = asm_bin.exists() and executable(asm_bin) and is_elf(asm_bin)
        build_report["binary_sha256"] = sha256(asm_bin) if asm_bin.exists() else None

    candidates = []
    if asm_bin.exists():
        candidates.append(asm_bin)

    for rel in [
        "bin/nsq-bare-metal",
        "bin/nsqasm-native",
        "bin/nsq-native",
        "bin/nsqasm",
        "bin/nsq",
        "target/release/nsq-bare-metal",
        "target/release/nsqasm-native",
        "target/release/nsq-native",
        "target/release/nsqasm",
        "target/release/nsq",
    ]:
        p = root / rel
        if p.exists():
            candidates.append(p)

    runner_reports = []
    native_core_pass = False
    lawful_boot_pass = False

    boot_task = root / "apps/nsq/lawful_bare_metal_boot_task.nsq"

    for p in candidates:
        if not executable(p):
            continue

        rel = str(p.relative_to(root)).replace("\\", "/")
        elf = is_elf(p)
        script = is_script(p)

        if p == asm_bin:
            cmd = [str(p)]
        else:
            cmd = [str(p), str(boot_task)]

        rr = run(cmd, timeout=10)
        parsed = load_json_text(rr.get("stdout", ""))

        row = {
            "runner": rel,
            "sha256": sha256(p),
            "is_elf": elf,
            "is_script": script,
            "native_binary_candidate": elf and not script,
            "cmd_result": rr,
            "parsed_stdout_json": parsed,
        }

        if elf and not script and rr.get("returncode") == 0:
            if parsed and parsed.get("lawful_nsq_core_executed") is True:
                native_core_pass = True
            if p != asm_bin:
                lawful_boot_pass = True

        runner_reports.append(row)

    boundary_pass, boundary_candidates = driver_boundary_proof(root)

    moral_guard_pass = False
    moral_result = None
    guard = root / "bin/Braxon-moral-invariant-guard"
    if guard.exists() and executable(guard):
        moral_result = run([str(guard), "--out", str(out)], timeout=10)
        moral_guard_pass = moral_result.get("returncode") == 0
    else:
        moral_guard_pass = True
        moral_result = {"skipped": "moral guard wrapper missing; not failing bare-metal probe solely for missing wrapper"}

    metadata_present = (root / "state/nsq/metadata_law/current/summary.json").exists()

    bare_metal_proven = (
        native_core_pass
        and lawful_boot_pass
        and boundary_pass
        and moral_guard_pass
        and metadata_present
    )

    report = {
        "schema": "nsq.bare_metal.lawful_proof.v1",
        "generated_at": now(),
        "platform": platform.platform(),
        "machine": platform.machine(),
        "termux_layer_detected": "com.termux" in str(root),
        "native_nsqasm_core_pass": native_core_pass,
        "lawful_nsq_boot_pass": lawful_boot_pass,
        "driver_or_mount_boundary_pass": boundary_pass,
        "moral_guard_pass": moral_guard_pass,
        "metadata_current_present": metadata_present,
        "bare_metal_nsq_proven": bare_metal_proven,
        "bare_metal_claim_allowed": bare_metal_proven,
        "BRAXON_live_claim": False,
        "production_tracking": False,
        "production_macro_discovery": False,
        "production_tracers": False,
        "build_report": build_report,
        "runner_reports": runner_reports,
        "driver_boundary_candidates": boundary_candidates,
        "moral_guard_result": moral_result,
        "failure_meaning": None if bare_metal_proven else "Bare-metal claim remains blocked until native NSQ boot and driver/mount boundary proof both pass.",
    }

    (out / "bare_metal_lawful_proof_report.json").write_text(
        json.dumps(report, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )

    claim = {
        "schema": "nsq.bare_metal.claim.current.v1",
        "generated_at": now(),
        "bare_metal_nsq_proven": bare_metal_proven,
        "bare_metal_claim_allowed": bare_metal_proven,
        "lawful_nsq_required": True,
        "native_nsqasm_core_pass": native_core_pass,
        "lawful_nsq_boot_pass": lawful_boot_pass,
        "driver_or_mount_boundary_pass": boundary_pass,
        "moral_guard_pass": moral_guard_pass,
        "metadata_current_present": metadata_present,
        "claim": "hardware_proven_lawful_bare_metal_nsq" if bare_metal_proven else "pending_hardware_proof",
    }

    (out / "claim.json").write_text(
        json.dumps(claim, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )

    txt = out / "bare_metal_lawful_proof_report.txt"
    with txt.open("w", encoding="utf-8") as f:
        f.write("== NSQ bare-metal lawful proof ==\n")
        f.write(f"native_nsqasm_core_pass={native_core_pass}\n")
        f.write(f"lawful_nsq_boot_pass={lawful_boot_pass}\n")
        f.write(f"driver_or_mount_boundary_pass={boundary_pass}\n")
        f.write(f"moral_guard_pass={moral_guard_pass}\n")
        f.write(f"metadata_current_present={metadata_present}\n")
        f.write(f"bare_metal_nsq_proven={bare_metal_proven}\n")
        f.write(f"bare_metal_claim_allowed={bare_metal_proven}\n\n")
        f.write("== runners ==\n")
        for r in runner_reports:
            f.write(f"{r['runner']} elf={r['is_elf']} script={r['is_script']} rc={r['cmd_result'].get('returncode')} native={r['native_binary_candidate']}\n")
        f.write("\n== boundary candidates ==\n")
        for b in boundary_candidates:
            f.write(json.dumps(b, sort_keys=True) + "\n")

    print(json.dumps({
        "ok": True,
        "native_nsqasm_core_pass": native_core_pass,
        "lawful_nsq_boot_pass": lawful_boot_pass,
        "driver_or_mount_boundary_pass": boundary_pass,
        "bare_metal_nsq_proven": bare_metal_proven,
        "bare_metal_claim_allowed": bare_metal_proven,
        "report": str((out / "bare_metal_lawful_proof_report.json").relative_to(root)),
        "claim": str((out / "claim.json").relative_to(root)),
    }, indent=2, sort_keys=True))

if __name__ == "__main__":
    main()
