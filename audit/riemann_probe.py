#!/usr/bin/env python3
"""Numerical RH probe; evidence only, not a proof.

The probe verifies a supplied list of known low-lying ordinates and scans a
bounded rectangle for candidate zeros away from Re(s)=1/2. Any candidate needs
independent high-precision certification before it could disprove RH.
"""
from __future__ import annotations

import json
import math
import sys
from pathlib import Path

import mpmath as mp

OUT = Path(__file__).with_name("riemann_probe_result.json")
KNOWN_T = [
    14.134725141734693790457251983562,
    21.022039638771554992628479593896,
    25.010857580145688763213790992562,
    30.424876125859513210311897530584,
    32.935061587739189690662368964074,
    37.586178158825671257217763480705,
    40.918719012147495187398126914633,
    43.327073280914999519496122165406,
    48.005150881167159727942472749427,
    49.773832477672302181916784678563,
]


def main() -> None:
    mp.mp.dps = 80
    residuals = []
    for t in KNOWN_T:
        value = mp.zeta(mp.mpc("0.5", str(t)))
        residuals.append({"t": str(t), "abs_zeta": mp.nstr(abs(value), 30)})

    off_line_candidates = []
    # Bounded exploratory grid only. It is deliberately not called a proof.
    for real_part in [mp.mpf("0.25"), mp.mpf("0.35"), mp.mpf("0.65"), mp.mpf("0.75")]:
        for t in [mp.mpf(14) + mp.mpf(i) / 10 for i in range(0, 371)]:
            magnitude = abs(mp.zeta(mp.mpc(real_part, t)))
            if magnitude < mp.mpf("0.05"):
                off_line_candidates.append({
                    "real_part": str(real_part),
                    "imaginary_part": str(t),
                    "abs_zeta": mp.nstr(magnitude, 30),
                    "status": "unverified_candidate_requires_root_certification",
                })

    result = {
        "schema": "braxon.riemann_probe.v1",
        "precision_decimal_digits": 80,
        "known_critical_line_zeros_sampled": len(KNOWN_T),
        "critical_line_residuals": residuals,
        "off_line_grid_candidates": off_line_candidates,
        "bounded_scan": {"real_parts": ["0.25", "0.35", "0.65", "0.75"], "t_min": "14", "t_max": "51", "step": "0.1"},
        "conclusion": "no_global_conclusion",
        "proof_status": "not_a_proof",
        "disproof_status": "no_certified_counterexample",
    }
    OUT.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({
        "known_zeros_checked": len(KNOWN_T),
        "max_critical_line_residual": max(float(row["abs_zeta"]) for row in residuals),
        "off_line_candidates": len(off_line_candidates),
        "conclusion": result["conclusion"],
    }, indent=2))


if __name__ == "__main__":
    main()
