#!/usr/bin/env python3
from __future__ import annotations
import csv, hashlib, json, re
from collections import Counter, defaultdict
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
SRC_MAIN=ROOT/'reconstruction/source_inputs/scene_index_main_active.tsv'
SRC_15000=ROOT/'reconstruction/source_inputs/scene_index_original_15000.tsv'
BASE=ROOT/'crates/wowas-final-edition-v10'
OUT=BASE/'canon/active/reconciled_15000'
FIELDS=['record_id','record_kind','book_num','book_title','scene_id','source_layer','source_type','clean_title','title_status','brief_scene_description','inferred_character_names','inferred_character_ids','book_active_cast','book_key_pressure','source_trace','corridor_region_anchor','county_anchor','ecology_pressure_mode','creature_refs','transformation_notes','domain_flags','quest_hook','world_introduction_anchor','coverage_status','alignment_status','applied_patch_ids','prose_status','source_path']

def read_tsv(p):
 with p.open(newline='',encoding='utf-8',errors='replace') as f:return list(csv.DictReader(f,delimiter='\t'))
def source(path):
 candidates=[BASE/'canon'/path, BASE/path, BASE/'canon'/('patches/v10/'+path), BASE/'canon'/('patches/v9/wowas_v8_polish/'+path)]
 for p in candidates:
  if p.exists(): return p
 return None
def band_matches(book,band):
 m=re.findall(r'\d+',str(band)); return not m or int(book or 0) in {int(x) for x in m} or (len(m)>=2 and int(m[0])<=int(book or 0)<=int(m[1]))
def norm(r,kind,idx,source_path):
 raw_book=str(r.get('book_num',r.get('book','')) or ''); bm=re.search(r'(?:Book[_ -]?|B)(\d{1,2})',raw_book,re.I); book=bm.group(1) if bm else (raw_book if raw_book.isdigit() else ''); sid=str(r.get('scene_id','') or r.get('encounter_id','') or r.get('character_id','') or r.get('background_id','') or f'ROW-{idx:07d}')
 return {'record_id':f'{kind}:{idx:07d}:{sid}','record_kind':kind,'book_num':book,'book_title':r.get('book_title',r.get('book','')),'scene_id':sid,'source_layer':r.get('source_layer',kind.upper()),'source_type':r.get('source_type',kind.upper()),'clean_title':r.get('clean_title',r.get('event_shape',r.get('name',r.get('species_name','')))),'title_status':r.get('title_status','structured_candidate'),'brief_scene_description':r.get('brief_scene_description',r.get('memorable_hook',r.get('plot_incorporation',r.get('event_shape','')))),'inferred_character_names':r.get('inferred_character_names',r.get('core_character',r.get('character_anchor',''))),'inferred_character_ids':r.get('inferred_character_ids',''),'book_active_cast':r.get('book_active_cast',r.get('satellite_character','')),'book_key_pressure':r.get('book_key_pressure',r.get('stakes',r.get('pressure',''))),'source_trace':r.get('source_trace',r.get('source_anchor',r.get('source_profiles',''))),'corridor_region_anchor':r.get('corridor_region_anchor',r.get('route_or_place','')),'county_anchor':r.get('county_anchor',''),'ecology_pressure_mode':r.get('ecology_pressure_mode',r.get('ecology_role','')),'creature_refs':r.get('creature_refs',r.get('creature_id',r.get('species_name',''))),'transformation_notes':r.get('transformation_notes',''),'domain_flags':r.get('domain_flags','characters'),'quest_hook':r.get('quest_hook',r.get('plot_use','')),'world_introduction_anchor':r.get('world_introduction_anchor',r.get('route_or_place','')),'coverage_status':r.get('coverage_status','candidate'),'alignment_status':'unpatched','applied_patch_ids':'','prose_status':'no_generated_prose','source_path':source_path}
def main():
 main_base=read_tsv(SRC_MAIN); recovered=read_tsv(SRC_15000); rows=[norm(r,'main_scene',i+1,'source_inputs/scene_index_main_active.tsv') for i,r in enumerate(main_base)] + [norm(r,'recovered_15000_candidate',len(main_base)+i+1,'source_inputs/scene_index_original_15000.tsv') for i,r in enumerate(recovered)]; base=main_base; led=[]; nextid=len(rows)+1
 # Sequential explicit additions and corrections.
 for ver,path in [('v6','wowas_scene_patch_v6.tsv'),('v10','wowas_scene_patch_v10.tsv'),('v11','wowas_scene_patch_v11.tsv')]:
  p=source(path)
  if not p: continue
  patch=read_tsv(p); led.append({'order':len(led)+1,'version':ver,'path':str(p.relative_to(ROOT)),'rows':len(patch),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()})
  if ver=='v6':
   for r in patch:
    x=norm(r,'patch_scene_addition',nextid,str(p.relative_to(ROOT))); x['alignment_status']='patch_added'; x['applied_patch_ids']='v6'; rows.append(x); nextid+=1
  else:
   for r in patch:
    target=r.get('target',''); scope=r.get('book_band','')
    matched=0
    for x in rows:
     if (not scope or band_matches(x['book_num'],scope)) and (ver=='v11' or not target or target in x['scene_id'] or target in x['clean_title']):
      x['applied_patch_ids']=('|'.join(filter(None,[x['applied_patch_ids'],r.get('patch_id',r.get('patch_code',''))])))
      x['alignment_status']='patched_metadata'; matched+=1
    if not matched: led.append({'order':len(led)+1,'version':ver,'unmatched_patch':r.get('patch_id',r.get('patch_code','')),'target':target,'status':'unresolved_rule'})
 # Structured candidate lanes: candidates only, never prose.
 lanes=[('character_candidate','active/generated/wowas_generated_characters_5000.tsv'),('beat_candidate','active/generated/wowas_character_encounters.tsv'),('encounter_candidate','active/generated/wowas_wildlife_encounters.tsv'),('world_population_candidate','active/generated/wowas_desert_population.tsv')]
 for kind,rel in lanes:
  p=source(rel)
  if not p: continue
  data=read_tsv(p); led.append({'order':len(led)+1,'version':'structured','path':str(p.relative_to(ROOT)),'rows':len(data),'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'prose_imported':False})
  for r in data:
   x=norm(r,kind,nextid,str(p.relative_to(ROOT))); x['alignment_status']='structured_candidate'; rows.append(x); nextid+=1
 # Never claim generated candidate rows are prose or complete scenes.
 rows.sort(key=lambda x:(int(x['book_num'] or 0),x['record_kind'],x['scene_id'],x['record_id']))
 OUT.mkdir(parents=True,exist_ok=True)
 for p in OUT.glob('*'): p.unlink()
 full=OUT/'scene_index_reconciled_metadata.tsv'
 with full.open('w',newline='',encoding='utf-8') as f:
  w=csv.DictWriter(f,fieldnames=FIELDS,delimiter='\t'); w.writeheader(); w.writerows(rows)
 counts=Counter(x['record_kind'] for x in rows); status=Counter(x['alignment_status'] for x in rows)
 manifest={'schema':'wowas.reconciled.scene.metadata.v1','base_source':'origin/main active scene index; reconstruction-only exact 15000-row object retained as candidate layer','main_base_rows':len(main_base),'recovered_15000_candidate_rows':len(recovered),'base_rows':len(base),'total_rows':len(rows),'record_kind_counts':dict(counts),'alignment_status_counts':dict(status),'patch_apply_order':led,'prose_policy':'Generated prose is excluded. Structured candidates and beats are metadata only and must not count as authored prose or complete manuscript scenes.','duplicate_scene_ids':sum(v>1 for v in Counter(x['scene_id'] for x in rows).values()),'main_source_hash':hashlib.sha256(SRC_MAIN.read_bytes()).hexdigest(),'recovered_candidate_hash':hashlib.sha256(SRC_15000.read_bytes()).hexdigest(),'output_sha256':hashlib.sha256(full.read_bytes()).hexdigest()}
 (OUT/'scene_index_reconciled_metadata.manifest.json').write_text(json.dumps(manifest,indent=2,sort_keys=True)+'\n',encoding='utf-8'); print(json.dumps({'base_rows':len(base),'total_rows':len(rows),'record_kind_counts':dict(counts),'alignment_status_counts':dict(status),'duplicate_scene_ids':manifest['duplicate_scene_ids']},sort_keys=True))
if __name__=='__main__':main()
