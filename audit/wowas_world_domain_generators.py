#!/usr/bin/env python3
"""Generate deterministic provenance maps for WOWAS world/content domains."""
from __future__ import annotations

import csv
import hashlib
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1] / "crates/wowas-final-edition-v10"
CANON = ROOT / "canon"
ACTIVE = CANON / "active"
GEN = ACTIVE / "generated"
TREE = CANON / "canonical_story_tree"

DOMAIN_TERMS = {
    "artifact": ("artifact", "relic", "cloak", "gem", "cello", "diary", "blue light", "second sun", "weapon"),
    "location": ("oasis", "river", "road", "gate", "basin", "marsh", "ashfields", "thornridge", "deepwood", "county", "zone", "city", "mountain"),
    "faction": ("faction", "court", "kingdom", "council", "house", "order", "guild", "tribe"),
    "magic": ("magic", "spell", "frequency", "resonance", "thread", "stasis", "dark matter"),
    "economy": ("economy", "market", "debt", "trade", "trader", "caravan", "resource"),
    "hazard": ("hazard", "storm", "ripper", "pressure", "danger", "mortality", "mutation", "crisis"),
    "route": ("route", "road", "corridor", "path", "crossing", "switchback", "underpass", "trail"),
    "quest": ("quest", "objective", "obligation", "accounting", "rescue", "census", "return", "gate"),
    "structure": ("structure", "shrine", "well", "bridge", "portal", "tower", "shelter", "settlement"),
    "culture": ("culture", "custom", "ritual", "family", "chosen", "memorial", "law", "tradition"),
    "population": ("population", "lives", "census", "mortality", "settlement", "species"),
}


def serial(domain: str, value: str) -> str:
    return f"WDM-{domain.upper()}-{hashlib.sha1(f'{domain}|{value}'.encode()).hexdigest()[:12]}"


def tsv(value: str) -> str:
    return re.sub(r"[\t\r\n]+", " ", value).strip()


def source_text() -> list[tuple[str, str]]:
    paths = list(TREE.glob("books/*/book_content.txt")) + [TREE / "all_25_books_latest_full_cycle.txt", TREE / "world/wowas_world_zone_map.json", CANON / "wowas_magic_system_patch_v10.md"]
    out = []
    for path in paths:
        if path.exists(): out.append((path.relative_to(ROOT).as_posix(), path.read_text(encoding="utf-8", errors="replace")))
    return out


def extract_json_strings(path: Path) -> list[str]:
    if not path.exists(): return []
    try: data = json.loads(path.read_text(encoding="utf-8", errors="replace"))
    except json.JSONDecodeError: return []
    values = []
    def walk(node):
        if isinstance(node, dict):
            for key, value in node.items():
                if key.lower() in {"name", "title", "zone", "region", "county", "location", "landmark", "route"} and isinstance(value, str): values.append(value)
                walk(value)
        elif isinstance(node, list):
            for value in node: walk(value)
    walk(data)
    return values


def extract_domain(domain: str, texts: list[tuple[str, str]]) -> list[dict[str, str]]:
    terms = DOMAIN_TERMS[domain]
    rows = {}
    for source, text in texts:
        lower = text.lower()
        for term in terms:
            if term not in lower: continue
            for match in re.finditer(r"(?i)(?:[A-Z][A-Za-z0-9'’-]+(?:\s+[A-Z][A-Za-z0-9'’-]+){0,4})", text):
                name = match.group(0).strip(" .,:;()[]")
                if len(name) < 3 or len(name) > 100: continue
                context = text[max(0, match.start()-180):min(len(text), match.end()+180)]
                if term not in context.lower(): continue
                key = re.sub(r"\s+", " ", name).lower()
                rec = rows.setdefault(key, {"serial": serial(domain, key), "domain": domain, "name": name, "source_refs": set(), "term_hits": set(), "status": "reference_extracted_pending_domain_review"})
                rec["source_refs"].add(source); rec["term_hits"].add(term)
    return [{**{k: v for k, v in rec.items() if k not in {"source_refs", "term_hits"}}, "source_refs": "|".join(sorted(rec["source_refs"])), "term_hits": "|".join(sorted(rec["term_hits"])), "interaction_layers": "timeline|relationships|scenes|quests|world", "generator_status": "implemented_reference_map"} for rec in rows.values()]


def write(path: Path, rows: list[dict[str, str]]) -> None:
    fields = ["serial", "domain", "name", "source_refs", "term_hits", "interaction_layers", "generator_status", "status"]
    with path.open("w", newline="", encoding="utf-8") as fh:
        out = csv.DictWriter(fh, fieldnames=fields, delimiter="\t"); out.writeheader(); out.writerows(rows)


def main() -> int:
    texts = source_text()
    zone_values = extract_json_strings(TREE / "world/wowas_world_zone_map.json")
    for value in zone_values: texts.append(("canonical_story_tree/world/wowas_world_zone_map.json", value))
    total = 0
    counts = []
    for domain in DOMAIN_TERMS:
        rows = extract_domain(domain, texts)
        write(GEN / f"wowas_{domain}_domain_map.tsv", rows)
        counts.append(f"{domain}={len(rows)}"); total += len(rows)
    print(f"total_domain_records={total} " + " ".join(counts))
    return 0

if __name__ == "__main__": raise SystemExit(main())
