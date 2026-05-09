# NSQ Boundary Architecture

## Purpose

NSQ stays canonical inside Braxon.
Foreign tooling consumes projections exported from NSQ at the boundary.

## Internal strata

1. Canonical NSQ semantics
   - alternating full binary anchors and multipositional levers
   - preserved source surfaces
   - semantic records and calibration context
2. Royal Court component layer
   - compositor
   - lexer
   - parser
   - linter
   - optimizer
   - router
   - scheduler
   - inspector
3. Native spine runtime
   - runtime dispatch
   - court routing
   - calibration and preservation paths
4. Foreign boundary layer
   - debugger exports
   - security checker exports
   - static analyzer exports
   - compiler and build exports
   - binary and system interface exports
   - foreign language/runtime exports

## Boundary law

- foreign tools do not read canonical NSQ directly unless they already speak NSQ
- foreign tools receive export packages, traces, manifests, or translated artifacts
- host-width carriers are allowed only inside those export packages
- importers must translate foreign inputs into canonical NSQ or reject them

## Required boundary outputs

- court trace projection
- canonical artifact manifest
- stack-surface projection bundle
- debug/security/analyzer export bundle
- runtime observer export stream

## Required boundary inputs

- foreign source text
- system interface events
- toolchain metadata
- debugger/analyzer requests

All such inputs must cross a translation membrane before they touch canonical NSQ semantics.
