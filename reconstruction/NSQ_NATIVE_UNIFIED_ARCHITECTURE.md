# NSQ-Native Unified Reconstruction Architecture

## Purpose

The final Reconstruction target is a single self-contained runtime whose **brain-stem, intent transport, arbitration, and state surfaces are NSQ-native**. LLVM, JIT, MLIR, MLP/R, Hammer, Dwarf, Bolt, AST tooling, tree-sitter, Archer, Alien, Zig, Nix, and related systems are translation or build inputs. They are not parallel runtime authorities and are not permitted to override the NSQ bus contract.

> The implementation rule is: external capability may be translated into a validated NSQ contract, but no external label, generated claim, or secondary runtime becomes authoritative merely by being named.

## Authority layers

| Layer | Authority | Responsibility | Forbidden behavior |
|---|---|---|---|
| Brain stem | NSQ core and NSQ system | Intent gradients, address ownership, validation, reconstruction, and fail-closed decisions | Calling a second runtime authoritative or bypassing NSQ validation |
| Council | NSQ-native ten-member council | Six brain poles plus four sensory bodies, with explicit ownership and arbitration | Same-space writes without ownership or silent model substitution |
| Bus | NSQ bus and address registry | Keeps virtual parameters alive, assigns leases, records conflicts, and activates or idles surfaces | Untracked shared-memory mutation or destructive override |
| Translation | LLVM/JIT/MLIR and complementary tool adapters | Converts source/build/analysis capabilities into NSQ schemas and tests | Treating tool output as proof of runtime capability |
| Narrative | WoWAS: Whispers of Willow and Stone | Imagination, story fuel, character/world priors, and daydream prompts | Being used as system fact, safety proof, or release authority |
| Facts | Versioned system documentation and fact records | Real-world, platform, protocol, and operational knowledge with provenance and freshness | Mixing fictional narrative with fact records or accepting unsupported truth claims |
| Presentation | Native X surface and touch-oriented target shell | Displays NSQ state directly, including user-selected font variants and multi-portal views | Hiding state behind an unvalidated abstraction or claiming Android execution was tested when it was not |

## NSQ-native intent contract

Every action enters the system as a typed NSQ intent record containing an intent identifier, source surface, eight-dimensional gradient, requested capability, target address set, lease policy, provenance, and validation status. The NSQ brain stem resolves it into one of four outcomes: `accepted`, `queued`, `deferred`, or `rejected`.

The Target Field is the coordinate destination for the gradient. Its implementation is `crates/braxon-core/src/target_field.rs`, persisted at `state/braxon/target_field.json`, and derived from the validated council-ten manifest if absent. It does not authorize an action by itself; the NSQ validator and bus lease rules remain authoritative.

## Council and piston arbitration

The ten active council surfaces are six brain poles and four sensory bodies. Each active surface owns a disjoint address lease. A piston-style operation is represented as an acquire, hold, commit, or release cycle. A write may proceed only while its lease is current, its address set is disjoint from active conflicting leases, its intent passes NSQ validation, and its evidence record is attached to the bus. Idle surfaces may be activated dynamically, but activation is itself an NSQ intent and cannot silently steal a live address.

## Dynamic activation and daydreaming

Daydreaming is a bounded, interruptible workload class. It runs only on idle addresses, yields to system and user-critical intents, records its source as `wowas_narrative`, and may propose imagination outputs. It cannot mutate system facts, release gates, security state, or address ownership without a separate validated intent. The scheduler must preserve the difference between a speculative narrative proposal and an operational action.

## Narrative and facts

Whispers of Willow and Stone is retained as a first-class personal narrative source. Its records are tagged as narrative, carry source and version metadata, and feed imagination, analogy, character, and world-generation surfaces. System documentation and real-world facts are stored in a separate fact namespace with provenance, retrieval date, confidence, and invalidation metadata. No narrative sentence is promoted to fact without an explicit fact-ingestion and validation record.

## Native presentation and target contract

The presentation contract is a native X display surface with direct NSQ state rendering, user-selectable font variants, and multiple independently permissioned portals. The target shell is touch-oriented and designed for a non-rooted Moto G deployment. This repository currently validates the host-side contracts and does not claim that an Android build or device deployment has been completed. Android-specific build execution remains a separate gated validation task.

## Translation boundaries

LLVM/JIT/MLIR and the complementary tools are mapped into NSQ contracts for compilation, scheduling, intermediate representation, dependency graphs, parsing, packaging, and reproducibility. A translator must emit the source hash, toolchain identity, requested NSQ capability, generated contract, and validation status. Invalid generated files, prompt-only assets, placeholder stubs, broken manifests, and unsupported dependency pins remain excluded until repaired and independently validated.

## Completion criteria

The unified target is complete only when the NSQ intent path, council lease arbitration, Target Field, dynamic activation, narrative/facts separation, native presentation contract, and translation records all have executable schemas, tests, and evidence. A document describing a capability is not accepted as proof that the capability exists.
