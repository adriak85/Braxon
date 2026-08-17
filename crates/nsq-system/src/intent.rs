use nsq_core::{NsqFinalLeverPosition, NsqIntentGradientFrame, NsqIntentScaleAnchor, NsqIntentVariable};
use serde::{Deserialize, Serialize};
use std::path::Path;
use crate::source::{SourceArtifact, SourceKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntentDomain {
    Foundation,
    Intent,
    Encoding,
    Routing,
    Execution,
    Calibration,
    Interface,
    Documentation,
    HistoricalEvidence,
    BuildInfrastructure,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentRecord {
    pub path: String,
    pub domain: IntentDomain,
    pub canonical: bool,
    pub source_kind: SourceKind,
    pub gradient: NsqIntentGradientFrame,
    pub rationale: String,
}

pub fn extract_intent(a: &SourceArtifact) -> IntentRecord {
    let p = a.path.to_ascii_lowercase();
    let domain = if a.historical { IntentDomain::HistoricalEvidence }
    else if p.contains("intent") || p.contains("gradient") { IntentDomain::Intent }
    else if p.contains("stamp") || p.contains("watermark") || p.contains("compress") { IntentDomain::Encoding }
    else if p.contains("court") || p.contains("archon") || p.contains("kingdom") || p.contains("route") { IntentDomain::Routing }
    else if p.contains("execute") || p.contains("runtime") || p.contains("showdown") { IntentDomain::Execution }
    else if p.contains("calibr") || p.contains("bench") || p.contains("test") { IntentDomain::Calibration }
    else if p.contains("cli") || p.contains("compose") || p.contains("ingest") { IntentDomain::Interface }
    else if matches!(a.source_kind, SourceKind::Documentation) { IntentDomain::Documentation }
    else if p.contains("build") || p.contains("tower") || p.contains("llvm") || p.contains("script") { IntentDomain::BuildInfrastructure }
    else if p.contains("core") || p.ends_with("lib.rs") { IntentDomain::Foundation }
    else { IntentDomain::Other };

    let canonical = !a.historical;
    let mut positions: [NsqFinalLeverPosition; 8] = [563; 8];
    if a.historical { positions[NsqIntentVariable::Time.index()] = 200; positions[NsqIntentVariable::Truth.index()] = 900; }
    if matches!(domain, IntentDomain::Foundation) { positions[NsqIntentVariable::Motive.index()] = 1000; positions[NsqIntentVariable::Form.index()] = 1000; }
    if matches!(domain, IntentDomain::Intent) { positions[NsqIntentVariable::Motive.index()] = 950; positions[NsqIntentVariable::Truth.index()] = 950; }
    if matches!(domain, IntentDomain::Execution) { positions[NsqIntentVariable::Force.index()] = 900; positions[NsqIntentVariable::Form.index()] = 900; }
    if matches!(domain, IntentDomain::Calibration) { positions[NsqIntentVariable::Truth.index()] = 1000; positions[NsqIntentVariable::Time.index()] = 850; }
    let frame = NsqIntentGradientFrame { variable_positions: positions, scale_anchors: [
        NsqIntentScaleAnchor::SelfObjectScale,
        NsqIntentScaleAnchor::RelationalGroupScale,
        NsqIntentScaleAnchor::SystemWorldScale,
        NsqIntentScaleAnchor::UniversalFieldScale,
    ]};
    let rationale = format!("{} → {:?}; canonical={}, source={:?}", Path::new(&a.path).display(), domain, canonical, a.source_kind);
    IntentRecord { path: a.path.clone(), domain, canonical, source_kind: a.kind, gradient: frame, rationale }
}
