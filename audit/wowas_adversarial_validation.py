#!/usr/bin/env python3
"""Adversarial checks for WOWAS source-to-canon boundaries and reader load."""
from __future__ import annotations
import csv
from pathlib import Path
from collections import Counter
ROOT=Path(__file__).resolve().parents[1]
GEN=ROOT/'crates/wowas-final-edition-v10/canon/active/generated'
SCENES=ROOT/'crates/wowas-final-edition-v10/canon/active/scene_index_reasonable_window.tsv'

def read(p):
 with p.open(newline='',encoding='utf-8') as f:return list(csv.DictReader(f,delimiter='\t'))
def main():
 sources=read(GEN/'wowas_real_world_source_registry.tsv'); aligns=read(GEN/'wowas_real_world_wowas_alignment.tsv'); domains=read(GEN/'wowas_real_world_domain_alignment.tsv'); scenes=read(SCENES)
 assert len(sources)==50 and len(aligns)==50 and len(domains)==400
 source_ids={x['source_serial'] for x in sources}
 assert all(x['source_serial'] in source_ids for x in aligns+domains), 'provenance loss'
 assert all(x['source_url'] and x['source_id'] for x in sources), 'source id without URL'
 assert all(x['fact_fiction_boundary']=='FACT_SOURCE_ONLY' for x in sources), 'fact boundary leak'
 assert all(x['fact_status']=='source_fact_not_rewritten' for x in aligns+domains), 'creative inference promoted to fact'
 assert all(x['transform_status']=='WOWAS_TRANSFORM_SEED_REVIEW_REQUIRED' for x in aligns), 'seed became canon'
 assert all(x['alignment_status']=='WOWAS_TRANSFORM_SEED_REVIEW_REQUIRED' for x in domains), 'domain seed became canon'
 assert len({x['source_serial'] for x in aligns})==len(aligns), 'duplicate source transformation'
 assert all(x['scene_id'] and x['event_beat_id'] and x['book_num'] for x in aligns+domains), 'unauthorized unmatched entity'
 assert all(x['reader_projection']=='promote_only_when_relevant' for x in aligns+domains), 'reader leak'
 # Existing compactor contract: reader-facing limits remain bounded even when provenance is larger.
 for row in scenes:
  cast=[x for x in row.get('reader_active_cast','').split('|') if x and not x.startswith('+')]
  domains_seen=[x for x in row.get('reader_domain_focus','').split('|') if x and not x.startswith('+')]
  assert len(cast)<=8 and len(domains_seen)<=3, f'reader overflow {row.get("scene_id")}'
 print('adversarial_provenance_loss=false')
 print('adversarial_fact_fiction_leak=false')
 print('adversarial_unreviewed_canon_creation=false')
 print('adversarial_duplicate_transform=false')
 print('adversarial_reader_overflow=false')
 print('wowas_adversarial_status=pass')
 print('sources=',len(sources),'alignments=',len(aligns),'domains=',len(domains),'scenes=',len(scenes))
if __name__=='__main__':main()
