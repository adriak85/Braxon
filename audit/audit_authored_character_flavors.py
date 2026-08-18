#!/usr/bin/env python3
from __future__ import annotations
import csv, json, re, hashlib
from collections import Counter, defaultdict
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
BASE=ROOT/'crates/wowas-final-edition-v10'
OUT=ROOT/'reconstruction'
SOURCES=[
 'canon/canonical_story_tree/characters/06_CHARACTER_REGISTRY.json',
 'canon/wowas_character_timeline_lattice_v2.tsv',
 'canon/wowas_character_timeline_lattice_UNIFIED_v14.tsv',
 'canon/wowas_orbit_file_v2.tsv',
 'canon/wowas_protected_support_cast_v7.tsv',
 'canon/control/prose_and_tone_guide_v14.json',
 'canon/patches/PROSE_AND_TONE_GUIDE.json',
 'canon/patches/v10/wowas_prose_and_tone_patch_v10.md',
 'canon/patches/v10/wowas_dialogue_and_play_insertion_patch_v10.md',
 'canon/patches/v12/wowas_quality_romance_calendar_and_resonance_patch_v12.md',
 'canon/patches/v12/wowas_scene_connector_cast_tracker_apply_patch_v12.md',
 'canon/canonical_story_tree/characters/01_NAMED_CAST_TOP300.md',
 'canon/canonical_story_tree/characters/04_SOURCE_HERO_ENGINE.md',
 'canon/canonical_story_tree/characters/05_SELF_CORRECTING_CANON_RULES.md',
]

def read_json(path):
 try:return json.loads(path.read_text(encoding='utf-8'))
 except Exception:return None

def flatten(obj, prefix=''):
 out=[]
 if isinstance(obj,dict):
  for k,v in obj.items():out.extend(flatten(v, prefix+'.'+k if prefix else k))
 elif isinstance(obj,list):
  for i,v in enumerate(obj):out.extend(flatten(v,f'{prefix}[{i}]'))
 else: out.append((prefix,str(obj)))
 return out

def rows(path):
 with path.open(newline='',encoding='utf-8',errors='replace') as f:return list(csv.DictReader(f,delimiter='\t'))

def names_from_text(text):
 # preserve explicit canonical IDs and name labels; do not infer prose characters from arbitrary words
 found=set(re.findall(r'wowas::[A-Za-z0-9_]+',text))
 for line in text.splitlines():
  m=re.match(r'#{1,6}\s+(.+?)\s*$',line)
  if m and len(m.group(1))<120: found.add(m.group(1).strip())
 return sorted(found)

def main():
 evidence=[]; source_stats=[]; authored_ids=set(); flavor_fields=defaultdict(list); dynamics=[]
 for rel in SOURCES:
  p=BASE/rel
  if not p.exists(): continue
  text=p.read_text(encoding='utf-8',errors='replace'); h=hashlib.sha256(p.read_bytes()).hexdigest(); source_stats.append({'path':str(p.relative_to(ROOT)),'bytes':p.stat().st_size,'lines':text.count('\n')+1,'sha256':h})
  evidence.extend((rel,k,v) for k,v in flatten(read_json(p)) if read_json(p) is not None and any(x in k.lower() for x in ('name','id','role','function','voice','tone','source','personality','relationship','orbit','flavor','pressure','emotion','dialogue','dynamic','arc')))
  if p.suffix.lower() in {'.md','.txt'}: names_from_text(text)
  if p.suffix.lower()=='.tsv':
   for r in rows(p):
    for key in ('character_id','character_code','canonical_id','name','canonical_name','from_character','to_character'):
     if r.get(key): authored_ids.add(r[key])
    if r.get('source_basis') or r.get('role_summary') or r.get('book_function_notes') or r.get('pressure_translation'):
     dynamics.append({'source':rel,'character':r.get('canonical_name',r.get('character_name',r.get('from_character',''))),'role_summary':r.get('role_summary',''),'function_notes':r.get('book_function_notes',''),'pressure':r.get('pressure_translation',r.get('personality_pressure','')),'source_basis':r.get('source_basis','')})
  else:
   for _,v in flatten(read_json(p) or {}):
    if v.startswith('wowas::'): authored_ids.add(v)
 genp=BASE/'canon/active/generated/wowas_generated_characters_5000.tsv'
 gen=[]
 if genp.exists():
  gen=rows(genp)
 gen_ids={r.get('character_id','') for r in gen if r.get('character_id')}
 gen_source={r.get('source_anchor','') for r in gen if r.get('source_anchor')}
 missing=sorted(x for x in authored_ids if x and x not in gen_ids)
 matched=sorted(x for x in authored_ids if x and x in gen_ids)
 report={'schema':'wowas.authored.character.flavor.audit.v1','policy':'Original authored guidance and non-generator lattices are preserved. Generator rows may receive structured flavor constraints but no prose is created here.','source_stats':source_stats,'authored_identifier_count':len(authored_ids),'generated_identifier_count':len(gen_ids),'authored_ids_matched_in_generated':len(matched),'authored_ids_missing_from_generated':len(missing),'missing_authored_ids':missing[:1000],'dynamics_evidence_count':len(dynamics),'dynamics_evidence_sample':dynamics[:500],'rule_keyword_evidence_count':len(evidence),'rule_keyword_evidence_sample':evidence[:1000],'generator_source_anchor_count':len(gen_source),'checks':{'original_sources_read':bool(source_stats),'generated_prose_created':False,'original_lattice_overwritten':False,'identity_matching_key':'canonical identifier where available; name aliases require explicit mapping','required_realization_guards':['record_id not scene_id for output identity','style gate before manuscript promotion','rolling state ledger across book boundaries','no generated prose in this audit']}}
 (OUT/'WOWAS_AUTHORED_CHARACTER_FLAVOR_AUDIT.json').write_text(json.dumps(report,indent=2,sort_keys=True)+'\n',encoding='utf-8')
 md=['# WOWAS Authored Character Flavor Audit','',f"This audit read **{len(source_stats)} source files**, identified **{len(authored_ids)} authored/lattice identifiers**, and compared them with **{len(gen_ids)} generated identifiers**. It does not generate prose or rewrite original authored sources.",'','## Integrity boundary','', '> Character flavor is treated as structured guidance: voice, pressure response, role, relationship function, source stack, dynamic constraints, and continuity invariants. It is not converted into finished prose by this audit.','', '## Results','', '| Measure | Value |','|---|---:|',f'| Source files read | {len(source_stats)} |',f'| Authored/lattice identifiers | {len(authored_ids)} |',f'| Generated identifiers | {len(gen_ids)} |',f'| Authored identifiers matched in generated registry | {len(matched)} |',f'| Authored identifiers not matched in generated registry | {len(missing)} |',f'| Dynamics evidence rows | {len(dynamics)} |','', '## Required realization safeguards','', 'The attached synchronization note correctly identifies three safeguards: use `record_id` rather than raw `scene_id` for duplicate rows; stage prose and require tone/style/token checks before promotion; and carry a rolling state ledger across book boundaries. These are recorded as requirements, not claimed as executed realization behavior.','', '## Source inventory','', '| Source | Lines | Bytes | SHA-256 |','|---|---:|---:|---|']
 for s in source_stats: md.append(f"| `{s['path']}` | {s['lines']} | {s['bytes']} | `{s['sha256']}` |")
 md += ['', '## Interpretation', '', f"The audit found **{len(missing)} authored identifiers not directly matched** by the generated registry. Those should not be silently regenerated or flattened. They need explicit alias/canonical-ID mapping or a source-backed preservation lane. The presence of a generated row is not evidence that its voice, dynamic, or source-specific flavor has been faithfully carried over."]
 (OUT/'WOWAS_AUTHORED_CHARACTER_FLAVOR_AUDIT.md').write_text('\n'.join(md)+'\n',encoding='utf-8')
 print(json.dumps({'source_files':len(source_stats),'authored_ids':len(authored_ids),'generated_ids':len(gen_ids),'matched':len(matched),'missing':len(missing),'dynamics_rows':len(dynamics),'generated_prose_created':False},sort_keys=True))
if __name__=='__main__':main()
