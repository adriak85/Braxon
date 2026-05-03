# nsq-core

`nsq-core` defines the canonical NSQ semantic shape. It models the alternating full-binary anchors and multipositional levers that make up the base-8 switch topology, and it treats host integer widths as boundary carriers only.

## Responsibilities
- validate anchors as `0|1`
- validate levers in the canonical `1..=1126` range
- stabilize lever positions from applied hertz samples and correction windows
- expose canonical switch and word structures used by higher runtime layers
- define the royal court surface enum

## Key types
- `FullBinaryAnchor`
- `MultipositionalLever`
- `NuPair`
- `CanonicalSwitchShape`
- `CanonicalBase8Bit`
- `NuCell`, `NuWord`, and `CourtSurface`

## Inputs and outputs
- Input: boundary-carrier values or hertz windows
- Output: validated canonical NSQ structures suitable for native runtime use

## Workspace links
- Consumed by `nsq-runtime` and `Braxon-core`
- This crate is semantic authority, not a transport or packing layer
