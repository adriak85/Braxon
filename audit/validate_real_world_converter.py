#!/usr/bin/env python3
from __future__ import annotations
import csv
from collections import Counter
from pathlib import Path
import requests

ROOT=Path(__file__).resolve().parents[1]
BASE=ROOT/'crates/wowas-final-edition-v10/canon/active/generated'
SERIEL=ROOT/'crates/wowas-final-edition-v10/reconstruction/SERIEL_CROSSWALK.tsv'

def read(name):
    with (BASE/name).open(newline='',encoding='utf-8') as f:return list(csv.DictReader(f,delimiter='\t'))

def main():
    sources=read('wowas_real_world_source_registry.tsv')
    aligns=read('wowas_real_world_wowas_alignment.tsv')
    domains=read('wowas_real_world_domain_alignment.tsv')
    assert len(sources)==50 and len(aligns)==50 and len(domains)==400
    assert Counter(x['source_type'] for x in sources)==Counter({'city':30,'landmark':20})
    assert Counter(x['domain'] for x in domains)==Counter({k:50 for k in ('location','artifact','faction','culture','hazard','route','quest','character_world_role')})
    assert all(x['source_url'] and x['source_id'] and x['citation_required']=='true' for x in sources)
    assert all(x['fact_fiction_boundary']=='FACT_SOURCE_ONLY' for x in sources)
    assert all(x['scene_id'] and x['event_beat_id'] and x['book_num'] and x['citation_url'] for x in aligns+domains)
    assert all(x['reader_projection']=='promote_only_when_relevant' for x in aligns+domains)
    assert all(x['digital_variance_policy']=='presentation_may_vary; source_fact_and_canon_immutable' for x in aligns+domains)
    url_status=[]
    for row in sources:
        try:
            response=requests.get(row['source_url'],timeout=20,allow_redirects=True)
            url_status.append((row['source_id'],response.status_code))
        except requests.RequestException as exc:
            url_status.append((row['source_id'],f'ERROR:{type(exc).__name__}'))
    with SERIEL.open(newline='',encoding='utf-8') as f:
        cross=list(csv.DictReader(f,delimiter='\t'))
    assert all(row.get('link_status')!='unlinked' for row in cross)
    print('sources=50 cities=30 landmarks=20')
    print('domain_seeds=400 eight_domains_complete=true')
    print('alignment_citations_and_scene_beats=true')
    print('reader_projection_bounds=true')
    print('seriel_records=',len(cross),'seriel_unlinked=0')
    print('url_status=',dict(url_status))

if __name__=='__main__':main()
