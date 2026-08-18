#!/usr/bin/env python3
"""Generate the next ten Mersenne record-candidate work units.

This produces targets, not claims of primality. The status remains pending until
sieving, Lucas-Lehmer proof, independent reproduction, and record comparison
have all completed.
"""
from __future__ import annotations

import math
import csv
from pathlib import Path

RECORD_EXPONENT = 136_279_841
TARGET_COUNT = 10
OUT = Path(__file__).with_name("prime_record_targets.tsv")


def is_prime(n: int) -> bool:
    if n < 2:
        return False
    if n % 2 == 0:
        return n == 2
    divisor = 3
    while divisor * divisor <= n:
        if n % divisor == 0:
            return False
        divisor += 2
    return True


def main() -> None:
    exponents = []
    candidate = RECORD_EXPONENT + 1
    while len(exponents) < TARGET_COUNT:
        if is_prime(candidate):
            exponents.append(candidate)
        candidate += 1
    with OUT.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.writer(handle, delimiter="\t")
        writer.writerow([
            "record_id", "family", "exponent", "decimal_digits", "candidate",
            "status", "proof_required", "independent_verification_required",
        ])
        for ordinal, exponent in enumerate(exponents, start=1):
            digits = math.floor(exponent * math.log10(2)) + 1
            writer.writerow([
                f"prime-record-{ordinal:02d}",
                "mersenne",
                exponent,
                digits,
                f"2^{exponent}-1",
                "pending",
                "lucas-lehmer",
                "yes",
            ])
    print(OUT)
    for ordinal, exponent in enumerate(exponents, start=1):
        print(f"prime-record-{ordinal:02d}\t2^{exponent}-1")


if __name__ == "__main__":
    main()
