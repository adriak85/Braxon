# FAISS Surface

This surface records how FAISS-backed vector search stays outside canonical NSQ
truth and only participates as a boundary retrieval aid.

Rules:

- Canonical NSQ meaning stays in base-8 switch topology.
- FAISS indexes are retrieval carriers, not semantic truth.
- Retrieved vectors must re-enter through explicit boundary translation before
  court or runtime surfaces consume them.
- Vector search metadata must preserve provenance, index identity, and source
  document linkage.

Current workspace intent:

- keep FAISS as an optional retrieval/export surface
- do not let FAISS naming redefine knowledge-graph or vector-imprint truth
- allow future Android and operator surfaces to reference a stable FAISS
  registry without collapsing runtime semantics into host-width assumptions
