#!/usr/bin/env python3
from __future__ import annotations
import csv, json, re
from collections import Counter
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]; BASE=ROOT/'crates/wowas-final-edition-v10'; SCENE=BASE/'canon/wowas_clean_scene_index_v2.tsv'
def read(p):
 with p.open(newline='',encoding='utf-8',errors='replace') as f:return list(csv.DictReader(f,delimiter='\t'))
def classify(r):
 layer=(r.get('source_layer','')+' '+r.get('source_type','')+' '+r.get('source_trace','')).upper(); title=(r.get('clean_title','')+' '+r.get('old_title','')).lower(); desc=r.get('brief_scene_description','')
 if 'SOURCE_DERIVED_RECONSTRUCTION' in layer or 'RECONSTRUCTED CONTINUITY SCENE' in title or 'FILLED TO TARGET SCENE COUNT' in desc.upper(): return 'rejected_scaffold_tier4'
 if any(x in layer for x in ('DIRECT_SOURCE','ACTUAL_SOURCE','COMPILECAT')):
  if 'PLACED_FILE' in layer or 'SCENE_EXPANSION_EXTRACT' in layer: return 'accepted_tier1_or_2'
  return 'requires_prose_realization_tier2'
 if any(x in layer for x in ('REWRITTEN_BEAT_END','REWRITTEN_FROM_DETAIL','REWRITTEN_BOOK_OPEN','REWRITTEN_PRESSURE_PATTERN','TONE AND STRUCTURE LOCK')): return 'requires_prose_realization_tier3'
 if any(x in title for x in ('near ness =','presence','ghost =','farewell =','waking =','rebirth =')): return 'requires_prose_realization_tier3'
 return 'unresolved_source_classification'
def main():
 rows=read(SCENE); counts=Counter(classify(r) for r in rows); by_book=Counter((r.get('book_num',''),classify(r)) for r in rows)
 result={'source_file':str(SCENE.relative_to(ROOT)),'rows':len(rows),'unique_scene_ids':len({r.get('scene_id','') for r in rows}),'classification_counts':dict(sorted(counts.items())),'by_book':{b:{k:v for (bb,k),v in by_book.items() if bb==b} for b in sorted({r.get('book_num','') for r in rows},key=lambda x:int(x or 0))},'rules_source':'canon/patches/v10/wowas_scene_authority_cleanup_patch_v10.md','authority_policy':'Only accepted_tier1_or_2 records are eligible for canonical authority; all other rows require explicit prose realization or remain non-authoritative.'}
 (ROOT/'reconstruction/WOWAS_SOURCE_AUTHORITY_AUDIT.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n',encoding='utf-8'); print(json.dumps({'rows':result['rows'],'unique_scene_ids':result['unique_scene_ids'],'classification_counts':result['classification_counts']},sort_keys=True))
if __name__=='__main__':main()
