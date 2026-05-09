# NSQ Documentation Index

## ASM Operating Law

- `docs/nsq/NSQ_ASM_OPERATING_LAW.md`
- `specs/nsq/NSQ_ASM_OPERATING_LAW_SPEC.md`
- `apps/nsq/asm_operating_law.nsq`
- `config/nsq/asm_operating_law.nsq`
- `state/nsq/asm_operating_law/current.json`
- `state/nsq/translation_pipeline/asm_to_binary_boundary.json`

ASM is the operating, recode, and optimization form for NSQ.

Braxon binary translation is downstream output, not NSQ source truth.

## Moral Invariant Guard

- `specs/Braxon/BRAXON_PERSONAL_MORAL_INVARIANT.md`
- `apps/nsq/moral_invariant_guard.nsq`
- `config/nsq/moral_invariant_guard.nsq`
- `state/braxon/moral_invariant/current.json`
- `bin/Braxon-moral-invariant-guard`
- `bin/Braxon-moral-invariant-guard-loop`

The moral invariant is not a goal file.

Goals may be rewritten.

The moral invariant may not be morphed, inverted, or overridden by metadata, ASM, binary translation, generated files, or future tools.

## Recode Simulation

- `tools/nsq_asm_operating_law/nsq_asm_recode_sim.py`
- `bin/nsq-asm-recode-sim`

The recode simulation sends other language surfaces into ASM operating form and then through three NSQ passes. The third pass is measured.

This is simulation proof only. It is not a live Braxon completion claim.

## ASM/C/C8 Macro Benchmark

- `apps/nsq/asm_c_c8_macro_benchmark.nsq`
- `config/nsq/asm_c_c8_macro_benchmark.nsq`
- `specs/nsq/NSQ_ASM_C_C8_MACRO_BENCHMARK_CONTRACT.md`
- `tools/nsq_asm_operating_law/nsq_asm_c_c8_macro_bench.py`
- `bin/nsq-asm-c-c8-macro-bench`
- `state/nsq/asm_c_c8_macro_benchmark/current.json`

This benchmark compares ASM operating form, C, C8+ASM, and NSQ across three rounds.

Rounds 1 and 2 discover macros. Any macro used more than three times is promoted. Round 3 is the scored round.

## Production Benchmark Runtime Rule

The ASM/C/C8+ASM/NSQ benchmark uses instrumentation only in rounds 1 and 2.

Round 3 is the scored production-like runtime path:

- macro tracking disabled
- tracer collection disabled
- discovery instrumentation disabled
- promoted macros available
- 10-second timeout per surface by default

This prevents profiling overhead from being mistaken for production runtime cost.
