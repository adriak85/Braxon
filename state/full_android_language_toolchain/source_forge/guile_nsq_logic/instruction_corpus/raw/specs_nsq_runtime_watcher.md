# NSQ Runtime Watcher

## Purpose

The runtime watcher is the observability layer for Braxon.
It is separate from the Royal Court and separate from the knowledge graph.

## Watched surfaces

- lexer
- parser
- compiler
- compositor
- router
- scheduler
- inspector

## Output kinds

- route events
- preservation checkpoints
- calibration observations
- boundary export traces
- proof execution traces

## Separation rule

The watcher may observe and export.
It may not become the semantic source of truth for the court or the canonical substrate.
