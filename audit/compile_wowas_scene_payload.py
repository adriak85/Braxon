#!/usr/bin/env python3
"""Compile one deterministic WOWAS scene payload; never generate or promote prose."""
from __future__ import annotations
import argparse, csv, hashlib, json, re, sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
BASE = ROOT / "crates/wowas-final-edition-v10/canon/active"
META = BASE / "reconciled_15000/scene_index_reconciled_metadata.tsv"
FLAVOR = BASE / "authored_flavor/authored_character_flavor_lattice.tsv"
DYNAMICS = BASE / "authored_flavor/authored_dynamics_lattice.tsv"
PIP = BASE / "authored_flavor/pip_leadership_constraints.tsv"
GEN_FLAVOR = BASE / "authored_flavor/generated_character_flavor_constraints.tsv"
RELATIONSHIPS = BASE / "generated/wowas_relationship_ledger.tsv"
BOOKS = BASE / "generated/wowas_book_contract_map_33.tsv"
RESONANCE = ROOT / "crates/wowas-final-edition-v10/canon/patches/v12/wowas_quality_romance_calendar_and_resonance_patch_v12.md"
TONE_GUIDE = ROOT / "crates/wowas-final-edition-v10/canon/control/prose_and_tone_guide_v14.json"
PREFLIGHT = ROOT / "reconstruction/WOWAS_REALIZATION_PREFLIGHT.json"
INSTRUCTION_AUDIT = ROOT / "reconstruction/WOWAS_SOURCE_INSTRUCTION_AUDIT.json"
SCHEMA = ROOT / "reconstruction/WOWAS_SCENE_PAYLOAD_SCHEMA_v1.json"
DEFAULT_STATE = ROOT / "reconstruction/WOWAS_PAYLOAD_STATE_LEDGER.json"

ACTIVE_WATERMARK = "BRAXON_NSQ_BASE8_LEVER_SCALE_GE220000_PROVEN225370_V1"
INTENT_SCHEMA = "braxon.nsq_native.intent.v1"
REFLEX_PHASES = ["Publish", "Reconcile", "DeltaCommit"]
INTENT_VARIABLES = ["motive", "agency", "truth", "force", "scope", "time", "relation", "form"]
CANONICAL_NAMES = {
    "ryl(o|os|edge|os vayne johnson)": "Rylos Vayne Johnson",
    "boojay": "Rylos Vayne Johnson",
    "ryl(os)? vayne johnson": "Rylos Vayne Johnson",
    "indalwin( on['’]rylder)?( willowjayce)?": "Indalwin On’Rylder Willowjayce",
    "pip": "Indalwin On’Rylder Willowjayce",
}


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def relpath(path: Path) -> str:
    try:
        return str(path.relative_to(ROOT))
    except ValueError:
        return str(path)


def read_tsv(path: Path) -> list[dict[str, str]]:
    if not path.exists():
        raise FileNotFoundError(relpath(path))
    with path.open("r", encoding="utf-8", errors="replace", newline="") as fh:
        return list(csv.DictReader(fh, delimiter="\t"))


def source_record(path: Path) -> dict[str, str]:
    return {"path": relpath(path), "sha256": sha256_bytes(path.read_bytes())}


def source_matches(path: Path, terms: tuple[str, ...], limit: int = 24) -> list[dict[str, Any]]:
    matches = []
    for number, line in enumerate(path.read_text(encoding="utf-8", errors="replace").splitlines(), 1):
        if any(term.lower() in line.lower() for term in terms):
            matches.append({"line": number, "text": line.strip()})
            if len(matches) >= limit:
                break
    return matches


def norm(value: str) -> str:
    return re.sub(r"\s+", " ", value.strip().lower().replace("’", "'"))


def canonicalize(value: str) -> str:
    n = norm(value)
    for pattern, canonical in CANONICAL_NAMES.items():
        if re.fullmatch(pattern, n):
            return canonical
    return value.strip()


def sanitize_active_text(value: str) -> str:
    # Active payload text must use the canonical identity; source files remain hashed and untouched.
    return re.sub(r'(?i)\bBoojay\b', 'Rylos Vayne Johnson', str(value or ''))


def sanitize_object(value: Any) -> Any:
    if isinstance(value, str):
        return sanitize_active_text(value)
    if isinstance(value, list):
        return [sanitize_object(item) for item in value]
    if isinstance(value, dict):
        return {key: sanitize_object(item) for key, item in value.items()}
    return value


def parse_json_field(record: dict[str, str], field: str) -> dict[str, Any]:
    raw = record.get(field, '')
    if not raw:
        raise ValueError(f"selected metadata row is missing {field}")
    value = json.loads(raw)
    if not isinstance(value, dict):
        raise ValueError(f"selected metadata field {field} must be a JSON object")
    return value


def split_values(value: str) -> list[str]:
    return [canonicalize(x) for x in re.split(r"\s*\|\s*|\s*,\s*", value or "") if x.strip()]


def book_key(value: str) -> str:
    raw = (value or "").strip()
    if raw.upper().startswith("B"):
        return raw.upper()
    if raw.isdigit():
        return f"B{int(raw):02d}"
    return raw


def unique(rows: list[dict[str, str]], key: str, label: str) -> None:
    values = [r.get(key, "") for r in rows]
    if any(not value for value in values):
        raise ValueError(f"{label}: empty {key}")
    if len(values) != len(set(values)):
        raise ValueError(f"{label}: duplicate {key}")


def select_record(meta: list[dict[str, str]], record_id: str | None, book: str | None) -> dict[str, str]:
    unique(meta, "record_id", "scene metadata")
    if record_id:
        matches = [r for r in meta if r.get("record_id") == record_id]
    else:
        wanted = book_key(book or "B01")
        matches = [r for r in meta if book_key(r.get("book_num", "")) == wanted]
    if len(matches) != 1:
        raise ValueError(f"selection must resolve exactly one record_id; matches={len(matches)}")
    return matches[0]


def names_from_scene(row: dict[str, str]) -> list[str]:
    values = split_values(row.get("inferred_character_names", "")) + split_values(row.get("book_active_cast", ""))
    result: list[str] = []
    for value in values:
        # Preserve canonical names while excluding structural markers.
        if value and value.lower() not in {"drawn entities", "unmapped", "none"} and value not in result:
            result.append(value)
    return result


def load_state(path: Path, active_names: list[str], record: dict[str, str]) -> dict[str, Any]:
    if not path.exists():
        return {"policy": "bounded_immediate_context_only", "status": "initial_state_absent", "source": None, "characters": [], "bridges": [], "excluded_reason": "No prior ledger is permitted to be invented for the first payload."}
    state = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(state, dict):
        raise ValueError("state ledger must be a JSON object")
    rows = state.get("characters", [])
    selected = []
    for item in rows:
        if not isinstance(item, dict):
            continue
        name = canonicalize(str(item.get("canonical_name", item.get("character_name", ""))))
        if name in active_names:
            selected.append(item)
    bridges = [b for b in state.get("bridges", []) if isinstance(b, dict) and (b.get("book_num") in {None, record.get("book_num")} or b.get("record_id") == record.get("record_id"))]
    return {"policy": "bounded_immediate_context_only", "status": "loaded", "source": source_record(path), "characters": selected, "bridges": bridges}


def compile_payload(record: dict[str, str], paths: dict[str, Path], state_path: Path) -> dict[str, Any]:
    flavor = read_tsv(paths["flavor"])
    dynamics = read_tsv(paths["dynamics"])
    pip = read_tsv(paths["pip"])
    generated_flavor = read_tsv(paths["generated_flavor"])
    relationships = read_tsv(paths["relationships"])
    books = read_tsv(paths["books"])
    for rows, key, label in [(flavor, "canonical_id", "authored flavor"), (dynamics, "dynamic_id", "authored dynamics"), (pip, "record_id", "Pip constraints")]:
        unique(rows, key, label)
    if record.get("prose_status") != "no_generated_prose":
        raise ValueError("selected coordinate is not explicitly no_generated_prose")
    record = {key: sanitize_active_text(value) if key not in {"record_id", "source_path", "canonical_hash_seed"} else value for key, value in record.items()}
    semantic_intent = parse_json_field(record, "semantic_intent")
    nsq_coordinates = parse_json_field(record, "nsq_coordinates")
    reflexor_bounce = parse_json_field(record, "reflexor_bounce")
    active_names = names_from_scene(record)
    canonical_active = sorted(set(canonicalize(x) for x in active_names))
    flavor_rows = [r for r in flavor if canonicalize(r.get("canonical_name", "")) in canonical_active or any(canonicalize(a) in canonical_active for a in split_values(r.get("aliases", "")))]
    dynamics_rows = [r for r in dynamics if canonicalize(r.get("from_character", "")) in canonical_active or canonicalize(r.get("to_character", "")) in canonical_active]
    pip_rows = [r for r in pip if r.get("record_id") == record.get("record_id") or (r.get("scene_id") == record.get("scene_id") and r.get("book_num") == record.get("book_num"))]
    relation_rows = [r for r in relationships if r.get("scene_link") == record.get("scene_id") or (r.get("book_num") == record.get("book_num") and canonicalize(r.get("character_name", "")) in canonical_active)]
    book_rows = [r for r in books if book_key(r.get("book_num", "")) == book_key(record.get("book_num", ""))]
    if len(book_rows) != 1:
        raise ValueError(f"book contract must resolve exactly once; matches={len(book_rows)}")
    unmapped = [r.get("generated_character_id") for r in generated_flavor if r.get("book_anchor") == record.get("book_num") and r.get("authored_canonical_id", "").startswith("unmapped:")]
    preflight = json.loads(PREFLIGHT.read_text(encoding="utf-8")) if PREFLIGHT.exists() else {}
    audit = json.loads(INSTRUCTION_AUDIT.read_text(encoding="utf-8")) if INSTRUCTION_AUDIT.exists() else {}
    if not RESONANCE.exists() or not TONE_GUIDE.exists():
        raise FileNotFoundError("resonance patch and prose/tone guide are required inputs")
    gradient = {name: 220000 for name in INTENT_VARIABLES}
    payload: dict[str, Any] = {
        "schema": "wowas.scene_payload.v1",
        "coordinate": {"record_id": record["record_id"], "record_kind": record.get("record_kind"), "book_num": record.get("book_num"), "book_key": book_key(record.get("book_num", "")), "book_title": record.get("book_title"), "scene_id": record.get("scene_id"), "scene_id_authority": "context_only_record_id_authoritative", "source_layer": record.get("source_layer"), "source_type": record.get("source_type"), "title": record.get("clean_title"), "active_cast": canonical_active},
        "intent": {"schema": INTENT_SCHEMA, "semantic_variables": nsq_coordinates.get("variables", gradient), "gradient_source": nsq_coordinates.get("source", "repository metadata"), "scale_anchors": nsq_coordinates.get("scale_anchors", ["self_object_scale", "relational_group_scale", "system_world_scale", "universal_field_scale"]), "repository_semantic_intent": semantic_intent, "source_fields": {k: record.get(k, "") for k in ["brief_scene_description", "book_key_pressure", "quest_hook", "domain_flags", "coverage_status", "alignment_status", "transformation_notes"]}, "derivation_policy": "metadata states events; authored lattices constrain subtext; the model may not add unstated canon"},
        "reflexor": {"phase_order": reflexor_bounce.get("phase_order", REFLEX_PHASES), "environmental_inputs": reflexor_bounce.get("environmental_inputs", {k: record.get(k, "") for k in ["ecology_pressure_mode", "county_anchor", "corridor_region_anchor", "creature_refs", "world_introduction_anchor", "transformation_notes"]}), "repository_reflexor_bounce": reflexor_bounce, "native_contract": {"native_reflexor": "nsq-core::NativeNsqReflexor", "operation": "orbit", "changed_values_only": True, "watermark_refresh": True, "same_space_override": False, "ghost_memory": "bounded external state only; no invented resident hardware state"}},
        "constraints": {"character_flavor": sanitize_object(flavor_rows), "dynamics": sanitize_object(dynamics_rows), "pip_leadership": sanitize_object(pip_rows), "generated_character_flavor": sanitize_object([r for r in generated_flavor if r.get("book_anchor") == record.get("book_num")]), "relationship_ledger": sanitize_object(relation_rows), "book_contract": sanitize_object(book_rows[0]), "unmapped_anchor_ids": sorted(set(unmapped)), "unmapped_anchor_policy": "quarantine; no invented authored identity"},
        "state_slice": load_state(state_path, canonical_active, record),
        "resonance": {"policy": "source-backed modifiers only; no private alignment values are guessed", "patch": source_record(RESONANCE), "tone_guide": source_record(TONE_GUIDE), "patch_matches": source_matches(RESONANCE, ("resonance", "calendar", "occasion", "alignment")), "applied_patch_ids": [x for x in (record.get("applied_patch_ids", "").split("|")) if x]},
        "provenance": {"preflight": preflight, "instruction_audit": {"schema": audit.get("schema"), "status": audit.get("status"), "source": source_record(INSTRUCTION_AUDIT) if INSTRUCTION_AUDIT.exists() else None}},
        "watermark": {"active_braxon_watermark": ACTIVE_WATERMARK, "algorithm": "SHA-256", "record_id": record["record_id"], "input_hashes": {key: source_record(path) for key, path in {**paths, "resonance": RESONANCE, "tone_guide": TONE_GUIDE}.items()}, "schema_hash": source_record(SCHEMA), "preflight_hash": source_record(PREFLIGHT) if PREFLIGHT.exists() else None, "payload_hash": None},
        "execution_boundary": {"prose_generation_permitted": False, "no_generated_prose": True, "staging_required": True, "promotion_requires_human_review": True, "required_checks": ["record_id uniqueness", "source hash verification", "authored constraint presence", "tone/cadence/style gate", "bounded rolling state", "canonical watermark verification"], "unmapped_anchor_policy": "fail closed for realization; remain explicit in payload", "generated_prose_field": None}
    }
    canonical = json.dumps(payload, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()
    payload["watermark"]["payload_hash"] = sha256_bytes(canonical)
    return payload


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--record-id")
    ap.add_argument("--book", default="B01")
    ap.add_argument("--output", type=Path, required=True)
    ap.add_argument("--state-ledger", type=Path, default=DEFAULT_STATE)
    args = ap.parse_args()
    paths = {"metadata": META, "flavor": FLAVOR, "dynamics": DYNAMICS, "pip": PIP, "generated_flavor": GEN_FLAVOR, "relationships": RELATIONSHIPS, "books": BOOKS}
    meta = read_tsv(META)
    record = select_record(meta, args.record_id, args.book)
    payload = compile_payload(record, paths, args.state_ledger)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(payload, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps({"status": "pass", "record_id": record["record_id"], "output": relpath(args.output), "payload_hash": payload["watermark"]["payload_hash"], "prose_generation_permitted": False, "unmapped_anchor_count": len(payload["constraints"]["unmapped_anchor_ids"])}, sort_keys=True))
    return 0

if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, ValueError, json.JSONDecodeError, FileNotFoundError) as exc:
        print(json.dumps({"status": "blocked", "error": str(exc), "prose_generation_permitted": False}, sort_keys=True), file=sys.stderr)
        raise SystemExit(2)
