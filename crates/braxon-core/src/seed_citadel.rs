//! Seed-first materialization for the NSQ cognitive substrate.
//!
//! This module deliberately separates logical completeness from physical
//! residency. A seed describes the canonical sections and deterministic
//! expansion rules; a citadel records the complete logical parameter/state
//! domain while only an active window needs to be resident on the bus.
//!
//! It is a construction primitive, not a claim that neural weights can be
//! reconstructed from arbitrary text. Real learned weights still require a
//! cryptographic seed/artifact whose reconstruction contract is defined by
//! the producer.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

pub const UNIVERSAL_TOKENIZER_VERSION: &str = "nsq.universal.token.sync.v1";
pub const CITadel_MATERIALIZATION_VERSION: &str = "nsq.citadel.seed.materialization.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TokenSection {
    pub section: String,
    pub namespace: u16,
    pub symbols: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UniversalToken {
    pub universal_id: u64,
    pub canonical: String,
    pub projections: BTreeMap<String, u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UniversalTokenizerSeed {
    pub version: String,
    pub universal: TokenSection,
    pub sections: Vec<TokenSection>,
}

impl UniversalTokenizerSeed {
    pub fn canonical() -> Self {
        let names = [
            "maverick_logic",
            "qwen_synthesis",
            "devstral_arbiter",
            "deepseek_analyzer",
            "gemma_limbic",
            "llama_reasoner",
            "image_body",
            "voice_body",
            "video_body",
            "spatial_body",
        ];
        let sections = names
            .iter()
            .enumerate()
            .map(|(i, name)| TokenSection {
                section: (*name).to_string(),
                namespace: (i + 1) as u16,
                symbols: Vec::new(),
            })
            .collect();
        Self {
            version: UNIVERSAL_TOKENIZER_VERSION.to_string(),
            universal: TokenSection {
                section: "universal".to_string(),
                namespace: 0,
                symbols: vec!["<bos>".into(), "<eos>".into(), "<intent>".into(), "<state>".into()],
            },
            sections,
        }
    }

    pub fn synchronize(&self, canonical: &str) -> UniversalToken {
        let universal_id = stable_id(canonical);
        let projections = self
            .sections
            .iter()
            .map(|section| (section.section.clone(), (universal_id % 4_294_967_291) as u32))
            .collect();
        UniversalToken {
            universal_id,
            canonical: canonical.to_string(),
            projections,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SeedMaterializationPlan {
    pub version: String,
    pub seed_id: String,
    pub logical_parameter_bits: u128,
    pub logical_state_bits: u128,
    pub active_window_bits: u64,
    pub tokenizer: UniversalTokenizerSeed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CitadelState {
    pub seed_id: String,
    pub logical_complete: bool,
    pub materialized_window_start: u128,
    pub materialized_window_bits: u64,
    pub parameter_digest: u64,
    pub tokenizer_version: String,
}

pub fn build_seed_plan(seed_id: &str, logical_parameter_bits: u128, logical_state_bits: u128) -> SeedMaterializationPlan {
    SeedMaterializationPlan {
        version: CITadel_MATERIALIZATION_VERSION.to_string(),
        seed_id: seed_id.to_string(),
        logical_parameter_bits,
        logical_state_bits,
        active_window_bits: 1 << 20,
        tokenizer: UniversalTokenizerSeed::canonical(),
    }
}

pub fn materialize_window(plan: &SeedMaterializationPlan, start: u128, bits: u64) -> CitadelState {
    let available = plan.logical_parameter_bits.saturating_sub(start);
    let resident = available.min(bits as u128) as u64;
    let mut h = std::collections::hash_map::DefaultHasher::new();
    plan.seed_id.hash(&mut h);
    start.hash(&mut h);
    resident.hash(&mut h);
    plan.tokenizer.version.hash(&mut h);
    CitadelState {
        seed_id: plan.seed_id.clone(),
        logical_complete: start.saturating_add(resident as u128) <= plan.logical_parameter_bits,
        materialized_window_start: start,
        materialized_window_bits: resident,
        parameter_digest: h.finish(),
        tokenizer_version: plan.tokenizer.version.clone(),
    }
}

fn stable_id(value: &str) -> u64 {
    let mut h = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut h);
    h.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed_builds_logically_complete_citadel_without_full_residency() {
        let plan = build_seed_plan("demo-citadel", 1 << 40, 1 << 24);
        let state = materialize_window(&plan, 0, plan.active_window_bits);
        assert!(state.logical_complete);
        assert_eq!(state.materialized_window_bits, plan.active_window_bits);
        assert!((state.materialized_window_bits as u128) < plan.logical_parameter_bits);
    }

    #[test]
    fn universal_token_is_shared_across_all_sections() {
        let tokenizer = UniversalTokenizerSeed::canonical();
        let token = tokenizer.synchronize("willow");
        assert_eq!(token.projections.len(), tokenizer.sections.len());
        assert!(token.universal_id != 0);
    }
}
