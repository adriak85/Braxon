use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const NSQ_NATIVE_INTENT_SCHEMA: &str = "braxon.nsq_native.intent.v1";
pub const NSQ_NATIVE_BUS_SCHEMA: &str = "braxon.nsq_native.bus.v1";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NsqIntent {
    pub schema: String,
    pub intent_id: String,
    pub source_surface: String,
    pub capability: String,
    pub gradient: [f64; 8],
    pub target_addresses: Vec<String>,
    pub provenance: String,
    pub narrative: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntentOutcome {
    Accepted,
    Queued,
    Deferred,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NsqIntentDecision {
    pub intent_id: String,
    pub outcome: IntentOutcome,
    pub reason: String,
    pub owner_surface: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PistonPhase {
    Acquire,
    Hold,
    Commit,
    Release,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddressLease {
    pub address: String,
    pub owner_surface: String,
    pub intent_id: String,
    pub phase: PistonPhase,
    pub generation: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CouncilSurface {
    pub surface_id: String,
    pub role: String,
    pub address_prefix: String,
    pub active: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DaydreamWorkload {
    pub workload_id: String,
    pub source: String,
    pub interruptible: bool,
    pub max_steps: u32,
    pub yielded_to_system_intent: bool,
}

#[derive(Debug, Default)]
pub struct NsqNativeBus {
    leases: BTreeMap<String, AddressLease>,
    generation: u64,
    surfaces: BTreeMap<String, CouncilSurface>,
}

impl NsqIntent {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != NSQ_NATIVE_INTENT_SCHEMA {
            return Err("NSQ intent schema mismatch".to_string());
        }
        if self.intent_id.trim().is_empty() || self.capability.trim().is_empty() {
            return Err("NSQ intent identity and capability are required".to_string());
        }
        if self.gradient.iter().any(|value| !value.is_finite()) {
            return Err("NSQ intent gradient contains a non-finite value".to_string());
        }
        if self.target_addresses.is_empty() {
            return Err("NSQ intent must name at least one target address".to_string());
        }
        if self.narrative && self.provenance != "wowas_narrative" {
            return Err("narrative intent must identify wowas_narrative provenance".to_string());
        }
        Ok(())
    }
}

impl NsqNativeBus {
    pub fn new(council: impl IntoIterator<Item = CouncilSurface>) -> Result<Self, String> {
        let mut bus = Self::default();
        for surface in council {
            if bus.surfaces.insert(surface.surface_id.clone(), surface).is_some() {
                return Err("duplicate council surface".to_string());
            }
        }
        if bus.surfaces.len() != 10 {
            return Err("NSQ council must contain exactly ten surfaces".to_string());
        }
        Ok(bus)
    }

    pub fn council(&self) -> impl Iterator<Item = &CouncilSurface> {
        self.surfaces.values()
    }

    pub fn decide(&mut self, intent: &NsqIntent) -> NsqIntentDecision {
        if let Err(reason) = intent.validate() {
            return NsqIntentDecision { intent_id: intent.intent_id.clone(), outcome: IntentOutcome::Rejected, reason, owner_surface: None };
        }
        if intent.narrative && intent.capability.starts_with("system.") {
            return NsqIntentDecision { intent_id: intent.intent_id.clone(), outcome: IntentOutcome::Rejected, reason: "narrative intent cannot mutate system capability".to_string(), owner_surface: None };
        }
        let owner = self.surfaces.values().find(|surface| {
            surface.active && intent.target_addresses.iter().all(|address| address.starts_with(&surface.address_prefix))
        });
        let Some(owner) = owner else {
            return NsqIntentDecision { intent_id: intent.intent_id.clone(), outcome: IntentOutcome::Deferred, reason: "no active council surface owns the target address".to_string(), owner_surface: None };
        };
        if intent.target_addresses.iter().any(|address| self.leases.contains_key(address)) {
            return NsqIntentDecision { intent_id: intent.intent_id.clone(), outcome: IntentOutcome::Queued, reason: "target address is held by another piston lease".to_string(), owner_surface: Some(owner.surface_id.clone()) };
        }
        self.generation += 1;
        for address in &intent.target_addresses {
            self.leases.insert(address.clone(), AddressLease { address: address.clone(), owner_surface: owner.surface_id.clone(), intent_id: intent.intent_id.clone(), phase: PistonPhase::Acquire, generation: self.generation });
        }
        NsqIntentDecision { intent_id: intent.intent_id.clone(), outcome: IntentOutcome::Accepted, reason: "NSQ address lease acquired".to_string(), owner_surface: Some(owner.surface_id.clone()) }
    }

    pub fn advance_piston(&mut self, intent_id: &str, phase: PistonPhase) -> Result<(), String> {
        let mut found = false;
        for lease in self.leases.values_mut().filter(|lease| lease.intent_id == intent_id) {
            lease.phase = phase.clone();
            found = true;
        }
        if !found { return Err("intent has no active piston lease".to_string()); }
        if phase == PistonPhase::Release { self.leases.retain(|_, lease| lease.intent_id != intent_id); }
        Ok(())
    }

    pub fn active_addresses(&self) -> BTreeSet<String> {
        self.leases.keys().cloned().collect()
    }

    pub fn activate(&mut self, surface_id: &str) -> Result<(), String> {
        let surface = self.surfaces.get_mut(surface_id).ok_or_else(|| "unknown council surface".to_string())?;
        surface.active = true;
        Ok(())
    }

    pub fn daydream(&self, workload: DaydreamWorkload, system_intent_pending: bool) -> Result<DaydreamWorkload, String> {
        if workload.source != "wowas_narrative" || !workload.interruptible || workload.max_steps == 0 {
            return Err("daydream workload must be bounded, interruptible, and narrative-sourced".to_string());
        }
        let mut result = workload;
        result.yielded_to_system_intent = system_intent_pending;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn council() -> Vec<CouncilSurface> {
        (0..10).map(|index| CouncilSurface { surface_id: format!("surface-{index}"), role: if index < 6 { "brain" } else { "sensory" }.to_string(), address_prefix: format!("council/{index}/"), active: index == 0 }).collect()
    }

    fn intent(id: &str, address: &str) -> NsqIntent {
        NsqIntent { schema: NSQ_NATIVE_INTENT_SCHEMA.to_string(), intent_id: id.to_string(), source_surface: "test".to_string(), capability: "system.reconstruct".to_string(), gradient: [0.0; 8], target_addresses: vec![address.to_string()], provenance: "system".to_string(), narrative: false }
    }

    #[test]
    fn piston_prevents_same_space_override() {
        let mut bus = NsqNativeBus::new(council()).unwrap();
        assert_eq!(bus.decide(&intent("a", "council/0/state")).outcome, IntentOutcome::Accepted);
        assert_eq!(bus.decide(&intent("b", "council/0/state")).outcome, IntentOutcome::Queued);
        bus.advance_piston("a", PistonPhase::Release).unwrap();
        assert_eq!(bus.decide(&intent("b", "council/0/state")).outcome, IntentOutcome::Accepted);
    }

    #[test]
    fn narrative_cannot_mutate_system() {
        let mut bus = NsqNativeBus::new(council()).unwrap();
        let mut request = intent("story", "council/0/state");
        request.narrative = true;
        request.provenance = "wowas_narrative".to_string();
        assert_eq!(bus.decide(&request).outcome, IntentOutcome::Rejected);
    }

    #[test]
    fn daydream_yields_to_system_intent() {
        let bus = NsqNativeBus::new(council()).unwrap();
        let workload = DaydreamWorkload { workload_id: "dream-1".to_string(), source: "wowas_narrative".to_string(), interruptible: true, max_steps: 8, yielded_to_system_intent: false };
        assert!(bus.daydream(workload, true).unwrap().yielded_to_system_intent);
    }
}
