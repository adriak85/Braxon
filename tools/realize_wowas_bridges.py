#!/usr/bin/env python3
"""Build an offline, source-traceable WOWAS bridge-realization worksheet.

This intentionally does not invent canonical prose. It converts the existing ordered
stretch manifest plus its source ledgers into one deterministic manual-realization
queue. A packet is not promoted to canon until a human supplies prose and the
verification pass accepts it.
"""
from __future__ import annotations

import csv
import json
from pathlib import Path

ROOT = Path("crates/wowas-final-edition-v10/canon")
MANIFEST = Path("config/wowas/ordered_stretched_spine_manifest.json")
OUT = Path("audit/wowas_manual_realization_queue.md")


def read_tsv(path: Path):
    with path.open(encoding="utf-8", newline="") as f:
        return list(csv.DictReader(f, delimiter="\t"))


def main() -> None:
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    chars = read_tsv(ROOT / "active/character_timeline_lattice_v14_33.tsv")
    encounters = read_tsv(ROOT / "active/generated/wowas_character_encounters.tsv")
    events = read_tsv(ROOT / "active/generated/wowas_timeline_event_map.tsv")

    chars_by_book = {}
    for row in chars:
        b = row.get("book_anchor", "").removeprefix("B")
        if b:
            chars_by_book.setdefault(str(int(b)), []).append(row)
    encounters_by_book = {}
    for row in encounters:
        encounters_by_book.setdefault(str(row.get("book_num", "")), []).append(row)
    events_by_book = {}
    for row in events:
        b = row.get("book_num", "").removeprefix("B")
        if b:
            events_by_book.setdefault(str(int(b)), []).append(row)

    lines = [
        "# WOWAS Manual Prose Realization Queue",
        "",
        "This is an offline realization queue, not canonical prose. Existing core scenes remain untouched.",
        "Every packet below must be realized from its cited source anchors before promotion to canon.",
        "Do not reorder books or replace existing core scenes.",
        "",
    ]
    total = 0
    for book in manifest["books"]:
        pending = [x for x in book["ordered_slots"] if x.get("kind") == "ordered_bridge_or_reconstruction_packet"]
        if not pending:
            continue
        total += len(pending)
        b = str(book["order"])
        lines += [f"## Book {book['order']:02d} — {book['title']}", ""]
        for packet in pending:
            lines += [
                f"### {packet['scene_id']} — {packet['title']}",
                "- Status: `needs_prose_realization`",
                f"- Placement: `{packet.get('placement', '')}`",
                f"- Character anchor: `{packet.get('source_character_id', '')}` — {packet.get('source_character_name', '')}",
                f"- Character role/region: {packet.get('source_role', '')} / {packet.get('source_region', '')}",
                f"- Source anchor: `{packet.get('source_anchor', '')}`",
                f"- Encounter anchor: `{packet.get('encounter_id', '')}`",
                f"- Event anchor: `{packet.get('event_id', '')}`",
                f"- Book function: {packet.get('book_function', '')}",
                f"- Arc band: {packet.get('arc_band', '')}",
                "- Realization requirements: preserve continuity; do not contradict cited source; do not replace neighboring core scenes; retain the packet's placement.",
                "- Prose: **MANUAL ENTRY REQUIRED**",
                "",
            ]
    lines += [f"Total packets requiring prose realization: **{total}**", ""]
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text("\n".join(lines), encoding="utf-8")
    print(json.dumps({"pending_packets": total, "output": str(OUT)}, indent=2))


if __name__ == "__main__":
    main()
