# Braxon / Codex Boundary

Braxon and Codex are separate entities.

Braxon is the user's system/project identity.

Codex is an external branded agent/tool the user sometimes uses to help build.

## Rules

1. Do not unify Braxon and Codex.
2. Do not describe Codex as part of Braxon.
3. Do not describe Braxon as Codex.
4. Do not merge their roles, identity, authority, memory, runtime, or agency.
5. Codex may be treated only as an external helper/tool used during development.
6. Codex is not part of the permanent Braxon architecture.
7. Codex is not a runtime component.
8. Codex is not a dependency required by users.
9. Codex must not be built into Braxon.
10. Codex should be replaced by Braxon as soon as Braxon can perform the needed local build/conversion work itself.

## Current Build Focus

The immediate focus is not to expand Codex usage.

The immediate focus is to get Braxon working on the user's own system with enough local intelligence and human-grade knowledge to participate in building, converting, repairing, and improving the project directly.

Braxon must become the local/offline builder target.

Codex can assist the build process temporarily, especially for inspection, conversion planning, and code-generation support, but every useful result should be converted into the Braxon/NSQ system rather than making Codex part of the final design.

## Replacement Path

```text
Codex temporary help
→ inspect/build/convert current gaps
→ commit current-native Braxon/NSQ material
→ Braxon gains local build capability
→ Codex dependency shrinks
→ Braxon replaces Codex as the local project builder
```

## Offline Mission Tie-In

The final system must help users who may have no reliable internet access. Therefore, no Codex-dependent workflow can count as final runtime capability.

A capability counts only when Braxon can run it locally/offline after lawful source-edge materialization and current-native conversion.
