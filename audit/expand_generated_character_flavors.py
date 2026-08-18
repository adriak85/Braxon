#!/usr/bin/env python3
from __future__ import annotations
import csv,json,re,hashlib
from pathlib import Path
from collections import defaultdict,Counter
ROOT=Path(__file__).resolve().parents[1]; BASE=ROOT/'crates/wowas-final-edition-v10'; OUT=BASE/'canon/active/authored_flavor'

def tsv(p):
 with p.open(newline='',encoding='utf-8',errors='replace') as f:return list(csv.DictReader(f,delimiter='\t'))
def norm(s):return re.sub(r'[^a-z0-9]','',s.lower())
def main():
 authored=tsv(OUT/'authored_character_flavor_lattice.tsv'); dyn=tsv(OUT/'authored_dynamics_lattice.tsv'); gen=tsv(BASE/'canon/active/generated/wowas_generated_characters_5000.tsv')
 by_anchor={}
 for a in authored:
  n=norm(a['canonical_name']); short=norm(a['canonical_name'].split('(')[0]);
  for key in (n,short):
   if key: by_anchor[key]=a
 relations=defaultdict(list)
 for d in dyn:
  for side in ('from_character','to_character'):
   key=norm(d.get(side,''))
   for ak,a in by_anchor.items():
    if key and (key in ak or ak in key): relations[a['canonical_id']].append(d)
 out=[]
 for idx,g in enumerate(gen,1):
  anchor=g.get('source_anchor','').replace(' source orbit','').strip(); a=None
  for k,v in by_anchor.items():
   if norm(anchor) in k or k in norm(anchor): a=v; break
  if not a:
   a={'canonical_id':'unmapped:'+norm(anchor),'canonical_name':anchor,'locked_traits':'','sources':'','voice_constraint':'','pressure_response':'','dynamic_function':'','role':'','prose_policy':'metadata only'}
  rels=relations.get(a['canonical_id'],[]); d=rels[(idx-1)%len(rels)] if rels else {}
  out.append({'generated_character_id':g.get('character_id',''),'generated_name':g.get('name',''),'book_anchor':g.get('book_anchor',''),'generated_role':g.get('role',''),'generated_tier':g.get('tier',''),'source_anchor':g.get('source_anchor',''),'authored_canonical_id':a.get('canonical_id',''),'authored_canonical_name':a.get('canonical_name',''),'inherited_locked_traits':a.get('locked_traits',''),'inherited_sources':a.get('sources',''),'voice_constraint':a.get('voice_constraint',''),'pressure_response':a.get('pressure_response',''),'dynamic_function':a.get('dynamic_function',''),'dynamic_relation_type':d.get('relation_type',''),'dynamic_counterpart':d.get('to_character','') if d.get('from_character')==a.get('canonical_name','') else d.get('from_character',''),'dynamic_polarity':d.get('polarity',''),'dynamic_pressure_translation':d.get('pressure_translation',''),'variation_policy':'vary scene behavior through authored pressure/dynamic fields; do not clone prose or identity','prose_status':'no_generated_prose'})
 p=OUT/'generated_character_flavor_constraints.tsv'
 with p.open('w',newline='',encoding='utf-8') as f:
  fields=list(out[0]); w=csv.DictWriter(f,fieldnames=fields,delimiter='\t');w.writeheader();w.writerows(out)
 role_counts=Counter(x['generated_role'] for x in out); unmapped=[x for x in out if x['authored_canonical_id'].startswith('unmapped:')]
 manifest={'schema':'wowas.generated.character.flavor.constraints.v1','rows':len(out),'source_generated_registry':str((BASE/'canon/active/generated/wowas_generated_characters_5000.tsv').relative_to(ROOT)),'source_authored_lattice':str((OUT/'authored_character_flavor_lattice.tsv').relative_to(ROOT)),'source_dynamics_lattice':str((OUT/'authored_dynamics_lattice.tsv').relative_to(ROOT)),'role_counts':dict(role_counts),'unmapped_source_anchor_rows':len(unmapped),'prose_created':False,'identity_policy':'Generated rows retain generated IDs; authored canonical IDs are constraints only.','diversity_policy':'Repeated procedural roles are not treated as distinct authored personalities. Scene realization must vary behavior using source-backed voice, pressure, relationship, book state, and event context; no prose is emitted here.','required_checks':['record_id over scene_id for output naming','tone/style gate before prose promotion','rolling state ledger across books','no identity overwrite','no generated prose']}
 (OUT/'generated_character_flavor_constraints.manifest.json').write_text(json.dumps(manifest,indent=2,sort_keys=True)+'\n')
 print(json.dumps({'rows':len(out),'unmapped':len(unmapped),'role_cardinality':len(role_counts),'prose_created':False},sort_keys=True))
if __name__=='__main__':main()
