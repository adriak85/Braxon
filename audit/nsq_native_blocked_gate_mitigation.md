# NSQ-Native Mitigation Plan for the 13 Blocked Gates

## Executive conclusion

The blocked gates are not solved by adding adapters around PyTorch, llama.cpp, Android libraries, Guile, or Zig. Those systems may be used as **behavioral oracles**, corpus generators, and independent comparison implementations, but they must not become the authoritative runtime. The required architecture is:

> reference intent extraction → NSQ semantic normalization → native NSQ capability contract → NSQ execution → observation → oracle comparison → correction or certified equivalence

The current reconstruction branch already has the dynamic-parameter, initiative-cluster, Reflexor, activation, bounded-aperture, and fail-closed authority boundaries needed for this direction. It does not yet contain a native tensor/model execution substrate, a native optimizer/training substrate, real reference-equivalence corpora for Guile/Zig/tool companions, a physical Android acceptance harness, or a clean-room release gate. Those are the actual blockers.

The authoritative campaign matrix is `audit/validation_campaign_matrix.json`. This report expands the 13 entries classified `BLOCKED` there and provides a staged mitigation plan.

## Authority rule

External systems are permitted only in one of four non-authoritative roles: a reference oracle, a corpus or model-artifact producer, an independent verifier, or a physical deployment harness. They must not write canonical NSQ parameters, execution state, proof state, or WOWAS authority state.

PyTorch explicitly warns that complete reproducibility is not guaranteed across releases, commits, platforms, or CPU/GPU executions, so a training oracle must record the exact release, device, seeds, deterministic-algorithm settings, and data-loader configuration rather than assuming bitwise identity [1]. llama.cpp demonstrates a practical C/C++ inference surface and supports GGUF models, quantization, CPU/GPU backends, and Android builds, but those are reference capabilities to extract—not a reason to make llama.cpp the Braxon runtime [2]. Android’s NDK provides the native-code packaging and ABI mechanism needed for a physical device harness, but the NSQ/Blaixe implementation must remain the authoritative execution path [3].

## Blocked-gate inventory and mitigation

| Gate | Current reason for `BLOCKED` | Intent to extract | NSQ-native rebuild | Required infrastructure | Unblock acceptance |
|---:|---|---|---|---|---|
| 23. Model-inference integration | No actual model inference run exists. | Load model artifacts, tokenize input, execute tensor operations, maintain KV/context state, emit output and telemetry. | Native model manifest, tensor-address map, tokenizer contract, NSQ tensor operation set, KV-cache pages, deterministic scheduler, candidate-output emission into `CandidateIntent`. | A pinned open model artifact with license, tokenizer vocabulary, test prompts, expected outputs, tensor metadata, and an independent oracle runner. | NSQ-native inference runs the same fixed prompts and artifact, emits deterministic tokens or an explicitly bounded tolerance, produces a `DynamicPipelineReceipt`, and matches the oracle on token/hidden-state/output criteria without the oracle writing NSQ state. |
| 24. Training benchmark | Existing benchmark is synthetic and not a same-dataset training comparison. | Stream examples, compute loss and gradients, update parameters, checkpoint, resume, and measure quality and resources. | Native dataset stream, tensor/gradient representation, loss expressions, optimizer initiative clusters, checkpoint/reconstruction contract, persistent training receipt. | Fixed dataset split, model initialization, optimizer, precision, batch schedule, seed policy, evaluation set, quality metric, hardware telemetry, and baseline runner. | Dense, reactive, predictive, and persistent-predictive paths train the same model from the same initialization and data under the same stopping rule; final quality and loss meet predefined tolerances; every update is attributable to an NSQ generation and receipt. |
| 25. Training sparsity | No real optimizer or dataset sweep. | Vary the fraction of parameters or gradients updated while retaining objective semantics. | Gradient sparsity map, dependency-selective optimizer cluster, sparse update receipt, dense fallback, and correctness guard against dropped required gradients. | A dataset/model whose update sparsity can be measured or controlled, plus dense reference training. | Sweep 0–100% update sparsity; report loss, quality, examined parameters, materialized gradients, bytes, time, and rejected updates; native and dense final results are equivalent within the declared tolerance. |
| 26. Training prediction | Execution-prediction surface is not training evidence. | Predict next active gradients/parameters or optimizer work, stage them, validate them, and correct misses. | Predictive optimizer contract with non-authoritative staged updates, activation receipts, miss/correction accounting, and dense/reactive fallback. | A predictor independent from the authoritative optimizer, deterministic replay, and telemetry for false positives, false negatives, avoided work, and correction cost. | At 0% prediction accuracy, training remains correct; at higher accuracy, net benefit is measured rather than assumed; prediction can never mutate canonical weights before observation and reconciliation. |
| 34. Reproducibility benchmark | Fresh clean-room build was not executed. | Rebuild the same source and evidence from a clean checkout without developer-machine artifacts. | NSQ-native manifest of toolchain, source tree, generated assets, model hashes, test commands, and result hashes. | Fresh clone, pinned Rust/toolchain/dependency lock, isolated cache policy, no untracked artifacts, and a second execution environment. | Fresh clone → build → test → benchmark → package succeeds; source/model/evidence hashes and deterministic decisions match; wall-clock values may differ but are explicitly excluded from identity. |
| 36. Thermal/resource benchmark | No physical Android telemetry exists. | Execute sustained native workload and measure CPU, memory, bandwidth, energy, temperature, throttling, and throughput. | Native telemetry receipt, resource-budget parameter set, bounded-window scheduler, pressure response, and fail-closed degradation state. | Physical target device, USB/ADB access, signed test APK or native package, thermal and battery telemetry, fixed workload, ambient conditions, and repeated runs. | Sustained run reports temperature, power, CPU frequency, resident memory, active aperture, throughput, and failures; no claim is made from sandbox measurements alone. |
| 37. Android 16 acceptance | No physical non-rooted Moto G acceptance run. | Package and launch the native stack on the specified non-rooted Android target with the intended native surface. | Blaixe/NSQ native library or executable boundary, ABI contract, storage/address lease contract, native UI/device event surface, and recovery path. | Exact Moto G model/SoC, Android 16 device, non-rooted developer mode, build toolchain, signing key, install path, test application, and acceptance script. | On the physical device, the stack starts without root, loads a known fixture, executes a semantic link, activates/releases a bounded window, handles a failed capability, and records a reproducible receipt. This gate cannot be marked passed from an emulator or desktop run. |
| 38. Guile semantic equivalence | Intent/catalog work is not representative semantic equivalence. | Preserve observable Guile behavior: evaluation, mutation, errors, continuations or control semantics claimed by the project, and I/O boundaries. | NSQ semantic contracts and conformance corpus; each supported form becomes a native intent/relationship, not a Guile interpreter hidden behind NSQ. | Version-pinned Guile reference, curated corpus, expected values/errors/effects, unsupported-feature manifest, and differential runner. | Every claimed supported corpus case produces equivalent result, error class, side-effect trace, and determinism signature; unsupported cases are explicitly blocked rather than silently accepted. |
| 39. Zig semantic equivalence | Intent/catalog work is not compiler/language semantic equivalence. | Preserve the claimed Zig behavior: parsing, type checking, compile-time evaluation, code-generation obligations, and diagnostics within scope. | Native NSQ syntax/semantic tree, type/dependency expressions, diagnostic records, and bounded code-generation intent contracts. | Version-pinned Zig reference, conformance corpus, target triple, compiler flags, expected diagnostics/artifacts, and differential comparison tool. | A declared subset passes reference-vs-NSQ result and diagnostic equivalence; any unimplemented compiler feature has a recorded disposition and cannot be counted as migrated. |
| 40. Companion-tool equivalence | Tool intent catalog is not a reference-equivalence suite. | Extract each tool’s observable contract, including inputs, outputs, errors, ordering, resource ownership, and side effects. | One NSQ-native capability contract per supported tool family, with shared authority and no wrapper delegation. | Pinned reference versions, golden corpus, CLI/API fixtures, output normalizers, error corpus, and per-tool coverage map. | Each claimed tool family has executable equivalence cases and a disposition for every uncovered operation; catalog presence alone is insufficient. |
| 42. Global WOWAS audit | Payload compiler and controls exist, but universal subsystem compliance is incomplete. | Enforce identity, metadata, provenance, authorization, realization, release, and audit across every subsystem. | A single WOWAS boundary contract attached to dynamic parameters, initiative clusters, capability activation, receipts, model artifacts, and generated outputs; prose generation remains disabled where required. | Cross-subsystem manifest, record_id coverage, provenance graph, rejected-artifact ledger, payload compiler, and audit that fails on bypass paths. | Every participating subsystem emits the required record_id/provenance/authorization fields; rejected and deprecated material remains explicitly classified; a cross-subsystem audit has zero unexplained bypasses. |
| 44. Universal migration audit | Historical disposition inventory exists, but every migration gate is not proven complete. | Account for every historical component as implemented, translated, deprecated with replacement, or blocked with evidence. | NSQ capability registry with source-intent hash, canonical replacement, equivalence status, and blocker reference. | Complete-tree inventory including hidden/nonstandard/generated files, branch/commit provenance, duplicate comparison, and disposition database. | Every intended historical capability has exactly one canonical status and owner; no source disappears through omission; duplicates and deprecated artifacts point to a replacement or explicit rejection. |
| 45. Full-system clean-room build | Fresh clone/build/test/benchmark/package was not executed. | Prove the final repository is self-contained and does not depend on developer-machine state. | Reproducible NSQ build manifest, package manifest, generated-artifact policy, and evidence bundle. | Fresh clone, network/cache policy, locked dependencies, model fixture acquisition or included hashes, package builder, and isolated runner. | From a fresh clone, the approved commands produce the same test/evidence classifications and package; all required artifacts are either in the repository or acquired through declared, hashed inputs. |

## Native model-inference workstream

The first implementation workstream should not start with an API client. It should start with the **native model artifact contract**. The contract needs a `record_id`, artifact hash, architecture identifier, tensor manifest, dtype, shape, layout, quantization scheme, tokenizer identity, vocabulary hash, context limits, and provenance. The manifest becomes an NSQ canonical object. A tensor is represented as a logically addressable region with an active aperture lease; loading a page is an activation event, not an implicit resident-state assumption.

The next layer is the **NSQ tensor and operator substrate**. The minimum useful inference subset is embedding lookup, normalization, matrix multiplication, elementwise operations, reduction, positional encoding, attention score/value operations, masking, sampling, and cache append/read. Each operator must have a deterministic input/output contract, dependency set, byte accounting, generation, and release behavior. The implementation must support a dense fallback because sparse or predictive execution cannot be allowed to change correctness.

The tokenizer must be rebuilt as a native semantic contract as well. It needs vocabulary lookup, normalization policy, segmentation, token-to-byte offsets, special-token policy, and deterministic round-trip tests. A reference tokenizer may generate golden vectors, but the NSQ tokenizer owns canonical token records in the native path.

The **KV/context subsystem** must be treated as a first-class dynamic parameter and memory problem. Each cache region requires a record identity, layer/head/position coordinates, dtype and byte length, generation, lease state, and eviction/reconstruction contract. The benchmark must separately report logical context entries, active cache bytes, released bytes, and peak resident bytes. It must not report zero physical usage merely because a logical counter is zero.

Finally, the model output must enter the existing intent pipeline. The model may propose text or structured fields. `CandidateIntent::extract` remains non-authoritative. `DynamicParameterSet::canonicalize` validates type, provenance, confidence, dependencies, and constraints. Initiative clusters then express the executable relationships, and the Reflexor controls activation, observation, correction, and release.

## Native training workstream

Training requires more than routing inference through a different loop. The native substrate needs an **NSQ training state contract** containing model generation, optimizer generation, dataset shard/record identity, batch identity, loss expression, gradient regions, optimizer state regions, learning-rate schedule, precision, clipping policy, and checkpoint watermark. The optimizer state must be first-class because momentum, variance, and mixed-precision loss scaling can be comparable to or larger than the parameters being updated.

The fair comparison must use one pinned model initialization, dataset bytes and split, tokenizer, batch order, objective, optimizer, precision, seed, stopping rule, evaluation metric, and hardware class. The baseline may be PyTorch or another reference implementation, but it is an oracle and comparator. The native path must perform the authoritative parameter and optimizer updates in NSQ.

The first real training target should be deliberately small and deterministic: a tiny transformer or MLP with a public dataset fixture, a fixed tokenizer, and a fixed number of steps. The objective is not to prove large-model acceleration immediately. It is to prove that dense, reactive, predictive, and persistent-predictive paths converge to equivalent loss/quality within a declared numerical tolerance, while producing complete telemetry for examined parameters, active bytes, gradient bytes, optimizer bytes, corrections, and wall time.

Prediction must be tested as an optimization only. At 0% accuracy, the native path must fall back to authoritative observed updates and remain correct. At intermediate accuracy, false positives and false negatives must be measured separately. A prediction must never write canonical weights before observation and reconciliation. A prediction that is not observed is a candidate staging record, not a parameter generation.

## Required implementation layers

| Layer | Native NSQ deliverable | Gate dependencies |
|---|---|---|
| Artifact identity | Model/tokenizer/dataset manifest with hashes and provenance | 23–26, 34, 37, 45 |
| Tokenization | Native vocabulary and segmentation contract | 21, 23, 24, 28 |
| Tensor substrate | Addressable tensor regions, dtype/shape/layout, bounded activation | 23–27, 35–37 |
| Operator substrate | Deterministic tensor operations and dependency graph | 23–26, 38–40 |
| KV/context | Dynamic cache regions, release/reconstruct, pressure behavior | 23, 28, 35–37 |
| Optimizer substrate | Loss, gradient, optimizer state, checkpoint, resume | 24–26 |
| Predictive training | Non-authoritative staging, correction, fallback | 26 |
| Oracle harness | Differential output/loss/gradient/diagnostic comparison | 23–26, 38–40 |
| Device harness | Native package, telemetry, acceptance script | 36–37 |
| Clean-room harness | Fresh clone, locked build, evidence package | 34, 45 |
| WOWAS authority | Cross-subsystem provenance and rejection audit | 42, 44 |

## Recommended order of execution

The correct order is dependency-first, not headline-first. First build the artifact, tokenizer, tensor, operator, and KV contracts in NSQ. Then establish a tiny deterministic inference corpus and an oracle differential runner. Once inference is equivalent, add native loss/gradient/optimizer state and a tiny training corpus. Only after dense training equivalence passes should sparse and predictive training be measured.

In parallel, build the clean-room manifest and source-disposition audit. The Guile, Zig, and companion-tool gates should each receive a declared semantic subset and differential corpus; they should not be marked migrated because their names appear in a catalog. Physical Android and thermal gates must wait until the native stack is packaged and the device harness can collect real telemetry.

## Exact unblock criteria for the two central blocked claims

### Model-inference integration is unblocked only when all of the following are true

1. A pinned model and tokenizer artifact have hashes, licenses, record IDs, and provenance.
2. NSQ owns artifact loading, tensor addresses, operator execution, KV/context state, output sampling, and release/reconstruction.
3. A fixed prompt corpus runs through both the reference oracle and NSQ-native execution.
4. Outputs, token IDs, or declared numerical observables match under a documented tolerance.
5. The model cannot directly mutate canonical NSQ parameters; its output enters candidate intent and canonicalization.
6. Activation, correction, release, and failure paths emit receipts and are replayable.
7. The complete inference run succeeds from a clean checkout without hidden runtime services.

### Training comparison is unblocked only when all of the following are true

1. Dense NSQ training reaches reference-equivalent loss and final quality on a fixed dataset/model fixture.
2. The same initialization, optimizer, precision, schedule, batch order, seed policy, stopping criteria, and hardware class are recorded.
3. Gradients, parameter generations, optimizer state, checkpoints, and resume behavior are comparable.
4. Reactive, predictive, and persistent-predictive paths are compared against dense training on the same objective.
5. Prediction accuracy is swept from 0% to 100%; 0% must remain correct.
6. The report includes tokens/examples per second, wall time, convergence steps, loss, quality, active/resident memory, bandwidth, energy where available, parameters examined, materialized bytes, prediction overhead, correction overhead, and avoided work.
7. Any speedup is reported only after quality equivalence and deterministic replay pass. A synthetic sparse microbenchmark cannot substitute for this gate.

## Current blocker classification

The central blockers are now better understood as missing **native capability surfaces and acceptance infrastructure**, not missing orchestration. The repository has enough Reflexor and NSQ authority machinery to host the next layers, but it does not yet have the native tensor/operator/optimizer implementations or the external fixtures required to validate them.

That means the immediate next build target should be a small, fully native NSQ inference fixture followed by a tiny native training fixture. The reference implementation should be used to generate expected behavior and detect divergence, while the canonical execution, state, and receipts remain NSQ-owned.

## References

[1]: https://docs.pytorch.org/docs/2.13/notes/randomness.html "PyTorch reproducibility and determinism guidance"

[2]: https://github.com/ggml-org/llama.cpp "llama.cpp reference inference implementation and supported model/runtime surfaces"

[3]: https://developer.android.com/ndk "Android NDK native-code and device deployment documentation"
