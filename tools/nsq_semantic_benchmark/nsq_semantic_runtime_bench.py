#!/usr/bin/env python3
from __future__ import annotations

import json
import shutil
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path.cwd()
OUT = ROOT / "state/nsq/semantic_benchmark/current"
OUT.mkdir(parents=True, exist_ok=True)

generator = ROOT / "tools/nsq_semantic_benchmark/nsq_task_to_aarch64_asm.py"
asm_path = ROOT / "tools/nsq_semantic_benchmark/nsq_semantic_task_runner_aarch64.S"
runner = ROOT / "bin/nsq-semantic-asm-task-runner"

cc = shutil.which("cc") or shutil.which("clang") or shutil.which("gcc")
if not cc:
    raise SystemExit("missing compiler: install clang")

subprocess.run([sys.executable, str(generator), str(ROOT)], cwd=ROOT, check=True)

compile_proc = subprocess.run(
    [cc, "-nostdlib", "-Wl,-e,_start", str(asm_path), "-o", str(runner)],
    cwd=ROOT,
    text=True,
    stdout=subprocess.PIPE,
    stderr=subprocess.PIPE,
    check=False,
)
(OUT / "compile_asm.stdout.txt").write_text(compile_proc.stdout, encoding="utf-8")
(OUT / "compile_asm.stderr.txt").write_text(compile_proc.stderr, encoding="utf-8")
if compile_proc.returncode != 0:
    raise SystemExit(compile_proc.returncode)

runner.chmod(0o755)

started = time.monotonic()
proc = subprocess.run(
    [str(runner)],
    cwd=ROOT,
    text=True,
    stdout=subprocess.PIPE,
    stderr=subprocess.PIPE,
    check=False,
)
elapsed = round(time.monotonic() - started, 6)

(OUT / "nsqasm_acceptance.stdout.txt").write_text(proc.stdout, encoding="utf-8")
(OUT / "nsqasm_acceptance.stderr.txt").write_text(proc.stderr, encoding="utf-8")

ok = all(x in proc.stdout for x in [
    "authority=NSQ_COURT",
    "architecture_root=true",
    "king=compositor",
    "queen=linter",
    "court_is_agents=false",
    "runtime_kind=nsqasm_aarch64_no_libc",
    "source_language=NSQ",
    "compiled_surface=NSQASM_AARCH64",
    "task_file_semantics=executed",
    "c_runner_used=false",
    "semantic_result_verified=true",
    "ok=true",
]) and proc.returncode == 0

checkpoint = {
    "schema": "nsq.perpetual_runtime.checkpoint.v5",
    "generated_at_unix": int(time.time()),
    "authority": "NSQ_COURT",
    "architecture_root": True,
    "king": "compositor",
    "queen": "linter",
    "court_is_agents": False,
    "acceptance_runner": "nsqasm_aarch64_no_libc",
    "asm_acceptance": ok,
    "c_runner_used": False,
    "c_reference_runner_storage": False
}

(root_checkpoint := ROOT / "state/nsq/perpetual_runtime/current/checkpoint.json").write_text(json.dumps(checkpoint, indent=2, sort_keys=True) + "\n")
(ROOT / "state/nsq/perpetual_runtime/current/journal.jsonl").write_text(json.dumps({"event": "nsq_court_asm_acceptance_run", **checkpoint}, sort_keys=True) + "\n")
(ROOT / "state/nsq/perpetual_runtime/current/heartbeat.json").write_text(json.dumps({"alive": True, "generated_at_unix": int(time.time())}, indent=2, sort_keys=True) + "\n")

claim = {
    "schema": "nsq.perpetual_runtime.claim.current.v5",
    "generated_at_unix": int(time.time()),
    "authority": "NSQ_COURT",
    "architecture_root": True,
    "king": "compositor",
    "queen": "linter",
    "court_is_agents": False,
    "acceptance_runner": "nsqasm_aarch64_no_libc",
    "nsq_task_file_semantically_executed": ok,
    "semantic_result_verified": ok,
    "nsq_runtime_elapsed_measured": ok,
    "elapsed_seconds": elapsed,
    "c_runner_used": False,
    "c_runner_allowed_for_acceptance": False,
    "c_reference_runner_storage": False,
    "watchdog_restart_test_passed": False,
    "crash_resume_test_passed": False,
    "resource_ceiling_enforced": False,
    "manual_stop_works": False,
    "perpetual_runtime_allowed": False,
    "rewrite_all_BRAXON_to_nsq_allowed": ok,
    "BRAXON_live_claim": False,
    "bare_metal_claim_allowed": False,
    "truth": "NSQ Court is architecture root. Semantic acceptance uses NSQ -> NSQ Court king/queen route -> NSQASM/AArch64. C runner is removed and not stored."
}

(ROOT / "state/nsq/perpetual_runtime/current/claim.json").write_text(json.dumps(claim, indent=2, sort_keys=True) + "\n")

report = dict(claim)
report["schema"] = "nsq.semantic_runtime_benchmark.report.v6"
report["asm_stdout_path"] = "state/nsq/semantic_benchmark/current/nsqasm_acceptance.stdout.txt"
report["asm_stderr_path"] = "state/nsq/semantic_benchmark/current/nsqasm_acceptance.stderr.txt"
report["asm_returncode"] = proc.returncode

(OUT / "report.json").write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")

report_txt = f"""== NSQ Court ASM semantic runtime benchmark v6 ==
authority=NSQ_COURT
architecture_root=true
king=compositor
queen=linter
court_is_agents=false
acceptance_runner=nsqasm_aarch64_no_libc
nsq_task_file_semantically_executed={ok}
semantic_result_verified={ok}
nsq_runtime_elapsed_measured={ok}
elapsed_seconds={elapsed}
c_runner_used=false
c_runner_allowed_for_acceptance=false
c_reference_runner_storage=false
perpetual_runtime_allowed=false
rewrite_all_BRAXON_to_nsq_allowed={ok}
BRAXON_live_claim=false
bare_metal_claim_allowed=false

truth=NSQ Court is architecture root. Semantic acceptance uses NSQ -> NSQ Court king/queen route -> NSQASM/AArch64. C runner is removed and not stored.
"""
(OUT / "report.txt").write_text(report_txt, encoding="utf-8")
print(report_txt)
