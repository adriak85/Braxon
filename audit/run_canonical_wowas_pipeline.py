#!/usr/bin/env python3
"""Single canonical WOWAS materialization and validation entrypoint.

The pipeline has one authoritative scene writer (`wowas_completion.py`). All
other stages write distinct derived surfaces. A failing stage aborts the run;
there is no fallback writer or second scene index.
"""
from __future__ import annotations
import subprocess, sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
PY=sys.executable
STAGES=[
 ('scene_index', 'audit/wowas_completion.py'),
 ('world_domain_maps', 'audit/wowas_world_domain_generators.py'),
 ('real_world_converter', 'audit/wowas_real_world_converter.py'),
 ('linkage_maps', 'audit/wowas_missing_maps.py'),
 ('timeline_scheduler', 'audit/wowas_character_timeline_scheduler.py'),
 ('attention_projection', 'audit/wowas_resolve_attention.py'),
 ('beat_reformat_books', 'audit/wowas_beat_reformatter.py'),
 ('background_population', 'audit/wowas_materialize_background.py'),
 ('seriel_crosswalk', 'audit/wowas_seriel_crosswalk.py'),
 ('schema_gate', 'audit/wowas_schema_gate.py'),
 ('reasonable_window_gate', 'audit/wowas_reasonable_window_validation.py'),
 ('background_gate', 'audit/validate_background_population.py'),
 ('real_world_gate', 'audit/validate_real_world_converter.py'),
 ('adversarial_gate', 'audit/wowas_adversarial_validation.py'),
 ('originality_gate', 'audit/wowas_originality.py'),
 ('identity_gate', 'audit/validate_wowas_identity.py'),
 ('attention_gate', 'audit/wowas_character_attention.py'),
]
FORBIDDEN=('scene_index.tsv','scene_index_15000.tsv','scene_index_15000_reconciled.tsv','scene_index_reasonable_window_reconciled.tsv')
def main():
 for forbidden in FORBIDDEN:
  for root in (ROOT/'audit', ROOT/'crates', ROOT/'reconstruction', ROOT/'scripts'):
   for p in root.rglob('*'):
    if p == Path(__file__):
     continue
    if p.is_file() and p.suffix in {'.py','.rs','.sh','.toml','.json','.md'}:
     if forbidden in p.read_text(encoding='utf-8',errors='ignore') and p.name not in {'WOWAS_COMPLETION_REPORT.md','WOWAS_SCHEMA_MIGRATION_REPORT.md'}:
      raise SystemExit(f'deprecated path remains in executable/release surface: {p}:{forbidden}')
 for name, rel in STAGES:
  print(f'=== {name} ===', flush=True)
  subprocess.run([PY, str(ROOT/rel)], cwd=ROOT, check=True)
 print('canonical_wowas_pipeline=pass')
if __name__=='__main__':main()
