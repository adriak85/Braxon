# NSQ System

This crate is the consolidation boundary for the Braxon repository.

The rule is simple: every source artifact is discovered, classified, and assigned an NSQ intent record. Active files become canonical implementation nodes; historical `before_*`, backup, and archive artifacts remain evidence instead of silently becoming duplicate implementations.

The resulting plan is one cohesive NSQ-addressed system rather than a pile of crate-local meanings. The eight-dimensional gradient already defined by `nsq-core` is the semantic address space; this crate supplies repository-wide extraction, provenance, and reconstruction planning.

Use `SourceTree::scan(repository_root)`, then `RebuildPlanner::build(&tree)`, then `RebuildPlanner::validate(&plan)`.
