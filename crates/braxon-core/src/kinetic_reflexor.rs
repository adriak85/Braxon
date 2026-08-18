use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const KINETIC_REFLEXOR_SCHEMA: &str = "braxon.nsq.kinetic_reflexor.v1";
pub const WATERMARK_FAMILY: &str = "BRAXON_NSQ_BASE8_LEVER_SCALE_GE220000_PROVEN225370_V1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReflexorPhase {
    Publish,
    Reconcile,
    DeltaCommit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValueClass {
    Parameter,
    Weight,
    KvCache,
    Fact,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BusValue {
    pub key: String,
    pub class: ValueClass,
    pub value_hash: String,
    pub byte_len: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Watermark {
    pub family: String,
    pub generation: u64,
    pub phase: ReflexorPhase,
    pub state_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValueDelta {
    pub key: String,
    pub class: ValueClass,
    pub previous_hash: Option<String>,
    pub next_hash: String,
    pub byte_len: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardwareWriteAck {
    pub adapter_id: String,
    pub generation: u64,
    pub accepted: bool,
    pub written_keys: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReflexorReport {
    pub schema: String,
    pub phase: ReflexorPhase,
    pub generation: u64,
    pub watermark: Watermark,
    pub bus_values: usize,
    pub reconciled_values: usize,
    pub delta_values: usize,
    pub hardware_write_acknowledged: bool,
    pub reason: String,
}

#[derive(Debug, Default)]
pub struct KineticReflexor {
    phase: ReflexorPhase,
    generation: u64,
    bus: BTreeMap<String, BusValue>,
    reconciled: BTreeMap<String, BusValue>,
    local_hardware: BTreeMap<String, BusValue>,
    pending_delta: Vec<ValueDelta>,
    watermark: Watermark,
}

impl Default for ReflexorPhase {
    fn default() -> Self {
        Self::Publish
    }
}

impl Default for Watermark {
    fn default() -> Self {
        Self {
            family: WATERMARK_FAMILY.to_string(),
            generation: 0,
            phase: ReflexorPhase::Publish,
            state_hash: stable_hash(&[]),
        }
    }
}

impl KineticReflexor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn publish(
        &mut self,
        values: impl IntoIterator<Item = BusValue>,
    ) -> Result<ReflexorReport, String> {
        self.require_phase(ReflexorPhase::Publish)?;
        let mut next = BTreeMap::new();
        for value in values {
            if value.key.trim().is_empty()
                || value.value_hash.trim().is_empty()
                || value.byte_len == 0
            {
                return Err("bus values require key, hash, and nonzero byte length".to_string());
            }
            if next.insert(value.key.clone(), value).is_some() {
                return Err("duplicate bus value key".to_string());
            }
        }
        if next.is_empty() {
            return Err("publish requires at least one bus value".to_string());
        }
        self.bus = next;
        self.generation = self.generation.saturating_add(1);
        self.phase = ReflexorPhase::Reconcile;
        self.watermark = self.make_watermark(ReflexorPhase::Reconcile, map_hash(&self.bus));
        Ok(self.report(false, "published live values to the NSQ bus"))
    }

    pub fn reconcile(&mut self) -> Result<ReflexorReport, String> {
        self.require_phase(ReflexorPhase::Reconcile)?;
        if self.bus.is_empty() {
            return Err("cannot reconcile an empty bus".to_string());
        }
        self.reconciled = self.bus.clone();
        self.pending_delta = self
            .reconciled
            .values()
            .map(|value| ValueDelta {
                key: value.key.clone(),
                class: value.class,
                previous_hash: self
                    .local_hardware
                    .get(&value.key)
                    .map(|old| old.value_hash.clone()),
                next_hash: value.value_hash.clone(),
                byte_len: value.byte_len,
            })
            .filter(|delta| delta.previous_hash.as_deref() != Some(delta.next_hash.as_str()))
            .collect();
        self.phase = ReflexorPhase::DeltaCommit;
        self.watermark =
            self.make_watermark(ReflexorPhase::DeltaCommit, map_hash(&self.reconciled));
        Ok(self.report(false, "reconciled bus state into the system view"))
    }

    pub fn pending_delta(&self) -> &[ValueDelta] {
        &self.pending_delta
    }

    pub fn commit_hardware(&mut self, ack: HardwareWriteAck) -> Result<ReflexorReport, String> {
        self.require_phase(ReflexorPhase::DeltaCommit)?;
        if ack.adapter_id.trim().is_empty() {
            return Err("hardware adapter identity is required".to_string());
        }
        if ack.generation != self.generation {
            return Err("stale hardware acknowledgement rejected by watermark".to_string());
        }
        let expected: Vec<String> = self
            .pending_delta
            .iter()
            .map(|delta| delta.key.clone())
            .collect();
        if ack.accepted && ack.written_keys != expected {
            return Err("hardware acknowledgement does not match the pending delta".to_string());
        }
        if !ack.accepted {
            return Err(
                "hardware adapter rejected the delta; refresh cycle remains blocked".to_string(),
            );
        }
        for delta in &self.pending_delta {
            if let Some(value) = self.reconciled.get(&delta.key) {
                self.local_hardware.insert(delta.key.clone(), value.clone());
            }
        }
        self.pending_delta.clear();
        self.phase = ReflexorPhase::Publish;
        self.generation = self.generation.saturating_add(1);
        self.watermark =
            self.make_watermark(ReflexorPhase::Publish, map_hash(&self.local_hardware));
        Ok(self.report(
            true,
            "hardware delta acknowledged; committed state is the next refresh baseline",
        ))
    }

    pub fn phase(&self) -> ReflexorPhase {
        self.phase
    }
    pub fn generation(&self) -> u64 {
        self.generation
    }
    pub fn watermark(&self) -> &Watermark {
        &self.watermark
    }

    fn require_phase(&self, expected: ReflexorPhase) -> Result<(), String> {
        if self.phase == expected {
            Ok(())
        } else {
            Err(format!(
                "reflexor phase mismatch: expected {expected:?}, actual {:?}",
                self.phase
            ))
        }
    }
    fn make_watermark(&self, phase: ReflexorPhase, state_hash: String) -> Watermark {
        Watermark {
            family: WATERMARK_FAMILY.to_string(),
            generation: self.generation,
            phase,
            state_hash,
        }
    }
    fn report(&self, acknowledged: bool, reason: &str) -> ReflexorReport {
        ReflexorReport {
            schema: KINETIC_REFLEXOR_SCHEMA.to_string(),
            phase: self.phase,
            generation: self.generation,
            watermark: self.watermark.clone(),
            bus_values: self.bus.len(),
            reconciled_values: self.reconciled.len(),
            delta_values: self.pending_delta.len(),
            hardware_write_acknowledged: acknowledged,
            reason: reason.to_string(),
        }
    }
}

fn map_hash(values: &BTreeMap<String, BusValue>) -> String {
    stable_hash(
        &values
            .values()
            .flat_map(|value| [value.key.as_str(), value.value_hash.as_str()])
            .collect::<Vec<_>>(),
    )
}
fn stable_hash(parts: &[&str]) -> String {
    let mut acc = 0xcbf29ce484222325_u128;
    for part in parts {
        for byte in part.as_bytes() {
            acc ^= *byte as u128;
            acc = acc.wrapping_mul(0x100000001b3);
        }
    }
    format!("{acc:032x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn value(key: &str, hash: &str, class: ValueClass) -> BusValue {
        BusValue {
            key: key.to_string(),
            class,
            value_hash: hash.to_string(),
            byte_len: 64,
        }
    }

    #[test]
    fn three_phase_cycle_commits_only_changed_values_and_refreshes() {
        let mut reflexor = KineticReflexor::new();
        reflexor
            .publish([
                value("layer.0", "a", ValueClass::Parameter),
                value("kv.0", "k1", ValueClass::KvCache),
            ])
            .unwrap();
        reflexor.reconcile().unwrap();
        assert_eq!(reflexor.pending_delta().len(), 2);
        let ack = HardwareWriteAck {
            adapter_id: "approved-test-adapter".to_string(),
            generation: reflexor.generation(),
            accepted: true,
            written_keys: vec!["kv.0".to_string(), "layer.0".to_string()],
        };
        reflexor.commit_hardware(ack).unwrap();
        assert_eq!(reflexor.phase(), ReflexorPhase::Publish);
        reflexor
            .publish([
                value("layer.0", "a", ValueClass::Parameter),
                value("kv.0", "k2", ValueClass::KvCache),
            ])
            .unwrap();
        reflexor.reconcile().unwrap();
        assert_eq!(reflexor.pending_delta().len(), 1);
        assert_eq!(reflexor.pending_delta()[0].key, "kv.0");
    }

    #[test]
    fn stale_or_unacknowledged_hardware_writes_fail_closed() {
        let mut reflexor = KineticReflexor::new();
        reflexor
            .publish([value("fact.0", "f1", ValueClass::Fact)])
            .unwrap();
        reflexor.reconcile().unwrap();
        let stale = HardwareWriteAck {
            adapter_id: "adapter".to_string(),
            generation: 0,
            accepted: true,
            written_keys: vec!["fact.0".to_string()],
        };
        assert!(reflexor.commit_hardware(stale).is_err());
        let rejected = HardwareWriteAck {
            adapter_id: "adapter".to_string(),
            generation: reflexor.generation(),
            accepted: false,
            written_keys: vec![],
        };
        assert!(reflexor.commit_hardware(rejected).is_err());
        assert_eq!(reflexor.phase(), ReflexorPhase::DeltaCommit);
    }

    #[test]
    fn duplicate_publish_keys_are_rejected() {
        let mut reflexor = KineticReflexor::new();
        assert!(reflexor
            .publish([
                value("x", "a", ValueClass::Weight),
                value("x", "b", ValueClass::Weight)
            ])
            .is_err());
    }
}
