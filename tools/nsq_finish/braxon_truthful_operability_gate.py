#!/usr/bin/env python3
"""Generate derived launch-readiness state from the canonical Citadel seed route.

This gate does not inspect safetensors, GGUF, shards, download manifests, or
static donor payload claims. It invokes the tracked `Braxon runtime donors`
front door, which routes through the Kinetic Semantic Reflexor and proves the
Council Ten topology plus a bounded Citadel seed materialization/fire/release
cycle. It never promotes that seed-route proof into a learned-weight execution
or resident-runtime claim.
"""

from __future__ import annotations

import json
import subprocess
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DONOR_COMMAND = [
    "cargo",
    "run",
    "--locked",
    "--offline",
    "--",
    "runtime",
    "donors",
]


def run_canonical_donor_front_door() -> dict:
    completed = subprocess.run(
        DONOR_COMMAND,
        cwd=ROOT,
        check=False,
        text=True,
        capture_output=True,
    )
    if completed.returncode != 0:
        raise RuntimeError(
            "canonical donor front door failed: "
            + (completed.stderr.strip() or completed.stdout.strip() or f"exit={completed.returncode}")
        )
    try:
        return json.loads(completed.stdout)
    except json.JSONDecodeError as error:
        raise RuntimeError(f"canonical donor front door did not emit JSON: {error}") from error


def write_json(relative_path: str, document: dict) -> None:
    path = ROOT / relative_path
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(document, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def main() -> None:
    now = datetime.now(timezone.utc).isoformat()
    donor_front_door = run_canonical_donor_front_door()
    readiness = donor_front_door.get("readiness")
    if not isinstance(readiness, dict):
        raise RuntimeError("canonical donor front door omitted its readiness receipt")

    seed_window_proven = bool(
        readiness.get("configured_model_total_matches_contract")
        and readiness.get("council_wake_all_passed")
        and readiness.get("citadel_seed_materialized")
        and readiness.get("complete_ten_body_window_proven")
        and readiness.get("donor_parameter_synchronization_live")
        and readiness.get("materialized_body_total") == 10
        and readiness.get("nsq_fire_instruction_total") == 10
        and readiness.get("nsq_release_instruction_total") == 10
    )
    model_weight_execution_claimed = bool(readiness.get("model_weight_execution_claimed"))
    resident_runtime_constructed = bool(readiness.get("resident_runtime_constructed"))
    if model_weight_execution_claimed or resident_runtime_constructed:
        raise RuntimeError(
            "canonical donor receipt violated the non-resident seed-route boundary"
        )

    launch = {
        "schema": "Braxon.model_launch_readiness.v2",
        "canonicality": "canonical_active",
        "generated_at": now,
        "authority": "NSQ kinetic semantic reflexor",
        "canonical_donor_path": "Council Ten Citadel seed -> NSQ set/fire -> bounded lease release",
        "authoritative_seed_contract_path": readiness.get("authoritative_seed_contract_path"),
        "seed_id": readiness.get("seed_id"),
        "seed_hash": readiness.get("seed_hash"),
        "configured_model_total": readiness.get("configured_model_total"),
        "seed_window_ready": seed_window_proven,
        "can_attempt_seed_window_operation": seed_window_proven,
        "runtime_route_proven": seed_window_proven,
        "loaded_binding_proven": False,
        "model_weight_execution_claimed": False,
        "runtime_hot_live_proven": False,
        "resident_runtime_constructed": False,
        "status": (
            "canonical_seed_window_proven_model_weight_execution_unclaimed"
            if seed_window_proven
            else "canonical_seed_window_proof_failed_closed"
        ),
        "missing_for_model_weight_execution": [
            "independent_learned_weight_execution_receipt"
        ],
        "evidence": {
            "donor_front_door_action": donor_front_door.get("action"),
            "donor_readiness_capability": donor_front_door.get("donor_readiness_capability"),
            "intelligent_operation_capability": donor_front_door.get("intelligent_operation_capability"),
            "council_wake_all_passed": readiness.get("council_wake_all_passed"),
            "materialized_body_total": readiness.get("materialized_body_total"),
            "nsq_set_instruction_total": readiness.get("nsq_set_instruction_total"),
            "nsq_fire_instruction_total": readiness.get("nsq_fire_instruction_total"),
            "nsq_release_instruction_total": readiness.get("nsq_release_instruction_total"),
            "complete_ten_body_window_proven": readiness.get("complete_ten_body_window_proven"),
            "bands": readiness.get("bands"),
        },
        "rule": (
            "Only an executed Council Ten Citadel seed materialization, NSQ fire, and lease release "
            "can establish seed-window readiness. That proof must not be relabeled as learned model-weight "
            "execution, whole-model activation, or a resident runtime."
        ),
    }

    bus = {
        "schema": "Braxon.llm_bus_parameter_pressure_gate.v2",
        "canonicality": "canonical_active",
        "generated_at": now,
        "authority": "NSQ kinetic semantic reflexor",
        "llm_bus_seed_window_ready": seed_window_proven,
        "model_weight_execution_claimed": False,
        "resident_runtime_constructed": False,
        "status": (
            "citadel_seed_bus_window_materialized_fired_and_released"
            if seed_window_proven
            else "citadel_seed_bus_window_failed_closed"
        ),
        "component_status": {
            "council_wake_all_passed": readiness.get("council_wake_all_passed"),
            "materialized_body_total": readiness.get("materialized_body_total"),
            "nsq_fire_instruction_total": readiness.get("nsq_fire_instruction_total"),
            "nsq_release_instruction_total": readiness.get("nsq_release_instruction_total"),
            "complete_ten_body_window_proven": readiness.get("complete_ten_body_window_proven"),
        },
        "missing_for_model_weight_execution": [
            "independent_learned_weight_execution_receipt"
        ],
    }

    nsq = {
        "schema": "nsq.core_release_gate.v2",
        "canonicality": "canonical_active",
        "generated_at": now,
        "authority": "NSQ kinetic semantic reflexor",
        "citadel_seed_route_proven": seed_window_proven,
        "model_weight_execution_claimed": False,
        "resident_runtime_constructed": False,
        "status": (
            "core_seed_route_proven_release_scope_unclaimed"
            if seed_window_proven
            else "core_seed_route_failed_closed"
        ),
        "evidence": {
            "donor_readiness_schema": readiness.get("schema"),
            "complete_ten_body_window_proven": readiness.get("complete_ten_body_window_proven"),
            "seed_hash": readiness.get("seed_hash"),
        },
        "release_scope": "This receipt proves only the canonical on-demand Citadel seed route; it is not a whole-system release certification.",
    }

    write_json("state/braxon/braxon_model_launch_readiness.json", launch)
    write_json("state/braxon/braxon_llm_bus_parameter_pressure_gate.json", bus)
    write_json("state/nsq/nsq_core_release_gate.json", nsq)

    docs = ROOT / "docs/Braxon"
    specs = ROOT / "specs/Braxon"
    docs.mkdir(parents=True, exist_ok=True)
    specs.mkdir(parents=True, exist_ok=True)
    (docs / "BRAXON_TRUTHFUL_OPERABILITY_GATE.md").write_text(
        "# Braxon Canonical Citadel Seed Operability Gate\n\n"
        f"Generated: `{now}`\n\n"
        f"- `seed_window_ready`: `{str(seed_window_proven).lower()}`\n"
        "- `model_weight_execution_claimed`: `false`\n"
        "- `resident_runtime_constructed`: `false`\n\n"
        "The gate invokes the active donor front door. It accepts only a ten-body Council Ten "
        "Citadel seed materialization that sets, fires, and releases every NSQ body. Conventional "
        "safetensors indexes, shard counts, and raw donor downloads are not evidence for this canonical route.\n",
        encoding="utf-8",
    )
    (specs / "BRAXON_TRUTHFUL_OPERABILITY_GATE_CONTRACT.md").write_text(
        "# Braxon Canonical Citadel Seed Operability Gate Contract\n\n"
        "The canonical donor path is `Council Ten stack -> Citadel seed -> NSQ set/fire -> bounded lease release`. "
        "The gate must invoke the tracked donor front door and fail closed when any topology, materialization, "
        "actuation, or release proof is absent. The result must never claim learned model-weight execution, "
        "whole-model activation, or a resident runtime.\n",
        encoding="utf-8",
    )

    print(
        json.dumps(
            {
                "ok": seed_window_proven,
                "canonicality": "canonical_active",
                "seed_window_ready": seed_window_proven,
                "model_weight_execution_claimed": False,
                "resident_runtime_constructed": False,
                "status": launch["status"],
                "next": [
                    "Braxon runtime donors",
                    "Braxon runtime infer <configured-model> <prompt>",
                ],
            },
            indent=2,
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
