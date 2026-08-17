#!/usr/bin/env python3
from __future__ import annotations
import csv
from collections import Counter
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
P=ROOT/'crates/wowas-final-edition-v10/canon/active/generated/wowas_background_population_2000000.tsv'
def main():
 total=0; bad=0; serials=set(); roles=Counter(); zones=Counter(); scenes=set(); seeds=set(); beats=set()
 with P.open(newline='',encoding='utf-8') as f:
  for row in csv.DictReader(f,delimiter='\t'):
   total+=1; serial=row['population_serial']; serials.add(serial); roles[row['role']]+=1; zones[row['zone']]+=1; scenes.add(row['scene_id']); seeds.add(row['creature_seed_id']); beats.add(row['event_beat_id'])
   required=('population_serial','creature_seed_id','background_id','zone','role','scene_id','event_beat_id','book_num','world_function','provenance_chain','source_status','reader_projection','quality_status')
   if any(not row.get(k) for k in required) or 'background_population_v2>scene:' not in row['provenance_chain'] or not row['provenance_chain'].split('>')[0] or row['reader_projection']!='background_until_promoted' or row['source_status']!='CANONICAL_SEED_EXPANSION': bad+=1
 print('records=',total,'expected=2000000')
 print('unique_serials=',len(serials),'duplicate_serials=',total-len(serials))
 print('bad_rows=',bad,'scene_anchors=',len(scenes),'event_beats=',len(beats),'creature_seeds=',len(seeds))
 print('roles=',dict(roles)); print('zones=',dict(zones))
 print('quality_status=deterministic_seeded_requires_batch_review')
 assert total==2_000_000 and len(serials)==total and bad==0 and len(seeds)==5000 and len(scenes)>0 and len(beats)>0
if __name__=='__main__':main()
