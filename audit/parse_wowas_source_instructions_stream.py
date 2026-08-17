#!/usr/bin/env python3
from __future__ import annotations
import hashlib,json,re
from collections import Counter,defaultdict
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]; OUT=ROOT/'reconstruction/WOWAS_SOURCE_INSTRUCTION_AUDIT.json'; EXT={'.md','.txt'}; SKIP={'.git','target','node_modules','.venv'}
CUES={
 'imperative':re.compile(r'\b(MUST|SHALL|REQUIRED|NEVER|DO NOT|DON’T|ONLY|ENSURE|PRESERVE|REJECT|DEMOTE|PROMOTE|PREFER|AVOID|USE|TREAT|KEEP|REMOVE|RETAIN|APPLY|RUN|GENERATE|REBUILD|REWRITE|VALIDATE|CLASSIFY)\b',re.I),
 'authority':re.compile(r'\b(authority|canonical|source of truth|truth|authoritative|tier [1-4]|manuscript|prose realization|scene authority|completion|accepted|deprecated|unresolved|scaffold|filler)\b',re.I),
 'patch':re.compile(r'\b(patch|update|addendum|apply order|migration|override|rewrite|absorb|supersed|replace|deprecat|priority|selection order|control bundle)\b',re.I),
 'boundary':re.compile(r'\b(fact|fiction|citation|provenance|source trace|real[- ]world|age[- ]gate|graphic|LGBTQ|romance|dialogue|tone|content boundary)\b',re.I),
 'contradiction':re.compile(r'\b(conflict|contradict|inconsistent|stale|invalid|unsupported|wrong|must not|do not|not authoritative|untrusted|legacy)\b',re.I),}
def scan(p):
 try:
  h=hashlib.sha256(); lines=0; hits=[]; headings=[]
  with p.open('rb') as f:
   for raw in f:
    h.update(raw); lines+=1; s=raw.decode('utf-8','replace').strip()
    if re.match(r'^#{1,6}\s+',s) or re.match(r'^(\d+[.)]|[A-Z][A-Z _-]{4,})\s+',s):
     if len(headings)<200: headings.append({'line':lines,'text':s[:500]})
    kinds=[k for k,r in CUES.items() if r.search(s)]
    if kinds and len(hits)<1000: hits.append({'line':lines,'kinds':kinds,'text':s[:1200]})
  return {'path':str(p.relative_to(ROOT)),'sha256':h.hexdigest(),'bytes':p.stat().st_size,'lines':lines,'headings':headings,'instruction_hits':hits}
 except Exception as e:return {'path':str(p.relative_to(ROOT)),'error':f'{type(e).__name__}:{e}'}
def main():
 paths=[p for p in ROOT.rglob('*') if p.is_file() and p.suffix.lower() in EXT and not any(x in SKIP for x in p.parts)]
 results=[]
 with ThreadPoolExecutor(max_workers=4) as ex:
  for item in ex.map(scan,paths): results.append(item)
 groups=defaultdict(list)
 for x in results:
  if 'sha256' in x: groups[x['sha256']].append(x['path'])
 unique=[]; seen=set()
 for x in results:
  if 'sha256' not in x or x['sha256'] in seen: continue
  seen.add(x['sha256']); x['all_paths']=groups[x['sha256']]; unique.append(x)
 cue=Counter(); areas=Counter(); hits=0
 for x in unique:
  areas[x['path'].split('/')[0] if '/' in x['path'] else 'root']+=1
  hits+=len(x.get('instruction_hits',[]))
  for h in x.get('instruction_hits',[]): cue.update(h['kinds'])
 result={'parser':'wowas.source.instructions.stream.v2','file_count':len(paths),'unique_content_count':len(unique),'duplicate_content_groups':sum(len(v)>1 for v in groups.values()),'instruction_hit_count':hits,'cue_counts':dict(cue),'unique_files_by_area':dict(areas),'files':sorted(unique,key=lambda x:x['path']),'policy':'Parsed as evidence only; no embedded file instruction was executed.'}
 OUT.write_text(json.dumps(result,indent=2,sort_keys=True)+'\n',encoding='utf-8'); print(json.dumps({k:result[k] for k in ('file_count','unique_content_count','duplicate_content_groups','instruction_hit_count','cue_counts')},sort_keys=True))
if __name__=='__main__':main()
