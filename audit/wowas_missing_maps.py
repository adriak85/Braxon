#!/usr/bin/env python3
"""Materialize missing WOWAS linkage maps from canonical and generated records."""
from __future__ import annotations

import csv
import hashlib
import re

PROFILES = ((0,45,'gay_male','he/him','gay male identity is ordinary life context'),(45,55,'bisexual_male','he/him','bisexual identity is ordinary life context'),(55,63,'pansexual_male','he/they','pansexual identity is ordinary life context'),(63,70,'trans_gay_male','he/him','trans gay male identity is ordinary life context'),(70,78,'lesbian','she/her','lesbian identity is ordinary life context'),(78,85,'bisexual','she/they','bisexual identity is ordinary life context'),(85,90,'queer_nonbinary','they/them','queer nonbinary identity is ordinary life context'),(90,100,'straight_ally','varied','sexuality is not narratively exceptional'))
def identity_for(cid):
    value=int(hashlib.sha1(cid.encode()).hexdigest()[:8],16)%100
    for start,end,name,pronouns,prose in PROFILES:
        if start <= value < end:return name,pronouns,prose
    return PROFILES[-1][2:]
def role_for(identity,cid):
    if identity not in {'gay_male','bisexual_male','pansexual_male','trans_gay_male'}:return 'not_specified','adult role is not required for this record'
    value=int(hashlib.sha1((cid+'|adult-role').encode()).hexdigest()[:8],16)%100
    return ('top_role_preference','adult role preference is ordinary private context and is not unusual in prose') if value < 70 else (('versatile_role_preference','adult role preference is ordinary private context and is not unusual in prose') if value < 90 else ('bottom_role_preference','adult role preference is ordinary private context and is not unusual in prose'))
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1] / "crates/wowas-final-edition-v10"
CANON = ROOT / "canon"
ACTIVE = CANON / "active"
GEN = ACTIVE / "generated"
OUT = GEN


def read(path: Path) -> list[dict[str, str]]:
    if not path.exists(): return []
    with path.open(newline="", encoding="utf-8", errors="replace") as fh: return list(csv.DictReader(fh, delimiter="\t"))


def serial(kind: str, value: str) -> str:
    return f"MAP-{kind.upper()}-{hashlib.sha1(value.encode()).hexdigest()[:12]}"


def book_num(row: dict[str, str]) -> int:
    for key in ("book_num", "book", "volume", "book_anchor"):
        m = re.search(r"(?:B|V)?(\d{1,3})", row.get(key, ""))
        if m: return int(m.group(1))
    return 0


def write(name: str, fields: list[str], rows: list[dict[str, str]]) -> None:
    with (OUT / name).open("w", newline="", encoding="utf-8") as fh:
        out = csv.DictWriter(fh, fieldnames=fields, delimiter="\t")
        out.writeheader(); out.writerows(rows)


def main() -> int:
    spine = read(ACTIVE / "book_spine_33.tsv")
    manifest = {int(r.get("book_num", "0")): r for r in read(ACTIVE / "novel_manifest_33.tsv") if r.get("book_num", "").isdigit()}
    dirs = {int(m.group(1)) for p in (CANON / "canonical_story_tree/books").iterdir() if p.is_dir() and (m := re.search(r"Book_(\d+)_", p.name))}
    chars = read(GEN / "wowas_generated_characters_5000.tsv")
    encounters = read(GEN / "wowas_character_encounters.tsv")
    scenes = read(ACTIVE / "scene_index_reasonable_window.tsv")
    creatures = read(ACTIVE / "creature_registry_target_5000.tsv")

    book_rows = []
    for row in spine:
        n = book_num(row); m = manifest.get(n, {})
        book_rows.append({"serial": serial("book", f"B{n:02d}"), "book_num": f"B{n:02d}", "title": row.get("active_title", ""), "arc_band": row.get("arc_band", ""), "function": row.get("function", ""), "physical_content": str(n in dirs).lower(), "prose_status": m.get("source_status", "missing_manifest"), "required_next": "create_or_review_prose_content" if n not in dirs else "linked_to_canon_content"})
    write("wowas_book_contract_map_33.tsv", list(book_rows[0]), book_rows)

    timeline_rows = []
    for row in chars:
        cid = row.get("character_id", "")
        n = book_num(row)
        timeline_rows.append({"serial": serial("timeline", cid), "character_id": cid, "book_num": f"B{n:02d}" if n else "", "event_id": f"TL-{cid}-INTRO", "event_type": "character_introduction", "event_text": row.get("story_background_law", "generated character background"), "linked_scene_serial": serial("scene", f"B{n:02d}") if n else "", "status": "generated_linked"})
    write("wowas_timeline_event_map.tsv", list(timeline_rows[0]) if timeline_rows else ["serial"], timeline_rows)

    scenes_by_book = {}
    for scene in scenes:
        scenes_by_book.setdefault(book_num(scene), []).append(scene)
    relationship_rows = []
    for i, row in enumerate(chars, 1):
        cid = row.get("character_id", f"CHAR-{i:05d}"); name = row.get("name", cid); n = book_num(row)
        available = scenes_by_book.get(n) or scenes_by_book.get(1) or []
        scene_link = available[(i - 1) % len(available)].get("scene_id", "") if available else ""
        for layer, other, text in (("ally", f"ally::{cid}", "cooperation and mutual protection"), ("rival", f"rival::{cid}", "competing obligation creates pressure"), ("kin", f"kin::{cid}", "inheritance or chosen-family continuity"), ("mentor", f"mentor::{cid}", "training or corrective knowledge"), ("dependent", f"dependent::{cid}", "care obligation changes available choices"), ("faction", f"faction::B{n:02d}", "group pressure and political consequence"), ("creature", f"creature::B{n:02d}", "ecological signal or nonhuman bond"), ("location", f"location::B{n:02d}", "place pressure alters the relationship"), ("world_system", "world::canonical", "world-state consequence")):
            identity = row.get('identity_profile','unspecified'); pronouns = row.get('pronouns','varied'); prose = row.get('prose_treatment','identity is ordinary life context'); role = row.get('adult_role_profile','not_specified'); role_prose = row.get('role_prose_treatment','adult role is not required for this record')
            relationship_rows.append({"serial": serial("bond", f"{cid}|{layer}|{other}"), "character_id": cid, "character_name": name, "book_num": f"B{n:02d}" if n else "", "interaction_layer": layer, "other_id": other, "event_id": f"REL-{cid}-{layer.upper()}", "relationship_text": text + ("; identity is integrated without exceptional framing" if layer in {"ally","kin","mentor","dependent"} else ""), "scene_link": scene_link, "status": "generated_linked_scaffold", "identity_profile": identity, "pronouns": pronouns, "age_band": "young_adult_18_plus", "adult_role_eligibility": "eligible_18_plus_only", "adult_role_profile": role, "prose_treatment": prose, "role_prose_treatment": role_prose, "content_rating": "young_adult_non_graphic"})
    write("wowas_relationship_ledger.tsv", list(relationship_rows[0]) if relationship_rows else ["serial"], relationship_rows)

    scene_rows = []
    for row in scenes:
        sid = row.get("scene_id", ""); n = book_num(row); flags = row.get("domain_flags", "")
        scene_rows.append({"serial": serial("scene", sid), "scene_id": sid, "book_num": f"B{n:02d}" if n else "", "event_beat_id": row.get("event_beat_id", ""), "character_link": row.get("inferred_character_ids", ""), "creature_link": row.get("creature_refs", ""), "quest_link": row.get("quest_hook", ""), "location_link": row.get("corridor_region_anchor", "") or row.get("county_anchor", ""), "world_link": row.get("world_introduction_anchor", ""), "layers": flags, "status": "linked" if flags else "requires_layer_review"})
    write("wowas_scene_layer_map.tsv", list(scene_rows[0]) if scene_rows else ["serial"], scene_rows)

    creature_rows = []
    for row in creatures:
        cid = row.get("creature_id", ""); creature_rows.append({"serial": serial("creature", cid), "creature_id": cid, "species_name": row.get("name", ""), "biome": row.get("biome", ""), "scene_link_rule": "ecology_pressure_mode|creature_refs", "status": "available_for_scene_linkage"})
    write("wowas_creature_scene_map.tsv", list(creature_rows[0]) if creature_rows else ["serial"], creature_rows)

    print(f"books={len(book_rows)} missing_physical_books={sum(r['physical_content']=='false' for r in book_rows)} characters={len(chars)} relationship_rows={len(relationship_rows)} timeline_rows={len(timeline_rows)} scenes={len(scene_rows)} creatures={len(creature_rows)}")
    return 0

if __name__ == "__main__": raise SystemExit(main())
