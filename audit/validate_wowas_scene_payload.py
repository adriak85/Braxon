#!/usr/bin/env python3
"""Fail-closed validation for wowas.scene_payload.v1 artifacts."""
from __future__ import annotations
import argparse, hashlib, json, sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
REQUIRED_TOP = {"schema", "coordinate", "intent", "reflexor", "constraints", "state_slice", "resonance", "watermark", "execution_boundary"}
REQUIRED_CONSTRAINTS = {"character_flavor", "dynamics", "pip_leadership", "relationship_ledger"}


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def fail(message: str) -> None:
    raise ValueError(message)


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("payload", type=Path)
    args = ap.parse_args()
    payload_path = args.payload.resolve()
    x = json.loads(payload_path.read_text(encoding="utf-8"))
    if x.get("schema") != "wowas.scene_payload.v1":
        fail("schema mismatch")
    missing = REQUIRED_TOP - set(x)
    if missing:
        fail(f"missing top-level fields: {sorted(missing)}")
    c = x["coordinate"]
    if not c.get("record_id") or c.get("scene_id_authority") != "context_only_record_id_authoritative":
        fail("record_id is authoritative and scene_id must be context-only")
    if not c.get("book_key", "").startswith("B"):
        fail("book_key must be canonical B01-B33 form")
    if x["intent"].get("semantic_variables") is None or len(x["intent"]["semantic_variables"]) != 8:
        fail("intent must carry exactly eight semantic variables")
    if not x["resonance"].get("patch") or not x["resonance"].get("tone_guide"):
        fail("resonance and tone-guide source records are required")
    if x["reflexor"].get("phase_order") != ["Publish", "Reconcile", "DeltaCommit"]:
        fail("reflexor phase order does not match NativeNsqReflexor")
    if x["reflexor"]["native_contract"].get("changed_values_only") is not True:
        fail("reflexor must commit changed values only")
    if REQUIRED_CONSTRAINTS - set(x["constraints"]):
        fail("constraint layer incomplete")
    if x["execution_boundary"].get("prose_generation_permitted") is not False or x["execution_boundary"].get("no_generated_prose") is not True:
        fail("payload cannot authorize or contain generated prose")
    if x["execution_boundary"].get("generated_prose_field") is not None:
        fail("generated_prose_field must remain null")
    wm = x["watermark"]
    if wm.get("algorithm") != "SHA-256" or not wm.get("payload_hash"):
        fail("canonical watermark is incomplete")
    expected_hash = wm["payload_hash"]
    canon = dict(x)
    canon["watermark"] = dict(wm)
    canon["watermark"]["payload_hash"] = None
    actual_hash = hashlib.sha256(json.dumps(canon, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
    if actual_hash != expected_hash:
        fail("payload_hash mismatch")
    checked = 0
    for item in wm.get("input_hashes", {}).values():
        path = ROOT / item["path"]
        if not path.exists():
            fail(f"missing watermarked input: {item['path']}")
        if sha256(path) != item["sha256"]:
            fail(f"watermarked input changed: {item['path']}")
        checked += 1
    schema_item = wm.get("schema_hash")
    if not schema_item or sha256(ROOT / schema_item["path"]) != schema_item["sha256"]:
        fail("schema watermark mismatch")
    print(json.dumps({"status": "pass", "record_id": c["record_id"], "book_key": c["book_key"], "payload_hash": actual_hash, "watermarked_inputs": checked, "prose_generation_permitted": False}, sort_keys=True))
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, ValueError, json.JSONDecodeError, KeyError, TypeError) as exc:
        print(json.dumps({"status": "blocked", "error": str(exc), "prose_generation_permitted": False}, sort_keys=True), file=sys.stderr)
        raise SystemExit(2)
