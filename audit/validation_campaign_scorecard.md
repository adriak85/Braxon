# Reconstruction Validation Campaign Scorecard

This scorecard is generated from `validation_campaign_matrix.json`. It classifies evidence conservatively: a gate is not marked proven or equivalent without executable support, and physical/device/model/migration gates remain blocked when they were not actually run.

| Metric | Value |
|---|---:|
| Campaign gates | 45 |
| Proven | 1 |
| Measured | 24 |
| Equivalent | 0 |
| Blocked | 12 |
| Theoretical | 8 |
| Executed or evidenced gate coverage | 55.56% |
| Physical validation | BLOCKED |
| Guile migration | BLOCKED |
| Zig migration | BLOCKED |
| Android 16 acceptance | BLOCKED |
| WOWAS universal/global compliance | BLOCKED |
| Real training advantage | NOT ESTABLISHED |

The current repository evidence establishes a coherent native execution mechanism and several measured scaling properties. It does not establish whole-system semantic equivalence, real-device acceptance, or real-model training acceleration.

## Gate Matrix

| ID | Gate | Status | Evidence | Scope and limitation |
|---:|---|---|---|---|
| 1 | Baseline correctness / equivalence | MEASURED | crates/braxon-core/src/one_shot_objective_benchmark.rs; crates/braxon-core/src/performance_surface_benchmark.rs | Deterministic fixture equivalence; not whole-system semantic equivalence. |
| 2 | Logical-scale benchmark | MEASURED | crates/braxon-core/src/dynamic_parameter_runtime.rs | 2,000,000 logical parameters with selective examination; not 10^8. |
| 3 | Sparsity surface | MEASURED | crates/braxon-core/src/performance_surface_benchmark.rs; audit/performance_surface_results.json | 0,10,25,50,75,100 percent sweep. |
| 4 | Prediction-accuracy surface | MEASURED | crates/braxon-core/src/performance_surface_benchmark.rs; audit/performance_surface_results.json | 0,25,50,75,90,100 percent sweep with parameter-level hits and corrections. |
| 5 | Prediction-adversarial benchmark | MEASURED | crates/braxon-core/src/adversarial_integrated_benchmark.rs; audit/adversarial_integrated_benchmark_results.json | Hostile predictions are corrected and cannot become authoritative state. |
| 6 | Dependency-topology benchmark | THEORETICAL | crates/nsq-core/src/initiative_cluster.rs | Basic dependency selection exists; full topology family sweep not executed. |
| 7 | Dynamic-parameter benchmark | MEASURED | crates/nsq-core/src/dynamic_parameter.rs; crates/braxon-core/src/dynamic_parameter_runtime.rs | Canonicalization, prediction, observation, revision, provenance, and open-ended maps tested; deletion/domain mutation matrix incomplete. |
| 8 | Initiative-cluster benchmark | MEASURED | crates/nsq-core/src/initiative_cluster.rs; crates/braxon-core/src/initiative_cluster_runtime.rs | Selective recalculation and linked-cluster composability tested; full 10,000 topology campaign not executed. |
| 9 | Reflexor latency decomposition | THEORETICAL | crates/braxon-core/src/kinetic_reflexor.rs | Lifecycle exists; per-stage timing matrix not yet captured. |
| 10 | Reactive versus predictive benchmark | MEASURED | crates/braxon-core/src/dynamic_parameter_runtime.rs | Synthetic three-path microbenchmark; not real training. |
| 11 | Predictive break-even benchmark | THEORETICAL | crates/braxon-core/src/performance_surface_benchmark.rs | Surface records overhead but does not establish a general crossover. |
| 12 | JIT activation benchmark | MEASURED | crates/braxon-core/src/riemann_semantic_reflexor.rs | Activation, retry, unauthorized, and bounded failure cases tested. |
| 13 | Piston-memory scaling benchmark | MEASURED | crates/braxon-core/src/piston_memory.rs; crates/braxon-core/src/ghost_memory.rs | Bounded aperture and pressure behavior tested; no GB-scale physical run. |
| 14 | GhostMemoryBus benchmark | MEASURED | crates/braxon-core/src/ghost_memory.rs | Acquire, release, rotation, stale generation, and same-space protection covered by tests. |
| 15 | Storage versus resident-state benchmark | MEASURED | crates/braxon-core/src/piston_memory.rs; crates/braxon-core/src/ghost_memory.rs | Logical/released/aperture state measured in native fixtures; not physical RAM equivalence. |
| 16 | One-shot orchestration scaling | MEASURED | crates/braxon-core/src/one_shot_objective_benchmark.rs; audit/one_shot_scaling_results.json | 5,10,100,1,000,10,000 steps at one external interaction. |
| 17 | Persistent-learning benchmark | MEASURED | crates/braxon-core/src/riemann_semantic_reflexor.rs | Learning records, priorities, corrections, visited signatures, and terminal states tested. |
| 18 | Loop-detection benchmark | MEASURED | crates/braxon-core/src/riemann_semantic_reflexor.rs | Repeated-attempt stopping and deterministic terminal control tested. |
| 19 | Proof-boundary benchmark | PROVEN | crates/braxon-core/src/riemann_semantic_reflexor.rs | Prediction, numerical evidence, and single-source evidence cannot promote to independent proof state. |
| 20 | Riemann semantic-reflex benchmark | MEASURED | crates/braxon-core/src/riemann_semantic_reflexor.rs; audit/riemann_probe.py | Hypothesis routing, activation, observation, correction, loop control, and proof-state classification tested; RH unresolved. |
| 21 | Tokenizer benchmark | THEORETICAL | — | No native tokenizer throughput/equivalence campaign captured. |
| 22 | Model-intent extraction benchmark | MEASURED | crates/nsq-core/src/dynamic_parameter.rs; crates/braxon-core/src/dynamic_parameter_runtime.rs | Candidate extraction and canonicalization rejection paths tested. |
| 23 | Model-inference integration benchmark | BLOCKED | — | No actual model inference run in this campaign. |
| 24 | Training benchmark | BLOCKED | crates/braxon-core/src/dynamic_parameter_runtime.rs | Synthetic microbenchmark exists; same-dataset model-training comparison is not established. |
| 25 | Training sparsity benchmark | BLOCKED | — | No real optimizer/dataset training sweep. |
| 26 | Training prediction benchmark | BLOCKED | — | Execution prediction surface is not training evidence. |
| 27 | Model-scale benchmark | MEASURED | crates/braxon-core/src/dynamic_parameter_runtime.rs; crates/braxon-core/src/integrated_objective_benchmark.rs | Logical scale and bounded active window measured in native fixtures. |
| 28 | Long-context benchmark | THEORETICAL | crates/braxon-core/src/wowas_seeded.rs | Compact reconstruction exists; no conventional-context equivalence sweep. |
| 29 | Multi-objective benchmark | THEORETICAL | crates/braxon-core/src/initiative_cluster_runtime.rs | Single-objective trajectory only. |
| 30 | Concurrent-agent benchmark | THEORETICAL | crates/braxon-core/src/kinetic_reflexor.rs | Isolation primitives exist; concurrent contention campaign not executed. |
| 31 | Fault-injection benchmark | MEASURED | crates/braxon-core/src/riemann_semantic_reflexor.rs; crates/braxon-core/src/semantic_link.rs; crates/braxon-core/src/kinetic_reflexor.rs | Malformed, unauthorized, stale, and unresolved paths fail closed; full corruption matrix incomplete. |
| 32 | Crash/recovery benchmark | THEORETICAL | crates/nsq-core/src/initiative_cluster.rs | Deterministic reconstruction is tested; process-kill recovery points are not. |
| 33 | Determinism benchmark | MEASURED | crates/braxon-core/src/initiative_cluster_runtime.rs; crates/braxon-core/src/one_shot_objective_benchmark.rs | Deterministic replay and final-result equivalence tested in fixtures. |
| 34 | Reproducibility benchmark | MEASURED | audit/run_clean_room_validation.sh; audit/clean_room_validation_result.json | Fresh reconstruction clone under Rust 1.96.0 passed 72 Braxon-core library tests; full-system package and all-workspace acceptance remain outside this gate. |
| 35 | Memory-pressure benchmark | MEASURED | crates/braxon-core/src/piston_memory.rs; crates/braxon-core/src/ghost_memory.rs | Bounded pressure and fail-closed aperture tests; not device telemetry. |
| 36 | Thermal/resource benchmark | BLOCKED | — | Requires physical Android device telemetry. |
| 37 | Android 16 acceptance | BLOCKED | — | No physical non-rooted Moto G acceptance run. |
| 38 | Guile semantic-equivalence benchmark | BLOCKED | — | Catalog/intent reconstruction is not representative semantic equivalence. |
| 39 | Zig semantic-equivalence benchmark | BLOCKED | — | Catalog/intent reconstruction is not representative semantic equivalence. |
| 40 | Companion-tool equivalence benchmark | BLOCKED | audit/expanded/FINAL_EXPANDED_AUDIT.md | Tool intent catalog is not a complete reference-equivalence suite. |
| 41 | Universal architecture audit | MEASURED | audit/expanded/FINAL_EXPANDED_AUDIT.md; audit/expanded/function_inventory.tsv | Repository audit evidence exists; universal absence of every bypass is not a theorem. |
| 42 | Global WOWAS audit | BLOCKED | audit/compile_wowas_scene_payload.py | Payload/compiler controls exist; universal subsystem compliance is not completed. |
| 43 | Global semantic-authority audit | MEASURED | crates/nsq-core/src/dynamic_parameter.rs; crates/braxon-core/src/riemann_semantic_reflexor.rs | Native authority boundaries are tested in covered paths; universal audit remains incomplete. |
| 44 | Universal migration audit | BLOCKED | audit/expanded/overlap_classification.txt | Historical disposition inventory exists; every migration gate is not proven complete. |
| 45 | Full-system clean-room build | BLOCKED | — | Fresh clone/build/package gate not executed in this campaign. |

## Reproducibility

| Field | Value |
|---|---|
| Branch | reconstruction |
| Commit | 07f83925be5e8e1287de3a6eb8f6c7c17341026a |
| Origin commit | 07f83925be5e8e1287de3a6eb8f6c7c17341026a |
