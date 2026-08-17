//! NSQ System — the cohesive reconstruction layer for the entire Braxon tree.
//!
//! The system does not copy the repository's historical fragments into another
//! pile. It treats every repository file as a source artifact, extracts its
//! role into the eight-dimensional NSQ intent gradient, and reduces duplicate
//!/historical artifacts to one canonical intent node.

pub mod intent;
pub mod rebuild;
pub mod source;

pub use intent::{IntentDomain, IntentRecord};
pub use rebuild::{RebuildPlan, RebuildPlanner};
pub use source::{SourceArtifact, SourceKind, SourceTree};
