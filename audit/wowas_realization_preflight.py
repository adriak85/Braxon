#!/usr/bin/env python3
from __future__ import annotations
import csv,json,hashlib
from pathlib import Path
from collections import Counter
ROOT=Path(__file__).resolve().parents[1]; BASE=ROOT/'crates/wowas-final-edition-v10'; FL=BASE/'canon/active/authored_flavor'
META=BASE/'canon/active/reconciled_15000/scene_index_reconciled_metadata.tsv'
CON=FL/'generated_character_flavor_constraints.tsv'; MECH=FL/'source_media_mechanism_catalog.tsv'; DYN=FL/'authored_dynamics_lattice.tsv'; CHAR=FL/'authored_character_flavor_lattice.tsv'; PIP=FL/'pip_leadership_constraints.tsv'; RES=FL/'resurrection_viewpoint_matrix.tsv'

def tsv(p):
 with p.open(newline='',encoding='utf-8',errors='replace') as f:return list(csv.DictReader(f,delimiter='\t'))
def main():
 failures=[]; checks=[]
 for p in (META,CON,MECH,DYN,CHAR,PIP,RES):
  if not p.exists(): failures.append(f'missing:{p.relative_to(ROOT)}')
  else: checks.append({'path':str(p.relative_to(ROOT)),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()})
 if failures: raise SystemExit(json.dumps({'status':'blocked','failures':failures},indent=2))
 meta=tsv(META); con=tsv(CON); mechs=tsv(MECH); dyn=tsv(DYN); chars=tsv(CHAR); pip=tsv(PIP); res=tsv(RES)
 if len({r.get('record_id','') for r in meta})!=len(meta): failures.append('metadata record_id collision')
 if len({r.get('generated_character_id','') for r in con})!=len(con): failures.append('generated character ID collision')
 if any(r.get('prose_status')!='no_generated_prose' for r in con): failures.append('generated prose status present in constraint layer')
 if any(not r.get('canonical_id') for r in chars): failures.append('authored character missing canonical_id')
 if any(not r.get('abstract_mechanisms') for r in mechs): failures.append('source mechanism missing abstract mechanism')
 if not dyn: failures.append('authored dynamics lattice empty')
 if not pip: failures.append('Pip leadership constraints empty')
 if pip and any('not a martyr' not in r.get('pip_invariant','').lower() for r in pip): failures.append('Pip non-martyr invariant missing')
 if pip and len({r.get('record_id','') for r in pip})!=len(pip): failures.append('Pip constraint record_id collision')
 if pip and any(r.get('prose_status')!='no_generated_prose' for r in pip): failures.append('Pip constraint prose status invalid')
 if not res: failures.append('resurrection viewpoint matrix empty')
 if res and any(r.get('prose_status')!='no_generated_prose' for r in res): failures.append('resurrection matrix prose status invalid')
 if res and any('Rylos' in r.get('target_name','') and r.get('final_state_rule')!='final_death_only' for r in res): failures.append('Rylos death-only lock violated')
 if res and len({r.get('matrix_id','') for r in res})!=len(res): failures.append('resurrection matrix ID collision')
 checks += [{'name':'metadata_unique_record_id','value':len(meta)},{'name':'generated_constraints_unique_id','value':len(con)},{'name':'authored_characters','value':len(chars)},{'name':'authored_dynamics','value':len(dyn)},{'name':'source_mechanism_rows','value':len(mechs)},{'name':'pip_leadership_rows','value':len(pip)},{'name':'pip_leadership_modes','value':len(set(r.get('leadership_mode','') for r in pip))},{'name':'resurrection_viewpoint_rows','value':len(res)},{'name':'resurrection_viewpoints','value':len(set(r.get('viewpoint_character_id','') for r in res))},{'name':'scene_id_duplicate_groups','value':sum(v>1 for v in Counter(r.get('scene_id','') for r in meta).values())}]
 contract={'schema':'wowas.realization.preflight.v1','status':'pass' if not failures else 'blocked','prose_generation_permitted':False,'failure_mode':'fail_closed','required_inputs':['reconciled metadata keyed by record_id','authored_character_flavor_lattice.tsv','authored_dynamics_lattice.tsv','source_media_mechanism_catalog.tsv','generated_character_flavor_constraints.tsv','pip_leadership_constraints.tsv','resurrection_viewpoint_matrix.tsv'],'required_runtime_guards':['write only to staging first','tone/cadence/style/token gate before promotion','rolling state ledger across books','record_id for output identity; scene_id is non-unique context only','no direct copying from source media; abstract mechanisms only','unmapped anchors remain quarantined'],'checks':checks,'failures':failures,'unmapped_anchor_policy':'unmapped anchors may not receive invented authored identity; they remain explicit until source-dispositioned'}
 (ROOT/'reconstruction/WOWAS_REALIZATION_PREFLIGHT.json').write_text(json.dumps(contract,indent=2,sort_keys=True)+'\n')
 print(json.dumps({'status':contract['status'],'prose_generation_permitted':False,'failures':failures,'checks':len(checks)},sort_keys=True))
 if failures: raise SystemExit(2)
if __name__=='__main__':main()
