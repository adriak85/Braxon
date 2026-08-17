from __future__ import annotations
import csv, re
from collections import defaultdict
from pathlib import Path
p=Path('/home/ubuntu/Braxon/crates/wowas-final-edition-v10/canon/active/scene_index_reasonable_window.tsv')
rows=list(csv.DictReader(p.open(newline='',encoding='utf-8',errors='replace'),delimiter='\t'))
def norm(s): return re.sub(r'\s+',' ',re.sub(r'[^a-z0-9 ]',' ',(s or '').lower())).strip()
for field in ('scene_id','brief_scene_description'):
 d=defaultdict(list)
 for r in rows:
  k=norm(r.get(field,''));
  if k: d[k].append(r)
 print('===',field,'===')
 for k,rs in sorted(d.items(),key=lambda kv:-len(kv[1])):
  if len(rs)>1:
   print(len(rs),k[:180],[(r.get('scene_id'),r.get('source_type'),r.get('source_trace'),r.get('book_num')) for r in rs[:6]])
print('=== source types ===')
from collections import Counter
print(Counter(r.get('source_type','') for r in rows))
