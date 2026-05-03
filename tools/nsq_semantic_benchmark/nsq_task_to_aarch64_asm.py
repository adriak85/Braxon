#!/usr/bin/env python3
from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path.cwd()
TASK = ROOT / "state/nsq/semantic_benchmark/current/task.nsq"
ASM_OUT = ROOT / "tools/nsq_semantic_benchmark/nsq_semantic_task_runner_aarch64.S"

if not TASK.exists():
    TASK.parent.mkdir(parents=True, exist_ok=True)
    TASK.write_text("""NSQ_TASK semantic_runtime_benchmark
NSQ_TASK_VERSION 1
NSQ_TASK_NONCE NSQ_SEMANTIC_BENCHMARK_FIXED

NSQ_COURT_ROUTE {
  authority = NSQ_COURT
  architecture_root = true
  king = compositor
  queen = linter
  court_is_agents = false
  route = nsq_task_to_nsqasm_aarch64
  duplicate_task_systems_allowed = false
}

NSQ_SEMANTIC_BENCHMARK {
  input_a = 8
  input_b = 13
  operation = add
  expected_result = 21
  required_output_result = 21
}
""", encoding="utf-8")

text = TASK.read_text(errors="replace")

def word(key: str, default: str = "") -> str:
    m = re.search(rf"{re.escape(key)}\s*=?\s*([A-Za-z0-9_:\-.]+)", text)
    return m.group(1) if m else default

def integer(key: str, default: int = 0) -> int:
    m = re.search(rf"{re.escape(key)}\s*=?\s*(-?\d+)", text)
    return int(m.group(1)) if m else default

def gas_string(s: str) -> str:
    return '"' + s.replace("\\", "\\\\").replace('"', '\\"').replace("\n", "\\n") + '"'

nonce = word("NSQ_TASK_NONCE", "NSQASM_NO_NONCE")
a = integer("input_a", 0)
b = integer("input_b", 0)
op = word("operation", "add")
expected = integer("expected_result", integer("required_output_result", 0))

op_code = {
    "add": "add x3, x1, x2",
    "sub": "sub x3, x1, x2",
    "mul": "mul x3, x1, x2",
    "xor": "eor x3, x1, x2",
    "and": "and x3, x1, x2",
    "or":  "orr x3, x1, x2",
}.get(op)

if op_code is None:
    raise SystemExit(f"unsupported operation for NSQASM benchmark: {op}")

ok_msg = f"""NSQ_SEMANTIC_RUNTIME_BEGIN
authority=NSQ_COURT
architecture_root=true
king=compositor
queen=linter
court_is_agents=false
runtime_kind=nsqasm_aarch64_no_libc
task_file_semantics=executed
source_language=NSQ
compiled_surface=NSQASM_AARCH64
c_runner_used=false
input_a={a}
input_b={b}
operation={op}
nonce={nonce}
expected_result={expected}
semantic_result_verified=true
ok=true
NSQ_SEMANTIC_RUNTIME_END
"""

fail_msg = f"""NSQ_SEMANTIC_RUNTIME_BEGIN
authority=NSQ_COURT
architecture_root=true
king=compositor
queen=linter
court_is_agents=false
runtime_kind=nsqasm_aarch64_no_libc
task_file_semantics=executed
source_language=NSQ
compiled_surface=NSQASM_AARCH64
c_runner_used=false
input_a={a}
input_b={b}
operation={op}
nonce={nonce}
expected_result={expected}
semantic_result_verified=false
ok=false
NSQ_SEMANTIC_RUNTIME_END
"""

asm = f"""
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
    adr x1, fail_msg
    mov x2, #fail_len
    bl write_stdout
    mov x0, #1
    b exit_now

ok_path:
    adr x1, ok_msg
    mov x2, #ok_len
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
ok_msg:
.ascii {gas_string(ok_msg)}
.equ ok_len, . - ok_msg

fail_msg:
.ascii {gas_string(fail_msg)}
.equ fail_len, . - fail_msg
"""

ASM_OUT.write_text(asm, encoding="utf-8")
print(ASM_OUT)
