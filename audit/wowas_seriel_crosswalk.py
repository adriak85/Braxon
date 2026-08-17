#!/usr/bin/env python3
"""Build the WOWAS SERIEL crosswalk and bottom-footnote index.

Every source record gets one stable serial. Related records are linked by shared
book, character/name, event, creature, quest, location, and world fields. The
outputs are regenerated from canonical inputs so no hand-maintained thousand-
row update is required.
"""
from __future__ import annotations

import csv
import hashlib
import re
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1] / "crates/wowas-final-edition-v10"
CANON = ROOT / "canon"
ACTIVE = CANON / "active"
GENERATED = ACTIVE / "generated"
RECON = ROOT / "reconstruction"
OUT_CROSSWALK = RECON / "SERIEL_CROSSWALK.tsv"
OUT_FOOTNOTES = RECON / "SERIEL_FOOTNOTES.md"

SOURCES = (
    ("book", ACTIVE / "novel_manifest_33.tsv", "book_num"),
    ("character", GENERATED / "wowas_generated_characters_5000.tsv", "character_id"),
    ("timeline", ACTIVE / "character_timeline_lattice_v14_33.tsv", "character_id"),
    ("encounter", GENERATED / "wowas_character_encounters.tsv", "encounter_id"),
    ("scene", ACTIVE / "scene_index_reasonable_window.tsv", "scene_id"),
    ("merged_beats", ACTIVE / "wowas_merged_plot_beats.tsv", "merge_serial"),
    ("creature", ACTIVE / "creature_registry_target_5000.tsv", "creature_id"),
    ("book_map", GENERATED / "wowas_book_contract_map_33.tsv", "serial"),
    ("timeline_map", GENERATED / "wowas_timeline_event_map.tsv", "serial"),
    ("relationship_map", GENERATED / "wowas_relationship_ledger.tsv", "serial"),
    ("scene_map", GENERATED / "wowas_scene_layer_map.tsv", "serial"),
    ("creature_map", GENERATED / "wowas_creature_scene_map.tsv", "serial"),
    ("timeline_schedule", GENERATED / "wowas_character_timeline_schedule.tsv", "schedule_serial"),
    ("scene_groups", GENERATED / "wowas_scene_character_group_map.tsv", "scene_id"),
    ("world_roles", GENERATED / "wowas_character_world_role_map.tsv", "canonical_character_id"),
    ("attention", GENERATED / "wowas_character_attention_projection.tsv", "projection_serial"),
    ("user_preferences", GENERATED / "wowas_user_preference_profile.template.tsv", "user_id"),
    ("resolved_projection", GENERATED / "wowas_resolved_user_projection.tsv", "projection_serial"),
    ("real_world_sources", GENERATED / "wowas_real_world_source_registry.tsv", "source_serial"),
    ("real_world_alignment", GENERATED / "wowas_real_world_wowas_alignment.tsv", "alignment_serial"),
    ("real_world_domains", GENERATED / "wowas_real_world_domain_alignment.tsv", "domain_alignment_serial"),
)


def read_tsv(path: Path) -> list[dict[str, str]]:
    if not path.exists():
        return []
    with path.open(newline="", encoding="utf-8", errors="replace") as fh:
        return list(csv.DictReader(fh, delimiter="\t"))


def clean(value: str) -> str:
    return re.sub(r"[^a-z0-9]+", " ", value.lower()).strip()


def book_key(row: dict[str, str]) -> str:
    for key in ("book_num", "book", "volume", "book_anchor"):
        value = row.get(key, "")
        match = re.search(r"(?:B|V)?(\d{1,3})", value)
        if match:
            return f"B{int(match.group(1)):02d}"
    return ""


def identity(kind: str, row: dict[str, str], key: str, ordinal: int) -> str:
    value = row.get(key, "").strip() or f"{kind}-{ordinal:08d}"
    digest = hashlib.sha1(f"{kind}|{value}|{book_key(row)}".encode()).hexdigest()[:10]
    return f"SRL-{kind.upper()}-{digest}"


def tokens(row: dict[str, str]) -> set[str]:
    values = []
    for key in ("character_id", "name", "core_character", "satellite_character", "inferred_character_names", "inferred_character_ids", "creature_id", "creature_refs", "book_num", "book_title", "event_beat_id", "quest_hook", "world_introduction_anchor", "corridor_region_anchor", "county_anchor", "ecology_pressure_mode"):
        values.append(row.get(key, ""))
    return {part for value in values for part in clean(value).split() if len(part) >= 3}


def footnote(serial: str, kind: str, key: str, source: Path) -> str:
    return f"[{serial}] {kind}:{key} from {source.relative_to(ROOT).as_posix()}"


def build() -> list[dict[str, str]]:
    records: list[dict[str, str]] = []
    for kind, path, key in SOURCES:
        for ordinal, row in enumerate(read_tsv(path), 1):
            record_id = row.get(key, "") or f"{kind}-{ordinal:08d}"
            records.append({
                "serial_id": identity(kind, row, key, ordinal),
                "record_type": kind,
                "record_id": record_id,
                "book_key": book_key(row),
                "source_file": path.relative_to(ROOT).as_posix(),
                "source_key": key,
                "relationship_tokens": "|".join(sorted(tokens(row))),
                "bottom_footnote_serial": identity(kind, row, key, ordinal),
                "linked_serials": "",
                "link_status": "unlinked",
            })
    root_records = []
    for root_kind in ("canon", "characters", "relationships", "timeline", "events", "creatures", "locations", "quests", "world"):
        root_records.append({
            "serial_id": f"SRL-ROOT-{root_kind.upper()}",
            "record_type": "root",
            "record_id": root_kind,
            "book_key": "",
            "source_file": "canon/active",
            "source_key": "root",
            "relationship_tokens": root_kind,
            "bottom_footnote_serial": f"SRL-ROOT-{root_kind.upper()}",
            "linked_serials": "",
            "link_status": "root",
        })
    records.extend(root_records)
    token_index: dict[str, set[int]] = defaultdict(set)
    book_index: dict[str, set[int]] = defaultdict(set)
    for i, record in enumerate(records):
        for token in record["relationship_tokens"].split("|"):
            if token:
                token_index[token].add(i)
        if record["book_key"]:
            book_index[record["book_key"]].add(i)
    for i, record in enumerate(records):
        candidates: list[int] = []
        seen: set[int] = set()
        if record["book_key"]:
            for candidate in sorted(book_index.get(record["book_key"], set())):
                if candidate != i and candidate not in seen:
                    candidates.append(candidate); seen.add(candidate)
                    if len(candidates) >= 120: break
        if len(candidates) < 120:
            for token in record["relationship_tokens"].split("|"):
                for candidate in sorted(token_index.get(token, set())):
                    if candidate != i and candidate not in seen:
                        candidates.append(candidate); seen.add(candidate)
                        if len(candidates) >= 120: break
                if len(candidates) >= 120: break
        # Keep linkage bounded and deterministic; all records retain their own serial.
        linked = sorted({records[j]["serial_id"] for j in candidates if records[j]["record_type"] != record["record_type"]})[:80]
        if record["record_type"] != "root" and not linked:
            fallback = {
                "book": "SRL-ROOT-CANON", "character": "SRL-ROOT-CHARACTERS", "timeline": "SRL-ROOT-TIMELINE",
                "encounter": "SRL-ROOT-RELATIONSHIPS", "scene": "SRL-ROOT-EVENTS", "creature": "SRL-ROOT-CREATURES",
            }.get(record["record_type"], "SRL-ROOT-CANON")
            linked = [fallback]
            record["link_status"] = "linked_via_root"
        else:
            record["link_status"] = "linked" if linked else "root"
        record["linked_serials"] = "|".join(linked)
    return records


def write(records: list[dict[str, str]]) -> None:
    RECON.mkdir(parents=True, exist_ok=True)
    fields = list(records[0]) if records else ["serial_id"]
    with OUT_CROSSWALK.open("w", newline="", encoding="utf-8") as fh:
        writer = csv.DictWriter(fh, fieldnames=fields, delimiter="\t")
        writer.writeheader()
        writer.writerows(records)
    with OUT_FOOTNOTES.open("w", encoding="utf-8") as fh:
        fh.write("# WOWAS SERIEL Bottom-Footnote Index\n\n")
        fh.write("Each canonical record receives a deterministic serial at the bottom of its footnote. The crosswalk is regenerated from canonical inputs; it is not manually maintained.\n\n")
        fh.write("| Serial | Record | Source | Linked serial count |\n|---|---|---|---:|\n")
        for record in records:
            links = [x for x in record["linked_serials"].split("|") if x]
            fh.write(f"| `{record['bottom_footnote_serial']}` | `{record['record_type']}:{record['record_id']}` | `{record['source_file']}` | {len(links)} |\n")
        fh.write("\n## Footnote rule\n\n")
        fh.write("> The serial is the final item in each record footnote and maps that piece to the generated crosswalk and every related canonical record discovered by stable identity, book, event, character, creature, quest, location, and world tokens.\n")


def main() -> int:
    records = build()
    write(records)
    linked = sum(record["link_status"] in {"linked", "linked_via_root", "root"} for record in records)
    print(f"records={len(records)} linked={linked} unlinked={len(records)-linked} crosswalk={OUT_CROSSWALK} footnotes={OUT_FOOTNOTES}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
