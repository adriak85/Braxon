#!/usr/bin/env python3
"""Build complete WOWAS character world-role maps and user-focused projections."""
from __future__ import annotations
import csv, hashlib
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]/'crates/wowas-final-edition-v10'
GEN=ROOT/'canon/active/generated'
CHARS=GEN/'wowas_generated_characters_5000.tsv'
SCHEDULE=GEN/'wowas_character_timeline_schedule.tsv'
ROLE_OUT=GEN/'wowas_character_world_role_map.tsv'
ATTN_OUT=GEN/'wowas_character_attention_projection.tsv'
PREFS=GEN/'wowas_user_preference_profile.template.tsv'

def read(p):
 with p.open(newline='',encoding='utf-8',errors='replace') as f:return list(csv.DictReader(f,delimiter='\t'))
def write(p,rows):
 fields=list(rows[0]);
 with p.open('w',newline='',encoding='utf-8') as f:
  w=csv.DictWriter(f,fieldnames=fields,delimiter='\t');w.writeheader();w.writerows(rows)
def sid(kind,value):return f'VAR-{kind.upper()}-'+hashlib.sha1((kind+'|'+value).encode()).hexdigest()[:12]

def main():
 chars=read(CHARS); schedule=read(SCHEDULE)
 sched={}
 for r in schedule: sched.setdefault(r.get('character_id',''),[]).append(r)
 roles=[]; attention=[]
 ranked=sorted(chars,key=lambda x:(int(x.get('selection_score','0') or 0) if str(x.get('selection_score','0')).isdigit() else 0,x.get('character_id','')),reverse=True)
 rank={x.get('character_id',''):i for i,x in enumerate(ranked)}
 for r in chars:
  cid=r.get('character_id',''); role=r.get('role','background actor'); book=r.get('book_anchor',''); region=r.get('region',''); tier=r.get('tier','')
  canon=sid('canon',cid)
  world_role='primary_actor' if 'core' in tier or 'active' in tier else ('supporting_actor' if 'support' in tier or r.get('selection_score','0').isdigit() and int(r.get('selection_score','0'))>=100 else 'background_actor')
  phases='|'.join(sorted({x.get('timeline_phase','') for x in sched.get(cid,[]) if x.get('timeline_phase')}))
  scene_ids='|'.join(dict.fromkeys(x.get('assigned_scene_id','') for x in sched.get(cid,[]) if x.get('assigned_scene_id')))
  roles.append({'canonical_character_id':canon,'character_id':cid,'name':r.get('name',''),'world_role':world_role,'functional_role':role,'book_anchor':book,'region':region,'house_pressure':r.get('house_pressure',''),'background_function':r.get('story_background_law',''),'timeline_phases':phases,'assigned_scene_count':str(len(sched.get(cid,[]))),'assigned_scene_ids':scene_ids,'relationship_layers':'ally|rival|kin|mentor|dependent|faction|creature|location|world_system','world_state_policy':'background_actor_remains_coherent_until_promoted','provenance_status':'complete_world_role_map'})
  score=int(r.get('selection_score','0') or 0) if str(r.get('selection_score','0')).isdigit() else 0
  position=rank.get(cid,len(chars)); default='promotable' if position < max(1,len(chars)//10) else ('supporting' if position < max(2,len(chars)*3//10) else 'background')
  attention.append({'projection_serial':sid('projection',cid),'canonical_character_id':canon,'character_id':cid,'default_attention_tier':default,'promotion_triggers':'user_favorite|explicit_mention|repeated_interaction|direct_relationship|active_quest|user_preference_match','demotion_triggers':'scene_capacity|user_dismissal|arc_complete|no_recent_relevance','max_reader_cast_weight':'1','variant_dimensions':'appearance|voice|role_emphasis|relationship_proximity','identity_mutation':'forbidden','timeline_mutation':'forbidden','consent_required':'true','age_gate_required':'true','reader_load_policy':'promote_one_or_two; keep_background_in_world_map','reversible':'true'})
 write(ROLE_OUT,roles);write(ATTN_OUT,attention)
 write(PREFS,[{'user_id':'example-user','favorite_character_ids':'','preferred_regions':'','preferred_roles':'','preferred_relationship_layers':'','promote_mentioned_characters':'true','max_promoted_characters_per_scene':'2','consent_to_variant_presentation':'false'}])
 print(f'characters={len(chars)} role_rows={len(roles)} attention_rows={len(attention)} fully_scheduled={sum(bool(x["timeline_phases"]) for x in roles)}')
if __name__=='__main__':main()
