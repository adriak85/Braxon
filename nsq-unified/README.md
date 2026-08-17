# NSQ Unified System

This directory is the canonical semantic destination for the repository.

The source tree is treated as evidence, not as the architecture. Every file is assigned an NSQ record keyed by its repository path and content digest. Large files are consumed in deterministic sections rather than skipped. Historical/back-up files remain traceable but do not automatically become duplicate runtime modules.

The translation pipeline is:

`source file -> section stream -> intent extraction -> NSQ node -> dependency edges -> canonical module -> verification`

No source file is silently discarded. A source can resolve to executable NSQ, a specification/contract node, a test/benchmark node, a generated artifact record, or an explicit archival/evidence node. The distinction is semantic, not based only on filename.

The canonical NSQ system is organized around intent and dependency, not the original crate/folder layout.
