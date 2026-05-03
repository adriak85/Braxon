# nsq-runtime

`nsq-runtime` is the current native-runtime incorporation surface for NSQ. It defines runtime-lane descriptors, maps court routes, builds canonical cell/lever reports, and carries the first active slices for Python 3 and the offline model lane while other language/runtime lanes remain defined for later recode slices.

## Responsibilities
- publish the native runtime lane registry and per-lane status
- keep language and runtime interfaces routed through court surfaces
- encode ingress into canonical NSQ cells and lever-position reports
- expose the Python 3 runtime slice and the offline model runtime slice
- track runtime semantic context readiness for downstream consumers
- audit sovereign lifecycle readiness before any destructive reset or wipe is considered safe

## Key types
- `NativeRuntimeRegistry` and `NativeRuntimeLaneDescriptor`
- `Python3RuntimeLane`, `Python3Ingress`, and `RuntimeSliceReport`
- `OfflineModelLane` and `OfflineInferenceReport`

## Inputs and outputs
- Input: native ingress text, model requests, and semantic-context state
- Output: canonical runtime slice reports and offline inference reports with lever-position data

## Workspace links
- Depends on `nsq-core`
- Used by the root Braxon command center to report runtime readiness and semantic-consumer health
