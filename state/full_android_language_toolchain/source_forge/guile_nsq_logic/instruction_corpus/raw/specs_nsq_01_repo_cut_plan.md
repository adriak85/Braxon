# NSQ Repo Cut Plan

## Keep
- specs/nsq/*
- nsq/examples/*
- one parser/compiler crate for NSQ
- one native artifact format
- one decode/inspect binary
- one proof runner

## Quarantine
Move these out of proof path:
- placeholder score fillers
- bootstrap matrix score scripts
- fake compare harnesses
- wrapper-only command launchers
- debug-only trace surfaces
- JSON-first proof reporters that are not part of native lane

## Native lane target
nsq source file
-> nsq-compile
-> .nsqb native artifact
-> nsq-inspect / nsq-decode

## Record families
- noise
- triple
- membrane

## Required packed fields
Noise:
- symbol_id
- macro_id
- switch_a
- switch_b
- pos
- amp

Triple:
- subject_id
- relation_macro_id
- object_id
- layer
- plane
- anchor_delta
- weight
- flags

Membrane:
- cell_id
- state_id
- flux
- gate
- phase
