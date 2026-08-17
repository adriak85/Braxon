#!/usr/bin/env python3
"""Resolve a user-facing WOWAS character projection from explicit preferences."""
from __future__ import annotations
import csv, sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]/'crates/wowas-final-edition-v10/canon/active/generated'
ATTN=ROOT/'wowas_character_attention_projection.tsv'; ROLES=ROOT/'wowas_character_world_role_map.tsv'

def read(p):
 with p.open(newline='',encoding='utf-8') as f:return list(csv.DictReader(f,delimiter='\t'))
def parse(value):return {x.strip() for x in value.split('|') if x.strip()}
def main():
 prefs={}
 if len(sys.argv)>1:
  with open(sys.argv[1],newline='',encoding='utf-8') as f:prefs=next(csv.DictReader(f,delimiter='\t'),{})
 attn=read(ATTN); roles={r['character_id']:r for r in read(ROLES)}
 favorites=parse(prefs.get('favorite_character_ids','')); preferred_roles=parse(prefs.get('preferred_roles','')); preferred_regions=parse(prefs.get('preferred_regions',''))
 max_promoted=int(prefs.get('max_promoted_characters_per_scene','2') or 2)
 out=[]
 for row in attn:
  role=roles.get(row['character_id'],{}); reasons=[]
  if row['character_id'] in favorites: reasons.append('user_favorite')
  if role.get('functional_role','') in preferred_roles: reasons.append('preferred_role')
  if role.get('region','') in preferred_regions: reasons.append('preferred_region')
  tier='promoted' if reasons else row['default_attention_tier']
  out.append({'projection_serial':row['projection_serial'],'canonical_character_id':row['canonical_character_id'],'character_id':row['character_id'],'resolved_attention_tier':tier,'promotion_reasons':'|'.join(reasons) or 'default_world_relevance','max_promoted_per_scene':str(max_promoted),'identity_mutation':row['identity_mutation'],'timeline_mutation':row['timeline_mutation'],'consent_required':row['consent_required'],'age_gate_required':row['age_gate_required'],'reversible':row['reversible']})
 out.sort(key=lambda r:(r['resolved_attention_tier']!='promoted',r['character_id']))
 p=ROOT/'wowas_resolved_user_projection.tsv'
 with p.open('w',newline='',encoding='utf-8') as f:
  w=csv.DictWriter(f,fieldnames=list(out[0]),delimiter='\t');w.writeheader();w.writerows(out)
 print(f'characters={len(out)} promoted={sum(r["resolved_attention_tier"]=="promoted" for r in out)} max_promoted_per_scene={max_promoted} canonical_mutation_forbidden=true')
if __name__=='__main__':main()
