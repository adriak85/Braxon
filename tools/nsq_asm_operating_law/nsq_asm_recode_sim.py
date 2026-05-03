#!/usr/bin/env python3
import argparse
import hashlib
import json
import time
from datetime import datetime, timezone
from pathlib import Path

SURFACES = ["rust", "c", "python", "shell", "json", "toml", "markdown", "nsq"]

def now():
    return datetime.now(timezone.utc).isoformat()

def h(s: str) -> str:
    return hashlib.sha256(s.encode("utf-8", errors="replace")).hexdigest()

def make_param(surface: str, i: int) -> str:
    return f"{surface}:param:{i}:lever:{i % 1126}:anchor:{i % 2}:weight:{(i * 17) % 997}"

def to_asm(surface: str, param: str, i: int) -> str:
    digest = h(param)[:16]
    return f"ASM_OP surface={surface} idx={i} digest={digest} op=RECODE_TO_NSQ_OPERATING_FORM"

def nsq_pass(line: str, pass_no: int) -> str:
    digest = h(f"pass={pass_no}|{line}")[:24]
    return f"NSQ_PASS_{pass_no} digest={digest} value_preserved=true lineage_preserved=true :: {line}"

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", required=True)
    ap.add_argument("--packets", type=int, default=16384)
    args = ap.parse_args()

    out = Path(args.out)
    out.mkdir(parents=True, exist_ok=True)

    started = time.perf_counter()
    counts = {}
    final_mix = 0
    third_pass_units = 0

    for surface in SURFACES:
        surface_started = time.perf_counter()
        pass3_count = 0
        for i in range(args.packets):
            p = make_param(surface, i)
            asm = to_asm(surface, p, i)
            nsq1 = nsq_pass(asm, 1)
            nsq2 = nsq_pass(nsq1, 2)
            nsq3 = nsq_pass(nsq2, 3)
            final_mix ^= int(h(nsq3)[:16], 16)
            pass3_count += 1
        elapsed = time.perf_counter() - surface_started
        counts[surface] = {
            "packets": args.packets,
            "third_pass_packets": pass3_count,
            "third_pass_packets_per_second": round(pass3_count / elapsed, 3) if elapsed else None,
            "elapsed_seconds": round(elapsed, 6),
        }
        third_pass_units += pass3_count

    elapsed_total = time.perf_counter() - started
    report = {
        "schema": "nsq.asm_operating_law.recode_sim.v1",
        "generated_at": now(),
        "simulation_only": True,
        "BRAXON_live_claim": False,
        "surfaces": SURFACES,
        "packets_per_surface": args.packets,
        "total_third_pass_units": third_pass_units,
        "third_pass_units_per_second": round(third_pass_units / elapsed_total, 3) if elapsed_total else None,
        "elapsed_seconds": round(elapsed_total, 6),
        "final_mix": f"{final_mix:016x}",
        "counts": counts,
        "meaning": [
            "This is a deterministic recode simulation, not proof of live Braxon execution.",
            "Other language surfaces are recoded into ASM operating form, then passed through NSQ three times.",
            "The scored measurement is the third NSQ pass."
        ]
    }

    (out / "nsq_asm_recode_sim_report.json").write_text(
        json.dumps(report, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )

    with (out / "nsq_asm_recode_sim_report.txt").open("w", encoding="utf-8") as f:
        f.write("== NSQ ASM recode simulation ==\n")
        f.write(f"simulation_only={report['simulation_only']}\n")
        f.write(f"BRAXON_live_claim={report['BRAXON_live_claim']}\n")
        f.write(f"packets_per_surface={report['packets_per_surface']}\n")
        f.write(f"total_third_pass_units={report['total_third_pass_units']}\n")
        f.write(f"third_pass_units_per_second={report['third_pass_units_per_second']}\n")
        f.write(f"elapsed_seconds={report['elapsed_seconds']}\n")
        f.write(f"final_mix={report['final_mix']}\n\n")
        for surface, row in counts.items():
            f.write(f"{surface}: third_pass_packets={row['third_pass_packets']} third_pass_packets_per_second={row['third_pass_packets_per_second']} elapsed={row['elapsed_seconds']}\n")

    print(json.dumps(report, indent=2, sort_keys=True))

if __name__ == "__main__":
    main()
