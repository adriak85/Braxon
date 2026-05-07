# Braxon Materialization Boundary

Braxon does not depend on permanent full static local model-weight storage.

The correct model is:
- seed / seed-bloom material carried by the bus
- conditional refresh, not wasteful millisecond discard
- stored operations and frameworks
- stamp wake triggers
- materialization / rehydration paths
- proof-boundary reporting

Do not treat missing full FP32 local shards as proof Braxon is dead.

Do treat missing materialization proof as a boundary that must be honestly reported.

Never fake materialized models.
Never count pointer stubs as real shards.
Never call a manifest-only state hot-live unless hot-live proof exists.
Keep donor lane and target lane separate.
