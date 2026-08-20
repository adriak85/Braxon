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
events = tsv(ROOT / 'active/generated/wowas_timeline_event_map.tsv')
encounters = tsv(ROOT / 'active/generated/wowas_character_encounters.tsv')
flavor = tsv(ROOT / 'active/authored_flavor/authored_character_flavor_lattice.tsv')

active = [r for r in scenes if r.get('source_layer') == 'DIRECT_SOURCE' and r.get('title_status') == 'kept' and r.get('brief_scene_description', '').strip() and r.get('source_type') in {'APPROVED_CHUNK', 'SCENE_EXPANSION_EXTRACT'}]
seen = set()
active_unique = []
for row in active:
    if row['scene_id'] not in seen:
        seen.add(row['scene_id'])
        active_unique.append(row)

by_book = defaultdict(list)
for row in active_unique:
    by_book[row['book_num']].append(row)
chars_by_book = defaultdict(list)
for row in chars:
    book = row.get('book_anchor', '').removeprefix('B')
    if book:
        chars_by_book[str(int(book))].append(row)
events_by_book = defaultdict(list)
for row in events:
    events_by_book[str(int(row.get('book_num', '').removeprefix('B')))].append(row)
enc_by_book = defaultdict(list)
for row in encounters:
    enc_by_book[str(row.get('book_num', ''))].append(row)

core = {'Pip', 'Pip (Indalwin Willowjayce)', 'Mack', 'Xethrolund', 'Solvaenkyr', 'Dervish', 'Rylos (Rylos Vayne Johnson)', 'Rolzen', 'Daisy May', 'Majiskii', 'Neith', 'Ursula'}
flavor_ids = {r.get('canonical_id') for r in flavor}
books = []
for spine_row in spine:
    number = spine_row['book_num']
    scene_rows = by_book.get(number, [])
    generated_rows = chars_by_book.get(number, [])
    eligible = [r for r in generated_rows if r.get('tier') in {'story', 'support', 'recurring-background'} and r.get('name') not in core]
    eligible.sort(key=lambda r: (r.get('tier', ''), r.get('character_id', '')))
    placements = []
    if scene_rows:
        for i, row in enumerate(eligible):
            scene = scene_rows[i % len(scene_rows)]
            placements.append({
                'character_id': row.get('character_id'),
                'character_name': row.get('name'),
                'tier': row.get('tier'),
                'role': row.get('role'),
                'house_pressure': row.get('house_pressure'),
                'region': row.get('region'),
                'source_anchor': row.get('source_anchor'),
                'orbit_group': hashlib.sha256('|'.join([row.get('source_anchor', ''), row.get('role', ''), row.get('region', '')]).encode()).hexdigest()[:16],
                'scene_id': scene['scene_id'],
                'scene_title': scene['clean_title'],
                'placement': 'generated_interweave',
                'reason': 'generated lattice row assigned to an existing active scene; core beat remains authoritative',
                'prose_status': 'requires_realization',
            })
    event_placements = []
    pending_events = []
    if scene_rows:
        for i, event in enumerate(events_by_book.get(number, [])[:len(scene_rows)]):
            scene = scene_rows[i]
            event_placements.append({
                'event_id': event.get('event_id'),
                'event_type': event.get('event_type'),
                'event_text': event.get('event_text'),
                'linked_generated_character': event.get('character_id'),
                'scene_id': scene['scene_id'],
                'scene_title': scene['clean_title'],
                'placement': 'event_interweave',
                'status': 'requires_prose_realization',
            })
    else:
        pending_events = [
            {
                'event_id': event.get('event_id'),
                'event_type': event.get('event_type'),
                'event_text': event.get('event_text'),
                'linked_generated_character': event.get('character_id'),
                'placement': 'reconstruction_packet',
                'status': 'requires_scene_reconstruction',
            }
            for event in events_by_book.get(number, [])
        ]
    reconstruction = None if scene_rows else {
        'required': True,
        'book_function': spine_row['function'],
        'arc_band': spine_row['arc_band'],
        'scene_packet_count_required': max(1, min(12, len(eligible) // 4 or 1)),
        'generated_character_candidates': len(eligible),
        'generated_event_candidates': len(events_by_book.get(number, [])),
        'prose_status': 'not_promotable_until_scene_packet_rebuilt',
    }
    books.append({
        'book_num': int(number),
        'book_code': spine_row['book_code'],
        'title': spine_row['active_title'],
        'core_function': spine_row['function'],
        'active_scene_count': len(scene_rows),
        'generated_character_candidates': len(eligible),
        'generated_character_placements': placements,
        'event_candidates': len(events_by_book.get(number, [])),
        'event_placements': event_placements,
        'pending_event_reconstruction': pending_events,
        'encounter_candidates': len(enc_by_book.get(number, [])),
        'reconstruction_packet': reconstruction,
        'status': 'rebalanced_index_ready' if scene_rows else 'reconstruction_packet_ready',
    })

payload = {
    'schema': 'wowas.canon.rebalance.v1',
    'rules': {
        'core_beats_preserved': True,
        'generated_characters_never_replace_core': True,
        'story_support_recurring_only_for_direct_scene_placement': True,
        'one_generated_event_per_active_scene_per_pass': True,
        'deep_background_environment_only_unless_explicit_encounter': True,
        'books_without_active_scene_packets_not_promoted_to_prose': True,
        'generated_rows_without_authored_flavor_remain_evidence_bound': True,
    },
    'counts': {
        'books': len(books),
        'active_scene_count': len(active_unique),
        'generated_character_rows': len(chars),
        'generated_event_rows': len(events),
        'encounter_rows': len(encounters),
        'flavor_rows': len(flavor),
        'placements': sum(len(b['generated_character_placements']) for b in books),
        'event_placements': sum(len(b['event_placements']) for b in books),
        'pending_event_reconstruction': sum(len(b['pending_event_reconstruction']) for b in books),
        'reconstruction_packet_count': sum(b['reconstruction_packet'] is not None for b in books),
    },
    'books': books,
    'cross_book_orbit_groups': sorted({p['orbit_group'] for b in books for p in b['generated_character_placements']}),
    'source_hashes': {
        str(path.relative_to(ROOT)): hashlib.sha256(path.read_bytes()).hexdigest()
        for path in [
            ROOT / 'active/book_spine_33.tsv',
            ROOT / 'wowas_clean_scene_index_v2.tsv',
            ROOT / 'active/character_timeline_lattice_v14_33.tsv',
            ROOT / 'active/generated/wowas_timeline_event_map.tsv',
            ROOT / 'active/generated/wowas_character_encounters.tsv',
            ROOT / 'active/authored_flavor/authored_character_flavor_lattice.tsv',
        ]
    },
}
out = Path('audit/wowas_canon_rebalance.json')
out.parent.mkdir(parents=True, exist_ok=True)
out.write_text(json.dumps(payload, indent=2, ensure_ascii=False) + '\n', encoding='utf-8')
config_out = Path('config/wowas/canon_rebalance_manifest.json')
config_out.parent.mkdir(parents=True, exist_ok=True)
config_out.write_text(json.dumps(payload, indent=2, ensure_ascii=False) + '\n', encoding='utf-8')
print(json.dumps({
    'counts': payload['counts'],
    'books_ready': sum(b['status'] == 'rebalanced_index_ready' for b in books),
    'books_with_active_scene_packets': sum(b['active_scene_count'] > 0 for b in books),
    'books_with_reconstruction_packets': sum(b['reconstruction_packet'] is not None for b in books),
    'books_gapped': [b['book_num'] for b in books if b['reconstruction_packet'] is not None],
}, indent=2))
