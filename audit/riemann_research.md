# Riemann Hypothesis investigation: source findings

Clay Mathematics Institute source: https://www.claymath.org/millennium/riemann-hypothesis/

Clay currently labels the Riemann Hypothesis **Unsolved**. Its statement is that all non-obvious (nontrivial) zeros of the Riemann zeta function have real part 1/2. Clay says the statement has been checked for the first 10,000,000,000,000 solutions, but explicitly distinguishes this computation from a proof for every nontrivial zero.

The present task therefore has three separate outcomes:

1. A counterexample search can disprove RH only if a rigorously certified zero with real part different from 1/2 is found.
2. Numerical zero verification can provide evidence and test implementations, but cannot prove RH globally.
3. A proof attempt must establish a theorem covering all nontrivial zeros or an accepted equivalent criterion, with every analytic continuation, limiting argument, and inequality justified.

Captured 2026-08-18.

## Bounded probe result

The saved `riemann_probe.py` was run with 80-decimal-digit arithmetic. It evaluated ten standard low-lying zeros on Re(s)=1/2; the largest residual in this run was approximately 7.25e-15 because the supplied ordinates were finite decimal approximations rather than arbitrary-precision root certificates. A bounded exploratory grid at Re(s) in {0.25, 0.35, 0.65, 0.75} and 14 <= Im(s) <= 51 with step 0.1 produced no candidate with |zeta(s)| < 0.05.

This is consistent with RH and is not a proof. The grid does not certify the absence of zeros between samples, and the low-zero residuals do not certify exact roots. The run found no certified counterexample and established no global theorem.
