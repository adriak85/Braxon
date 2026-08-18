#!/usr/bin/env python3
from __future__ import annotations
import csv,json,hashlib
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]; BASE=ROOT/'crates/wowas-final-edition-v10'; OUT=BASE/'canon/active/authored_flavor'; AUD=ROOT/'reconstruction/WOWAS_MAIN_PARTY_RESURRECTION_AUDIT.json'
def main():
 audit=json.loads(AUD.read_text(encoding='utf-8')); party=audit['core_main_party']; names=[r['name'] for r in party]
 rows=[]
 for target in party:
  target_name=target['name']; final_state='final_death_only' if 'Rylos' in target_name else 'source_resolution_required'
  for vp in party:
   rows.append({'matrix_id':f"resurrection:{target['canonical_id']}:{vp['canonical_id']}",'target_character_id':target['canonical_id'],'target_name':target_name,'viewpoint_character_id':vp['canonical_id'],'viewpoint_name':vp['name'],'book':'25','final_state_rule':final_state,'death_resurrection_parity':'required' if final_state!='final_death_only' else 'death must remain consequential and final','viewpoint_requirement':'distinct_main_party_viewpoint_required','required_consequence':'relationship state, world state, self-understanding, and future action must change','anti_martyr_rule':'return is not a reward for self-erasure; survival/resurrection must preserve agency and shared rebuilding','source_resolution_status':'structured_requirement_not_scene_level_proof','prose_status':'no_generated_prose'})
 p=OUT/'resurrection_viewpoint_matrix.tsv';
 with p.open('w',newline='',encoding='utf-8') as f:
  fields=list(rows[0]);w=csv.DictWriter(f,fieldnames=fields,delimiter='\t');w.writeheader();w.writerows(rows)
 manifest={'schema':'wowas.final.book.resurrection.viewpoint.matrix.v1','targets':len(party),'viewpoints':len(party),'rows':len(rows),'book':'25','main_party_names':names,'rylos_rule':'Rylos Vayne Johnson is the only one of Pip’s lovers whose death is locked to the ending; no resurrection is inferred for him.','other_targets_rule':'All other resurrection/survival outcomes require explicit final-book scene evidence; this matrix is a required coverage contract, not proof of prose realization.','prose_generated':False,'source_audit':str(AUD.relative_to(ROOT)),'source_sha256':hashlib.sha256(AUD.read_bytes()).hexdigest()}
 (OUT/'resurrection_viewpoint_matrix.manifest.json').write_text(json.dumps(manifest,indent=2,ensure_ascii=False,sort_keys=True)+'\n')
 print(json.dumps({'targets':len(party),'viewpoints':len(party),'rows':len(rows),'prose_generated':False},ensure_ascii=False,sort_keys=True))
if __name__=='__main__':main()
