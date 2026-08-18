# Prime-record research

Source: https://www.mersenne.org/primes/?press=M136279841

GIMPS identifies 2^136,279,841 - 1 (M136279841) as the largest known prime, with 41,024,320 decimal digits. The announcement says it was found on October 12 and that definitive Lucas–Lehmer tests were run by different programs on different CPU/GPU hardware. It names Prime95, PRPLL/GpuOwl, Mlucas, and CUDALucas as independent verification paths, with Mlucas confirmation on October 19. The record standard therefore requires both a candidate larger than the current record and definitive, independently reproduced primality verification, not merely a probable-prime result.

Key architectural implication: a native pipeline can generate and verify structured candidates, but record status is an external comparative claim that must be checked against the authoritative record database and independently verified across separate implementations/hardware.

Captured 2026-08-18.
