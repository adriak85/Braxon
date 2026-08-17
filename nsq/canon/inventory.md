# Repository intent inventory

Source baseline: `ced6af253888bf194375692de86d0678dc70d847`.

This inventory deliberately classifies directories by intent instead of pretending path names are architecture. The full repository tree was traversed through the Git tree surfaces, with the NSQ, benchmark, configuration, source, test, and tooling branches inspected for their semantic role.

## Root-level surfaces

| Surface | Canonical role | Treatment |
|---|---|---|
| `apps` | application/interface behavior | extract contracts into `80_interfaces` |
| `artifacts` | measured/generated evidence | extract measurements into `70_benchmark`; archive raw artifacts |
| `asm` | low-level substrate | extract behavior into `40_substrate` |
| `assets` | semantic/runtime assets | retain only referenced assets; map ownership |
| `benchmarks` | empirical comparison | canonical evidence in `70_benchmark` |
| `bin` | command entrypoints | reduce to thin runtime/tool adapters |
| `config` | build/runtime configuration | normalize into identity/substrate/runtime configuration |
| `crates` | implementation modules | extract behavior; remove duplicate authority |
| `docs` | explanatory intent | preserve as design rationale attached to nodes |
| `generated` | derived material | never architectural authority |
| `hounds` | scanners/guardrails | convert useful invariants to proof constraints |
| `models` | model-facing behavior/data | extract semantic contracts |
| `nsq` | existing canonical candidate | primary source of semantic spine and proof intent |
| `prompts` | interaction/semantic ingress | extract intent, not prompt duplication |
| `reviewed_dropin_20260417` | historical integration snapshot | archive after intent extraction |
| `scripts` | operational/build workflows | collapse into deterministic tool surfaces |
| `specs` | formal contracts | preserve as constraints/invariants |
| `src` | implementation substrate | extract behavior into substrate/runtime planes |
| `state` | runtime/build history | evidence only; do not make state authoritative |
| `tests` | executable assertions | migrate assertions into proof plane |
| `tools` | developer/analysis tooling | retain only tools required by canonical workflow |

## Hidden/tooling surfaces

`.codex`, `.githooks`, and `.hooks` are treated as workflow/integration mechanisms. They may constrain development, but they do not define NSQ semantics.

## NSQ branches inspected

The existing `nsq` tree contains separate semantic, runtime, proof, generation, language-capture, operational, benchmark, and compatibility surfaces. Their intent is now represented in `intent_map.nsq` and `rebuild.nsq`; the target is one dependency direction rather than parallel authorities.

## Benchmark branches inspected

The repository contains multiple comparison families (`nsq_vs_c`, `code_corpus_vs_c`, `info_vs_c`, `open_output_vs_c`, and `repo_reality_vs_c`) plus native-pressure measurements and benchmark artifacts. These remain evidence attached to capabilities, not scattered claims.

## Reconstruction rule

A source path is not copied into the final system merely because it contains code. Its intent is extracted into one canonical node. Multiple implementations of the same intent become one authority plus explicitly marked adapters or historical evidence.
