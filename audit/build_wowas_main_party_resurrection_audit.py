#!/usr/bin/env python3
from __future__ import annotations
import json,re,csv,hashlib
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]; BASE=ROOT/'crates/wowas-final-edition-v10'; OUT=ROOT/'reconstruction'
def txt(p): return p.read_text(encoding='utf-8',errors='replace') if p.exists() else ''
def main():
 regp=BASE/'canon/canonical_story_tree/characters/06_CHARACTER_REGISTRY.json'; reg=json.loads(regp.read_text()); chars=reg.get('characters',[])
 docs=[]
 for p in [BASE/'canon/wowas_endgame_judgment_matrix_v10.tsv',BASE/'canon/patches/v10/wowas_endgame_judgment_matrix_v10.tsv',BASE/'canon/canonical_story_tree/_scene_index.tsv',BASE/'canon/active/reconciled_15000/scene_index_reconciled_metadata.tsv']:
  if p.exists(): docs.append((p,txt(p)))
 main_party=[c for c in chars if c.get('tier')==0]
 # Include named recurring tier-one characters with direct Pip relationship evidence, but keep core tier explicit.
 pip_related=[]
 for c in chars:
  blob=json.dumps(c,ensure_ascii=False).lower()
  if c not in main_party and ('pip' in blob or 'indalwin' in blob or 'willowjayce' in blob): pip_related.append(c)
 def evidence(name, terms):
  vals=[]
  for p,t in docs:
   if name.lower() not in t.lower() and (name.split('(')[0].strip().lower() not in t.lower()): continue
   for i,line in enumerate(t.splitlines(),1):
    low=line.lower()
    if any(x in low for x in terms) and (name.lower() in low or name.split('(')[0].strip().lower() in low): vals.append({'path':str(p.relative_to(ROOT)),'line':i,'text':line[:1000]})
  return vals
 rows=[]
 for c in main_party:
  name=c.get('name',''); rows.append({'canonical_id':c.get('id',''),'name':name,'tier':c.get('tier'),'role':c.get('role',''),'first_book':c.get('first_book',''),'last_book':c.get('last_book',''),'pronouns':c.get('pronouns',''),'locked_traits':c.get('locked_traits',[]),'magic':c.get('magic',[]),'sources':c.get('sources',[]),'brief_description':f"{c.get('role','').strip()}; locked traits: {'; '.join(c.get('locked_traits',[])[:5])}",'death_evidence':evidence(name,['death','dies','died','mortal wound']),'resurrection_evidence':evidence(name,['resurrect','reviv','rebirth','return','restore']),'viewpoint_evidence':evidence(name,['viewpoint','pov','perspective','witness','from'])})
 payload={'schema':'wowas.main.party.resurrection.audit.v1','core_main_party_count':len(rows),'core_main_party':[{k:v for k,v in r.items() if k not in ('death_evidence','resurrection_evidence','viewpoint_evidence')} | {'death_evidence_count':len(r['death_evidence']),'resurrection_evidence_count':len(r['resurrection_evidence']),'viewpoint_evidence_count':len(r['viewpoint_evidence']),'evidence_samples':{'death':r['death_evidence'][:3],'resurrection':r['resurrection_evidence'][:3],'viewpoint':r['viewpoint_evidence'][:3]}} for r in rows],'pip_related_named_count':len(pip_related),'pip_related_named_ids':[c.get('id','') for c in pip_related],'resurrection_requirements':{'death_and_resurrection_parity':'each resurrection must carry comparable emotional, relational, and world-state consequence to the death; a mention alone is insufficient','viewpoint_requirement':'resurrection must be witnessed/understood from distinct viewpoints in the main party; no single-viewpoint collapse','identity_rule':'Rylos Vayne Johnson is canonical; Pip is Indalwin On’Rylder Willowjayce; Pip is his nickname','fate_rule':'Rylos is the only one of Pip’s lovers who dies, at the end, per user-specified lock; all other survival/death rules require explicit final-book source evidence','prose_generated':False},'source_hash':hashlib.sha256(regp.read_bytes()).hexdigest()}
 (OUT/'WOWAS_MAIN_PARTY_RESURRECTION_AUDIT.json').write_text(json.dumps(payload,indent=2,ensure_ascii=False,sort_keys=True)+'\n')
 md=['# WOWAS Main Party and Resurrection Audit','',f"The core main party is defined from the source registry’s Tier 0 records: **{len(rows)} characters**. The table below reports source-backed role and locked-trait descriptions, not newly authored prose.",'','## Core main party','', '| Name | Role | Books | Brief source-backed description | Death evidence | Resurrection evidence | Viewpoint evidence |','|---|---|---|---|---:|---:|---:|']
 for r in rows: md.append(f"| **{r['name']}** | {r['role']} | {r['first_book']}–{r['last_book']} | {r['brief_description'].replace('|','/')} | {len(r['death_evidence'])} | {len(r['resurrection_evidence'])} | {len(r['viewpoint_evidence'])} |")
 md += ['', '## Required final-book behavior','', 'A resurrection is not complete merely because a character returns. It must carry a consequence comparable to the death: changed relationships, altered world state, changed self-understanding, and a distinct response from multiple main-party viewpoints. The audit records evidence counts and leaves any insufficiently evidenced character blocked from prose promotion.','', '## Canonical identity locks','', 'Rylos must appear as **Rylos Vayne Johnson**. `Riledge` and `Boojay` are deprecated aliases and may remain only inside explicit historical/deprecation evidence. Pip must resolve to **Indalwin On’Rylder Willowjayce**, with **Pip** as his nickname.','', '## Boundary','', 'No prose was generated. Missing or weak resurrection/viewpoint evidence is a gate failure to be repaired through source-backed metadata and scene planning, not filled with invented narrative.']
 (OUT/'WOWAS_MAIN_PARTY_RESURRECTION_AUDIT.md').write_text('\n'.join(md)+'\n',encoding='utf-8')
 print(json.dumps({'core_main_party_count':len(rows),'names':[r['name'] for r in rows],'pip_related_named_count':len(pip_related),'prose_generated':False},ensure_ascii=False,sort_keys=True))
if __name__=='__main__':main()
