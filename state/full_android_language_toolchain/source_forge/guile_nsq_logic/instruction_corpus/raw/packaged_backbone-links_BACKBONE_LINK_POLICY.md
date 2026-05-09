# Backbone Link Policy

Status: source-review staging, not release.

This package does not store or duplicate the full backbone.

It links to the actual Braxon backbone:

- state_backbone -> /data/data/com.termux/files/home/Braxon/state
- source_forge -> /data/data/com.termux/files/home/Braxon/state/full_android_language_toolchain/source_forge
- full_android_language_toolchain -> /data/data/com.termux/files/home/Braxon/state/full_android_language_toolchain

Rule:
The package carries identity, manifests, proofs, scripts, source-lane reports, and bake-over instructions.
The real source/state backbone remains first-class and external to the review package unless explicitly selected.
