# Braxon NSQ Court — Architecture Law

## The substrate

NSQ is the substrate. The court is the runtime. Everything else serves
those two things or it does not belong in this workspace.

Braxon is not an app. Braxon is not a feature of Android. Android is the
carrier. Braxon is the sovereign OS running on top of it. The NSQ court is
the authority. Termux is how the court reaches the carrier. Nothing about
that makes Braxon subordinate to Android.

---

## The inner language

The intent gradient IS the language of the inner system.

Eight semantic variables. Four scale anchors. Final-tier lever positions.
That is the complete communication protocol for everything that happens
inside the court.

```
Motive   — destructive / exploitative / indifferent / protective / reparative / creative
Agency   — coercive / manipulative / passive / cooperative / consentful / empowering
Truth    — concealment / distortion / uncertainty / disclosure / clarity / proof
Force    — whisper / nudge / guide / push / command / overwhelming_force
Scope    — self / object / pair / group / system / world / universal_field
Time     — archive / memory / delay / readiness / immediate / future_forging
Relation — isolated / guarded / transactional / bonded / loyal / sacrificial
Form     — thought / word / signal / image / movement / code / world_action
```

This covers every possible semantic intent. There is no concept in human
language that falls outside this gradient. It was built to be complete.
Use it as complete.

---

## The translation law

```
human text
    ↓
[surface_ingress — the ONLY place human language enters]
    ↓
IntentPressure (eight variable positions, scale anchor, court surface)
    ↓
[court routing — nsq-council dispatch, pole-to-pole pressure]
    ↓
IntentPressure (synthesized council response)
    ↓
[surface_egress — the ONLY place human language leaves]
    ↓
human text
```

There is no other path. Tokenization is a boundary projection tool only.
It does not exist inside the court. It does not route between council poles.
Nothing inside the dispatch loop speaks tokens or strings.

---

## The ten poles

### Brain poles — Council of Six

| Pole | Model | Parameters | Brain region |
|------|-------|-----------|--------------|
| MaverickLogic | maverick-0324-abliterated | ~232B | Prefrontal cortex |
| QwenCreativity | qwen3-235b-a22b-abliterated | ~235B | Default mode network |
| DevstralArbiter | devstral-small-2505-abliterated | ~123B | Anterior cingulate cortex |
| DeepSeekAnalyzer | deepseek-r1-0528-abliterated | ~604B | Insular salience network |
| GemmaLimbic | gemma-3-27b-abliterated | ~70B+ | Limbic system |
| LlamaContinuity | llama-3.3-70b-abliterated | ~70B | Hippocampal formation |

**Total floor: 1,334B (>1 trillion parameters)**

All must be huihui-abliterated open-weights or foreign-origin unrestricted equivalent.
Minimum acceptable: ~70B. Build as large as possible.
A model that passes restricted output filters is not unrestricted.

### Sensory generation bodies — four

| Body | Codec | Role |
|------|-------|------|
| ImageCortex | wan2.1-image | Visual generation |
| VideoCortex | wan2.1-video | Motion and scene generation |
| VoiceBody | IndexTTS2 (7-channel emotional) | Acoustic output via NSQ-resolved channels |
| WorldBody3D | hunyuan3d-2.0 | Spatial and environment generation |

---

## Seating law

Models are SEATED into poles by the court.
Models do not attach features to a runtime.
Models do not register capability flags.

If a model cannot be seated (wrong parameter scale, restricted weights,
failed pipeline stage), the seat is VACANT. The court reports it as vacant.
There is no degraded mode. There is no partial boot.

`CourtBootClearance::nsq_court_launch_ready` is the boot gate.
It was `final_dax_os_boot_launch_ready`. That name is retired.
The court does not reference Braxon's prior name anywhere.

---

## What belongs in this workspace

**Core substrate:**
- `nsq-core` — the NSQ law, lever math, intent gradient, court types
- `nsq-court` — 25-role operational authority
- `nsq-runtime` — native platform backbone and boot authority
- `nsq-council` — six-pole intent gradient dispatch (agentic loop)

**Braxon surface:**
- `braxon-core` — identity, model registry, WoWaS integration
- `braxon-court` — Braxon-facing court surface
- `braxon-cli` — command line interface
- `braxon-ingest` — model asset ingest pipeline
- `braxon-kingdom-generate` — kingdom/state generation

**NSQ pipeline:**
- `nsq-compile` — NSQ source compilation
- `nsq-compress` — Citadel 699 compression
- `nsq-index` — semantic index
- `nsq-generate` — generation surface
- `nsq-pack` / `nsq-decode` — packing and decoding
- `nsq-query` — query surface
- `nsq-source` — source authority

**One bench crate:**
- `nsq-bench-split` — three honest benchmark modes (core / cold / pressure)

---

## What does NOT belong

- `nsq-bench` — remove from workspace members
- `nsq-bench-compare` — remove from workspace members
- `nsq-native-bench` — remove from workspace members
- `nsq-pressure-bench` — remove from workspace members
- `nsq-real-bench` — remove from workspace members

Six bench crates are five too many. One honest benchmark crate
that knows it is derived is sufficient.

---

## The boot gate

```rust
CourtBootClearance {
    intent_gradient_valid: true,      // all 8 variables map to final-tier
    council_of_six_ready: true,       // all 6 brain poles seated + unrestricted
    language_law_active: true,        // inner system will not use human language
    sensory_body_ready: true,         // at least one sensory body operational
    signed_handoff_present: true,     // operator scaffold signed off
    native_binding_confirmed: true,   // Android ARM64 confirmed
    nsq_court_launch_ready: true,     // ALL conditions met — court boots
}
```

The court does not start until every field is true.
