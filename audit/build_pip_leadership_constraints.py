#!/usr/bin/env python3
from __future__ import annotations
import csv,json,hashlib,re
from pathlib import Path
from collections import Counter
ROOT=Path(__file__).resolve().parents[1]; BASE=ROOT/'crates/wowas-final-edition-v10'; META=BASE/'canon/active/reconciled_15000/scene_index_reconciled_metadata.tsv'; OUT=BASE/'canon/active/authored_flavor'

def main():
 with META.open(newline='',encoding='utf-8',errors='replace') as f: rows=list(csv.DictReader(f,delimiter='\t'))
 out=[]
 for r in rows:
  blob=' '.join(r.get(k,'') for k in r)
  if not re.search(r'(^|[^a-z])pip([^a-z]|$)|indalwin|willowjayce',blob,re.I): continue
  context=(r.get('source_type','')+' '+r.get('record_kind','')+' '+r.get('domain_flags','')+' '+r.get('quest_hook','')+' '+r.get('book_key_pressure','')).lower()
  if any(x in context for x in ('advers','rival','conflict','threat','antag')): mode='challenger_or_adversary'; response='Tests Pip’s reasoning and exposes the cost or blind spot in his chosen action; opposition is not proof that Pip must sacrifice himself.'
  elif any(x in context for x in ('creature','ecology','wildlife','pressure')): mode='world_pressure_witness'; response='The living world answers Pip’s choices materially; the encounter makes him adapt and repair rather than perform martyrdom.'
  elif any(x in context for x in ('quest','world_introduction','population','place','corridor')): mode='builder_or_beneficiary'; response='The other party contributes a real piece of the world and receives a real piece of the rebuilding; Pip is a coordinator, not a savior above participation.'
  elif any(x in context for x in ('beat','encounter','character')): mode='peer_or_witness'; response='The character sees Pip choose the best available action under uncertainty and responds from their own values, not automatic worship or dependence.'
  else: mode='stateful_encounter'; response='The encounter must change both sides’ state while preserving Pip’s agency, limits, and capacity to keep building.'
  out.append({'record_id':r.get('record_id',''),'scene_id':r.get('scene_id',''),'book_num':r.get('book_num',''),'record_kind':r.get('record_kind',''),'source_path':r.get('source_path',''),'leadership_mode':mode,'encounter_response':response,'pip_invariant':'Pip is an iconic leader, not a martyr: outcomes are not guaranteed, but he gives his best and chooses the best available action he can honestly see.','anti_martyr_rule':'Do not reward self-erasure, needless suffering, or solitary salvation. Let Pip delegate, accept help, revise decisions, survive, and remain responsible without becoming the only bearer of loss.','action_principle':'Choose the best available action with incomplete knowledge; make the reasoning legible through consequence, not speeches.','rebuild_directive':'After loss, preserve memory without becoming trapped by it; build the next piece of the world with other people.','love_life_directive':'Favor love, continued life, mutual care, humor, work, and renewed making as active practices rather than sentimental outcomes.','prose_status':'no_generated_prose'})
 out.sort(key=lambda x:x['record_id'])
 p=OUT/'pip_leadership_constraints.tsv';
 with p.open('w',newline='',encoding='utf-8') as f:
  fields=list(out[0]) if out else ['record_id'];w=csv.DictWriter(f,fieldnames=fields,delimiter='\t');w.writeheader();w.writerows(out)
 manifest={'schema':'wowas.pip.leadership.constraints.v1','source_metadata':str(META.relative_to(ROOT)),'source_sha256':hashlib.sha256(META.read_bytes()).hexdigest(),'pip_linked_rows':len(out),'leadership_modes':dict(Counter(x['leadership_mode'] for x in out)),'prose_generated':False,'invariant':'iconic leader, not martyr; best available action under uncertainty; rebuild through love and life','identity':'record_id is authoritative; scene_id is contextual and may duplicate','policy':'Every Pip-linked record receives a distinct response mode. No character is reduced to worship, dependence, or a copy of Pip.'}
 (OUT/'pip_leadership_constraints.manifest.json').write_text(json.dumps(manifest,indent=2,sort_keys=True)+'\n')
 print(json.dumps({'pip_linked_rows':len(out),'leadership_modes':dict(Counter(x['leadership_mode'] for x in out)),'prose_generated':False},sort_keys=True))
if __name__=='__main__':main()
