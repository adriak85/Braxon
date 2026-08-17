#!/usr/bin/env python3
"""Fail-closed originality and scale audit for the WoWAS operational surfaces."""
from __future__ import annotations
import csv, hashlib, json, re
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1] / "crates/wowas-final-edition-v10"
INDEX = ROOT / "canon/active/scene_index_reasonable_window.tsv"
CHAR = ROOT / "canon/active/character_timeline_lattice_v14_33.tsv"
CREATURE = ROOT / "canon/active/creature_registry_target_5000.tsv"
REGISTRY = ROOT / "canon/canonical_story_tree/characters/06_CHARACTER_REGISTRY.json"

def read_tsv(path):
    with path.open(newline="", encoding="utf-8", errors="replace") as f: return list(csv.DictReader(f, delimiter="\t"))
def norm(text): return re.sub(r"\s+", " ", re.sub(r"[^a-z0-9 ]", " ", (text or "").lower())).strip()
def duplicates(rows, field):
    c=Counter(norm(r.get(field,"")) for r in rows if norm(r.get(field,"")))
    return {k:v for k,v in c.items() if v>1}

def main():
    scenes=read_tsv(INDEX); chars=read_tsv(CHAR); creatures=read_tsv(CREATURE)
    ids=duplicates(scenes,"scene_id"); descriptions=duplicates(scenes,"brief_scene_description")
    char_ids=duplicates(chars,"character_id"); char_names=duplicates(chars,"name")
    creature_ids=duplicates(creatures,"creature_id"); creature_names=duplicates(creatures,"species_name")
    source_counts=Counter(r.get("source_type","") for r in scenes)
    main_story=sum(1 for r in scenes if r.get("main_story_status") == "MAIN_STORY_EXPANDED" or r.get("source_type") in {"DIRECT_SOURCE","APPROVED_CHUNK","MAIN_STORY","MAIN_STORY_EXPANDED"})
    event_beats=sum(1 for r in scenes if r.get("event_beat_id") or "event" in norm(r.get("source_type")) or "beat" in norm(r.get("source_type")) or "event" in norm(r.get("source_trace")) or "beat" in norm(r.get("source_trace")))
    registry=json.loads(REGISTRY.read_text())
    target=registry.get("world_population_target",0)
    print(f"scenes={len(scenes)} duplicate_ids={len(ids)} duplicate_descriptions={len(descriptions)}")
    print(f"characters={len(chars)} duplicate_ids={len(char_ids)} duplicate_names={len(char_names)}")
    print(f"creatures={len(creatures)} duplicate_ids={len(creature_ids)} duplicate_names={len(creature_names)}")
    print(f"main_story={main_story} event_beats={event_beats} source_types={dict(source_counts)}")
    background = ROOT / "canon/active/generated/wowas_background_population_2000000.tsv"
    materialized = sum(1 for _ in background.open(encoding="utf-8")) - 1 if background.exists() else 0
    print(f"population_target={target} materialized_background_records={materialized}")
    report=Path("/home/ubuntu/Braxon-final-audit/coverage/wowas_originality_report.tsv"); report.parent.mkdir(parents=True,exist_ok=True)
    with report.open("w") as f:
        f.write("metric\tvalue\n")
        for k,v in [("scenes",len(scenes)),("duplicate_scene_ids",len(ids)),("duplicate_scene_descriptions",len(descriptions)),("characters",len(chars)),("duplicate_character_ids",len(char_ids)),("duplicate_character_names",len(char_names)),("creatures",len(creatures)),("duplicate_creature_ids",len(creature_ids)),("duplicate_creature_names",len(creature_names)),("main_story",main_story),("event_beats",event_beats),("population_target",target),("materialized_background_records",materialized)]: f.write(f"{k}\t{v}\n")
    if ids or descriptions or char_ids or char_names or creature_ids or creature_names: return 2
    return 0
if __name__ == "__main__": raise SystemExit(main())
