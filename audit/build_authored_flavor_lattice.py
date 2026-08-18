#!/usr/bin/env python3
from __future__ import annotations
import csv,json,hashlib,re
from pathlib import Path
from collections import defaultdict
ROOT=Path(__file__).resolve().parents[1]; BASE=ROOT/'crates/wowas-final-edition-v10'; OUT=BASE/'canon/active/authored_flavor'

def load(rel):
 p=BASE/rel
 return p, json.loads(p.read_text()) if p.suffix=='.json' else list(csv.DictReader(p.open(newline='',encoding='utf-8',errors='replace'),delimiter='\t'))
def vals(x):
 if isinstance(x,list): return ' | '.join(str(v) for v in x)
 return str(x or '')
def active_text(x):
 # Derived active constraints must not emit the deprecated alias; source traces remain hashed separately.
 return re.sub(r'(?i)\bBoojay\b', 'Rylos Vayne Johnson', str(x or ''))
def main():
 regp,reg=load('canon/canonical_story_tree/characters/06_CHARACTER_REGISTRY.json')
 chars=reg.get('characters',[]) if isinstance(reg,dict) else []
 timelinep,timeline=load('canon/wowas_character_timeline_lattice_UNIFIED_v14.tsv')
 orbitp,orbits=load('canon/wowas_orbit_file_v2.tsv')
 protectedp,protected=load('canon/wowas_protected_support_cast_v7.tsv')
 byid={r.get('character_code',''):r for r in timeline}; byname={r.get('canonical_name','').lower():r for r in timeline}
 fields=['canonical_id','canonical_name','aliases','tier','pronouns','age','species','first_book','last_book','faction','role','locked_traits','shadow','magic','sources','voice_constraint','pressure_response','dynamic_function','relationship_functions','timeline_fields','source_registry','prose_policy']
 rows=[]
 for c in chars:
  cid=c.get('id',''); name=c.get('name',''); t=byid.get(cid) or byname.get(name.lower(),{})
  t = dict(t)
  t['role_summary'] = active_text(t.get('role_summary',''))
  t['book_function_notes'] = active_text(t.get('book_function_notes',''))
  rows.append({'canonical_id':cid,'canonical_name':active_text(name),'aliases':active_text(vals(c.get('aliases',[]))),'tier':c.get('tier',''),'pronouns':c.get('pronouns',''),'age':c.get('age',''),'species':c.get('species',''),'first_book':c.get('first_book',''),'last_book':c.get('last_book',''),'faction':c.get('faction',''),'role':c.get('role',''),'locked_traits':active_text(vals(c.get('locked_traits',[]))),'shadow':active_text(c.get('shadow','')),'magic':active_text(vals(c.get('magic',[]))),'sources':active_text(vals(c.get('sources',[]))),'voice_constraint':t.get('role_summary','') or t.get('activity_mode',''),'pressure_response':t.get('peril_state',''),'dynamic_function':t.get('book_function_notes',''),'relationship_functions':'','timeline_fields':json.dumps(t,sort_keys=True),'source_registry':'06_CHARACTER_REGISTRY.json + wowas_character_timeline_lattice_UNIFIED_v14.tsv','prose_policy':'metadata only; no generated prose'})
 # Relationship/orbit rows are separate and cannot overwrite character identity.
 dyn=[]
 for i,r in enumerate(orbits,1):
  dyn.append({'dynamic_id':f"orbit:{i:05d}",'from_character':r.get('from_character',''),'to_character':r.get('to_character',''),'relation_type':r.get('relation_type',''),'polarity':r.get('polarity',''),'orbit_scope':r.get('orbit_scope',''),'first_book':r.get('first_book',''),'last_book':r.get('last_book',''),'active_books':r.get('active_books',''),'pressure_translation':r.get('pressure_translation',''),'wonder_fear_effect':r.get('wonder_fear_effect',''),'source_basis':r.get('source_basis',''),'prose_policy':'metadata only; no generated prose'})
 OUT.mkdir(parents=True,exist_ok=True)
 with (OUT/'authored_character_flavor_lattice.tsv').open('w',newline='',encoding='utf-8') as f:
  w=csv.DictWriter(f,fieldnames=fields,delimiter='\t',lineterminator='\n');w.writeheader();w.writerows(rows)
 with (OUT/'authored_dynamics_lattice.tsv').open('w',newline='',encoding='utf-8') as f:
  w=csv.DictWriter(f,fieldnames=list(dyn[0]) if dyn else ['dynamic_id'],delimiter='\t',lineterminator='\n');w.writeheader();w.writerows(dyn)
 manifest={'schema':'wowas.authored.flavor.dynamics.lattice.v1','character_rows':len(rows),'dynamic_rows':len(dyn),'source_files':[str(p.relative_to(ROOT)) for p in (regp,timelinep,orbitp,protectedp)],'source_hashes':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in (regp,timelinep,orbitp,protectedp)},'policy':'Original authored character identity and flavor are preserved. Generated registries may consume this lattice as constraints, but this stage does not generate prose or rewrite originals.','invariants':['canonical IDs remain stable','aliases remain explicit','voice/pressure/dynamic fields are separate from identity','relationship rows do not overwrite character rows','no generated prose is present']}
 (OUT/'authored_flavor_lattice.manifest.json').write_text(json.dumps(manifest,indent=2,sort_keys=True)+'\n')
 print(json.dumps({'character_rows':len(rows),'dynamic_rows':len(dyn),'generated_prose':False},sort_keys=True))
if __name__=='__main__':main()
