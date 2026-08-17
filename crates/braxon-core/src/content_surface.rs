use serde::{Deserialize, Serialize};

pub const NARRATIVE_SCHEMA: &str = "braxon.wowas.narrative.v1";
pub const FACT_SCHEMA: &str = "braxon.system.fact.v1";
pub const DAYDREAM_SCHEMA: &str = "braxon.daydream.workload.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NarrativeRecord {
    pub schema: String,
    pub record_id: String,
    pub title: String,
    pub text: String,
    pub source: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FactRecord {
    pub schema: String,
    pub fact_id: String,
    pub statement: String,
    pub source_uri: String,
    pub retrieved_at: String,
    pub confidence: String,
    pub invalidated: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DaydreamFrame {
    pub schema: String,
    pub workload_id: String,
    pub step: u32,
    pub prompt: String,
    pub source: String,
    pub yielded: bool,
    pub proposed_action: Option<String>,
}

impl NarrativeRecord {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != NARRATIVE_SCHEMA || self.source != "wowas_narrative" {
            return Err("narrative must use the WoWAS narrative schema and provenance".to_string());
        }
        if self.record_id.trim().is_empty() || self.text.trim().is_empty() {
            return Err("narrative record requires an id and text".to_string());
        }
        Ok(())
    }
}

impl FactRecord {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != FACT_SCHEMA {
            return Err("fact schema mismatch".to_string());
        }
        if self.fact_id.trim().is_empty() || self.statement.trim().is_empty() || self.source_uri.trim().is_empty() {
            return Err("fact requires an id, statement, and source URI".to_string());
        }
        if self.invalidated {
            return Err("invalidated fact cannot enter the active fact surface".to_string());
        }
        if !matches!(self.confidence.as_str(), "low" | "medium" | "high") {
            return Err("fact confidence must be low, medium, or high".to_string());
        }
        Ok(())
    }

    pub fn from_narrative(_: &NarrativeRecord) -> Result<Self, String> {
        Err("narrative cannot be promoted to system fact without external provenance".to_string())
    }
}

pub fn daydream_frame(workload_id: &str, step: u32, prompt: &str, system_intent_pending: bool) -> Result<DaydreamFrame, String> {
    if workload_id.trim().is_empty() || prompt.trim().is_empty() {
        return Err("daydream frame requires a workload id and prompt".to_string());
    }
    Ok(DaydreamFrame {
        schema: DAYDREAM_SCHEMA.to_string(),
        workload_id: workload_id.to_string(),
        step,
        prompt: prompt.to_string(),
        source: "wowas_narrative".to_string(),
        yielded: system_intent_pending,
        proposed_action: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn narrative_is_not_a_fact() {
        let narrative = NarrativeRecord { schema: NARRATIVE_SCHEMA.to_string(), record_id: "willow-1".to_string(), title: "A river remembers".to_string(), text: "A fictional image".to_string(), source: "wowas_narrative".to_string(), version: "1".to_string() };
        narrative.validate().unwrap();
        assert!(FactRecord::from_narrative(&narrative).is_err());
    }

    #[test]
    fn valid_fact_requires_provenance() {
        let fact = FactRecord { schema: FACT_SCHEMA.to_string(), fact_id: "rust-version".to_string(), statement: "The workspace declares a Rust toolchain".to_string(), source_uri: "file:///home/ubuntu/Braxon/rust-toolchain.toml".to_string(), retrieved_at: "2026-08-17".to_string(), confidence: "high".to_string(), invalidated: false };
        assert!(fact.validate().is_ok());
    }

    #[test]
    fn daydream_yields_when_system_work_exists() {
        let frame = daydream_frame("dream-1", 1, "Imagine a new council vista", true).unwrap();
        assert!(frame.yielded);
        assert_eq!(frame.source, "wowas_narrative");
    }
}
