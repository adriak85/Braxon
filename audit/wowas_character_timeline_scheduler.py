#!/usr/bin/env python3
"""Schedule all generated character timelines onto existing WOWAS scenes."""
from __future__ import annotations

import csv
import hashlib
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1] / "crates/wowas-final-edition-v10"
GEN = ROOT / "canon/active/generated"
ACTIVE = ROOT / "canon/active"
CHARS = GEN / "wowas_generated_characters_5000.tsv"
SCENES = ACTIVE / "scene_index_reasonable_window.tsv"
OUT_SCHEDULE = GEN / "wowas_character_timeline_schedule.tsv"
OUT_GROUPS = GEN / "wowas_scene_character_group_map.tsv"
PHASES = ("introduction", "development", "relationship_consequence", "return_or_resolution")


def read(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8", errors="replace") as fh: return list(csv.DictReader(fh, delimiter="\t"))


def write(path: Path, rows: list[dict[str, str]]) -> None:
    fields = list(rows[0])
    with path.open("w", newline="", encoding="utf-8") as fh:
        out = csv.DictWriter(fh, fieldnames=fields, delimiter="\t"); out.writeheader(); out.writerows(rows)


def book_num(row: dict[str, str]) -> int:
    return int(row.get("book_anchor", row.get("book_num", "B0")).lstrip("BV") or 0)


def group_id(row: dict[str, str]) -> str:
    key = "|".join((row.get("book_anchor", ""), row.get("tier", ""), row.get("role", ""), row.get("region", "")))
    return "CG-" + hashlib.sha1(key.encode()).hexdigest()[:12]


def main() -> int:
    chars = read(CHARS); scenes = read(SCENES)
    scenes_by_book = defaultdict(list)
    for scene in scenes: scenes_by_book[int(scene.get("book_num", "0") or 0)].append(scene)
    grouped = defaultdict(list)
    for char in chars: grouped[(book_num(char), group_id(char))].append(char)
    schedule = []
    scene_groups = defaultdict(list)
    for (book, gid), members in sorted(grouped.items()):
        available = scenes_by_book.get(book, [])
        if not available: available = scenes_by_book.get(1, [])
        for ordinal, char in enumerate(sorted(members, key=lambda x: x.get("character_id", ""))):
            for phase_index, phase in enumerate(PHASES):
                scene = available[(ordinal * len(PHASES) + phase_index) % len(available)]
                sid = scene.get("scene_id", "")
                schedule.append({
                    "schedule_serial": "CTS-" + hashlib.sha1(f"{char.get('character_id')}|{phase}|{sid}".encode()).hexdigest()[:12],
                    "character_id": char.get("character_id", ""),
                    "character_name": char.get("name", ""),
                    "book_num": f"B{book:02d}",
                    "character_group_id": gid,
                    "group_basis": "book|tier|role|region",
                    "timeline_phase": phase,
                    "assigned_scene_id": sid,
                    "event_beat_id": scene.get("event_beat_id", ""),
                    "relationship_layer": "relationship_obligation" if phase == "relationship_consequence" else "characters|world|creatures|quests",
                    "capacity_mode": "shared_existing_scene",
                    "target_policy": "no_new_scene_created",
                })
                scene_groups[sid].append(f"{char.get('character_id')}:{gid}:{phase}")
    write(OUT_SCHEDULE, schedule)
    group_rows = [{"scene_id": sid, "character_group_count": str(len(set(x.split(":")[1] for x in values))), "character_assignment_count": str(len(values)), "character_group_assignments": "|".join(sorted(values)), "capacity_mode": "shared_existing_scene"} for sid, values in sorted(scene_groups.items())]
    write(OUT_GROUPS, group_rows)
    print(f"characters={len(chars)} schedule_rows={len(schedule)} scenes_used={len(scene_groups)} groups={len(grouped)} target_scene_count={len(scenes)} main_story_target_preserved=true")
    return 0

if __name__ == "__main__": raise SystemExit(main())
