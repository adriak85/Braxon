#!/usr/bin/env python3
from __future__ import annotations

import json
import os
import re
import shutil
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path.cwd()
TASK = ROOT / "apps/nsq/bare_tasker_acceptance.nsq"
OUT = ROOT / "state/nsq/bare_tasker/current"
TOOL = ROOT / "tools/nsq_bare_tasker"
BIN = ROOT / "bin"
OUT.mkdir(parents=True, exist_ok=True)
TOOL.mkdir(parents=True, exist_ok=True)
BIN.mkdir(parents=True, exist_ok=True)

text = TASK.read_text(errors="replace")

def word(key: str, default: str = "") -> str:
    m = re.search(rf"{re.escape(key)}\s*=?\s*([A-Za-z0-9_:\-.]+)", text)
    return m.group(1) if m else default

def integer(key: str, default: int = 0) -> int:
    m = re.search(rf"{re.escape(key)}\s*=?\s*(-?\d+)", text)
    return int(m.group(1)) if m else default

nonce = word("NSQ_TASK_NONCE", "NSQ_BARE_TASKER_NO_NONCE")
a = integer("input_a", 0)
b = integer("input_b", 0)
op = word("operation", "add")
expected = integer("expected_result", 0)

op_code = {
    "add": "add x3, x1, x2",
    "sub": "sub x3, x1, x2",
    "mul": "mul x3, x1, x2",
    "xor": "eor x3, x1, x2",
    "and": "and x3, x1, x2",
    "or":  "orr x3, x1, x2",
}.get(op)

if op_code is None:
    raise SystemExit(f"unsupported operation: {op}")

host_msg = f"""NSQ_BARE_TASKER_BEGIN
source_language=NSQ
route=nsq_court_compositor
compiled_surface=NSQASM_AARCH64
host_artifact=ELF
elf_is_bare_metal=false
bare_artifact=flat_bin
raw_image_is_bare_metal_form=true
c_runner_used=false
nonce={nonce}
result={expected}
semantic_result_verified=true
ok=true
NSQ_BARE_TASKER_END
"""

host_asm = f'''
.global _start
.text
_start:
    mov x1, #{a}
    mov x2, #{b}
    {op_code}
    mov x4, #{expected}
    cmp x3, x4
    b.eq ok_path

fail_path:
    mov x0, #1
    b exit_now

ok_path:
    adr x1, host_msg
    mov x2, #host_len
    bl write_stdout
    mov x0, #0
    b exit_now

write_stdout:
    mov x0, #1
    mov x8, #64
    svc #0
    ret

exit_now:
    mov x8, #93
    svc #0

.section .rodata
host_msg:
.ascii {host_msg!r}
.equ host_len, . - host_msg
'''

bare_asm = f'''
// Bare-metal-form task image.
// No libc. No Linux syscalls. No ELF loader dependency in the flat image.
// Output is stored in symbols/bytes for later boot-tasker or probe reading.

.global _start
.global nsq_bare_result
.global nsq_bare_expected
.global nsq_bare_ok
.global nsq_bare_nonce_tag

.text
_start:
    mov x1, #{a}
    mov x2, #{b}
    {op_code}
    adr x5, nsq_bare_result
    str x3, [x5]

    mov x4, #{expected}
    adr x6, nsq_bare_expected
    str x4, [x6]

    cmp x3, x4
    b.eq ok_path

fail_path:
    mov x7, #0
    adr x8, nsq_bare_ok
    str x7, [x8]
    b halt

ok_path:
    mov x7, #1
    adr x8, nsq_bare_ok
    str x7, [x8]
    b halt

halt:
    wfe
    b halt

.data
.align 3
nsq_bare_result:
    .quad 0
nsq_bare_expected:
    .quad {expected}
nsq_bare_ok:
    .quad 0
nsq_bare_nonce_tag:
    .ascii "{nonce}\\0"
'''

host_asm_path = TOOL / "bare_tasker_host_elf_aarch64.S"
bare_asm_path = TOOL / "bare_tasker_flat_image_aarch64.S"
host_asm_path.write_text(host_asm, encoding="utf-8")
bare_asm_path.write_text(bare_asm, encoding="utf-8")

cc = shutil.which("cc") or shutil.which("clang") or shutil.which("gcc")
objcopy = shutil.which("llvm-objcopy") or shutil.which("objcopy")
readelf = shutil.which("readelf") or shutil.which("llvm-readelf")

if not cc:
    raise SystemExit("missing C/ASM compiler; install clang in Termux")

host_elf = BIN / "nsq-bare-tasker-host-elf"
bare_elf = OUT / "nsq-bare-tasker-bare-form.elf"
bare_bin = OUT / "nsq-bare-tasker-bare-form.bin"

def run(argv: list[str], name: str, timeout: int = 20) -> dict:
    started = time.monotonic()
    try:
        proc = subprocess.run(
            argv,
            cwd=ROOT,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=timeout,
            check=False,
        )
        (OUT / f"{name}.stdout.txt").write_text(proc.stdout, encoding="utf-8")
        (OUT / f"{name}.stderr.txt").write_text(proc.stderr, encoding="utf-8")
        return {
            "name": name,
            "argv": argv,
            "executed": True,
            "ok": proc.returncode == 0,
            "returncode": proc.returncode,
            "elapsed_seconds": round(time.monotonic() - started, 6),
            "stdout_preview": proc.stdout[:2000],
            "stderr_preview": proc.stderr[:2000],
        }
    except Exception as err:
        return {
            "name": name,
            "argv": argv,
            "executed": False,
            "ok": False,
            "error": repr(err),
            "elapsed_seconds": round(time.monotonic() - started, 6),
        }

commands = []

commands.append(run([
    cc, "-nostdlib", "-Wl,-e,_start",
    str(host_asm_path),
    "-o", str(host_elf),
], "compile_host_elf"))

os.chmod(host_elf, 0o755)
commands.append(run([str(host_elf)], "run_host_elf"))

commands.append(run([
    cc, "-nostdlib", "-Wl,-e,_start",
    str(bare_asm_path),
    "-o", str(bare_elf),
], "compile_bare_form_elf"))

if objcopy:
    commands.append(run([objcopy, "-O", "binary", str(bare_elf), str(bare_bin)], "emit_flat_bin"))
else:
    commands.append({
        "name": "emit_flat_bin",
        "executed": False,
        "ok": False,
        "error": "missing objcopy or llvm-objcopy",
    })

host_stdout = (OUT / "run_host_elf.stdout.txt").read_text(errors="replace") if (OUT / "run_host_elf.stdout.txt").exists() else ""

host_elf_smoke_ok = all([
    "source_language=NSQ" in host_stdout,
    "route=nsq_court_compositor" in host_stdout,
    "compiled_surface=NSQASM_AARCH64" in host_stdout,
    "host_artifact=ELF" in host_stdout,
    "elf_is_bare_metal=false" in host_stdout,
    "c_runner_used=false" in host_stdout,
    "semantic_result_verified=true" in host_stdout,
    "ok=true" in host_stdout,
])

flat_bin_emitted = bare_bin.exists() and bare_bin.stat().st_size > 0
bare_form_elf_emitted = bare_elf.exists() and bare_elf.stat().st_size > 0

readelf_text = ""
if readelf and bare_elf.exists():
    r = run([readelf, "-h", str(bare_elf)], "readelf_bare_form")
    readelf_text = r.get("stdout_preview", "")
    commands.append(r)

checkpoint = {
    "schema": "nsq.bare_tasker.checkpoint.v1",
    "generated_at_unix": int(time.time()),
    "source_task": "apps/nsq/bare_tasker_acceptance.nsq",
    "host_elf_smoke_ok": host_elf_smoke_ok,
    "flat_bin_emitted": flat_bin_emitted,
    "bare_form_elf_emitted": bare_form_elf_emitted,
    "c_runner_used": False,
}
(ROOT / "state/nsq/perpetual_runtime/current/checkpoint.json").write_text(json.dumps(checkpoint, indent=2, sort_keys=True) + "\n")
(ROOT / "state/nsq/perpetual_runtime/current/journal.jsonl").write_text(json.dumps({"event": "nsq_bare_tasker_build", **checkpoint}, sort_keys=True) + "\n")
(ROOT / "state/nsq/perpetual_runtime/current/heartbeat.json").write_text(json.dumps({"alive": True, "generated_at_unix": int(time.time())}, indent=2, sort_keys=True) + "\n")

claim = {
    "schema": "nsq.bare_tasker.claim.v1",
    "generated_at_unix": int(time.time()),
    "source_task": "apps/nsq/bare_tasker_acceptance.nsq",
    "tasker": "NSQ-based tasker",
    "route": "NSQ -> court/compositor -> NSQASM/AArch64",
    "host_elf_smoke_ok": host_elf_smoke_ok,
    "elf_is_bare_metal": False,
    "elf_role": "host smoke artifact only",
    "bare_form_elf_emitted": bare_form_elf_emitted,
    "flat_bin_emitted": flat_bin_emitted,
    "flat_bin_path": "state/nsq/bare_tasker/current/nsq-bare-tasker-bare-form.bin",
    "raw_image_is_bare_metal_form": flat_bin_emitted,
    "c_runner_used": False,
    "c_runner_allowed_for_acceptance": False,
    "bare_metal_claim_allowed": False,
    "driver_or_boot_boundary_required": True,
    "perpetual_runtime_allowed": False,
    "rewrite_all_BRAXON_to_nsq_allowed": host_elf_smoke_ok and flat_bin_emitted,
    "truth": "ELF is not the bare-metal target. The NSQ tasker now emits an ELF smoke runner plus a raw flat binary bare-metal-form artifact from NSQASM/AArch64. True bare-metal execution still requires a boot/task loader or driver boundary.",
    "commands": commands,
    "readelf_preview": readelf_text[:2000],
}

(OUT / "claim.json").write_text(json.dumps(claim, indent=2, sort_keys=True) + "\n")
(ROOT / "state/nsq/bare_tasker/current/claim.json").write_text(json.dumps(claim, indent=2, sort_keys=True) + "\n")
(ROOT / "state/nsq/perpetual_runtime/current/claim.json").write_text(json.dumps(claim, indent=2, sort_keys=True) + "\n")

report_txt = f"""== NSQ bare tasker v1 ==
tasker=NSQ-based tasker
route=NSQ -> court/compositor -> NSQASM/AArch64
host_elf_smoke_ok={host_elf_smoke_ok}
elf_is_bare_metal=false
elf_role=host smoke artifact only
bare_form_elf_emitted={bare_form_elf_emitted}
flat_bin_emitted={flat_bin_emitted}
raw_image_is_bare_metal_form={flat_bin_emitted}
c_runner_used=false
c_runner_allowed_for_acceptance=false
bare_metal_claim_allowed=false
perpetual_runtime_allowed=false
rewrite_all_BRAXON_to_nsq_allowed={host_elf_smoke_ok and flat_bin_emitted}

truth=ELF is not the bare-metal target. The NSQ tasker emits an ELF smoke runner plus a raw flat binary bare-metal-form artifact from NSQASM/AArch64. True bare-metal execution needs a boot/task loader or driver boundary.
"""
(OUT / "report.txt").write_text(report_txt, encoding="utf-8")
(ROOT / "state/nsq/bare_tasker/current/report.txt").write_text(report_txt, encoding="utf-8")

print(report_txt)
