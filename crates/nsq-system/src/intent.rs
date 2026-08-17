use crate::source::{SourceArtifact, SourceKind};
use nsq_core::{NsqFinalLeverPosition, NsqFinalSide, NsqIntentGradientFrame, NsqIntentScaleAnchor};
use serde::{Deserialize, Serialize};
use std::path::Path;

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

fn position(value: u64) -> NsqFinalLeverPosition {
    NsqFinalLeverPosition::new(value, NsqFinalSide::Positive)
}

pub fn extract_intent(a: &SourceArtifact) -> IntentRecord {
    let p = a.path.to_ascii_lowercase();
    let domain = if a.historical {
        IntentDomain::HistoricalEvidence
    } else if p.contains("intent") || p.contains("gradient") {
        IntentDomain::Intent
    } else if p.contains("stamp") || p.contains("watermark") || p.contains("compress") {
        IntentDomain::Encoding
    } else if p.contains("court")
        || p.contains("archon")
        || p.contains("kingdom")
        || p.contains("route")
    {
        IntentDomain::Routing
    } else if p.contains("execute") || p.contains("runtime") || p.contains("showdown") {
        IntentDomain::Execution
    } else if p.contains("calibr") || p.contains("bench") || p.contains("test") {
        IntentDomain::Calibration
    } else if p.contains("cli") || p.contains("compose") || p.contains("ingest") {
        IntentDomain::Interface
    } else if matches!(a.kind, SourceKind::Documentation) {
        IntentDomain::Documentation
    } else if p.contains("build")
        || p.contains("tower")
        || p.contains("llvm")
        || p.contains("script")
    {
        IntentDomain::BuildInfrastructure
    } else if p.contains("core") || p.ends_with("lib.rs") {
        IntentDomain::Foundation
    } else {
        IntentDomain::Other
    };

    let mut motive = 563;
    let agency = 563;
    let mut truth = 563;
    let mut force = 563;
    let scope = 563;
    let mut time = 563;
    let relation = 563;
    let mut form = 563;
    if a.historical {
        time = 200;
        truth = 900;
    }
    if matches!(domain, IntentDomain::Foundation) {
        motive = 1000;
        form = 1000;
    }
    if matches!(domain, IntentDomain::Intent) {
        motive = 950;
        truth = 950;
    }
    if matches!(domain, IntentDomain::Execution) {
        force = 900;
        form = 900;
    }
    if matches!(domain, IntentDomain::Calibration) {
        truth = 1000;
        time = 850;
    }
    let gradient = NsqIntentGradientFrame {
        motive: position(motive),
        agency: position(agency),
        truth: position(truth),
        force: position(force),
        scope: position(scope),
        time: position(time),
        relation: position(relation),
        form: position(form),
        scale_anchor: match domain {
            IntentDomain::HistoricalEvidence => NsqIntentScaleAnchor::Systemic,
            IntentDomain::Routing | IntentDomain::Execution => NsqIntentScaleAnchor::Relational,
            IntentDomain::Documentation | IntentDomain::Interface => NsqIntentScaleAnchor::Local,
            _ => NsqIntentScaleAnchor::Universal,
        },
    };
    let canonical = !a.historical;
    let rationale = format!(
        "{} → {:?}; canonical={}, source={:?}",
        Path::new(&a.path).display(),
        domain,
        canonical,
        a.kind
    );
    IntentRecord {
        path: a.path.clone(),
        domain,
        canonical,
        source_kind: a.kind,
        gradient,
        rationale,
    }
}
