# REPO CORE CONSOLIDATION PLAN

## Goal
Collapse the workspace into one canonical solid core with NSQ as native normalization backbone and BRAXON as runtime consumer and orchestrator.

## Canonical truths
- specs/nsq/*
- specs/repo/*
- config/kingdom/court_canonical.json
- crates/nsq-source/*
- crates/nsq-compile/*
- crates/nsq-pack/*
- crates/nsq-inspect/*
- crates/nsq-proof/*
- crates/nsq-cli/*

## Derived surfaces
- config/nsq_court.json
- config/braxon_court.json
- generated manifests
- benchmark reports
- decode views
- proof outputs
- release pack outputs

## Remove or quarantine
- duplicated doctrine files after merge
- benchmark private notes from runtime reach
- repeated generated optimizer outputs
- ad hoc shell residue
- repeated grep dumps
- strace output from tracked source payloads

## Runtime boundary
Benchmark specs and notes must never surface in runtime commands, repl, status, doctor, or artifact payloads.

## Dependency boundary
No foreign runtime is required.
Dialect support must lower internally through NSQ hook and compile surfaces only.

## Success condition
- one canonical repo
- one source chain
- one compile chain
- deterministic proof
- direct repo intake works
- benchmark private notes cannot surface at runtime
