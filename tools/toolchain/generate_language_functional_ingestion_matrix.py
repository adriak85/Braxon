#!/usr/bin/env python3
"""Generate the functional language-ingestion matrix from the NSQ authority registry."""
from __future__ import annotations

import json
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
REGISTRY = ROOT / "config/nsq/nsq_runtime_language_registry.json"
OUT = ROOT / "config/nsq/language_functional_ingestion_matrix.json"
TARGET = "aarch64-linux-android"

TOOLCHAINS = {
    "rust": ["rustc", "cargo", "rust-std"],
    "c": ["clang", "lld", "compiler-rt", "bionic headers"],
    "cpp": ["clang++", "lld", "libc++", "libc++abi"],
    "c_plus": ["clang++", "lld", "libc++", "libc++abi"],
    "objective_c": ["clang", "Objective-C runtime"],
    "python": ["python3", "CPython runtime"],
    "python3": ["python3", "CPython runtime"],
    "assembly": ["clang", "assembler", "lld"],
    "asm": ["clang", "assembler", "lld"],
    "aarch64_asm": ["clang", "AArch64 assembler", "lld"],
    "arm64_asm": ["clang", "AArch64 assembler", "lld"],
    "armv7_asm": ["clang", "ARM assembler", "lld"],
    "x86_64_asm": ["clang", "x86_64 assembler", "lld"],
    "x86_asm": ["clang", "x86 assembler", "lld"],
    "riscv64_asm": ["clang", "RISC-V assembler", "lld"],
    "wat": ["wasm assembler", "wasm linker"],
    "wasm_text": ["wasm assembler", "wasm linker"],
    "go": ["go compiler", "Go runtime"],
    "java": ["javac", "JVM runtime"],
    "kotlin": ["kotlinc", "JVM runtime"],
    "javascript": ["node", "JavaScript runtime"],
    "typescript": ["typescript compiler", "JavaScript runtime"],
    "guile": ["guile"],
    "scheme": ["scheme runtime"],
    "common_lisp": ["common lisp runtime"],
    "lisp": ["lisp runtime"],
    "zig": ["zig"],
    "fortran": ["flang or gfortran"],
    "ada": ["GNAT"],
    "swift": ["swiftc"],
    "csharp": ["dotnet compiler", "dotnet runtime"],
    "fsharp": ["dotnet compiler", "dotnet runtime"],
    "haskell": ["GHC"],
    "ocaml": ["OCaml compiler"],
    "erlang": ["Erlang runtime"],
    "elixir": ["Elixir compiler", "Erlang runtime"],
    "julia": ["Julia runtime"],
    "dart": ["Dart SDK"],
    "lua": ["Lua runtime"],
    "ruby": ["Ruby runtime"],
    "perl": ["Perl runtime"],
    "php": ["PHP runtime"],
    "bash": ["POSIX shell"],
    "powershell": ["PowerShell runtime"],
    "nim": ["Nim compiler"],
}


def git_head() -> str:
    return subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip()


def main() -> None:
    registry = json.loads(REGISTRY.read_text())
    rows = []
    for surface in sorted(registry["surfaces"], key=lambda item: item["id"]):
        language = surface["id"]
        tools = TOOLCHAINS.get(language, [f"{surface.get('native_ingress', 'declared')} materialization toolchain"])
        rows.append({
            "id": language,
            "family": surface.get("family", "unspecified"),
            "target_environment": TARGET,
            "semantic_contract": {
                "authority": "config/nsq/nsq_runtime_language_registry.json",
                "nsq_capability": f"language:{language}",
                "kinetic_reflexor_route": f"Braxon language {language} <input>",
                "semantic_operation_state": "operable_on_demand",
                "resident_runtime": False,
            },
            "target_materialization": {
                "native_ingress": surface.get("native_ingress", "unspecified"),
                "required_local_tools": tools,
                "compiler_or_runtime_payload_state": "must be contained locally before native execution; missing payload must be reported exactly",
                "hidden_download_allowed": False,
            },
            "verification": {
                "semantic_route": f"Braxon language {language} semantic-ingestion verification",
                "native_execution": "requires a declared local compiler/runtime artifact and target-specific probe",
                "success_criterion": "semantic operation executes through language capability and operator action; native materialization is never claimed without local artifact evidence",
            },
            "provenance": {
                "upstream_payload_ownership": "varies by toolchain; preserve upstream license and notice",
                "private_eligibility": "only independently authored Braxon adapters or implementations after source-level review",
            },
        })
    matrix = {
        "schema": "braxon.language_functional_ingestion_matrix.v1",
        "authority": "BRAXON_FUNCTIONAL_LANGUAGE_INGESTION",
        "owner": "Michael David Norris",
        "repository_commit": git_head(),
        "target_environment": TARGET,
        "language_total": len(rows),
        "normal_runtime_policy": {
            "network_required": False,
            "external_api_required": False,
            "hidden_download_allowed": False,
            "semantic_routes_use_compact_nsq_reflexor_contracts": True,
        },
        "languages": rows,
    }
    OUT.write_text(json.dumps(matrix, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"output": str(OUT.relative_to(ROOT)), "language_total": len(rows), "target": TARGET}))


if __name__ == "__main__":
    main()
