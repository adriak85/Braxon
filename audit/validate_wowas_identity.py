#!/usr/bin/env python3
from __future__ import annotations
import csv
from collections import Counter
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
P=ROOT/'crates/wowas-final-edition-v10/canon/active/generated/wowas_generated_characters_5000.tsv'
def main():
 with P.open(newline='',encoding='utf-8') as f: rows=list(csv.DictReader(f,delimiter='\t'))
 required={'identity_profile','pronouns','age_band','adult_role_eligibility','adult_role_profile','identity_serial','content_rating'}
 missing=sum(1 for r in rows if any(not r.get(k) for k in required))
 identities=Counter(r['identity_profile'] for r in rows); roles=Counter(r['adult_role_profile'] for r in rows)
 print('characters=',len(rows),'identities=',dict(identities),'adult_roles=',dict(roles))
 if len(rows)!=5000 or missing or identities['gay_male'] < 2000 or roles['top_role_preference'] < 2000: raise SystemExit(1)
if __name__=='__main__':main()
