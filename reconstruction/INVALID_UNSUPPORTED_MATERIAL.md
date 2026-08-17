# Invalid, Unsupported, and Narrative Material Disposition

## Decision rule

Material was not rejected because a branch or document called it unsupported. It was compared against executable behavior, dependency integrity, tests, and the surrounding documentation. A source is promoted only when its capability can be expressed as a validated NSQ contract and compiled or executed in the Reconstruction workspace.

| Source material | Finding | Disposition | Executable replacement or evidence |
|---|---|---|---|
| Dax-Autonomous-System `command_parser.rs` | Command dispatch is explicitly a placeholder that only echoes `CREATE` and `GET` strings. | Do not import as runtime authority. Preserve the intent as an ingress requirement. | `crates/braxon-core/src/nsq_native.rs` validates typed `NsqIntent` records and performs real address arbitration. |
| Dax-Autonomous-System swarm agents | The exported swarm lifecycle exists, but rendering, narrative, and input agents have empty lifecycle methods and no NSQ state contract. | Do not claim a completed engine. Retain only the lifecycle concept. | Ten-surface council and piston leases are implemented and tested in `NsqNativeBus`. |
| DAX-FULL generated build outputs | Generated `target/` artifacts are duplicate build products, not maintained source, and cannot serve as a source of truth. | Exclude generated artifacts from promotion; retain hashes in the audit inventory. | Braxon source modules and workspace tests are the accepted implementation surface. |
| DAX-FULL deck/game placeholders | Audited files contain incomplete or placeholder behavior and do not establish a complete runtime path. | Exclude until independently repaired and validated; no silent transplantation. | Relevant intent is represented through NSQ contracts only where it has an executable acceptance test. |
| Whisper/Willow/Stone material | This is the user's personal narrative and imaginative source, not system fact or security authority. | Preserve as the WoWAS narrative namespace. | `NarrativeRecord` requires `wowas_narrative` provenance; promotion to `FactRecord` is rejected without external provenance. |
| Unverified real-world claims inside narrative material | A story statement is not evidence merely because it is emotionally or symbolically important. | Keep in narrative storage and expose it to daydreaming only. | `FactRecord` requires source URI, retrieval date, confidence, and non-invalidated status. |
| Android-specific source/build claims | The target is a non-rooted Moto G, but this workspace's validation policy forbids Android-target builds. | Do not report device deployment as complete. | Host-side NSQ contracts are validated; physical device and Android build remain an explicit external acceptance gate. |
| Direct-X presentation requirement | The repository has a host-side CLI/runtime surface but no validated direct X11 renderer in the current workspace. | Do not claim a native X GUI exists yet. | The architecture contract defines the boundary; implementation requires an X server and approved native display dependency before it can be honestly marked complete. |

## No-hidden-files policy

The absolute-tree inventories under `audit/expanded/` include dotfiles, uncommon extensions, generated records, backups, and branch-reference artifacts. Duplicate groups are recorded rather than silently collapsed. Generated build outputs are visible in the inventory but are not promoted as source implementation.

## Rebuild principle

When a rejected source contains useful intent, the intent is rebuilt as a small NSQ-native contract with tests. When it contains only a claim, placeholder, or unverifiable generated output, the claim remains rejected. This keeps the final runtime self-contained without importing invalid authority.
