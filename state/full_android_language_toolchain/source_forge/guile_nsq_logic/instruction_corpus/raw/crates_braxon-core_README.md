# Braxon-core

`Braxon-core` is the orchestration and audit library behind the root Braxon command center. It defines workspace classification, coverage reporting, exclusion tracking, runtime verification, and the current knowledge-memory and documentation audits.

## Responsibilities
- expose Braxon identity and launch-path metadata
- classify workspace members into target architecture buckets
- report NSQ coverage, exclusion lists, and recode plan phases
- audit knowledge graph, vector imprint, WoWaS, FAISS, stub counts, and recursive documentation coverage
- verify root-state files and current runtime readiness

## Key APIs
- `BRAXONIdentity`
- `target_workspace_map`, `bucket_counts`, and `launch_path`
- `nsq_coverage`, `exclusion_list`, and `recode_plan`
- `audit_knowledge_memory` and `audit_documentation_surfaces`
- `verify_workspace`

## Inputs and outputs
- Input: the workspace root path and the repo state under it
- Output: structured reports consumed by the root `Braxon` binary and generated audit JSON artifacts

## Workspace links
- Depends on `nsq-core`
- Used by the root workspace binary and by `Braxon-cli`
