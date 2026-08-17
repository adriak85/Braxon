#!/usr/bin/env python3
from __future__ import annotations
import csv,hashlib,json
from pathlib import Path
from wowas_source_authority_audit import classify,read
ROOT=Path(__file__).resolve().parents[1]; BASE=ROOT/'crates/wowas-final-edition-v10'; SOURCE=BASE/'canon/wowas_clean_scene_index_v2.tsv'; OUT=BASE/'canon/active/scene_index_authority'; FIELDS=['scene_id','book_num','book_title','era_band','slot_in_book','source_layer','source_type','old_title','clean_title','title_status','brief_scene_description','inferred_character_names','inferred_character_ids','book_active_cast','book_key_pressure','source_trace','corridor_region_anchor','county_anchor','ecology_pressure_mode','creature_refs','transformation_notes','authority_class']
def main():
 rows=read(SOURCE); buckets={'accepted_tier1_or_2':[],'requires_prose_realization':[],'rejected_scaffold':[],'unresolved':[]}
 for r in rows:
  c=classify(r); r=dict(r); r['authority_class']=c
  if c=='accepted_tier1_or_2': buckets[c].append(r)
  elif c.startswith('requires_prose'): buckets['requires_prose_realization'].append(r)
  elif c=='rejected_scaffold_tier4': buckets['rejected_scaffold'].append(r)
  else: buckets['unresolved'].append(r)
 OUT.mkdir(parents=True,exist_ok=True)
 for p in OUT.glob('*'): p.unlink()
 path=OUT/'scene_index_accepted.tsv'
 with path.open('w',newline='',encoding='utf-8') as f:
  w=csv.DictWriter(f,fieldnames=FIELDS,delimiter='\t'); w.writeheader(); w.writerows(buckets['accepted_tier1_or_2'])
 counts={'source_rows':len(rows),'accepted_rows':len(buckets['accepted_tier1_or_2']),'prose_realization_debt_rows':len(buckets['requires_prose_realization']),'rejected_scaffold_rows':len(buckets['rejected_scaffold']),'unresolved_rows':len(buckets['unresolved'])}
 manifest={'schema':'wowas.scene_index.authority.v2','authority':'evidence_backed_only','accepted_file':path.name,'accepted_bytes':path.stat().st_size,'accepted_sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'counts':counts,'rules_source':'canon/patches/v10/wowas_scene_authority_cleanup_patch_v10.md','promotion_rule':'Only accepted_tier1_or_2 rows are materialized. Debt, rejected, and unresolved rows are not written as canonical scene authority.'}
 (OUT/'scene_index_authority.manifest.json').write_text(json.dumps(manifest,indent=2,sort_keys=True)+'\n',encoding='utf-8'); print(json.dumps(counts,sort_keys=True))
if __name__=='__main__':main()
