#!/usr/bin/env python3
"""Verify target integrity without making unsupported Mersenne-prime claims."""
from __future__ import annotations

import csv
import math
from pathlib import Path

ROOT = Path(__file__).parent
TARGETS = ROOT / "prime_record_targets.tsv"
RECORD_EXPONENT = 136_279_841


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
    with TARGETS.open(encoding="utf-8", newline="") as handle:
        rows = list(csv.DictReader(handle, delimiter="\t"))
    assert len(rows) == 10, len(rows)
    exponents = [int(row["exponent"]) for row in rows]
    assert exponents == sorted(exponents)
    assert all(exponent > RECORD_EXPONENT for exponent in exponents)
    assert len(set(exponents)) == len(exponents)
    for row in rows:
        exponent = int(row["exponent"])
        digits = math.floor(exponent * math.log10(2)) + 1
        assert is_prime(exponent), exponent
        assert int(row["decimal_digits"]) == digits
        assert row["candidate"] == f"2^{exponent}-1"
        assert row["status"] == "pending"
        assert row["proof_required"] == "lucas-lehmer"
        assert row["independent_verification_required"] == "yes"
    print(f"verified_target_rows={len(rows)}")
    print(f"verified_prime_exponents={len(exponents)}")
    print("mersenne_primality_status=pending_external_ll_proof")
    print("record_acceptance_status=pending_authoritative_comparison")


if __name__ == "__main__":
    main()
