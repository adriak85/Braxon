# BRAXON NSQ Audit Register

Current verified state:
- Root `Braxon` entrance is declared in the workspace manifest and can be built directly.
- The live verification path now audits `config/nsq/knowledge_graph.json`.
- The live verification path now audits `config/nsq/vector_imprint.json`.
- The live verification path now audits WoWaS scene-index assets, placeholder correlation rows, and missing book directories.
- The live verification path now audits whether any `faiss`/`faiss1` file surfaces exist in the repo.
- The root CLI now emits a recursive dependency/system documentation audit artifact.

Implemented in this pass:
1. Registered the real root `Braxon` binary target in `Cargo.toml`.
2. Wired knowledge-graph, vector-imprint, WoWaS asset, placeholder, and FAISS counts into `Braxon-core::verify_workspace`.
3. Exposed the new audit counts through the root `Braxon status` and `Braxon verify` operator surfaces.
4. Added recursive workspace/dependency/system documentation auditing and a dedicated `Braxon docs` operator surface.

Deferred on purpose:
1. Real WoWaS final-prose assembly remains deferred until the offline queue/model lane carries it end to end.
2. A real NSQ recoder that emits `Braxon-32B_extended.nsqb` still does not exist.
3. Whole-core runtime acceptance remains blocked until the recoder and artifact verifier exist.
4. Derived boundary crates such as `nsq-index` still use host-width carriers and remain recode targets rather than canonical truth.

Current truth:
- NSQ canonical meaning remains the alternating anchor/lever base-8 switch shape.
- Host widths may act as temporary carriers at the boundary, but not as semantic truth.
- WoWaS index assets exist, but many correlation assets are still placeholders and must not be treated as resolved canonical knowledge.
