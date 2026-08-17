#!/usr/bin/env python3
from __future__ import annotations
import csv,hashlib,json,re
from collections import Counter,defaultdict
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]; BASE=ROOT/'crates/wowas-final-edition-v10'; OUT=BASE/'canon/active/reconciled_15000'; DATA=OUT/'scene_index_reconciled_metadata.tsv'
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
 with DATA.open(newline='',encoding='utf-8') as f: rows=list(csv.DictReader(f,delimiter='\t'))
 assert sum(r['record_kind']=='recovered_15000_candidate' for r in rows)==15000
 assert sum(r['record_kind']=='main_scene' for r in rows)>0
 assert all(r['prose_status']=='no_generated_prose' for r in rows)
 assert all(r['record_id'] and r['source_path'] for r in rows)
 groups=defaultdict(list)
 for r in rows: groups[r['scene_id']].append(r['record_id'])
 duplicate_ledger=[{'scene_id':sid,'count':len(ids),'record_ids':ids,'policy':'preserve_all_rows; do_not_collapse_without_source_resolution'} for sid,ids in sorted(groups.items()) if len(ids)>1]
 (OUT/'duplicate_scene_id_ledger.json').write_text(json.dumps({'schema':'wowas.duplicate.scene.ledger.v1','groups':len(duplicate_ledger),'rows_in_duplicate_groups':sum(x['count'] for x in duplicate_ledger),'entries':duplicate_ledger},indent=2,sort_keys=True)+'\n',encoding='utf-8')
 patch_paths=[]
 for p in sorted(BASE.rglob('*')):
  if not p.is_file() or any(x in p.parts for x in ('active','generated','canonical_story_tree')): continue
  n=p.name.lower()
  if any(k in n for k in ('patch','addendum','update','override','manifest','apply_order','selection_order','authority')) and p.suffix.lower() in {'.md','.txt','.tsv','.json'}:
   patch_paths.append({'path':str(p.relative_to(ROOT)),'bytes':p.stat().st_size,'sha256':sha(p),'version_rank':int((re.search(r'v(\d+)',p.as_posix(),re.I) or ['0','0'])[1])})
 patch_paths.sort(key=lambda x:(x['version_rank'],x['path']))
 manifest={'schema':'wowas.original.15000.alignment.v1','source':'main historical exact 15000-row artifact','total_rows':len(rows),'main_scene_rows':sum(r['record_kind']=='main_scene' for r in rows),'recovered_15000_candidate_rows':sum(r['record_kind']=='recovered_15000_candidate' for r in rows),'patch_scene_additions':sum(r['record_kind']=='patch_scene_addition' for r in rows),'structured_candidate_rows':sum(r['alignment_status']=='structured_candidate' for r in rows),'record_kind_counts':dict(Counter(r['record_kind'] for r in rows)),'alignment_status_counts':dict(Counter(r['alignment_status'] for r in rows)),'duplicate_scene_id_groups':len(duplicate_ledger),'duplicate_policy':'all rows preserved; duplicate groups ledgered; no source selected merely by label','generated_prose_imported':False,'patch_sources_recursive_inventory':patch_paths,'required_next_gate':'All patch instructions must resolve to applied metadata or explicit unresolved ledger entries; no prose is generated or imported.'}
 (OUT/'alignment_validation_manifest.json').write_text(json.dumps(manifest,indent=2,sort_keys=True)+'\n',encoding='utf-8'); print(json.dumps({k:manifest[k] for k in ('total_rows','main_scene_rows','recovered_15000_candidate_rows','patch_scene_additions','structured_candidate_rows','duplicate_scene_id_groups','generated_prose_imported')},sort_keys=True))
if __name__=='__main__':main()
