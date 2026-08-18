#!/usr/bin/env python3
import json
import sys
from pathlib import Path

EXPECTED = {
    "maverick_logic",
    "qwen_creativity",
    "arbiter_judge",
    "analyzer_auditor",
    "limbic_empath",
    "support_memory",
    "image_cortex",
    "video_cortex",
    "voice_body",
    "world_body_3d",
}


def fail(message: str) -> None:
    print(f"FAIL: {message}", file=sys.stderr)
    raise SystemExit(1)


def main() -> int:
    if len(sys.argv) != 2:
        fail("usage: verify_council_seed_manifest.py MANIFEST.json")
    path = Path(sys.argv[1])
    try:
        data = json.loads(path.read_text())
    except Exception as exc:
        fail(f"invalid manifest JSON: {exc}")
    policy = data.get("policy", {})
    for key in (
        "full_artifact_required",
        "tiny_fixture_allowed",
        "partial_payload_is_failure",
        "semantic_extraction_required",
        "nsq_bus_binding_required",
        "bounded_materialization_required",
        "provenance_required",
    ):
        if key not in policy:
            fail(f"missing policy field: {key}")
    if policy["full_artifact_required"] is not True:
        fail("full_artifact_required must be true")
    if policy["tiny_fixture_allowed"] is not False:
        fail("tiny fixtures must remain forbidden")
    if policy["partial_payload_is_failure"] is not True:
        fail("partial payloads must fail closed")
    if policy["semantic_extraction_required"] is not True:
        fail("semantic extraction must be mandatory")
    if policy["nsq_bus_binding_required"] is not True:
        fail("NSQ bus binding must be mandatory")
    lanes = data.get("lanes")
    if not isinstance(lanes, list):
        fail("lanes must be an array")
    names = [lane.get("lane") for lane in lanes]
    if len(lanes) != 10 or set(names) != EXPECTED:
        fail(f"expected exactly the ten Council lanes, got {names}")
    if len(set(names)) != len(names):
        fail("duplicate Council lane")
    for lane in lanes:
        for key in ("lane", "model_id", "source_repo", "revision", "artifact_family", "access", "bus_dialect", "semantic_projection"):
            if not lane.get(key):
                fail(f"lane {lane.get('lane')} missing {key}")
        if lane.get("model_id") == "deepseek-v3-671b-analyzer" and lane.get("independent_payload_required"):
            fail("analyzer role must not claim a fictitious independent payload")
    contract = data.get("integration_contract", {})
    required = set(contract.get("per_lane", []))
    needed = {
        "authoritative model index or modality manifest",
        "complete payload inventory and hashes",
        "semantic artifact-intent projection",
        "NSQ address allocation",
        "bounded reader receipt",
        "hot initiative-cluster registration",
        "bus ownership and piston lease",
        "parameter generation and rematerialization receipt",
        "independent oracle comparison",
    }
    if required != needed:
        fail("integration contract is incomplete")
    if contract.get("federation_barrier") != "all ten lanes registered and callable; partial federation fails closed":
        fail("federation barrier is missing or weakened")
    print("council_lanes=10")
    print("full_artifact_policy=true")
    print("semantic_extraction_required=true")
    print("nsq_bus_binding_required=true")
    print("partial_federation=fail_closed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
