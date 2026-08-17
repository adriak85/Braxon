# WOWAS Digital Variance and Attention Policy

## Canonical world map

Every generated character retains a stable canonical character ID and a complete world-role record. The record includes functional role, book and region placement, house pressure, background function, four bounded timeline phases, assigned scenes, and all supported interaction layers: ally, rival, kin, mentor, dependent, faction, creature, location, and world system.

The canonical record is authoritative for continuity and provenance. User interaction cannot silently rewrite the character's identity, age gate, timeline, or plot obligation.

## User-focused projection

The reader-facing layer is a projection over the canonical map. It promotes characters when the user favorites or explicitly mentions them, repeatedly interacts with them, shares a direct relationship, reaches an active quest involving them, or has a matching preference. Background characters remain coherent and available in the world map without being forced into every scene.

The default population is tiered as follows:

| Tier | Default population | Reader behavior |
|---|---:|---|
| Promotable | 500 | Eligible for immediate attention when relevant or user-selected |
| Supporting | 1,000 | Enters scenes when its arc, relationship, or world function is active |
| Background | 3,500 | Remains simulated and traceable without consuming reader-facing cast space |

A reader-facing scene promotes at most one or two additional characters. Full relationship and provenance data remains in the indexed layer rather than being emitted as an unreadable flood of text.

## Digital variance boundaries

Variance may change presentation dimensions such as appearance, voice, role emphasis, and relationship proximity when explicitly permitted by the user profile. Variance is reversible and carries a stable projection serial. Identity mutation and timeline mutation are forbidden. Consent and age gates are required before a variant presentation is used.

User preferences influence attention and compatible presentation; they do not erase canonical characters, overwrite world history, or create untracked plot branches.

## Executable evidence

The generated outputs are:

| File | Evidence |
|---|---|
| `wowas_character_world_role_map.tsv` | 5,000 complete world-role records |
| `wowas_character_attention_projection.tsv` | 5,000 bounded attention projections |
| `wowas_user_preference_profile.template.tsv` | Explicit preference and consent input template |
| `wowas_character_timeline_schedule.tsv` | 20,000 phase assignments over existing scenes |
| `SERIEL_CROSSWALK.tsv` | 161,677 linked provenance records with zero unlinked records |

The compacted scene window contains 2,019 scenes. This count is a capacity result, not a sacred target: generated beats are merged into existing anchors where possible, and full provenance is retained off-page.
