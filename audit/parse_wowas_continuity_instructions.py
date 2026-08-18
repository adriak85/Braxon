#!/usr/bin/env python3
from __future__ import annotations
import csv,json,hashlib,re
from collections import Counter,defaultdict
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]; SNAP=Path('/home/ubuntu/Braxon-main-reconstruction-20260817'); OUT=ROOT/'reconstruction'
TERMS=re.compile(r'pip|indalwin|willowjayce|rylos|vayne|boojay|riledge|surviv|die|death|dies|lover|lovers|fate|endgame|world state|state ledger|relationship|orbit|alias|deprecated|canonical|must|never|only',re.I)

def source_paths():
 paths=[]
 for fn in ('continuity-current-source-paths.txt','continuity-main-source-paths.txt'):
  p=SNAP/fn
  if p.exists(): paths.extend(x.strip() for x in p.read_text().splitlines() if x.strip())
 return sorted(set(paths))
def content(rel):
 p=ROOT/rel
 if p.exists(): return p
 # main source available only through git: materialized snapshots may not exist
 return None
def flatten_json(x,prefix=''):
 if isinstance(x,dict):
  for k,v in x.items(): yield from flatten_json(v,prefix+'.'+k if prefix else k)
 elif isinstance(x,list):
  for i,v in enumerate(x): yield from flatten_json(v,f'{prefix}[{i}]')
 else: yield prefix,str(x)
def main():
 paths=source_paths(); records=[]; aliases=[]; fate=[]; instructions=[]; relnotes=[]; stats=[]
 for rel in paths:
  p=content(rel)
  if not p or p.suffix.lower() not in {'.md','.txt','.json','.tsv','.py','.rs'}: continue
  text=p.read_text(encoding='utf-8',errors='replace'); stats.append({'path':rel,'lines':text.count('\n')+1,'bytes':len(text.encode()),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()})
  if p.suffix.lower()=='.json':
   try:
    obj=json.loads(text)
    for k,v in flatten_json(obj):
     if TERMS.search(k+' '+v):
      records.append({'path':rel,'line':k,'text':v[:1000],'kind':'json_field'})
      low=(k+' '+v).lower()
      if any(x in low for x in ('alias','deprecated','canonical','rylos','boojay','riledge','indalwin','pip')): aliases.append(records[-1])
      if any(x in low for x in ('death','dies','surviv','fate','lover','endgame')): fate.append(records[-1])
   except Exception: pass
  for i,line in enumerate(text.splitlines(),1):
   if not TERMS.search(line): continue
   rec={'path':rel,'line':i,'text':line[:1600]}
   records.append(rec)
   low=line.lower()
   if any(x in low for x in ('alias','deprecated','canonical','rylos','boojay','riledge','indalwin','pip')): aliases.append(rec)
   if any(x in low for x in ('death','dies','surviv','fate','lover','endgame','only one')): fate.append(rec)
   if any(x in low for x in ('relationship','orbit','trust','betray','love','dynamic','moving')): relnotes.append(rec)
   if any(x in low for x in ('must','never','should','required','apply','state ledger','world state')): instructions.append(rec)
 OUT.mkdir(exist_ok=True)
 payload={'schema':'wowas.recursive.continuity.instruction.audit.v1','source_file_count':len(stats),'matched_record_count':len(records),'alias_record_count':len(aliases),'fate_record_count':len(fate),'relationship_record_count':len(relnotes),'instruction_record_count':len(instructions),'source_stats':stats,'aliases_and_identity_sample':aliases[:250],'fate_and_survival_sample':fate[:250],'relationships_and_dynamics_sample':relnotes[:250],'instructions_sample':instructions[:250],'policy':'No prose generated. Labels are evidence only until reconciled against explicit source rules.','required_identity_locks':{'rylos_canonical':'Rylos Vayne Johnson','rylos_deprecated':['Riledge','Boojay'],'pip_canonical':'Indalwin On’Rylder Willowjayce','pip_nickname':'Pip'},'required_fate_rule':'Endgame fate must be taken from explicit source rules; do not infer universal mortality. User-specified lock: Rylos is the only one of Pip’s lovers who dies, at the end; survival list remains source-controlled.'}
 (OUT/'WOWAS_RECURSIVE_CONTINUITY_AUDIT.json').write_text(json.dumps(payload,indent=2,sort_keys=True)+'\n')
 md=['# WOWAS Recursive Continuity Instruction Audit','',f"The parser examined **{len(stats)} locally available source files** from the current and main-derived inventories. It extracted **{len(records)} matching records**, including **{len(aliases)} identity/alias records**, **{len(fate)} fate/survival records**, **{len(relnotes)} relationship/dynamics records**, and **{len(instructions)} explicit instruction records**.",'','## Canonical identity locks','', '| Entity | Canonical form | Deprecated forms |','|---|---|---|','| Rylos | Rylos Vayne Johnson | Riledge; Boojay |','| Pip | Indalwin On’Rylder Willowjayce; Pip is the nickname | Any drifted or synthetic replacement |','', '## Fate rule','', 'The audit records a source-controlled survival/death policy. The user-specific lock is that Rylos is the only one of Pip’s lovers who dies, and that death occurs at the end. This does not authorize a universal death rule for every character; every other fate must be resolved from explicit final-book and source documentation.','', '## Boundary','', 'This is a metadata and instruction audit. It does not generate prose, infer undocumented deaths, or overwrite original authored material.']
 (OUT/'WOWAS_RECURSIVE_CONTINUITY_AUDIT.md').write_text('\n'.join(md)+'\n')
 print(json.dumps({k:payload[k] for k in ('source_file_count','matched_record_count','alias_record_count','fate_record_count','relationship_record_count','instruction_record_count')},sort_keys=True))
if __name__=='__main__':main()
