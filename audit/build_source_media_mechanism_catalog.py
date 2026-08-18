#!/usr/bin/env python3
from __future__ import annotations
import json,csv,re,hashlib
from pathlib import Path
from collections import Counter
ROOT=Path(__file__).resolve().parents[1]; BASE=ROOT/'crates/wowas-final-edition-v10'; OUT=BASE/'canon/active/authored_flavor'

def abstract(text):
 s=text.lower(); out=[]
 rules=[('moral_pressure','duty|ethics|institution|approval|justice|wrong|right|accountability|redemption|sacrifice'),('marginal_agency','margin|back corridor|unofficial|excluded|outsider|unrecognized|underestimate'),('precision_memory','memory|records|archive|document|precision|knowledge|librarian'),('protective_care','care|healer|support|protect|guardian|family|emotional|companion'),('transformation_pressure','transformation|metamorph|shadow|monster|animal|wound|change'),('threshold_wonder','threshold|door|portal|wonder|mythic|otherworld|cosmic'),('strategic_conflict','military|strateg|faction|politic|operator|counterforce|rival'),('comic_relief_with_teeth','humor|comic|absurd|underestimated|playful'),('voice_restraint','quiet|clipped|specific|silence|does_not_speak|sparse'),('relationship_ambivalence','betrayal|damaged|toxic|co-dependent|love|trust|devotion|interdependence'),('embodied_place','market|road|city|forge|sea|world|landscape|ecology|place')]
 for k,p in rules:
  if re.search(p,s):out.append(k)
 return out or ['unclassified_source_mechanism']
def main():
 regp=BASE/'canon/canonical_story_tree/characters/06_CHARACTER_REGISTRY.json'; reg=json.loads(regp.read_text()); chars=reg['characters']
 fields=['canonical_id','canonical_name','source_reference','abstract_mechanisms','originality_rule','disallowed_copying','realization_use']
 rows=[]
 for c in chars:
  refs=c.get('sources',[]); text=' | '.join(map(str,refs+[c.get('shadow',''),c.get('role',''),c.get('locked_traits',[])])); mechs=abstract(text)
  rows.append({'canonical_id':c.get('id',''),'canonical_name':c.get('name',''),'source_reference':' | '.join(map(str,refs)),'abstract_mechanisms':' | '.join(mechs),'originality_rule':'Use mechanisms as constraints on pressure, voice, relationship, and choice; invent new names, events, imagery, and dialogue.','disallowed_copying':'Do not reproduce source character identity, plot, scene order, dialogue, distinctive phrasing, or protected setting expression.','realization_use':'Select a subset by book state, scene pressure, relationship state, and character history; never clone the source stack as a personality.'})
 p=OUT/'source_media_mechanism_catalog.tsv';
 with p.open('w',newline='',encoding='utf-8') as f:w=csv.DictWriter(f,fieldnames=fields,delimiter='\t');w.writeheader();w.writerows(rows)
 manifest={'schema':'wowas.source.media.mechanisms.v1','source':str(regp.relative_to(ROOT)),'source_sha256':hashlib.sha256(regp.read_bytes()).hexdigest(),'rows':len(rows),'mechanism_counts':dict(Counter(m for r in rows for m in r['abstract_mechanisms'].split(' | '))),'policy':'Influence is converted into abstract craft mechanisms only. No source expression is copied or imported.','generated_prose':False}
 (OUT/'source_media_mechanism_catalog.manifest.json').write_text(json.dumps(manifest,indent=2,sort_keys=True)+'\n')
 print(json.dumps({'rows':len(rows),'mechanisms':len(manifest['mechanism_counts']),'generated_prose':False},sort_keys=True))
if __name__=='__main__':main()
