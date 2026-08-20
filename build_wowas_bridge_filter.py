import csv
import hashlib
import json
from pathlib import Path

ROOT = Path('crates/wowas-final-edition-v10/canon')
REB = Path('config/wowas/ordered_stretched_spine_manifest.json')
FLAVOR = ROOT / 'active/authored_flavor/authored_character_flavor_lattice.tsv'
DYNAMICS = ROOT / 'active/authored_flavor/authored_dynamics_lattice.tsv'
MOMENTUM = ROOT / 'active/authored_flavor/relationship_momentum_constraints.tsv'

def tsv(path):
    with path.open(encoding='utf-8', newline='') as f:
        return list(csv.DictReader(f, delimiter='\t'))

rebalance = json.loads(REB.read_text(encoding='utf-8'))
flavor = tsv(FLAVOR)
dynamics = tsv(DYNAMICS)
momentum = tsv(MOMENTUM)
flavor_ids = {r.get('canonical_id') for r in flavor}
flavor_names = {r.get('canonical_name', '').strip().lower() for r in flavor}
core_targets = {'pip', 'indalwin willowjayce', 'rylos', 'rylos (rylos vane johnson)', 'mack', 'xethrolund', 'rolzen'}

def mentions_core(value):
    text = (value or '').lower()
    return any(name in text for name in core_targets)

def active_dynamic_for(book, candidate):
    values = [candidate.get('source_character_name', ''), candidate.get('source_role', ''), candidate.get('source_anchor', ''), candidate.get('source_region', '')]
    result = []
    for row in dynamics:
        if not (int(row['first_book']) <= book <= int(row['last_book'])):
            continue
        if any((row.get('from_character', '').lower() in v.lower() or row.get('to_character', '').lower() in v.lower()) for v in values for _ in [0]):
            result.append(row)
    return result

filtered_books = []
counts = {'bridge_packets': 0, 'environment_only': 0, 'lattice_only': 0, 'active_prose_eligible': 0, 'core_interaction_review': 0, 'rejected_missing_basis': 0}
for book in rebalance['books']:
    number = book['order']
    candidates = []
    for packet in book['ordered_slots']:
        if packet['kind'] != 'ordered_bridge_or_reconstruction_packet':
            continue
        counts['bridge_packets'] += 1
        name = packet.get('source_character_name', '').strip().lower()
        source_basis = [packet.get('source_role', ''), packet.get('source_region', ''), packet.get('source_anchor', ''), packet.get('book_function', ''), packet.get('arc_band', '')]
        basis_complete = all(bool(x.strip()) for x in source_basis)
        core_contact = any(mentions_core(x) for x in source_basis)
        linked_dynamics = active_dynamic_for(number, packet)
        authored_identity = packet.get('source_character_id') in flavor_ids or name in flavor_names
        if not basis_complete:
            classification = 'rejected_missing_basis'
            counts['rejected_missing_basis'] += 1
            prose_gate = 'blocked'
        elif core_contact:
            classification = 'core_interaction_review'
            counts['core_interaction_review'] += 1
            prose_gate = 'blocked_until_explicit_motive_and_consequence'
        elif authored_identity and linked_dynamics:
            classification = 'active_prose_eligible'
            counts['active_prose_eligible'] += 1
            prose_gate = 'still_requires_book_band_tone_review'
        elif authored_identity:
            classification = 'lattice_only'
            counts['lattice_only'] += 1
            prose_gate = 'blocked_without_relationship_or_event_stakes'
        else:
            classification = 'environment_only'
            counts['environment_only'] += 1
            prose_gate = 'blocked_generated_identity_has_no_authored_flavor'
        candidates.append({
            'scene_id': packet['scene_id'],
            'character_id': packet.get('source_character_id', ''),
            'character_name': packet.get('source_character_name', ''),
            'classification': classification,
            'prose_gate': prose_gate,
            'motive_basis': {
                'role': packet.get('source_role', ''),
                'region': packet.get('source_region', ''),
                'source_anchor': packet.get('source_anchor', ''),
                'book_function': packet.get('book_function', ''),
                'arc_band': packet.get('arc_band', ''),
            },
            'core_contact_review_required': core_contact,
            'active_dynamics': [d.get('dynamic_id') for d in linked_dynamics],
            'relationship_requirements': [
                {
                    'momentum_id': m.get('momentum_id'),
                    'relation_type': m.get('relation_type'),
                    'required_state_change': m.get('required_state_change'),
                    'required_emotional_consequence': m.get('required_emotional_consequence'),
                    'required_counteragency': m.get('required_counteragency'),
                    'required_lattice_carry_forward': m.get('required_lattice_carry_forward'),
                }
                for m in momentum
                if int(m['first_book']) <= number <= int(m['last_book']) and any(x.lower() in ' '.join(source_basis).lower() for x in [m.get('from_character',''), m.get('to_character','')])
            ],
        })
    filtered_books.append({'book_num': number, 'title': book['title'], 'candidates': candidates, 'status': book['status']})

payload = {
    'schema': 'wowas.bridge_candidate_filter.v1',
    'policy': {
        'generated_identity_without_authored_flavor_cannot_enter_active_prose': True,
        'core_character_contact_requires_explicit_motive_and_consequence_review': True,
        'environment_only_candidates_may_affect_reflexor_world_pressure_but_not_dialogue': True,
        'relationship_constraints_are_required_for_active_recurrence': True,
        'full_series_prose_promotion_remains_locked': True,
    },
    'counts': counts,
    'book_count': len(filtered_books),
    'books': filtered_books,
    'source_hashes': {
        str(p): hashlib.sha256(p.read_bytes()).hexdigest()
        for p in [REB, FLAVOR, DYNAMICS, MOMENTUM]
    },
}
for out in [Path('audit/wowas_bridge_candidate_filter.json'), Path('config/wowas/bridge_candidate_filter_manifest.json')]:
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(payload, indent=2, ensure_ascii=False) + '\n', encoding='utf-8')
print(json.dumps({'counts': counts, 'book_count': len(filtered_books)}, indent=2))
