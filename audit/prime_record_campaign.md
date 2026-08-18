# Ten-record prime campaign definition

The authoritative GIMPS record page identifies M136279841 = 2^136,279,841 - 1 as the current largest known prime with 41,024,320 decimal digits. The GIMPS PrimeNet activity page, read on 2026-08-18, reports a live distributed search with 3,202,130 registered computers, 102,183 work units, and multi-thousand potential TFLOP/s aggregate capacity. It exposes separate status columns for composite, verified, LL/PRP, and factoring work, showing that candidate testing and definitive verification are distinct stages.

For this project, “the next 10 records” is defined as ten sequential record-candidate work units above exponent 136,279,841 in the Mersenne family, each requiring:

1. a prime exponent p > 136,279,841;
2. a candidate M_p = 2^p - 1;
3. deterministic pre-sieving / probable-prime screening;
4. a definitive Lucas–Lehmer proof for a surviving candidate;
5. independent reproduction using a separate implementation or hardware path; and
6. comparison against the authoritative largest-known-prime record before any record claim.

The system may generate and manage the ten work units locally, but discovering ten new record-setting Mersenne primes is not honestly claimable from a bounded sandbox run. The campaign therefore records each target as pending, composite, probable-prime, proven-prime, independently-verified, or record-accepted. No status may be promoted by labels alone.

Sources:
- https://www.mersenne.org/primes/?press=M136279841
- https://www.mersenne.org/primenet/
