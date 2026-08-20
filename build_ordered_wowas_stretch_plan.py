import csv
import hashlib
import json
from collections import defaultdict
from pathlib import Path

ROOT = Path('crates/wowas-final-edition-v10/canon')

def tsv(path):
    with path.open(encoding='utf-8', newline='') as f:
        return list(csv.DictReader(f, delimiter='\t'))

spine = tsv(ROOT / 'active/book_spine_33.tsv')
scenes = tsv(ROOT / 'wowas_clean_scene_index_v2.tsv')
chars = tsv(ROOT / 'active/character_timeline_lattice_v14_33.tsv')
encounters = tsv(ROOT / 'active/generated/wowas_character_encounters.tsv')
events = tsv(ROOT / 'active/generated/wowas_timeline_event_map.tsv')

active = [r for r in scenes if r.get('source_layer') == 'DIRECT_SOURCE' and r.get('title_status') == 'kept' and r.get('brief_scene_description', '').strip() and r.get('source_type') in {'APPROVED_CHUNK', 'SCENE_EXPANSION_EXTRACT'}]
seen = set(); active_unique = []
for row in active:
    if row['scene_id'] not in seen:
        seen.add(row['scene_id']); active_unique.append(row)
by_book = defaultdict(list)
for row in active_unique: by_book[row['book_num']].append(row)
chars_by_book = defaultdict(list)
for row in chars:
    anchor = row.get('book_anchor', '').removeprefix('B')
    if anchor: chars_by_book[str(int(anchor))].append(row)
enc_by_book = defaultdict(list)
for row in encounters: enc_by_book[str(row.get('book_num', ''))].append(row)
events_by_book = defaultdict(list)
for row in events:
    raw = row.get('book_num', '').removeprefix('B')
    if raw: events_by_book[str(int(raw))].append(row)

plans = []
for spine_row in spine:
    number = spine_row['book_num']
    existing = by_book.get(number, [])
    target = max(12, min(48, len(existing) if len(existing) >= 12 else max(12, len(chars_by_book[number]) // 10)))
    target = max(target, len(existing))
    bridge_by_anchor = defaultdict(list)
    for i, row in enumerate(existing, 1):
        bridge_by_anchor[i - 1] = []
    added = target - len(existing)
    for j in range(added):
        # Assign each bridge to an existing-scene boundary; the existing scene order is never changed.
        if existing:
            left_index = min(len(existing) - 1, (j * len(existing)) // added)
            left = existing[left_index]
            right = existing[min(len(existing) - 1, left_index + 1)]
            packet_id = f"{spine_row['book_code']}_X{j + 1:03d}"
            slot_label = f"between:{left['scene_id']}→{right['scene_id']}"
            anchor = left_index
        else:
            packet_id = f"{spine_row['book_code']}_R{j + 1:03d}"
            slot_label = 'reconstruct_from_spine_function'
            anchor = 0
        candidates = chars_by_book[number]
        c = candidates[j % len(candidates)] if candidates else {}
        encounter = enc_by_book[number][j % len(enc_by_book[number])] if enc_by_book[number] else {}
        event = events_by_book[number][j % len(events_by_book[number])] if events_by_book[number] else {}
        bridge_by_anchor.setdefault(anchor, []).append({
            'ordinal': None,
            'scene_id': packet_id,
            'title': f"{spine_row['active_title']} — continuity and lived-world expansion {j + 1}",
            'kind': 'ordered_bridge_or_reconstruction_packet',
            'status': 'needs_prose_realization',
            'placement': slot_label,
            'source_character_id': c.get('character_id', ''),
            'source_character_name': c.get('name', ''),
            'source_role': c.get('role', ''),
            'source_region': c.get('region', ''),
            'source_anchor': c.get('source_anchor', ''),
            'encounter_id': encounter.get('encounter_id', ''),
            'event_id': event.get('event_id', ''),
            'book_function': spine_row['function'],
            'arc_band': spine_row['arc_band'],
            'must_not_reorder_spine': True,
            'must_not_replace_existing_core_scene': True,
        })
    slots = []
    if existing:
        for i, row in enumerate(existing):
            slots.append({
                'ordinal': i + 1,
                'scene_id': row['scene_id'],
                'title': row['clean_title'],
                'kind': 'existing_core_scene',
                'status': 'preserve_exact_order',
                'source_trace': row.get('source_trace', ''),
            })
            slots.extend(bridge_by_anchor.get(i, []))
    else:
        slots.extend(bridge_by_anchor.get(0, []))
    # sort existing and added into book-local order: existing slots remain in source order;
    # bridge packets use explicit placement labels and are not promoted as canonical until realized.
    plans.append({
        'order': int(number),
        'book_code': spine_row['book_code'],
        'title': spine_row['active_title'],
        'function': spine_row['function'],
        'arc_band': spine_row['arc_band'],
        'existing_scene_count': len(existing),
        'target_scene_packet_count': target,
        'added_bridge_or_reconstruction_packets': added,
        'ordered_slots': slots,
        'status': 'stretched_plan_ready',
    })

payload = {
    'schema': 'wowas.ordered_stretched_spine.v1',
    'series': 'Whispers of Willow and Stone',
    'invariants': {
        'book_order': list(range(1, 34)),
        'existing_scene_order_preserved': True,
        'no_book_merge': True,
        'no_book_reorder': True,
        'no_core_scene_replacement': True,
        'added_packets_require_prose_realization': True,
        'generated_character_links_are_lattice_candidates': True,
    },
    'counts': {
        'books': len(plans),
        'existing_scene_packets': sum(p['existing_scene_count'] for p in plans),
        'target_scene_packets': sum(p['target_scene_packet_count'] for p in plans),
        'added_packets': sum(p['added_bridge_or_reconstruction_packets'] for p in plans),
    },
    'books': plans,
    'source_hashes': {
        str(path.relative_to(ROOT)): hashlib.sha256(path.read_bytes()).hexdigest()
        for path in [ROOT / 'active/book_spine_33.tsv', ROOT / 'wowas_clean_scene_index_v2.tsv', ROOT / 'active/character_timeline_lattice_v14_33.tsv', ROOT / 'active/generated/wowas_character_encounters.tsv', ROOT / 'active/generated/wowas_timeline_event_map.tsv']
    },
}
for out in [Path('audit/wowas_ordered_stretch_plan.json'), Path('config/wowas/ordered_stretched_spine_manifest.json')]:
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(payload, indent=2, ensure_ascii=False) + '\n', encoding='utf-8')
print(json.dumps({'counts': payload['counts'], 'order': [b['order'] for b in plans], 'books_with_added_packets': sum(b['added_bridge_or_reconstruction_packets'] > 0 for b in plans)}, indent=2))
