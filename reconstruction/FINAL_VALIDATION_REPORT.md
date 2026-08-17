# Final Reconstruction Validation Report

## Published state

The `reconstruction` branch is published at commit `176edee74` on `https://github.com/adriak85/Braxon/tree/reconstruction`. The branch contains the executable NSQ-native runtime additions, Target Field implementation, narrative/fact/daydream contracts, root CLI integration, audit evidence, and invalid-material disposition.

## Executable features completed

The operator `bus` command now creates and validates an NSQ-native intent and obtains a council-owned address lease before returning the existing BraxonBus report. The bus prevents same-space override through piston phases. The council requires exactly ten surfaces and supports explicit dynamic activation. Narrative-sourced daydream frames are bounded and yield when system intent is pending.

The Target Field is executable, persisted, deterministically reconstructed when absent, reconciled against the council-ten manifest, and fail-closed for malformed or non-finite state. The root handover exposes the Target Field result.

WoWAS narrative records are separated from real-world fact records. Narrative records require `wowas_narrative` provenance and cannot be promoted to facts without external provenance. Facts require a source URI, retrieval date, confidence, and non-invalidated status. The root commands `content narrative`, `content fact`, and `content daydream` exercise these contracts directly.

## Validation

The Rust 1.96 workspace command `cargo test --workspace --all-targets` completed successfully with no failed tests. Focused tests for `NsqNativeBus`, Target Field persistence, narrative/fact separation, and daydream yielding also passed. Root CLI checks and live examples for `bus`, `content narrative`, `content fact`, and `content daydream` passed.

## Final-tree cleanup

The finalization pass removed **73 explicitly identified backup/recovery artifacts** from the branch tip after recording every path in `audit/final_backup_removal_manifest.tsv`. Legitimate upstream files whose names merely contain `deprecated`, `old`, or `backup` as part of test/source content were not removed. Large generated audit archives remain outside the published tip under the local final-audit area with their manifest, because GitHub transfer limits prevented storing those raw blobs in the branch.

Two stale verification entrypoints were repaired rather than bypassed. The terminal provenance verifier now matches the repository’s truthful fail-closed configuration, and the stamp runtime verifier now checks the existing `state/nsq/stamps/stamp_execution_topology.json` artifact instead of a nonexistent legacy path.

After cleanup, `rustup run 1.96.0 cargo test --workspace --all-targets`, the NSQ substrate scan, terminal provenance verification, and stamp runtime verification all passed.

## Explicit limits

A direct-X native GUI has not been falsely reported as complete: the current repository has a validated host-side CLI and runtime contract, but no physical X-server rendering acceptance test. Likewise, the non-rooted Moto G target is specified and guarded by platform boundaries, but Android-target builds and physical-device deployment were not run because the workspace validation policy forbids Android-target builds. Those are external acceptance gates, not hidden failures.

The largest audit records exceeded ordinary GitHub transfer limits. Their exact contents remain in the local absolute tree as gzip archives under `audit/expanded/compressed/`, with `MANIFEST.tsv` mapping each archive to its original path and Git blob identity. The published branch contains the compact audit records and manifest; no audit conclusion depends on silently discarding the oversized content.

## Truth boundary

This report claims only what compiled, executed, or passed a test. Architecture documents, WoWAS narrative material, generated outputs, and labels such as “source of truth” are not treated as runtime proof without executable evidence.
