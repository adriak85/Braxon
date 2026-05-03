# Repo Reality Benchmark Doctrine

## Core question
Which base handles a realistic software-repository intelligence task better as
human-readable, structurally correct, deterministic output?

## Real-world scenario
A damaged but still-usable local repo contains:
- multi-file source
- headers
- configs
- migrations
- docs
- logs
- diffs
- partial corruption

The system must recover enough structure to brief an operator honestly.

## Shared obligations
Each system must emit:
- symbol table
- include/import graph
- call graph edges
- externally exposed entrypoints
- suspicious findings
- human-readable briefing
- deterministic replay hash

## Primary scores
- obligation_recall
- obligation_precision
- readable_information_per_second
- structural_units_per_second

## Supporting scores
- determinism
- corruption_survival_rate
- null_output_rate
- failure_discipline

## Fairness
- same corpus
- same obligations
- same score function
- native strengths allowed
- no sabotage
- no compensatory handicapping
- no fake semantic inflation
