use serde::{Deserialize, Serialize};
use crate::{IntentDomain, IntentRecord, SourceTree};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RebuildPlan {
    pub source_count: usize,
    pub canonical_count: usize,
    pub historical_count: usize,
    pub records: Vec<IntentRecord>,
}

pub struct RebuildPlanner;

impl RebuildPlanner {
    /// Convert the complete source tree into one intent-addressed plan.
    /// Historical files are retained as evidence, but do not become competing
    /// implementations. Active artifacts become canonical NSQ nodes.
    pub fn build(tree: &SourceTree) -> RebuildPlan {
        let records: Vec<_> = tree.artifacts.iter().map(crate::intent::extract_intent).collect();
        let canonical_count = records.iter().filter(|r| r.canonical).count();
        let historical_count = records.iter().filter(|r| !r.canonical).count();
        RebuildPlan { source_count: records.len(), canonical_count, historical_count, records }
    }

    pub fn by_domain<'a>(plan: &'a RebuildPlan, domain: IntentDomain) -> impl Iterator<Item=&'a IntentRecord> {
        plan.records.iter().filter(move |r| r.domain == domain)
    }

    pub fn validate(plan: &RebuildPlan) -> Result<(), String> {
        if plan.source_count != plan.records.len() { return Err("source/intent cardinality mismatch".into()); }
        for r in &plan.records {
            let v = nsq_core::validate_intent_gradient_frame(&r.gradient);
            if !v.positions_inside_final_tier { return Err(format!("invalid NSQ gradient for {}", r.path)); }
        }
        Ok(())
    }
}
