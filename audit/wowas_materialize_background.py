#!/usr/bin/env python3
"""Stream the WOWAS background population from canonical creature seeds."""
from __future__ import annotations
import csv, hashlib
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
BASE=ROOT/'crates/wowas-final-edition-v10/canon/active'
CREATURES=BASE/'generated/wowas_generated_creatures_5000.tsv'
SCENES=BASE/'scene_index_reasonable_window.tsv'
OUT=BASE/'generated/wowas_background_population_2000000.tsv'

def rows(path):
 with path.open(newline='',encoding='utf-8',errors='replace') as f:return list(csv.DictReader(f,delimiter='\t'))
def main():
 creatures=rows(CREATURES); scenes=rows(SCENES); count=2_000_000
 fields=['population_serial','creature_seed_id','creature_name','background_id','zone','role','scene_id','event_beat_id','book_num','world_function','provenance_chain','source_status','reader_projection','quality_status']
 roles=('background_household','route_witness','market_worker','creature_steward','gate_archivist','river_courier','field_healer','orchard_keeper')
 zones=('willow-stone-county','glass-orchard','stone-fen','blue-light-road','ash-river','diamond-breakland','morrow-market','root-vale')
 digest=hashlib.sha256(); OUT.parent.mkdir(parents=True,exist_ok=True)
 with OUT.open('w',newline='',encoding='utf-8',buffering=1024*1024) as f:
  w=csv.DictWriter(f,fieldnames=fields,delimiter='\t');w.writeheader();digest.update(('\t'.join(fields)+'\n').encode())
  for i in range(count):
   c=creatures[i%len(creatures)]; s=scenes[(i*17)%len(scenes)]
   role=roles[(i//len(creatures))%len(roles)]; zone=zones[(i//len(roles))%len(zones)]
   chain=f"{c.get('generation_law','canonical_creature_registry')}>background_population_v2>scene:{s.get('scene_id','')}"
   row={'population_serial':f'WBP-{i+1:07d}','creature_seed_id':c.get('creature_id',''),'creature_name':c.get('species_name',''),'background_id':f'BACKGROUND-{i+1:07d}','zone':zone,'role':role,'scene_id':s.get('scene_id',''),'event_beat_id':s.get('event_beat_id',''),'book_num':s.get('book_num',''),'world_function':c.get('ecology_role','supporting_world_ecology'),'provenance_chain':chain,'source_status':'CANONICAL_SEED_EXPANSION','reader_projection':'background_until_promoted','quality_status':'deterministic_seeded_requires_batch_review'}
   line='\t'.join(row[x] for x in fields)+'\n';f.write(line);digest.update(line.encode())
 OUT.with_suffix('.tsv.summary').write_text(f'schema=wowas.background_population.v2\nrecords={count}\nsha256={digest.hexdigest()}\nresident_records=0\nprovenance=canonical_seed_to_scene_beat\nquality_status=deterministic_seeded_requires_batch_review\n',encoding='utf-8')
 print(f'records={count} source_creature_seeds={len(creatures)} scene_anchors={len(scenes)} resident_records=0 quality_status=deterministic_seeded_requires_batch_review')
if __name__=='__main__':main()
