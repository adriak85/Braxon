# Root Workspace Target Map

## Evaluate
- Current root state is a real package and binary entrance, so the normal operator flow can start at the root.
- `nsq-pack` and `nsq-inspect` did not agree on a native artifact marker, which made the pack/inspect lane internally inconsistent.
- `nsq-runtime` and `nsq-decode` are admitted into the workspace, but they still need native-lane cleanup and boundary cleanup.
- `nsq-source` still needs drift cleanup so source-ingress compatibility naming does not masquerade as runtime authority.
- Graphics/operator stack planning is required by law, but no root-facing graphics crate is placed yet.

## Classify
- root entrance/orchestrator: `Braxon`, `Braxon-core`
- NSQ core/canonical semantics: `nsq-core`, `nsq-source`, `nsq-compile`, `nsq-pack`, `nsq-inspect`, `nsq-compose`, `nsq-prime`, `nsq-runtime`
- Royal Court component layer: `nsq-court`, `Braxon-court`, `nsq-archon`, `nsq-lint`, `nsq-optimize`, `nsq-calibrate`
- graphics/operator stack: reserved for AGDK, wgpu, Bevy, egui, and integration surfaces
- platform entrances: `Braxon-cli`, `Braxon-ingest`, `nsq-cli`, `Braxon-showdown`, `Braxon-kingdom-generate`
- internal carrier/audit surfaces: `nsq-index`, `nsq-query`, `nsq-decode`, `nsq-generate`, `nsq-proof`
- legacy/retire/quarantine: `nsq-preserve`, `nsq-debug`, `nsq-profile`, `nsq-bench`, `nsq-bench-split`, `nsq-bench-compare`, `nsq-pressure-bench`, `nsq-real-bench`, `nsq-native-bench`

## Target Map
- Preferred launch path: `Braxon` root package -> `Braxon-core` orchestration -> `nsq-core` canonical semantics -> court routing -> platform or internal carrier entrance
- Root package responsibility: command center, workspace verification, target-map visibility, and court-aware operator status
- NSQ core responsibility: canonical base-8 switch law, native court-surface registry, and reusable semantic validation
- Court responsibility: route native runtime work through court surfaces instead of plugin-style detours
- Internal carrier responsibility: keep source, audit, manifest, and execution surfaces under NSQ authority unless an explicit user-requested export surface is added
- Metadata hook responsibility: preserve hook matrices as guidance/audit surfaces; do not delete them, and do not confuse them with native runtime completion
- Citadel699 model responsibility: treat request/return rebuild as stamp/macro reconstruction with MB-scale tiny-seed target material, not raw byte payload transfer
- Legacy responsibility: remain outside the canonical runtime lane until repaired, merged, or retired

## Initiated Implementation
- Root is now expected to be a real package and binary entrance, not only a workspace manifest.
- Root launch is now expected to deny thin placeholder boot and either open a live NSQ operator window or print exact finish steps.
- `nsq-core` carries the four-position alternating anchor/lever switch shape explicitly.
- `nsq-pack` and `nsq-inspect` share one deterministic native marker and report carrier units instead of byte-native meaning.
- `nsq-runtime` and `nsq-decode` are workspace members, so the next repair phase is native runtime cleanup, not admission.
- Android oaboot is now an explicit root-runtime profile instead of an implied metadata fragment.
- Nu128 install oversight is now explicit about one-chunk-at-a-time transport and whole-model viability staying behind NSQ recode.
- `Braxon-ingest` is the dedicated BRAXON crate for chunk-governed model ingress, donor-lane truth, and recode handoff.
- User-facing terminal drift away from thin-client and ZLM-era wording has started at the root entrance so the operator path stays NSQ-first.
- User-facing runtime/session identifiers are now normalized at the root CLI boundary even where deeper storage surfaces still carry legacy names.
- The root pre-commit lane now preserves metadata hooks while blocking destructive automation and new drift that flattens NSQ into byte or host-width semantics.
- The canonical six-model stack now records Citadel699 fully active unified reconstruction with MB-scale tiny-seed stamp/macro target semantics.
- The root entrance now exposes `apps` and `runtime` command groups, including direct `Braxon runtime python3 "<call>"` execution against `nsq-runtime` instead of a documentation-only promise.
