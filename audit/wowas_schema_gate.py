#!/usr/bin/env python3
from __future__ import annotations
import csv, hashlib, json, sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
GEN=ROOT/'crates/wowas-final-edition-v10/canon/active/generated'
INDEX=ROOT/'crates/wowas-final-edition-v10/canon/active'
OUT=ROOT/'reconstruction/WOWAS_SCHEMA_CACHE_MANIFEST.json'
SCHEMAS={
 'scene_index_reasonable_window.tsv':('scene_id','brief_scene_description','event_beat_id','domain_flags','coverage_status'),
 'wowas_generated_creatures_5000.tsv':('creature_id','species_name','biome','ecology_role','generation_law'),
 'wowas_relationship_ledger.tsv':('serial','character_id','other_id','event_id','scene_link','status'),
 'wowas_character_timeline_schedule.tsv':('schedule_serial','character_id','assigned_scene_id','event_beat_id'),
 'wowas_character_world_role_map.tsv':('canonical_character_id','world_role','timeline_phases'),
 'wowas_character_attention_projection.tsv':('projection_serial','canonical_character_id','default_attention_tier'),
 'wowas_real_world_source_registry.tsv':('source_serial','source_type','source_id','source_url','fact_fiction_boundary'),
 'wowas_real_world_wowas_alignment.tsv':('alignment_serial','source_serial','scene_id','event_beat_id','book_num','citation_url'),
 'wowas_real_world_domain_alignment.tsv':('domain_alignment_serial','source_serial','domain','scene_id','event_beat_id','citation_url'),
 'wowas_background_population_2000000.tsv':('population_serial','creature_seed_id','background_id','scene_id','event_beat_id','provenance_chain','quality_status'),
}
def sha(path):
 h=hashlib.sha256();
 with path.open('rb') as f:
  for block in iter(lambda:f.read(1024*1024),b''):h.update(block)
 return h.hexdigest()
def main():
 manifest={'schema_registry':'wowas.tsv.registry.v1','files':[]}; failures=[]
 for name,required in SCHEMAS.items():
  p=(INDEX/name) if name == 'scene_index_reasonable_window.tsv' else (GEN/name)
  if not p.exists(): failures.append(f'MISSING:{name}'); continue
  with p.open(newline='',encoding='utf-8',errors='replace') as f:
   reader=csv.reader(f,delimiter='\t'); header=next(reader,[]); rows=0; malformed=0
   missing=[x for x in required if x not in header]
   for row in reader:
    rows+=1
    if len(row)!=len(header): malformed+=1
  if missing: failures.append(f'MISSING_COLUMNS:{name}:{"|".join(missing)}')
  if malformed: failures.append(f'MALFORMED_ROWS:{name}:{malformed}')
  manifest['files'].append({'path':str(p.relative_to(ROOT)),'schema_version':'v1','columns':header,'required_columns':list(required),'rows':rows,'sha256':sha(p),'bytes':p.stat().st_size,'status':'valid' if not missing and not malformed else 'invalid'})
 manifest['status']='fail' if failures else 'pass'; manifest['failures']=failures
 OUT.write_text(json.dumps(manifest,indent=2,sort_keys=True)+'\n',encoding='utf-8')
 print(json.dumps({'status':manifest['status'],'files':len(manifest['files']),'failures':failures},sort_keys=True))
 if failures: raise SystemExit(1)
if __name__=='__main__':main()
