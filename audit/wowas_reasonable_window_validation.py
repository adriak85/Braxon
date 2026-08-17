#!/usr/bin/env python3
from __future__ import annotations
import csv, json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
CANON=ROOT/'crates/wowas-final-edition-v10/canon/active'
GEN=CANON/'generated'
def read(p):
 with p.open(newline='',encoding='utf-8',errors='replace') as f:return list(csv.DictReader(f,delimiter='\t'))
def main():
 scenes=read(CANON/'scene_index_reasonable_window.tsv')
 spine=read(CANON/'book_spine_33.tsv')
 timeline=read(GEN/'wowas_character_timeline_schedule.tsv')
 rel=read(GEN/'wowas_relationship_ledger.tsv')
 background=read(GEN/'wowas_background_population_2000000.tsv')
 fail=[]
 if len(spine)!=33:fail.append(f'spine={len(spine)}')
 if len(scenes)!=2019:fail.append(f'scenes={len(scenes)}')
 ids=[r.get('scene_id','') for r in scenes]
 if len(ids)!=len(set(ids)):fail.append('duplicate_scene_ids')
 desc=[' '.join(r.get('brief_scene_description','').lower().split()) for r in scenes]
 if len(desc)!=len(set(desc)):fail.append('duplicate_scene_descriptions')
 scene_ids=set(ids)
 event_ids={r.get('event_beat_id') for r in scenes if r.get('event_beat_id')}
 domains={d for r in scenes for d in r.get('domain_flags','').split('|') if d}
 if not {'characters','creatures','world_introduction','quests'}.issubset(domains):fail.append('incomplete_domain_coverage')
 if not event_ids:fail.append('no_event_beats')
 missing_timeline=sum(1 for r in timeline if r.get('assigned_scene_id') and r['assigned_scene_id'] not in scene_ids)
 missing_rel=sum(1 for r in rel if r.get('scene_link') and r['scene_link'] not in scene_ids)
 missing_bg=sum(1 for r in background if r.get('scene_id') and r['scene_id'] not in scene_ids)
 if missing_timeline:fail.append(f'timeline_scene_refs={missing_timeline}')
 if missing_rel:fail.append(f'relationship_scene_refs={missing_rel}')
 if missing_bg:fail.append(f'background_scene_refs={missing_bg}')
 result={'status':'fail' if fail else 'pass','spine_books':len(spine),'scenes':len(scenes),'unique_scene_ids':len(set(ids)),'unique_descriptions':len(set(desc)),'event_beats':len(event_ids),'domains':sorted(domains),'timeline_rows':len(timeline),'relationship_rows':len(rel),'background_rows':len(background),'missing_timeline_refs':missing_timeline,'missing_relationship_refs':missing_rel,'missing_background_refs':missing_bg,'failures':fail}
 (ROOT/'reconstruction/WOWAS_REASONABLE_WINDOW_VALIDATION.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n',encoding='utf-8')
 print(json.dumps(result,sort_keys=True))
 raise SystemExit(1 if fail else 0)
if __name__=='__main__':main()
