#!/usr/bin/env python3
from __future__ import annotations
import csv,json,hashlib,re
from pathlib import Path
from collections import Counter
ROOT=Path(__file__).resolve().parents[1]; BASE=ROOT/'crates/wowas-final-edition-v10'; FL=BASE/'canon/active/authored_flavor'; OUT=ROOT/'reconstruction'

def tsv(p):
 with p.open(newline='',encoding='utf-8',errors='replace') as f:return list(csv.DictReader(f,delimiter='\t'))
def main():
 chars=tsv(FL/'authored_character_flavor_lattice.tsv'); dyn=tsv(FL/'authored_dynamics_lattice.tsv')
 identity={'pip':{'canonical_id':'wowas::pip_indalwin_willowjayce','canonical_name':'Indalwin On’Rylder Willowjayce','nickname':'Pip','deprecated_variants':['Pip-no-path','any synthetic replacement not explicitly source-mapped'],'invariants':['iconic leader, not martyr','best available action under uncertainty','rebuild through love and life']},'rylos':{'canonical_id':'wowas::rylos_vayne_johnson','canonical_name':'Rylos Vayne Johnson','deprecated_variants':['Riledge','Boojay','any spelling or alias not explicitly retained as historical evidence'],'invariants':['accountability never redemption','Pip lover whose death occurs at the end under the explicit endgame rule']}}
 char_issues=[]
 for c in chars:
  if not c.get('canonical_id'): char_issues.append({'character':c.get('canonical_name',''),'issue':'missing canonical_id'})
  if not c.get('role'): char_issues.append({'character':c.get('canonical_name',''),'issue':'missing role'})
  if not c.get('first_book') or not c.get('last_book'): char_issues.append({'character':c.get('canonical_name',''),'issue':'missing book window'})
  if c.get('canonical_name','').lower().startswith('rylos') and c.get('canonical_name')!='Rylos (Rylos Vayne Johnson)': char_issues.append({'character':c.get('canonical_name',''),'issue':'Rylos canonical display drift'})
  if c.get('canonical_name','').startswith('Pip') and 'Indalwin' not in c.get('canonical_name',''): char_issues.append({'character':c.get('canonical_name',''),'issue':'Pip canonical display drift'})
 rel=[]
 for i,d in enumerate(dyn,1):
  kind=d.get('relation_type','')
  pivotal='high' if any(x in kind for x in ('betray','devotion','parent','transformation','soulmate','interdepend','metamorph','guardian')) else 'medium'
  rel.append({'momentum_id':f"momentum:{i:05d}",'dynamic_id':d.get('dynamic_id',f"dynamic:{i:05d}"),'from_character':d.get('from_character',''),'to_character':d.get('to_character',''),'relation_type':kind,'polarity':d.get('polarity',''),'first_book':d.get('first_book',''),'last_book':d.get('last_book',''),'momentum_level':pivotal,'required_state_change':'yes','required_emotional_consequence':'yes','required_counteragency':'yes','required_lattice_carry_forward':'yes','prose_status':'no_generated_prose','source_policy':'source-backed relationship function only; no invented fate'})
 with (FL/'relationship_momentum_constraints.tsv').open('w',newline='',encoding='utf-8') as f:
  fields=list(rel[0]);w=csv.DictWriter(f,fieldnames=fields,delimiter='\t');w.writeheader();w.writerows(rel)
 payload={'schema':'wowas.identity.fate.relationship.ledger.v1','identity_locks':identity,'named_character_count':len(chars),'relationship_count':len(rel),'character_issues':char_issues,'fate_policy':{'pip_lovers_rule':'Only Rylos Vayne Johnson among Pip’s lovers dies, and that death occurs at the end; all other fates require explicit source evidence.','universal_mortality_inference':'forbidden','survivors':'must be read from final-book/source rules, not guessed'},'relationship_policy':['Every major relationship must alter state, carry emotional consequence, retain counter-agency, and persist through the rolling ledger.','Momentous does not mean uniformly tragic; consequence may be care, refusal, repair, betrayal, revelation, or changed world practice.','No generated prose is created by this ledger.'],'source_hashes':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in (FL/'authored_character_flavor_lattice.tsv',FL/'authored_dynamics_lattice.tsv')}}
 (OUT/'WOWAS_IDENTITY_FATE_RELATIONSHIP_LEDGER.json').write_text(json.dumps(payload,indent=2,sort_keys=True)+'\n')
 print(json.dumps({'named_characters':len(chars),'relationships':len(rel),'character_issues':len(char_issues),'prose_generated':False},sort_keys=True))
if __name__=='__main__':main()
