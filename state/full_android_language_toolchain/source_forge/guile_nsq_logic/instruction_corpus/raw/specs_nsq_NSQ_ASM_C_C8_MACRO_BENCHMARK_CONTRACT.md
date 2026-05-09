# NSQ ASM/C/C8 Macro Benchmark Contract

## Purpose

Compare four surfaces under the same repeatable workload:

1. ASM operating form
2. C plain comparison surface
3. C8 with ASM macro surface
4. NSQ authority surface

This benchmark is a local deterministic proof harness. It is not a claim that Braxon is fully live.

## Rounds

The benchmark runs three rounds.

- Round 1 discovers macro candidates.
- Round 2 refines macro candidates.
- Round 3 is the scored production-like run.

Only round 3 is treated as the scored run.

## Runtime Instrumentation Rule

Macro tracking is allowed only in discovery/refinement rounds.

- Round 1 macro tracking: enabled.
- Round 2 macro tracking: enabled.
- Round 3 macro tracking: disabled.

The scored production-like round must not insert tracers, collect macro-use counts, run discovery instrumentation, or behave like a profiling/debug build.

Promoted macros may be used in round 3, but round 3 must behave like a production release path.

## Timeout Rule

Every compared surface has a per-surface timeout.

Default timeout: 10 seconds per surface per round.

If a surface times out, the result is partial but valid. The report must record:

- processed records
- total available records
- timeout status
- elapsed seconds
- work units completed before timeout

## Macro Rule

The run starts with the best seed macros currently known for NSQ/ASM operation:

- source hash
- lineage stamp
- authority check
- ASM recode operation
- C8 pack
- moral invariant guard
- metadata impact
- reverse dependency
- round trip validation
- binary boundary
- NSQ pass
- generated output is not source authority
- read everything
- check surroundings
- identify all aspects of all perceivable
- understand action
- make best decision
- act with the best of being

During rounds 1 and 2, the harness tracks macro use and searches for repeated normalized phrases.

Any macro candidate used more than three times is promoted into the next round.

Round 3 runs with promoted macros but without tracking or tracers.

## Honesty Rule

ASM is the operating law for NSQ execution, recode, and optimization.

C is a comparison surface.

C8 with ASM is a compressed macro comparison surface.

NSQ is the authority surface.

Binary translation remains downstream Braxon output.

No benchmark result overrides NSQ source truth or the protected moral invariant.
