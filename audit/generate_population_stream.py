#!/usr/bin/env python3
"""Stream deterministic two-million population records for WoWAS scale validation."""
from __future__ import annotations
import argparse, hashlib
from pathlib import Path

ZONES = ("willow-stone-county", "glass-orchard", "stone-fen", "blue-light-road", "ash-river", "diamond-breakland", "morrow-market", "root-vale")
ROLES = ("orchard keeper", "route witness", "market worker", "creature steward", "gate archivist", "river courier", "field healer", "background household")
CREATURES = ("moth-deer", "root hound", "glass heron", "basalt hare", "orchard eel", "lantern fox", "cinder elk", "moss bear")

def main():
    ap=argparse.ArgumentParser(); ap.add_argument('--count',type=int,default=2_000_000); ap.add_argument('--output',required=True); args=ap.parse_args()
    out=Path(args.output); out.parent.mkdir(parents=True,exist_ok=True); digest=hashlib.sha256();
    with out.open('w',encoding='utf-8', buffering=1024*1024) as f:
        header='population_id\tcreature_id\tbackground_id\tzone\trole\tprovenance\n'; f.write(header); digest.update(header.encode())
        for i in range(args.count):
            zone=ZONES[i % len(ZONES)]; role=ROLES[(i // len(ZONES)) % len(ROLES)]; creature=CREATURES[(i // 3) % len(CREATURES)]
            line=f'POP-{i+1:07d}\tCREATURE-{i+1:07d}-{creature}\tBACKGROUND-{i+1:07d}\t{zone}\t{role}\tWOWAS_DETERMINISTIC_POPULATION_V1\n'
            f.write(line); digest.update(line.encode())
    summary=out.with_suffix(out.suffix+'.summary')
    summary.write_text(f'schema=wowas.population.v1\nrecords={args.count}\nsha256={digest.hexdigest()}\nstreamed=true\nresident_records=0\n',encoding='utf-8')
    print(summary.read_text(),end='')
if __name__=='__main__': main()
